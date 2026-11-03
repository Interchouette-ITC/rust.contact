use super::project_page::ProjectDefinition;

pub static PROJECT: ProjectDefinition = ProjectDefinition {
    name: "rangular",
    category: "UI TEMPLATING",
    headline: "Angular-shaped templates for Leptos.",
    description: "rangular brings familiar HTML and SCSS templates to Leptos with compile-time lowering, a Rust host, and a build helper that discovers components and assembles their styles.",
    href: "https://rangular.interchouette.net/",
    link_label: "Visit rangular",
    repository: "https://github.com/Interchouette-ITC/rangular",
    repository_label: "Explore the source",
    logo: "/rangular-mark.png",
    logo_alt: "rangular crab mark",
    accent: "rangular",
    wip: false,
    features: &[
        ("Angular-Shaped Templates", "Write HTML templates with Angular-style directives: ngIf, ngFor, ngClass, property binding, and event handlers."),
        ("AOT Compilation", "Compile templates at build time to Leptos view!() macros for zero-runtime-overhead rendering."),
        ("Runtime Interpreter", "Load and interpret templates at runtime for dynamic content scenarios without recompilation."),
        ("SCSS Compilation", "Build-time SCSS-to-CSS compilation with Bootstrap SCSS variable support for theming."),
        ("Cargo Build Helper", "One build-script call discovers component folders, compiles HTML templates into Rust views, and combines global and component SCSS into a stylesheet."),
        ("Component Registry", "Declare and register reusable components with slots, inputs, and outputs in a familiar Angular pattern."),
        ("Provide / Inject", "Dependency injection hierarchy mirroring Angular's provide/inject pattern for shared state and services."),
    ],
    architecture: "The rangular-build helper scans convention-based component folders, compiles each HTML template into a generated Rust view, and combines optional global SCSS with component stylesheets. A short Cargo build script configures those input and output paths. The AOT compiler parses Angular-shaped templates and lowers them to Leptos view!() Rust code; the runtime interpreter instead evaluates expressions through the expr and host crates. Both paths render through Leptos in the browser as WebAssembly or on the server for SSR.",
    tech_stack: &["Rust", "Leptos", "rangular-build", "Cargo", "Trunk", "WebAssembly", "Bootstrap SCSS", "serde"],
};
