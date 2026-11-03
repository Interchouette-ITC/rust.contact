use super::project_page::ProjectDefinition;

pub static PROJECT: ProjectDefinition = ProjectDefinition {
    name: "OpenTrading",
    category: "TRADING WORKSTATION",
    headline: "Charting, signals, and AI-assisted workflows.",
    description: "OpenTrading is a trading workstation for charting, signal analysis, and AI-assisted workflows, built in Rust.",
    href: "https://open-trading.ai",
    link_label: "Visit open-trading.ai",
    repository: "https://github.com/open-trading-ai/OpenTrading",
    repository_label: "Explore the project",
    logo: "/OpenTrading-brand.png",
    logo_alt: "OpenTrading",
    accent: "opentrading",
    wip: true,
    features: &[
        (
            "Real-Time Charting",
            "Interactive financial charts with candlestick, line, and volume visualizations for market analysis.",
        ),
        (
            "Signal Analysis",
            "Technical indicator engine for generating and backtesting trading signals from market data.",
        ),
        (
            "AI-Assisted Workflows",
            "Integrated AI tools to assist with market research, signal interpretation, and trade analysis.",
        ),
        (
            "Desktop Applications",
            "Cross-platform desktop clients packaged with Electron and Tauri.",
        ),
        (
            "Self-Hosted Docker",
            "Complete Docker Compose stack for self-hosted deployment with backend, database, and frontend.",
        ),
        (
            "Multi-Language Stack",
            "Rust backend for performance-critical computation, TypeScript frontend for UI, and WASM for shared logic.",
        ),
    ],
    architecture: "OpenTrading uses a multi-service architecture: a Rust backend API handles market data ingestion, signal computation, and user management; a TypeScript frontend provides the interactive charting and dashboard UI; Electron and Tauri package the frontend for desktop use. WASM modules share computation logic between the Rust backend and browser frontend. The entire stack deploys via Docker Compose, making self-hosting straightforward. This separation allows the performance-critical trading logic to run in Rust while the UI leverages the rich TypeScript ecosystem.",
    tech_stack: &[
        "Rust",
        "Electron",
        "Tauri",
        "TypeScript",
        "Docker",
        "WebAssembly",
        "Docker Compose",
        "PostgreSQL",
    ],
};
