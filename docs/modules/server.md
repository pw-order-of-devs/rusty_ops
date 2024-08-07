# `rusty_server` module

[< Back to modules README](README.md)

## Design Purpose:

`rusty_server` is a main backend server application for `RustyOps`.\
It exposes `graphql` and `websocket` endpoints for data access for agents and web ui.

It contains scheduler-based functionalities:
- clean up expired agents
- reassign expired pipelines

## Environment variables:

The application is configured via environment variables:

### `rusty_server` server configuration:

- SERVER_ADDR:
  - ip address on which server should be exposed
  - optional
  - default: `0.0.0.0`
- SERVER_PORT:
  - port on which server should be exposed
  - optional
  - default: `8000`

### Internal configuration:

- PUBLIC_USER_REGISTER_ENABLED:
  - feature flag for enabling registration of users
  - optional
  - default: `false`
  - boolean
  - if true, `user registration` endpoint will not be secured
  - if false, only system administrator would be able to create new user accounts
- SCHEDULER_AGENTS_TTL:
  - period between ticks for cleaning up expired agents (in seconds)
  - optional
  - default: `60`
- SCHEDULER_PIPELINES_CLEANUP:
  - period between ticks for reassigning unfinished pipelines (in seconds)
  - optional
  - default: `60`

### Agent configuration:

- AGENT_TTL:
  - expiration time for agent registration (in seconds)
  - optional
  - default: `300`
- AGENTS_REGISTERED_MAX:
  - maximum amount of agents to be registered at once
  - optional
  - default: `24`
- AGENT_MAX_ASSIGNED_JOBS:
  - maximum amount of jobs that can be assigned to an agent at once
  - optional
  - default: `1`

For complete configuration, refer to application dependencies environment variables.

## Dependencies:

- [auth](auth.md)
- [auth_macro](auth_macro.md)
- [commons](commons.md)
- [domain](domain.md)
- [messaging](messaging.md)
- [persist](persist.md)

## Example configuration:

docker compose:
- mongodb:

```yaml
rustyServer:
  image: rusty-server
  ports:
    - "8000:8000"
  restart: no
  environment:
    - LOG_LEVEL=debug
    - MONGODB_DATABASE=rusty
    - MONGODB_HOST=192.168.0.2
    - MONGODB_PASSWORD=password
    - MONGODB_PORT=5454
    - MONGODB_USER=admin
    - RUSTY_PERSISTENCE=mongodb
    - SERVER_ADDR=0.0.0.0
    - SERVER_PORT=8000
  networks:
    - backend
```

- postgresql:

```yaml
rustyServer:
  image: rusty-server
  ports:
    - "8000:8000"
  restart: no
  environment:
    - LOG_LEVEL=debug
    - POSTGRESQL_DATABASE=rusty
    - POSTGRESQL_HOST=192.168.0.2
    - POSTGRESQL_PASSWORD=password
    - POSTGRESQL_PORT=5454
    - POSTGRESQL_SCHEMA=rusty
    - POSTGRESQL_USER=admin
    - RUSTY_PERSISTENCE=postgresql
    - SERVER_ADDR=0.0.0.0
    - SERVER_PORT=8000
  networks:
    - backend
```

- redis:

```yaml
rustyServer:
  image: rusty-server
  ports:
    - "8000:8000"
  restart: no
  environment:
    - LOG_LEVEL=debug
    - REDIS_HOST=redis
    - REDIS_PORT=6379
    - REDIS_PASSWORD=password
    - RUSTY_PERSISTENCE=redis
    - SERVER_ADDR=0.0.0.0
    - SERVER_PORT=8000
  networks:
    - backend
```
