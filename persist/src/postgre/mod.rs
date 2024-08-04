use bb8_postgres::bb8::Pool;
use bb8_postgres::tokio_postgres::{types::Type, NoTls, Row};
use bb8_postgres::PostgresConnectionManager;
use serde_json::{json, Map, Value};
use std::time::Duration;

use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use domain::commons::search::{SearchOptions, SortOptions};
use domain::RustyDomainItem;

use crate::shared::filter_results;
use crate::{Persistence, PersistenceBuilder};

/// Represents a `PostgreSQL` client.
#[derive(Clone, Debug)]
pub struct PostgreSQLClient {
    client: Pool<PostgresConnectionManager<NoTls>>,
    schema: String,
}

impl PostgreSQLClient {
    fn get_conn_string() -> String {
        format!(
            "postgres://{}{}:{}/{}",
            Self::get_credential(),
            var_or_default("POSTGRESQL_HOST", "localhost".to_string()),
            var_or_default("POSTGRESQL_PORT", 5432),
            var_or_default("POSTGRESQL_DATABASE", "postgres".to_string()),
        )
    }

    fn get_credential() -> String {
        match (
            var::<String>("POSTGRESQL_USER"),
            var::<String>("POSTGRESQL_PASSWORD"),
        ) {
            (Ok(user), Ok(pass)) => format!("{user}:{pass}@"),
            _ => String::new(),
        }
    }

    /// Executes a SQL statement using the underlying database connection.
    ///
    /// # Arguments
    ///
    /// * `sql` - The SQL statement to be executed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the SQL statement was executed successfully.
    /// Returns `Err(RustyError)` if there was an error executing the SQL statement.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn execute_sql(&self, sql: &str) -> Result<(), RustyError> {
        let conn = self.client.get().await?;
        conn.batch_execute(sql).await?;

        Ok(())
    }

    /// Execute SQL queries from a given directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - A string representing the directory path containing the SQL queries.
    ///
    /// # Returns
    ///
    /// * `Result<(), RustyError>` - A Result indicating success or failure. If successful, it returns `Ok(())`. Otherwise, it returns an `Err` containing a `RustyError` indicating the cause of the failure.
    ///
    /// # Errors
    ///
    /// This function can generate the following errors:
    ///
    /// * `RustyError` - If there was an error during the creation of the item.
    pub async fn execute_sql_dir(&self, base_path: &str) -> Result<(), RustyError> {
        for entry in std::fs::read_dir(base_path)? {
            let name = entry?.file_name();
            let name = name.to_string_lossy();
            let script = std::fs::read_to_string(&format!("{base_path}/{name}"))?;
            self.execute_sql(&script).await?;
        }

        Ok(())
    }
}

#[allow(clippy::manual_async_fn)]
impl PersistenceBuilder for PostgreSQLClient {
    type PersistentType = Self;

    async fn build() -> Self {
        Self::from_string(&Self::get_conn_string()).await
    }

    async fn from_string(conn: &str) -> Self {
        let manager = PostgresConnectionManager::new_from_stringlike(conn, NoTls)
            .expect("error while parsing postgresql connection string");

        let timeout = var_or_default("DB_CONNECT_TIMEOUT", 30);
        let max_pool_size = var_or_default("DB_POOL_MAX", 24);
        let pool = Pool::builder()
            .max_size(max_pool_size)
            .connection_timeout(Duration::from_secs(timeout))
            .build(manager)
            .await
            .expect("error while building postgresql client");

        Self {
            client: pool,
            schema: var_or_default("POSTGRESQL_SCHEMA", "public".to_string()),
        }
    }
}

impl Persistence for PostgreSQLClient {
    async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: &Option<Value>,
        options: &Option<SearchOptions>,
    ) -> Result<Vec<T>, RustyError> {
        let conn = self.client.get().await?;

        let statement = format!(
            "select * from {}.{index}{}",
            self.schema,
            parse_options(options)
        );
        let rows = conn
            .query(&statement, &[])
            .await?
            .iter()
            .map(parse_row)
            .collect::<Vec<Value>>();

        let result = filter_results(filter, &rows)
            .into_iter()
            .map(|value| serde_json::from_value(value))
            .collect::<Result<Vec<T>, _>>()?;
        Ok(result)
    }

    async fn get_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<Option<T>, RustyError> {
        let values: Vec<T> = self.get_all(index, &Some(filter), &None).await?;
        if values.len() == 1 {
            Ok(Some(values[0].clone()))
        } else {
            Ok(None)
        }
    }

    async fn get_list(&self, index: &str, id: &str) -> Result<Vec<String>, RustyError> {
        let conn = self.client.get().await?;
        let statement = format!("select * from {}.{index} where id = $1", self.schema);
        let row = conn.query_one(&statement, &[&id]).await?;
        let entries = parse_row(&row)
            .get("entries")
            .unwrap_or(&Value::Array(vec![]))
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect();
        Ok(entries)
    }

    async fn create<T: RustyDomainItem>(
        &self,
        index: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        let conn = self.client.get().await?;
        let (id, item) = (item.get_id(), serde_json::to_value(item)?);
        let values = parse_filter(&Some(item.clone()), false).join(", ");
        let statement = format!("insert into {}.{index} values ({values})", self.schema);
        let _ = conn.execute(&statement, &[]).await?;
        let _ = messaging::internal::send(
            &json!({ "index": index, "op": "create", "item": item }).to_string(),
        )
        .await;
        Ok(id)
    }

    async fn update<T: RustyDomainItem>(
        &self,
        index: &str,
        id: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        let conn = self.client.get().await?;
        let values = parse_filter(&Some(serde_json::to_value(item)?), true).join(", ");
        let statement = format!(
            "update {}.{index} set {values} where id = '{id}'",
            self.schema
        );
        let _ = conn.execute(&statement, &[]).await?;
        let _ = messaging::internal::send(
            &json!({ "index": index, "op": "update", "item": serde_json::to_string(item)? })
                .to_string(),
        )
        .await;
        Ok(id.to_string())
    }

    async fn append(&self, index: &str, id: &str, entry: &str) -> Result<u64, RustyError> {
        let conn = self.client.get().await?;
        let statement = format!(
            "select exists (select 1 from {}.{index} where id = '{id}')",
            self.schema,
        );
        let entry: Value = serde_json::from_str(&format!("[{entry}]"))?;
        if conn.query_one(&statement, &[]).await?.get(0) {
            let statement = format!(
                "update {}.{index} set entries = entries || $1::jsonb where id = $2",
                self.schema,
            );
            let _ = conn.execute(&statement, &[&entry, &id]).await?;
        } else {
            let statement = format!(
                "insert into {}.{index} (id, entries) values ($1, $2::jsonb)",
                self.schema,
            );
            let _ = conn.execute(&statement, &[&id, &entry]).await?;
        }
        Ok(1)
    }

    async fn delete_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<u64, RustyError> {
        let conn = self.client.get().await?;
        if filter.as_object().unwrap_or(&Map::new()).is_empty() {
            log::debug!("Expecting a filter - skipping");
            return Ok(0);
        }

        let values = parse_filter(&Some(filter), true);
        let where_clause = if values.is_empty() {
            String::new()
        } else {
            format!(" where {}", values.join(" and "))
        };
        let statement = format!("delete from {}.{}{}", self.schema, index, where_clause);
        let deleted = conn.execute(&statement, &[]).await?;
        Ok(deleted)
    }

    async fn delete_all(&self, index: &str) -> Result<u64, RustyError> {
        let conn = self.client.get().await?;
        let statement = format!("delete from {}.{}", self.schema, index);
        let deleted = conn.execute(&statement, &[]).await?;
        Ok(deleted)
    }

    async fn purge(&self) -> Result<(), RustyError> {
        let conn = self.client.get().await?;
        let statement = format!("drop schema if exists {} cascade", self.schema);
        conn.execute(&statement, &[])
            .await
            .map_err(|err| RustyError::PostgresSQLError(err.to_string()))
            .map(|_| ())
    }
}

fn parse_filter(filter: &Option<Value>, is_where: bool) -> Vec<String> {
    filter.clone().map_or_else(Vec::new, |filter| {
        filter
            .as_object()
            .unwrap_or(&Map::new())
            .iter()
            .map(|(key, value)| parse_value(key, value, is_where))
            .collect::<Vec<String>>()
    })
}

fn parse_value(key: &str, value: &Value, is_where: bool) -> String {
    let key = if is_where {
        format!("{key} = ")
    } else {
        String::new()
    };
    match value {
        Value::Bool(v) => format!("{key}{v}"),
        Value::Number(v) => format!("{key}{v}"),
        Value::String(v) => format!("{key}'{v}'"),
        Value::Array(v) => format!(
            "{key}ARRAY[{}]::varchar(36)[]",
            v.iter()
                .map(|i| parse_value("", i, false))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Value::Null => format!("{key}null"),
        Value::Object(_) => String::new(),
    }
}

fn parse_row(row: &Row) -> Value {
    let mut value = Map::new();
    for column in row.columns() {
        let column_name = column.name().to_string();
        let entry = match column.type_() {
            // add other types
            &Type::INT4 => Value::Number(row.get::<&str, i32>(&column_name).into()),
            &Type::VARCHAR | &Type::TEXT => row
                .get::<&str, Option<String>>(&column_name)
                .map_or_else(|| Value::Null, Value::String),
            &Type::VARCHAR_ARRAY | &Type::TEXT_ARRAY => row
                .get::<&str, Option<Vec<String>>>(&column_name)
                .map_or_else(
                    || Value::Null,
                    |value_vec| Value::Array(value_vec.into_iter().map(Value::String).collect()),
                ),
            &Type::JSONB => row
                .get::<&str, Option<Value>>(&column_name)
                .unwrap_or(Value::Null),
            &_ => Value::Null,
        };
        value.insert(column_name.clone(), entry);
    }
    Value::Object(value)
}

fn parse_options(options: &Option<SearchOptions>) -> String {
    if options.is_none() {
        return String::new();
    }

    let options = options.clone().unwrap_or_default();
    let sort_field = options.sort_field.unwrap_or_else(|| "id".to_string());
    let sort_mode = match options.sort_mode.unwrap_or_default() {
        SortOptions::Ascending => "asc".to_string(),
        SortOptions::Descending => "desc".to_string(),
    };

    format!(" order by {sort_field} {sort_mode}")
}
