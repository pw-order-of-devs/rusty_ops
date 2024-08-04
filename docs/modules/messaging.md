# `messaging` library

[< Back to modules README](README.md)

## Design Purpose:

`messaging` is a shared library providing the message-queue/pub-sub communication layer for the `RustyOps` system.\
Currently, it provides support for:
- Internal - tokio based pub/sub for communication inside a service,

In the future, more services may be supported

## Crate features:

- `internal`: pub/sub channel for usage inside a service
- `external`: support for external services: [`rabbitmq`]

## Environment variables:

### Messaging Type:

- RUSTY_MESSAGING:
  - messaging service type used
  - required
  - supported values:
    - RabbitMQ: `rabbitmq`|`rabbit`

### RabbitMQ:

- RABBITMQ_HOST
  - host of RabbitMQ instance
  - valid only if `RUSTY_MESSAGING` is set to `RabbitMQ` type
  - optional
  - default: `localhost`
- RABBITMQ_PORT
  - port of RabbitMQ instance
  - valid only if `RUSTY_MESSAGING` is set to `RabbitMQ` type
  - optional
  - default: `5672`
- RABBITMQ_USER
  - RabbitMQ credential: username
  - valid only if `RUSTY_MESSAGING` is set to `RabbitMQ` type
  - required
- RABBITMQ_PASSWORD
  - RabbitMQ credential: password
  - valid only if `RUSTY_MESSAGING` is set to `RabbitMQ` type
  - required

### Configuration:

- MQ_CONNECT_TIMEOUT
  - timeout for messaging connection in seconds
  - optional
  - default: `30`
- MQ_POOL_MAX
  - maximal size of messaging connection pool
  - optional
  - default: `24`
