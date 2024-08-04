# `commons` library

[< Back to modules README](README.md)

## Design Purpose:

`commons` is a shared library providing the basic functionalities for the `RustyOps` system. It contains:
- environment variables wrapper functions
- custom errors 
- hashing/encryption functions
- logging configuration

Future features:
- any features shared between modules that don't depend on domain or persistence should be placed here

## Crate features:

- `errors`: custom error handling
- `logging`: logging configuration
- `persist`: error handling for persistence module
- `messaging`: error handling for messaging module
- `ws`: error handling for websockets

## Environment variables:

- LOG_LEVEL:
    - level of application logging
    - optional
    - default: `info`
    - supported values: `trace`|`debug`|`info`|`warn`|`error`|`off`
- LOG_CONFIG_PATH:
    - location of `log4rs` configuration file
    - optional
    - if not provided, default configuration will be used

## Dependencies:

This crate should not have any dependencies on other `RustyOps` modules.
