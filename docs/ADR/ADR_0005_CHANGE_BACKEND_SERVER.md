# Choose replacement for backend web framework

[< Back to ADR README](README.md)

* Status: Decided
* Deciders: @PW
* Date: 2024-04-01

Technical Story: We are to determine the replacement for previously chosen Poem server, which is causing problems after upgrade to 3.0.x.

## Context and Problem Statement

We need to select an efficient and suitable replacement for Poem server. It is supposed to be able to serve GraphQL - integrating with async-graphql lib.\
Description of options is placed in #0003 ADR, so it won't be repeated in this document.

## Decision Drivers

* Efficiency and performance of the framework
* Support for GraphQL
* Ease of use and learning curve for the team
* Active development and community support

## Considered Options

* Axum
* Actix
* Warp
* Rocket

## Decision Outcome

Chosen option: "Axum", because of its modularity, type-safe routing and being backed by Tokio authors.

## Links

* [0003 - Backend server](ADR_0003_BACKEND_SERVER.md)
