use leptos::prelude::*;

mod components;

use components::{
    portfolio_home::PortfolioHome, project_page::ProjectPage, site_footer::SiteFooter,
    site_header::SiteHeader, site_layout::site_layout,
};

#[cfg(feature = "hydrate")]
#[component]
pub fn App() -> impl IntoView {
    let location = web_sys::window()
        .expect("browser window is available during hydration")
        .location();
    let path = location
        .pathname()
        .expect("browser pathname is available during hydration");
    let hostname = location
        .hostname()
        .expect("browser hostname is available during hydration");
    view! { <SitePage page=page_for_path(&path, &hostname) /> }
}

#[component]
fn SitePage(page: &'static str) -> impl IntoView {
    site_layout(
        page,
        Box::new(move || {
            view! {
                <SiteHeader />
                <main>
                    {match page {
                        "home" => view! { <PortfolioHome /> }.into_any(),
                        _ => view! { <ProjectPage page /> }.into_any(),
                    }}
                </main>
                <SiteFooter />
            }
            .into_any()
        }),
    )
}

#[cfg(feature = "ssr")]
pub fn render_page(page: &'static str) -> String {
    let owner = Owner::new_root(Some(std::sync::Arc::new(
        hydration_context::SsrSharedContext::new(),
    )));
    owner.with(|| view! { <SitePage page /> }.to_html())
}

#[cfg(any(feature = "hydrate", test))]
fn page_for_path(path: &str, hostname: &str) -> &'static str {
    // Subdomain routing: {project}.rust.contact maps directly to its project page.
    // The subdomain is the label before the first dot; e.g. "serenade.rust.contact"
    // → "serenade". open-trading.rust.contact → "opentrading".
    let subdomain = hostname.split('.').next().unwrap_or("");
    let from_subdomain = match subdomain {
        "serenade" => Some("serenade"),
        "rustashop" => Some("rustashop"),
        "rangular" => Some("rangular"),
        "open-trading" | "opentrading" => Some("opentrading"),
        "interfire" => Some("interfire"),
        "itcy" => Some("itcy"),
        _ => None,
    };
    if let Some(page) = from_subdomain {
        return page;
    }

    // Fallback: path-based routing for rust.contact/{project}/
    match path.trim_matches('/') {
        "serenade" => "serenade",
        "rustashop" => "rustashop",
        "rangular" => "rangular",
        "opentrading" => "opentrading",
        "interfire" => "interfire",
        "itcy" => "itcy",
        _ => "home",
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::{page_for_path, render_page};

    #[test]
    fn portfolio_routes_resolve() {
        assert_eq!(page_for_path("/", "rust.contact"), "home");
        assert_eq!(page_for_path("/serenade/", "rust.contact"), "serenade");
        assert_eq!(page_for_path("/rustashop", "rust.contact"), "rustashop");
        assert_eq!(page_for_path("/rangular/", "rust.contact"), "rangular");
        assert_eq!(
            page_for_path("/opentrading/", "rust.contact"),
            "opentrading"
        );
        assert_eq!(page_for_path("/interfire/", "rust.contact"), "interfire");
        assert_eq!(page_for_path("/itcy/", "rust.contact"), "itcy");
    }

    #[test]
    fn nav_links_use_base_url() {
        // In test/dev mode SITE_BASE_URL is not set so we get localhost URLs.
        // This test asserts the URL shape: links must be absolute and contain
        // the base, never bare root-relative paths like /serenade/.
        let base = crate::components::SITE_BASE_URL;
        let html = render_page("home");
        // Every project must appear as an absolute link, not a path.
        for slug in ["serenade", "rustashop", "rangular", "interfire", "itcy"] {
            let abs = format!("{base}/{slug}/");
            assert!(
                html.contains(&abs),
                "nav link for {slug} must be absolute ({abs}); got relative or missing"
            );
        }
        // opentrading maps to open-trading subdomain in production; in dev it's a path.
        assert!(
            html.contains(&format!("{base}/opentrading/")),
            "nav link for opentrading must be absolute"
        );
        // Root-relative hrefs must not appear.
        assert!(
            !html.contains("href=\"/serenade/\""),
            "bare /serenade/ path must not appear in rendered HTML"
        );
    }

    #[test]
    fn subdomain_routes_resolve() {
        assert_eq!(page_for_path("/", "serenade.rust.contact"), "serenade");
        assert_eq!(page_for_path("/", "rustashop.rust.contact"), "rustashop");
        assert_eq!(page_for_path("/", "rangular.rust.contact"), "rangular");
        assert_eq!(
            page_for_path("/", "open-trading.rust.contact"),
            "opentrading"
        );
        assert_eq!(
            page_for_path("/", "opentrading.rust.contact"),
            "opentrading"
        );
        assert_eq!(page_for_path("/", "interfire.rust.contact"), "interfire");
        assert_eq!(page_for_path("/", "itcy.rust.contact"), "itcy");
        // subdomain takes precedence over conflicting path
        assert_eq!(
            page_for_path("/rustashop", "serenade.rust.contact"),
            "serenade"
        );
    }

    #[test]
    fn each_page_prerenders_distinct_project_content() {
        let cases = [
            ("home", "Projects with their own purpose"),
            ("serenade", "Modular by design. Rust-native."),
            ("rustashop", "Commerce, built in Rust."),
            ("rangular", "Angular-shaped templates for Leptos."),
            (
                "opentrading",
                "Charting, signals, and AI-assisted workflows.",
            ),
            ("interfire", "Linux-first application firewall in Rust."),
            ("itcy", "AI operator for Interchouette ITC."),
        ];
        for (page, expected) in cases {
            let html = render_page(page);
            assert!(html.contains(expected), "missing {expected} on {page}");
            assert!(
                html.contains("rust.contact"),
                "shared brand absent on {page}"
            );
        }
    }

    #[test]
    fn portfolio_lists_added_projects() {
        let html = render_page("home");
        for expected in [
            "InterFire",
            "Linux-first application firewall",
            "OpenTrading",
            "trading workstation for charting",
            "ITCy",
            "AI operator for Interchouette ITC",
            "/InterFire-brand.png",
            "/OpenTrading-brand.png",
            "/ITCy-brand.png",
            "01-06",
            "see what it does",
            "/Rust.png",
        ] {
            assert!(html.contains(expected), "missing {expected} on home page");
        }
        let grid_start = html
            .find("class=\"project-grid\"")
            .expect("portfolio has a project grid");
        let project_grid = &html[grid_start..];
        let project_order = [
            "serenade",
            "rangular",
            "rustashop",
            "opentrading",
            "interfire",
            "itcy",
        ];
        let positions = project_order.map(|slug| {
            project_grid
                .find(&format!("project-card--{slug}"))
                .unwrap_or_else(|| panic!("missing {slug} project card"))
        });
        assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn project_pages_include_content_sections() {
        let pages = [
            "serenade",
            "rustashop",
            "rangular",
            "opentrading",
            "interfire",
            "itcy",
        ];
        for page in pages {
            let html = render_page(page);
            assert!(
                html.contains("KEY FEATURES"),
                "missing KEY FEATURES section on {page}"
            );
            assert!(
                html.contains("feature-card"),
                "missing feature cards on {page}"
            );
            assert!(
                html.contains("ARCHITECTURE"),
                "missing ARCHITECTURE section on {page}"
            );
            assert!(
                html.contains("architecture-prose"),
                "missing architecture prose on {page}"
            );
            assert!(
                html.contains("TECHNOLOGY STACK"),
                "missing TECHNOLOGY STACK section on {page}"
            );
            assert!(html.contains("tech-tag"), "missing tech tags on {page}");
        }
    }

    #[test]
    fn wip_badge_shows_on_work_in_progress_pages() {
        let wip_pages = ["rustashop", "opentrading", "interfire"];
        for page in wip_pages {
            let html = render_page(page);
            assert!(
                html.contains("Work in progress"),
                "missing WIP badge on {page}"
            );
            assert!(
                html.contains("project-hero__wip"),
                "missing WIP badge class on {page}"
            );
        }
        let stable_pages = ["serenade", "rangular", "itcy"];
        for page in stable_pages {
            let html = render_page(page);
            assert!(
                !html.contains("Work in progress"),
                "unexpected WIP badge on {page}"
            );
        }
    }

    #[test]
    fn project_enquiry_form_collects_quote_details_and_documents() {
        let html = render_page("home");
        for expected in [
            "https://formsubmit.co/hi@rust.contact",
            "multipart/form-data",
            "name=\"name\"",
            "name=\"email\"",
            "name=\"linkedin\"",
            "name=\"twitter\"",
            "name=\"project_type\"",
            "name=\"timeline\"",
            "name=\"budget\"",
            "name=\"message\"",
            "name=\"attachment\"",
            "name=\"_honey\"",
            "Send project brief",
        ] {
            assert!(
                html.contains(expected),
                "missing {expected} in project form"
            );
        }
    }
}

#[cfg(all(test, feature = "ssr"))]
#[test]
fn debug_home_html() {
    let html = render_page("home");
    // Check if project cards are present
    if let Some(pos) = html.find("Serenade") {
        eprintln!("Found 'Serenade' at position {}", pos);
        let start = pos.saturating_sub(100);
        let end = (pos + 100).min(html.len());
        eprintln!("Context: {}", &html[start..end]);
    } else {
        eprintln!("'Serenade' NOT found in HTML!");
        eprintln!("HTML length: {}", html.len());
        if let Some(pos) = html.find("project-grid") {
            let end = (pos + 200).min(html.len());
            eprintln!("project-grid context: {}", &html[pos..end]);
        }
    }
    // Check for escaped HTML
    if html.contains("&lt;strong&gt;") {
        eprintln!("WARNING: HTML is being escaped!");
    }
}
