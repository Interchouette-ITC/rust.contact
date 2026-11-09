use super::project_page::ProjectDefinition;

pub static PROJECT: ProjectDefinition = ProjectDefinition {
    name: "ITCy",
    category: "AI OPERATOR",
    headline: "AI operator for Interchouette ITC.",
    description: "ITCy is an AI operator for Interchouette ITC, helping draft and review social posts before publication.",
    href: "https://github.com/Interchouette-ITC/itcy",
    link_label: "Visit ITCy",
    repository: "https://github.com/Interchouette-ITC/itcy",
    repository_label: "Explore the project",
    logo: "/ITCy-brand.png",
    logo_alt: "ITCy owl mascot",
    accent: "itcy",
    wip: false,
    features: &[
        (
            "LinkedIn Post Drafting",
            "AI-assisted drafting of LinkedIn posts with tone and style guidance for professional content.",
        ),
        (
            "X/Twitter Operator",
            "Draft and schedule posts for X (Twitter) with character-aware formatting and thread support.",
        ),
        (
            "BAT Approval Workflow",
            "Draft-approve-ship pipeline requiring human review before any post goes live, ensuring quality control.",
        ),
        (
            "MCP Integration",
            "Model Context Protocol-based LinkedIn integration for authenticated API access and post management.",
        ),
        (
            "AI Disclosure",
            "Automatic AI-generated content disclosure labels to maintain transparency with social media audiences.",
        ),
        (
            "SQLite Storage",
            "Lightweight local persistence for drafts, published posts, and audit logs using SQLite.",
        ),
    ],
    architecture: "ITCy runs as a backend HTTP server that orchestrates the social media publishing workflow. The MCP-based LinkedIn module handles authenticated API communication for post creation and management. The publication workflow follows a PR-based approval pattern: AI drafts are created, then a human reviewer approves via the BAT (Before-After-Transition) check before the post is published. SQLite stores drafts, publication history, and audit trails locally. The BUSL-1.1 license ensures the operator remains available for personal and internal use while protecting commercial deployment.",
    tech_stack: &[
        "Rust",
        "SQLite",
        "MCP",
        "LinkedIn API",
        "X/Twitter API",
        "HTTP",
        "BUSL-1.1",
    ],
};
