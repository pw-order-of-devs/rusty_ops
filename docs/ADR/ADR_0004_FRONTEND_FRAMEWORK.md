# Choice of Frontend Framework

[< Back to ADR README](README.md)

* Status: Decided
* Deciders: @PW
* Date: 2024-03-04

Technical Story: Our project requires a frontend framework that will work with the Rust language to allow shared modules between backend and frontend.

## Context and Problem Statement

We need to decide on a frontend framework for our project. The key considerations are the ability to use Rust, smooth integration with our backend, robustness, and ease of use. The main options available to us are Leptos, Yew, Dioxus, Flutter, Vue, and React. Which is the most appropriate framework?

## Decision Drivers

* Compatibility with Rust language
* Community support and documentation
* Learning curve and ease of use
* Efficiency and speed
* Flexibility and versatility of the framework

## Considered Options

* Leptos
* Yew
* Dioxus
* Flutter
* Vue.js
* React.js

## Decision Outcome

Chosen option: "Leptos", because it is built with Rust and allows us to use Rust to write frontend code, facilitating shared modules between backend and frontend, and bringing the robustness of Rust to the frontend.

### Positive Consequences

* Seamless integration between the frontend and backend due to shared language
* High efficiency and speed from using Rust
* Flexibility with web design

### Negative Consequences

* Very new, therefore might lack extensive community support and resources
* May have a steep learning curve for those unfamiliar with Rust

## Pros and Cons of the Options

### Leptos

A frontend framework written in Rust.

* Good, because it allows for shared Rust code between frontend and backend
* Good, because it uses Rust's strong type system to prevent bugs
* Good, because it employs Rust's performance advantages in the frontend
* Bad, because the community around it is not as large or established
* Bad, because its young age may mean more frequent updates and changes
* Bad, because its documentation might not be as comprehensive

### Yew

A modern Rust framework for creating multi-threaded frontend apps with WebAssembly.

* Good, because it's one of the most popular Rust frameworks for the frontend
* Good, because it allows for shared Rust code between frontend and backend
* Good, because of its support for multithreading which can improve performance
* Bad, because setup can be complex, especially for beginners
* Bad, because it needs a specific environment for compiling to WebAssembly
* Bad, because it might have a steeper learning curve due to use of advanced Rust features

### Dioxus

A Rust framework designed for WebAssembly.

* Good, because it allows for shared Rust code between frontend and backend
* Good, because it makes use of Rust's robust type system for error prevention
* Bad, because it's a younger project with potentially less community help
* Bad, because it may lack certain features that more mature frameworks have

### Flutter

Provides a framework to develop beautiful native apps from a single codebase.

* Good, because it has a hot reload and rich set of widgets
* Good, because it provides a uniform development experience across platforms
* Good, because it enjoys a vibrant community and extensive libraries
* Bad, because it may be overkill for simpler web applications
* Bad, because the UI design is less flexible compared to pure CSS

### Vue.js

A progressive JavaScript framework.

* Good, because it's easy to learn and use
* Good, because it has broad community support
* Good, because it provides reactive components out of the box
* Bad, because it is less efficient compared to Rust-based frameworks
* Bad, because its virtual dom can be slower than an actual dom

### React.js

A JavaScript library for building user interfaces.

* Good, because it's widely used and has a large community
* Good, because it is versatile and can be used with various architectures
* Good, because the virtual DOM enables efficient updates and rendering
* Bad, because it may be complex to learn due to various concepts and paradigms
* Bad, because React's JSX can be off-putting to some developers