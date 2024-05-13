# `rusty_server` module

[< Back to modules README](README.md)

## Design Purpose:

`rusty_agent` is a main backend server application for `RustyOps`.\
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
    - AGENT_HOST=localhost
    - AGENT_PORT=7000
    - AGENT_USER=agent
    - AGENT_PASSWORD=password
    - SERVER_HOST=password
    - SERVER_PORT=password
    - SERVER_PROTOCOL=http
    - SUBSCRIPTION_ENABLED=true
    - SCHEDULER_GET_ASSIGNED=300
    - SCHEDULER_GET_UNASSIGNED=300
    - SCHEDULER_HEALTHCHECK=120
  networks:
    - backend
```
