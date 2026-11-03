use super::project_page::ProjectDefinition;

pub static PROJECT: ProjectDefinition = ProjectDefinition {
    name: "rustashop",
    category: "COMMERCE PLATFORM",
    headline: "Commerce, built in Rust.",
    description: "rustashop is an independent ITC product exploring a Rust-native commerce platform built on Serenade. Visit its official product sites for current details.",
    href: "https://rustashop.ai/",
    link_label: "Visit rustashop.ai",
    repository: "https://github.com/Interchouette-ITC/rustashop",
    repository_label: "Explore the project",
    logo: "/rustashop-brand.png",
    logo_alt: "rustashop · Modern commerce, Rust powered",
    accent: "rustashop",
    wip: true,
    features: &[
        (
            "Commerce API",
            "RESTful API for products, categories, carts, orders, and checkout flows with JSON request/response.",
        ),
        (
            "OpenAPI Explorers",
            "Auto-generated OpenAPI documentation with interactive explorers for both API and admin endpoints.",
        ),
        (
            "Postgres Persistence",
            "Production-grade data layer using SQLx for type-safe queries and SeaORM for entity modeling.",
        ),
        (
            "Shared Templates",
            "Reusable rangular template components shared across storefront and admin back-office interfaces.",
        ),
        (
            "Angular Storefront",
            "Full-featured Angular-based storefront with product catalog, cart management, and checkout.",
        ),
        (
            "Leptos/Rangular Shop",
            "Rust-native storefront rendered with Leptos via rangular templates, running as WASM in the browser.",
        ),
        (
            "Admin Back Office",
            "Management interface for product catalog, order processing, and store configuration.",
        ),
    ],
    architecture: "rustashop is built on the Serenade kernel, using its dependency injection, event system, and configuration layers. The application follows a domain-driven structure: business logic lives in domain models and services, while adapters handle persistence (Postgres via SQLx/SeaORM), HTTP transport (Actix Web), and UI rendering. Multiple frontend renderers coexist: an Angular SPA, a Leptos/rangular WASM storefront, and GPUI-based native desktop apps, all consuming the same API layer. This multi-renderer approach allows rustashop to serve web, desktop, and embedded contexts from a single codebase.",
    tech_stack: &[
        "Rust",
        "Serenade",
        "Actix Web",
        "SQLx",
        "SeaORM",
        "Angular",
        "Leptos",
        "rangular",
        "GPUI",
        "PostgreSQL",
        "OpenAPI",
    ],
};
