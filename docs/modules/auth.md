# `auth` library

[< Back to modules README](README.md)

## Design Purpose:

`auth` is a library providing authorization feature for the `RustyOps` system.\
It's purpose is to handle authorization of user using `Basic` and `Bearer` authentication modes.

[possible] Future features:
- API key authentication for agents or external applications
- Integration with external OpenID providers like keycloak

## Environment variables:

This crate should not introduce environment variables.

## Dependencies:

- [commons](commons.md)
- [domain](domain.md)
- [persist](persist.md)
