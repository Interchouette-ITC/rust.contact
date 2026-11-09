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
        let (title, description) = page_metadata(page);
        let canonical = match route {
            "" => "https://rust.contact/".to_owned(),
            "opentrading" => "https://open-trading.rust.contact/".to_owned(),
            other => format!("https://{other}.rust.contact/"),
        };
        let head = rust_contact_site::render_seo_head(title, description, &canonical);
        let content = rust_contact_site::render_page(page);
        let html = render_shell(&shell, &head, &content)?;

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

fn render_shell(shell: &str, head: &str, content: &str) -> anyhow::Result<String> {
    let head_marker = "<!--SERENADE_SSG_HEAD-->";
    let content_marker = "<!--SERENADE_SSG_CONTENT-->";
    if shell.matches(head_marker).count() != 1 || shell.matches(content_marker).count() != 1 {
        bail!("Trunk output must contain exactly one head and content marker");
    }
    let (before_head, after_head) = shell
        .split_once(head_marker)
        .context("Trunk output is missing the SEO head marker")?;
    let shell = format!("{before_head}{head}{after_head}");
    let (before_content, after_content) = shell
        .split_once(content_marker)
        .context("Trunk output is missing the page content marker")?;
    Ok(format!("{before_content}{content}{after_content}"))
}

fn page_metadata(page: &str) -> (&'static str, &'static str) {
    match page {
        "serenade" => (
            "Serenade - Rust framework by ITC",
            "Serenade is a modular application framework for Rust.",
        ),
        "rustashop" => (
            "rustashop - ITC Rust project",
            "Explore rustashop, an independent Rust project from ITC.",
        ),
        "rangular" => (
            "rangular - ITC Rust project",
            "Explore rangular, Angular-shaped Leptos templates with automated component and SCSS builds.",
        ),
        "opentrading" => (
            "OpenTrading - ITC Rust project",
            "Explore OpenTrading, a trading workstation built in Rust.",
        ),
        "interfire" => (
            "InterFire - ITC Rust project",
            "Explore InterFire, a Linux-first application firewall in Rust.",
        ),
        "itcy" => (
            "ITCy - ITC Rust project",
            "Explore ITCy, an AI operator for Interchouette ITC.",
        ),
        _ => (
            "Rust.Contact - Independent Rust projects from ITC",
            "Independent Rust products, frameworks and open-source projects from Interchouette-ITC.",
        ),
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn sitemap_lists_deployed_hosts() {
            let sitemap = include_str!("../../sitemap.xml");
            for host in [
                "rust.contact",
                "serenade.rust.contact",
                "rustashop.rust.contact",
                "rangular.rust.contact",
                "open-trading.rust.contact",
                "interfire.rust.contact",
                "itcy.rust.contact",
            ] {
                assert!(sitemap.contains(&format!("<loc>https://{host}/</loc>")));
            }
            assert_eq!(sitemap.matches("<loc>").count(), 7);
        }
    }
}
