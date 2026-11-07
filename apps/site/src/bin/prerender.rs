use std::fs;
use std::path::PathBuf;

use anyhow::{Context, bail};

fn main() -> anyhow::Result<()> {
    let app = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dist = app.join("dist");
    let shell_path = dist.join("index.html");
    let shell = fs::read_to_string(&shell_path)
        .with_context(|| format!("read Trunk output {}", shell_path.display()))?;

    for (route, page) in [
        ("", "home"),
        ("serenade", "serenade"),
        ("rustashop", "rustashop"),
        ("rangular", "rangular"),
        ("opentrading", "opentrading"),
        ("interfire", "interfire"),
        ("itcy", "itcy"),
    ] {
        let content_marker = "<!--SERENADE_SSG_CONTENT-->";
        if shell.matches(content_marker).count() != 1 {
            bail!("Trunk output must contain exactly one {content_marker} marker");
        }
        let canonical_marker = "SERENADE_SSG_CANONICAL";
        if shell.matches(canonical_marker).count() != 1 {
            bail!("Trunk output must contain exactly one {canonical_marker} marker");
        }
        let mut html = shell.replacen(content_marker, &rust_contact_site::render_page(page), 1);
        let (title, description, canonical) = page_metadata(page);
        html = html.replacen(canonical_marker, canonical, 1);
        html = html.replace(
            "<title>Rust.Contact - Independent Rust projects from ITC</title>",
            &format!("<title>{title}</title>"),
        );
        html = html.replace(
            "content=\"Rust.Contact is the ITC portfolio for independent Rust products and open-source projects.\"",
            &format!("content=\"{description}\""),
        );

        let output = if route.is_empty() {
            dist.join("index.html")
        } else {
            let directory = dist.join(route);
            fs::create_dir_all(&directory)
                .with_context(|| format!("create route directory {}", directory.display()))?;
            directory.join("index.html")
        };
        let html = serenade_minify::minify_html(&html)
            .with_context(|| format!("minify prerendered page {}", output.display()))?;
        fs::write(&output, html)
            .with_context(|| format!("write prerendered page {}", output.display()))?;
        println!("Prerendered /{route} to {}", output.display());
    }

    Ok(())
}

fn page_metadata(page: &str) -> (&'static str, &'static str, &'static str) {
    match page {
        "serenade" => (
            "Serenade - Rust framework by ITC",
            "Serenade is a modular application framework for Rust.",
            "https://serenade.rust.contact/",
        ),
        "rustashop" => (
            "rustashop - ITC Rust project",
            "Explore rustashop, an independent Rust project from ITC.",
            "https://rustashop.rust.contact/",
        ),
        "rangular" => (
            "rangular - ITC Rust project",
            "Explore rangular, Angular-shaped Leptos templates with automated component and SCSS builds.",
            "https://rangular.rust.contact/",
        ),
        "opentrading" => (
            "OpenTrading - ITC Rust project",
            "Explore OpenTrading, a trading workstation built in Rust.",
            "https://open-trading.rust.contact/",
        ),
        "interfire" => (
            "InterFire - ITC Rust project",
            "Explore InterFire, a Linux-first application firewall in Rust.",
            "https://interfire.rust.contact/",
        ),
        "itcy" => (
            "ITCy - ITC Rust project",
            "Explore ITCy, an AI operator for Interchouette ITC.",
            "https://itcy.rust.contact/",
        ),
        _ => (
            "Rust.Contact - Independent Rust projects from ITC",
            "Rust.Contact is the ITC portfolio for independent Rust products and open-source projects.",
            "https://rust.contact/",
        ),
    }
}
