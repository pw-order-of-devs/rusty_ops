# `persist` library

[< Back to modules README](README.md)

## Design Purpose:

`persist` is a shared library providing the database communication layer for the `RustyOps` system.\
Currently, it supports:
- MongoDB
- PostgreSQL
- Redis [not recommended for production]

In the future, more databases may be supported

## Crate features:

Currently, this crate does not define features.\
In the future, specific databases might be stored behind features, if specific components required it.

## Environment variables:

### Database Type:

- RUSTY_PERSISTENCE:
  - database type used in a given service
  - required
  - supported values:
    - MongoDB: `mongodb`|`mongo_db`|`mongo`
    - PostgreSQL: `postgresql`|`postgre`|`pg`
    - Redis: `redis`

### MongoDB:

- MONGODB_HOST
  - host of mongodb database instance
  - valid only if `RUSTY_PERSISTENCE` is set to `MongoDB` type
  - optional
  - default: `localhost`
- MONGODB_PORT
  - port of mongodb database instance
  - valid only if `RUSTY_PERSISTENCE` is set to `MongoDB` type
  - optional
  - default: `27017`
- MONGODB_DATABASE
  - name of mongodb database
  - valid only if `RUSTY_PERSISTENCE` is set to `MongoDB` type
  - optional
  - default: `test`
- MONGODB_USER
  - mongodb credential: username
  - valid only if `RUSTY_PERSISTENCE` is set to `MongoDB` type
  - required
- MONGODB_PASSWORD
  - mongodb credential: password
  - valid only if `RUSTY_PERSISTENCE` is set to `MongoDB` type
  - required

### PostgreSQL:

- POSTGRESQL_INIT_SCRIPT_PATH:
  - location of sql script initializing postgresql schema
  - valid only if `RUSTY_PERSISTENCE` is set to `PostgreSQL` type
  - optional
  - default: `/app/pg/init.sql`
- POSTGRESQL_HOST
  - host of postgresql database instance
  - valid only if `RUSTY_PERSISTENCE` is set to `PostgreSQL` type
  - optional
  - default: `localhost`
- POSTGRESQL_PORT
  - port of postgresql database instance
  - valid only if `RUSTY_PERSISTENCE` is set to `PostgreSQL` type
  - optional
  - default: `5432`
- POSTGRESQL_DATABASE
  - name of postgresql database
  - valid only if `RUSTY_PERSISTENCE` is set to `PostgreSQL` type
  - optional
  - default: `postgres`
- POSTGRESQL_SCHEMA
  - name of postgresql schema
  - valid only if `RUSTY_PERSISTENCE` is set to `PostgreSQL` type
  - optional
  - default: `public`
- POSTGRESQL_USER
  - postgresql credential: username
  - valid only if `RUSTY_PERSISTENCE` is set to `PostgreSQL` type
  - required
- POSTGRESQL_PASSWORD
  - postgresql credential: password
  - valid only if `RUSTY_PERSISTENCE` is set to `PostgreSQL` type
  - required

### Redis:

- REDIS_HOST
  - host of redis database instance
  - valid only if `RUSTY_PERSISTENCE` is set to `Redis` type
  - optional
  - default: `localhost`
- REDIS_PORT
  - port of redis database instance
  - valid only if `RUSTY_PERSISTENCE` is set to `Redis` type
  - optional
  - default: `6379`
- REDIS_USER
  - redis credential: username
  - valid only if `RUSTY_PERSISTENCE` is set to `Redis` type
  - required
- REDIS_PASSWORD
  - redis credential: password
  - valid only if `RUSTY_PERSISTENCE` is set to `Redis` type
  - required

### Configuration:

- DB_CONNECT_TIMEOUT
  - timeout for database operations in seconds
  - optional
  - default: `30`
- DB_POOL_MAX
  - maximal size of database connection pool
  - optional
  - default: `24`
