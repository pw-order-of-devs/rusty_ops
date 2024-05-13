# `rusty_init` module

[< Back to modules README](README.md)

## Design Purpose:

`rusty_init` is designed to initialize the `RustyOps` system. Its main functions involve:
- establishing database schemas
- creating required data like default users, roles, permissions
- initialization of components

Future features:
- upgrade system
- migrate between databases

## Environment variables:

The application is configured via environment variables:

- WIPE_DATA:
  - if set to true, all content of database is deleted before initialization
  - optional
  - default: `false`
  - boolean

For complete configuration, refer to application dependencies environment variables.

## Dependencies:

- [commons](commons.md)
- [domain](domain.md)
- [persist](persist.md)

## Example configuration:

docker compose:
```yaml
rustyServer:
  image: rusty-server
  ports:
    - "8000:8000"
  restart: no
  environment:
    - LOG_LEVEL=debug
    - WIPE_DATA=true
    - RUSTY_PERSISTENCE=mongodb
    - MONGODB_HOST=mongo
    - MONGODB_PORT=27017
    - MONGODB_USER=admin
    - MONGODB_PASSWORD=password
    - MONGODB_DATABASE=rusty
  networks:
    - backend
```
