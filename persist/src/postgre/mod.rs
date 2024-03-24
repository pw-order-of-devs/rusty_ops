use bb8_postgres::bb8::Pool;
use bb8_postgres::tokio_postgres::{types::Type, NoTls, Row};
use bb8_postgres::PostgresConnectionManager;
use serde_json::{Map, Value};
use std::pin::Pin;

use commons::env::{var, var_or_default};
use commons::errors::RustyError;
use domain::filters::search::{SearchOptions, SortOptions};
use domain::RustyDomainItem;

use crate::{Persistence, PersistenceBuilder};

/// Represents a `PostgreSQL` client.
#[derive(Clone, Debug)]
pub struct PostgreSQLClient {
    client: Pool<PostgresConnectionManager<NoTls>>,
    schema: String,
}

impl PostgreSQLClient {
    async fn build_client() -> Self {
        let manager =
            PostgresConnectionManager::new_from_stringlike(Self::get_conn_string(), NoTls)
                .expect("error while parsing postgresql connection string");

        let pool = Pool::builder()
            .max_size(24)
            .build(manager)
            .await
            .expect("error while building postgresql client");

        Self {
            client: pool,
            schema: var_or_default("POSTGRESQL_SCHEMA", "public".to_string()),
        }
    }

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
}

#[allow(clippy::manual_async_fn)]
impl PersistenceBuilder for PostgreSQLClient {
    type PersistentType = Self;

    async fn build() -> Self {
        Self::build_client().await
    }
}

impl Persistence for PostgreSQLClient {
    async fn get_all<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Option<Value>,
        options: Option<SearchOptions>,
    ) -> Result<Vec<T>, RustyError> {
        let conn = self.client.get().await?;

        let values = parse_filter(&filter, true);
        let where_clause = if values.is_empty() {
            String::new()
        } else {
            format!(" where {}", values.join(" and "))
        };
        let options = parse_options(&options);

        let statement = format!(
            "select * from {}.{}{}{}",
            self.schema, index, where_clause, options
        );
        let rows = conn.query(&statement, &[]).await?;
        let mut entries = vec![];
        for row in rows {
            entries.push(serde_json::from_value(parse_row(&row))?);
        }
        Ok(entries)
    }

    async fn get_one<T: RustyDomainItem>(
        &self,
        index: &str,
        filter: Value,
    ) -> Result<Option<T>, RustyError> {
        let conn = self.client.get().await?;
        if filter.as_object().unwrap_or(&Map::new()).is_empty() {
            log::debug!("Expecting a filter - skipping");
            return Ok(None);
        }

        let values = parse_filter(&Some(filter), true);
        let where_clause = if values.is_empty() {
            String::new()
        } else {
            format!(" where {}", values.join(" and "))
        };
        let statement = format!("select * from {}.{}{}", self.schema, index, where_clause);
        let rows = conn.query(&statement, &[]).await?;
        if let Some(row) = rows.first() {
            let value = serde_json::from_value(parse_row(row))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    async fn create<T: RustyDomainItem>(
        &self,
        index: &str,
        item: &T,
    ) -> Result<String, RustyError> {
        let conn = self.client.get().await?;
        let values = parse_filter(&Some(serde_json::to_value(item)?), false).join(", ");
        let statement = format!("insert into {}.{} values ({})", self.schema, index, values);
        let _ = conn.execute(&statement, &[]).await?;
        Ok(item.id())
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
            "update {}.{} set {} where id = '{}'",
            self.schema, index, values, id
        );
        let _ = conn.execute(&statement, &[]).await?;
        Ok(id.to_string())
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

    fn change_stream<'a, T: RustyDomainItem + 'static>(
        &'a self,
        _index: &'a str,
    ) -> Pin<Box<dyn futures_util::Stream<Item = T> + Send + 'a>> {
        todo!()
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
        Value::Null => format!("{key}null"),
        _ => String::new(),
    }
}

fn parse_row(row: &Row) -> Value {
    let mut value = Map::new();
    for column in row.columns() {
        let column_name = column.name().to_string();
        let entry = match column.type_() {
            // add other types
            &Type::VARCHAR | &Type::TEXT => row
                .get::<&str, Option<String>>(&column_name)
                .map_or_else(|| Value::Null, Value::String),
            &Type::INT4 => Value::Number(row.get::<&str, i32>(&column_name).into()),
            &_ => Value::Null,
        };
        value.insert(column_name.clone(), entry);
    }
    Value::Object(value)
}

fn parse_options(options: &Option<SearchOptions>) -> String {
    let options = options.clone().unwrap_or_default();
    let sort_field = options.sort_field.unwrap_or_else(|| "id".to_string());
    let sort_mode = match options.sort_mode.unwrap_or_default() {
        SortOptions::Ascending => "asc".to_string(),
        SortOptions::Descending => "desc".to_string(),
    };
    let page_number = options.page_number.unwrap_or(1);
    let page_number = if page_number == 0 { 1 } else { page_number };
    let page_size = options.page_size.unwrap_or(20);
    let page_size = if page_size == 0 { 20 } else { page_size };
    format!(
        " order by {} {} limit {} offset {}",
        sort_field,
        sort_mode,
        page_size,
        page_size * (page_number - 1),
    )
}
