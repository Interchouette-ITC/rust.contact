use super::project_page::ProjectDefinition;

pub static PROJECT: ProjectDefinition = ProjectDefinition {
    name: "InterFire",
    category: "APPLICATION FIREWALL",
    headline: "Linux-first application firewall in Rust.",
    description: "InterFire is a Linux-first application firewall with process-aware connection rules and network filtering, built in Rust.",
    href: "https://github.com/Interchouette-ITC/InterFire",
    link_label: "Visit InterFire",
    repository: "https://github.com/Interchouette-ITC/InterFire",
    repository_label: "Explore the project",
    logo: "/InterFire-brand.png",
    logo_alt: "InterFire - Firewall, secure, control",
    accent: "interfire",
    wip: true,
    features: &[
        ("Process-Aware Firewall", "Attribute outbound connections to their originating process and apply per-application rules."),
        ("eBPF TCP Observation", "Kernel-level eBPF programs observe TCP connections for real-time network visibility without performance penalty."),
        ("NFQUEUE Verdict Path", "Netfilter NFQUEUE integration (queue 4242) for packet-level accept/drop verdicts on outbound traffic."),
        ("TOML Rules", "Declarative rule configuration in TOML with process matching, port filtering, and action definitions."),
        ("TUI Control Plane", "Terminal-based user interface for monitoring active connections, managing rules, and viewing logs."),
        ("GPUI Desktop", "Native desktop application built with GPUI for visual firewall management and real-time connection monitoring."),
        ("systemd Packaging", "Production-ready systemd service configuration for daemon management and auto-restart."),
    ],
    architecture: "InterFire operates as a daemon (`interfired`) that coordinates between multiple subsystems: the eBPF observer attaches to the kernel to monitor TCP connections and attribute them to processes, while the NFQUEUE verdict path intercepts outbound packets for accept/drop decisions. Rules are defined in TOML and loaded by the rule engine, which matches connections against process names, ports, and protocols. The daemon exposes an IPC interface consumed by the CLI (`interfirectl`), the TUI (built with ratatui), and the GPUI desktop shell. This architecture separates the kernel-level observation from the user-facing control plane, allowing multiple interfaces to manage the same firewall state.",
    tech_stack: &["Rust", "Tokio", "aya eBPF", "nftables", "Netfilter", "ratatui", "GPUI", "nix", "tracing", "TOML"],
};
