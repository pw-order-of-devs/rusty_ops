# `auth_macro` procedural macro

[< Back to modules README](README.md)

## Design Purpose:

`auth_macro` is a procedural macro providing authorization decorator for functions.\
It should be a strictly single purpose library.

## Environment variables:

This crate should not introduce environment variables.

## Dependencies:

This crate should not have any dependencies on other `RustyOps` modules.

## Usage:

This macro includes 2 arguments:
- supported authentication type: `basic`|`bearer`
- list of user permissions to validate: [`resource`:`right`]

### Examples:

```rust
#[auth_macro::authenticate(basic, [])]
fn some_function() {
    // do something ...
    // do something more ...
}
```

```rust
#[auth_macro::authenticate(bearer, [RESOURCE:RIGHT])]
fn some_function() {
    // do something ...
    // do something more ...
}
```
