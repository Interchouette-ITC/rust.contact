use super::project_page::ProjectDefinition;

pub static PROJECT: ProjectDefinition = ProjectDefinition {
    name: "Serenade",
    category: "APPLICATION FRAMEWORK",
    headline: "Modular by design. Rust-native.",
    description: "Serenade is a modular application framework for Rust, built around an application kernel, reusable components, and bundles that keep composition explicit.",
    href: "https://serenade.rs/",
    link_label: "Visit serenade.rs",
    repository: "https://github.com/Interchouette-ITC/Serenade",
    repository_label: "Explore the framework",
    logo: "/serenade-brand.png",
    logo_alt: "Serenade · Rust application framework",
    accent: "serenade",
    wip: false,
    features: &[
        ("Application Kernel", "Lifecycle-managed kernel with boot, shutdown hooks, and structured logging via tracing."),
        ("Dependency Injection", "Service container with scoped and singleton lifetimes, plus an event dispatcher for decoupled communication."),
        ("TOML Configuration", "Type-safe configuration from TOML files with environment variable interpolation and validation."),
        ("Repository Contracts", "Framework-agnostic data access layer with trait-based repository patterns, no ORM imposed."),
        ("HTTP & Routing", "Framework-agnostic HTTP types with method routing, middleware pipelines, and request/response abstractions."),
        ("Actix Web Adapter", "Ready-to-use adapter for Actix Web with automatic integration of Serenade services and middleware."),
        ("Console Commands", "CLI command system with input options, output formatting, and progress indicators."),
        ("Scaffolding CLI", "Command-line tool to generate bundles, commands, and configuration stubs for rapid development."),
    ],
    architecture: "Serenade is built around a composable application kernel that orchestrates bundles, self-contained modules that register services, commands, and routes. The kernel manages the application lifecycle from boot to shutdown, coordinating dependency injection, event dispatch, and configuration. Unlike monolithic frameworks, Serenade does not impose a specific ORM or HTTP server; instead, it provides adapters (currently for Actix Web) that bridge the framework's abstractions to concrete implementations. This design lets teams swap infrastructure layers while keeping business logic in bundles that remain framework-agnostic.",
    tech_stack: &["Rust", "Actix Web", "Axum", "TOML", "tracing", "rustyline", "ratatui", "serde"],
};
