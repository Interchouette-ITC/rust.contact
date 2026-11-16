use anyhow::{Context, bail};
use rust_contact_site::PageMeta;
use serenade_view::{StaticDocumentWriter, TrunkAssetManifest, initialize_leptos_ssr};

fn main() -> anyhow::Result<()> {
    initialize_leptos_ssr()?;
    let manifest = TrunkAssetManifest::read_from_env()?;
    let [bundle] = manifest.wasm.as_slice() else {
        bail!("the site needs exactly one Trunk WASM bundle to hydrate");
    };
    if bundle.app_type != "main" || !bundle.inject_scripts {
        bail!("the site requires one main Trunk WASM bundle with script injection for hydration");
    }
    let assets = manifest
        .document_assets()
        .context("read Trunk's generated document assets")?;
    let writer = StaticDocumentWriter::from_env()?;

    for (route, page) in rust_contact_site::ROUTES {
        let (title, description) = page_metadata(page);
        let meta = PageMeta {
            title: title.to_owned(),
            description: description.to_owned(),
            canonical: canonical_url(page),
        };
        let document = rust_contact_site::render_document(page, meta, &assets)
            .with_context(|| format!("render {route}"))?;
        let output = writer.write(route, document)?;
        println!("Prerendered {route} to {}", output.display());
    }

    Ok(())
}

fn canonical_url(page: &str) -> String {
    match page {
        "home" => "https://rust.contact/".to_owned(),
        "opentrading" => "https://open-trading.rust.contact/".to_owned(),
        other => format!("https://{other}.rust.contact/"),
    }
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
