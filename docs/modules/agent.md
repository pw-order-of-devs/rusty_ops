# `rusty_agent` module

[< Back to modules README](README.md)

## Design Purpose:

`rusty_agent` is a service responsible for executing pipelines. It parses the pipeline template and runs each step.

It contains scheduler-based functionalities:
- healthcheck - update agent's ttl
- fetch unassigned pipelines for assignment
- fetch a pipeline assigned to given agent for execution

and websocket subscriptions [optional]:
- listen for registration of a pipeline

Future features:
- support execution in docker
- support more pipeline template features
- stream logs for live status update

## Environment variables:

The application is configured via environment variables:

### `rusty_agent` server configuration:

- AGENT_ADDR:
  - ip address on which agent should be exposed 
  - optional
  - default: `0.0.0.0`
- AGENT_PORT:
  - port on which agent should be exposed
  - optional
  - default: `8800`

### `rusty_agent` instance credentials:

- AGENT_USER:
  - agent credential: username
  - required
- AGENT_PASSWORD:
  - agent credential: password
  - required

### `rusty_server` server configuration:

- SERVER_PROTOCOL:
  - protocol of `rusty_server` service connection
  - optional
  - default: `https`
  - supported values: `http`|`https`
- SERVER_HOST:
  - address of `rusty_server` service
  - optional
  - default: `localhost`
- SERVER_PORT:
  - port of `rusty_server` service
  - optional
  - default: `8000`

### Internal configuration:

- SUBSCRIPTION_ENABLED:
  - feature flag for websocket subscriptions
  - optional
  - default: `true`
  - boolean
- SCHEDULER_GET_UNASSIGNED:
  - period between ticks for fetching unassigned pipelines scheduler (in seconds)
  - optional
  - default: `300`
- SCHEDULER_GET_ASSIGNED:
  - period between ticks for fetching assigned pipelines scheduler (in seconds)
  - optional
  - default: `300`
- SCHEDULER_HEALTHCHECK:
  - period between ticks for health notice to server (in seconds)
  - optional
  - default: `180`
  - should be smaller than `AGENT_TTL`

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
