# Choose backend web framework

[< Back to ADR README](README.md)

* Status: Decided
* Deciders: @PW
* Date: 2024-03-01

Technical Story: We are to determine the most suitable Rust web framework for serving a GraphQL endpoint in our project.

## Context and Problem Statement

We need to select an efficient and suitable Rust web server framework for hosting a GraphQL endpoint from Poem, Axum, Actix, Hyper, Warp, and Rocket. What is the most appropriate framework for our needs?

## Decision Drivers

* Efficiency and performance of the framework
* Support for GraphQL
* Ease of use and learning curve for the team
* Active development and community support

## Considered Options

* Axum
* Actix
* Hyper
* Warp
* Rocket
* Poem

## Decision Outcome

Chosen option: "Poem", because it provides intuitive and macro-based API, highly efficient and has nice performance. Its simplicity and shallow learning curve make it a good fit for our project.

### Positive Consequences

* Easy to use with a clean and intuitive macro-based API
* High performance and efficiency
* Shallow learning curve for the team

### Negative Consequences

* Relatively new compared to other choices, so it might not have extensive community support
* Lack of some advanced features present in more mature frameworks

## Pros and Cons of the Options

### Poem

A flexible and efficient web framework with a clean and straightforward API.

* Good, because it's easy to use with its macro-based API
* Good, because it's highly efficient with nice performance
* Good, because it has a minimalistic design favoring convention over configuration
* Good, because it supports GraphQL routing out of the box 
* Bad, because it's still new and might have less community support than older frameworks
* Bad, because lack of some more advanced features present in more mature frameworks

### Axum

A web application framework that focuses on modularity and type-safe routing.

* Good, because it ensures type safety
* Good, because it's modular and promotes separation of concerns
* Good, because it has solid community support as it's created by the authors of Tokio
* Bad, because it has a steeper learning curve due to its focus on type-level programming
* Bad, because it may require more setup and configuration than other options

### Actix

A powerful, pragmatic, and extremely fast web framework in Rust.

* Good, because it's very fast and efficient
* Good, because it's robust and well-documented
* Good, because it has built-in support for websockets and streaming responses
* Bad, because it may be an overkill for a simple GraphQL endpoint
* Bad, because its extensive feature set may increase complexity

### Hyper

A low-level HTTP library with focus on speed.

* Good, because it's highly efficient with nice performance
* Good, because it gives you control over lower-level details
* Good, because it is flexible and doesn't force any particular architectural style
* Bad, because it requires more boilerplate and setup
* Bad, because it may not be suitable for beginners due to its low-level nature

### Warp

A web framework focused on type safety, correctness, and performance.

* Good, because it's highly efficient and well-optimized
* Good, because it has a nice set of features and built-in conveniences
* Good, because it offers good safety guarantees due to its focus on type safety
* Bad, because it has a steep learning curve and might be more complex to set up
* Bad, because errors can often be hard to trace due to heavy use of macros

### Rocket

A web framework with focus on usability, security, extensibility, and speed.

* Good, because it's easy to use and has a clean and intuitive API
* Good, because it's secure and puts an emphasis on typing and correctness
* Good, because it has an active and supportive community
* Bad, because rocket_contrib currently requires nightly Rust
* Bad, because it's less flexible compared to other options due to its prescribed conventions
