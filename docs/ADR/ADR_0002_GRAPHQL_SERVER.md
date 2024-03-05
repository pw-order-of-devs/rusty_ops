# Choose GraphQL Server solution

[< Back to ADR README](README.md)

* Status: Decided
* Deciders: @PW
* Date: 2024-03-01

Technical Story: We need to select a GraphQL Server for our project. The primary decision is between async-graphql and juniper.

## Context and Problem Statement

We need to select a GraphQL Server Library for our Rust based project. The contenders are async-graphql, juniper and potentially others. Which is the most appropriate library for our needs?

## Decision Drivers

* Quality of Documentation
* Ease of Use
* Community Support
* Performance

## Considered Options

* async-graphql
* juniper

## Decision Outcome

Chosen option: "async-graphql", because of the excellent API design, commitment to safety, rich functionalities, and good performance. It is also actively maintained, which indicates good potential for better community support in the future.

### Positive Consequences

* Offers a type-safe and ergonomically designed API.
* Rich features such as supporting different protocols (HTTP, Websocket, etc), including good support for Apollo federation.
* Good performance which is crucial for our server.
* Actively maintained, indicating responsive community support.

### Negative Consequences

* Documentation might not be as comprehensive as the ones for more mature libraries.
* Some learning curve for developers not familiar with async-graphql.
* Not as well-known or popular, which could potentially impact hiring and onboarding new team members.

## Pros and Cons of the Options

### async-graphql

A high-performance server-side library that allows us to create GraphQL servers in a type-safe and idiomatic Rust style.

* Good, because it has a type-safe and ergonomic API design.
* Good, because it includes a wide array of features needed for a modern GraphQL API.
* Good, because it offers great performance for heavy server workloads.
* Good, because it has good potential for community support due to active maintenance.
* Bad, because its documentation might not be as comprehensive.
* Bad, because it may pose a steep learning curve for developers who aren't familiar with it.

### juniper

A GraphQL server library for Rust.

* Good, because it has strong community support.
* Good, because it's well-documented.
* Good, because it is a mature and well-known library in the Rust GraphQL community.
* Good, because it has clear design principles and philosophy.
* Bad, because it currently has limited async/await support.
* Bad, because it may not offer the same level of performance as newer GraphQL libraries.