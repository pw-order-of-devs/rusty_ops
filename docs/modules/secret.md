# `secret` library

[< Back to modules README](README.md)

## Design Purpose:

`secret` is a shared library providing the vault communication layer for the `RustyOps` system.\
Currently, it provides support for:
- Hashicorp Vault

In the future, more vaults may be supported

## Crate features:

Currently, this crate does not define features.\
In the future, specific databases might be guarded by features, if specific components required it.

## Environment variables:

### Hashicorp Vault:

- VAULT_PROTOCOL
  - protocol for vault api
  - optional
  - default: `https`
- VAULT_HOST
  - host of vault instance
  - optional
  - default: `localhost`
- VAULT_PORT
  - port of vault instance
  - optional
  - default: `8200`
- VAULT_TOKEN
  - token for vault api auth
  - required
