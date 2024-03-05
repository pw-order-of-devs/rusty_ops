# Choose API architecture

[< Back to ADR README](README.md)

* Status: Decided
* Deciders: @PW
* Date: 2024-03-01

Technical Story: Our project requires an API architecture. The data nature needs have us deciding among GraphQL, REST, and gRPC.

## Context and Problem Statement

We need an API architecture that provides flexibility, real-time capabilities, and ease of use for API consumers. Should we adopt GraphQL, REST, or gRPC?

## Decision Drivers

* The need for flexibility on the client side to specify exact data needs
* Significant reduction of data transfer
* Comprehensive response with a single request
* Migration and learning curve for the team

## Considered Options

* REST
* gRPC
* GraphQL

## Decision Outcome

Chosen option: "GraphQL", because it meets the main drivers of decision—it provides flexibility, enables data aggregation from multiple sources and reduces the number of requests to gather Full data.

### Positive Consequences

* Less data transfer as clients fetch exactly what they need
* Reduced number of requests due to getting multiple resources in a single request
* Aggregation of responses from multiple sources

### Negative Consequences

* Learning curve for developers not familiar with GraphQL
* Higher complexity in implementation compared to REST APIs
* A possible increase in server resource usage

## Pros and Cons of the Options

### GraphQL

Great choice for applications with complex requirements and contexts.

* Good, because it provides flexibility in querying
* Good, because it reduces the amount of data transferred by allowing clients to specify exactly what they need
* Good, because it enables the aggregation of responses from multiple sources.
* Good, because binding between the client and server is loose, making the API evolve easily over time.
* Bad, because there's a learning curve for the team
* Bad, because it is more complex to implement than REST
* Bad, because the queries could be complex and can increase server resource usage.

### REST

Best for simple applications.

* Good, because it's widely used and understood which leads to faster onboarding
* Good, because it has broad language and platform support
* Good, because it follows a standardized stateless, cacheable communication protocol.
* Good, because it uses HTTP methods which are easily understandable and interpretable.
* Bad, because it leads to Over-fetching or Under-fetching of data
* Bad, because flexible querying is not naturally supported leading to multiple endpoints.
* Bad, because it cannot aggregate responses from multiping endpoints in one request.

### gRPC

Suitable for systems requiring high efficiency and for inter-service communication in a microservices setup.

* Good, because it supports streaming, ideal for real-time applications
* Good, because it supports multiple programming languages which is good for polyglot projects.
* Good, because it has high performance and low latency.
* Good, because it supports automatic generation of client libraries.
* Bad, because it has a learning curve, especially because of protobuf.
* Bad, because debugging and tracing are difficult due to binary protocol.
* Bad, because it requires HTTP/2 for transport which might not be supported everywhere.