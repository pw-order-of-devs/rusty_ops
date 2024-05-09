# Adopting Svelte + SvelteKit for Frontend

[< Back to ADR README](README.md)

* Status: Decided
* Deciders: @PW
* Date: 2024-05-09

Technical Story: We are to determine the replacement for previously chosen Leptos frontend framework.

## Context

The frontend of the application was initially planned to use Leptos. However, we are considering a change. The Rust programming language, while suitable for our backend needs, could be challenging to adopt for frontend development at the current stage.

## Decision

We are switching to Svelte + SvelteKit for developing the frontend of our application.

## Reasons

1. **Ease of use:** Svelte has an intuitive syntax and less boilerplate code which allows for easy adoption.
2. **Performance:** As Svelte compiles the components at build time and directly manipulates the DOM, it results in better runtime performance.
3. **Server-side rendering and static site generation**: With SvelteKit, these major features for building modern web applications come out-of-the-box.
4. **Vibrant community:** A larger community translates to more resources and components readily available.

## Decision Outcome

1. The learning curve for developers not familiar with Svelte and SvelteKit.
2. Replacing existing frontend code to Svelte+SvelteKit based.
3. More productive and streamlined development processes.

## Links

* [0004 - Frontend framework](ADR_0004_FRONTEND_FRAMEWORK.md)
