use leptos::prelude::*;
use rangular_aot::HostCell;
use rangular_host::{Host, HostError, Value};

include!(concat!(env!("OUT_DIR"), "/site_nav.rs"));

#[cfg(feature = "hydrate")]
#[component]
pub fn App() -> impl IntoView {
    let path = web_sys::window()
        .expect("browser window is available during hydration")
        .location()
        .pathname()
        .expect("browser pathname is available during hydration");
    view! { <SitePage page=page_for_path(&path) /> }
}

#[component]
fn SitePage(page: &'static str) -> impl IntoView {
    view! {
        <div class=format!("site-shell site-shell--{page}")>
            <header class="site-header">
                <div class="brand-group">
                    <a class="brand" href="/" aria-label="Rust.Contact home">
                        <span class="brand__mark" aria-hidden="true"><span class="brand__crab">"🦀"</span></span>
                    </a>
                    <a class="brand-contact" href="mailto:hi@rust.contact">
                        <span>"Say hi at"</span>
                        <strong>"hi@rust.contact"</strong>
                    </a>
                </div>
                {site_nav(HostCell::new(SiteNavHost))}
                <div class="header-actions">
                    <a class="header-github-icon" href="https://github.com/Interchouette-ITC" aria-label="Interchouette on GitHub">
                        <img src="/logo3.png" alt="Interchouette" />
                    </a>
                    <a class="header-cta" href="https://github.com/Interchouette-ITC">
                        "ITC on GitHub"
                    </a>
                </div>
            </header>
            <main>
                {match page {
                    "serenade" => view! {
                        <ProjectPage
                            name="Serenade"
                            category="APPLICATION FRAMEWORK"
                            headline="Symfony-shaped. Rust-native."
                            description="Serenade is a Symfony-oriented framework for Rust: an application kernel, reusable components, and bundles that keep composition explicit."
                            href="https://serenade.rs/"
                            link_label="Visit serenade.rs"
                            repository="https://github.com/Interchouette-ITC/Serenade"
                            repository_label="Explore the framework"
                            logo="/serenade-brand.png"
                            logo_alt="Serenade · Rust application framework"
                            accent="serenade"
                        />
                    }.into_any(),
                    "rustashop" => view! {
                        <ProjectPage
                            name="rustashop"
                            category="RUST PRODUCT"
                            headline="Commerce, built in Rust."
                            description="rustashop is an independent ITC product exploring a Rust-native commerce platform built on Serenade. Visit its official product sites for current details."
                            href="https://rustashop.ai/"
                            link_label="Visit rustashop"
                            repository="https://github.com/Interchouette-ITC/rustashop"
                            repository_label="Explore the project"
                            logo="/rustashop-brand.png"
                            logo_alt="rustashop · Modern commerce, Rust powered"
                            accent="rustashop"
                        />
                    }.into_any(),
                    "rangular" => view! {
                        <ProjectPage
                            name="rangular"
                            category="UI TEMPLATING"
                            headline="Angular-shaped templates for Leptos."
                            description="rangular brings familiar HTML and SCSS templates to Leptos with compile-time lowering and a Rust host."
                            href="https://rangular.interchouette.net/"
                            link_label="Visit rangular"
                            repository="https://github.com/Interchouette-ITC/rangular"
                            repository_label="Explore the source"
                            logo="/rangular-mark.png"
                            logo_alt="rangular crab mark"
                            accent="rangular"
                        />
                    }.into_any(),
                    "opentrading" => view! {
                        <ProjectPage
                            name="OpenTrading"
                            category="TRADING WORKSTATION"
                            headline="Charting, signals, and AI-assisted workflows."
                            description="OpenTrading is a trading workstation for charting, signal analysis, and AI-assisted workflows, built in Rust."
                            href="https://github.com/open-trading-ai/OpenTrading"
                            link_label="Visit OpenTrading"
                            repository="https://github.com/open-trading-ai/OpenTrading"
                            repository_label="Explore the project"
                            logo="/OpenTrading-brand.png"
                            logo_alt="OpenTrading"
                            accent="opentrading"
                        />
                    }.into_any(),
                    "interfire" => view! {
                        <ProjectPage
                            name="InterFire"
                            category="APPLICATION FIREWALL"
                            headline="Linux-first application firewall in Rust."
                            description="InterFire is a Linux-first application firewall with process-aware connection rules and network filtering, built in Rust."
                            href="https://github.com/Interchouette-ITC/InterFire"
                            link_label="Visit InterFire"
                            repository="https://github.com/Interchouette-ITC/InterFire"
                            repository_label="Explore the project"
                            logo="/InterFire-brand.png"
                            logo_alt="InterFire - Firewall, secure, control"
                            accent="interfire"
                        />
                    }.into_any(),
                    "itcy" => view! {
                        <ProjectPage
                            name="ITCy"
                            category="AI OPERATOR"
                            headline="AI operator for Interchouette ITC."
                            description="ITCy is an AI operator for Interchouette ITC, helping draft and review social posts before publication."
                            href="https://github.com/Interchouette-ITC/itcy"
                            link_label="Visit ITCy"
                            repository="https://github.com/Interchouette-ITC/itcy"
                            repository_label="Explore the project"
                            logo="/ITCy-brand.png"
                            logo_alt="ITCy owl mascot"
                            accent="itcy"
                        />
                    }.into_any(),
                    _ => view! { <PortfolioHome /> }.into_any(),
                }}
            </main>
            <footer class="site-footer">
                <div class="footer-left">
                    <a class="brand brand--footer" href="/">
                        <span class="brand__mark" aria-hidden="true"><span class="brand__crab">"🦀"</span></span>
                        <span>"rust.contact"</span>
                    </a>
                    <p class="footer-description">"Independent Rust products and tools from " <a href="https://interchouette.net/">"Interchouette"</a> "."</p>
                    <nav class="footer-project-links" aria-label="Projects">
                        <a href="/serenade/">"Serenade"</a>
                        <a href="/rangular/">"rangular"</a>
                        <a href="/rustashop/">"rustashop"</a>
                        <a href="/opentrading/">"OpenTrading"</a>
                        <a href="/interfire/">"InterFire"</a>
                        <a href="/itcy/">"ITCy"</a>
                    </nav>
                    <small class="footer-tech">"Built with Rust and Serenade SSG."</small>
                </div>
                <div class="footer-right">
                    <p>"Have a project in mind?"</p>
                    <a class="footer-contact-link" href="/#contact">"Tell us what you're building " <span>"↑"</span></a>
                    <nav class="footer-links" aria-label="Organization links">
                        <a href="https://github.com/Interchouette-ITC">"Interchouette on GitHub"</a>
                        <a href="mailto:hi@rust.contact">"hi@rust.contact"</a>
                    </nav>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn PortfolioHome() -> impl IntoView {
    let sayings = [
        "I was told there'd be snacks.",
        "I work for crabsolutely nothing.",
        "This hard hat is mostly for the vibes.",
        "I came. I saw. I crabbed.",
        "Rust? I thought you said snacks.",
        "Safety first. Pinching second.",
        "I debug with my claws.",
        "If it compiles, I take the credit.",
        "I am here to shell out advice.",
        "Today's forecast: partly crabby.",
        "I put the hard in hard hat.",
        "Keep calm and crab on.",
        "I know a shortcut. It is sideways.",
        "No bugs, just tiny uninvited features.",
        "I am the designated pinch hitter.",
        "This meeting could have been a beach.",
        "Trust me, I am a crustacean engineer.",
        "I have got a shell of a work ethic.",
        "One small pinch for Ferris.",
        "I came for the code, stayed for the snacks.",
    ];
    let (saying, _set_saying) = signal(sayings[0]);

    #[cfg(feature = "hydrate")]
    Effect::new(move || {
        let index = (js_sys::Math::random() * sayings.len() as f64) as usize;
        _set_saying.set(sayings[index]);
    });

    view! {
        <section class="hero wrap">
            <div class="hero__artwork">
                <img class="hero__rust-art" src="/Rust.png" alt="Rust gear logo" />
            </div>
            <div class="hero__content">
                <p class="eyebrow"><span class="eyebrow__dot"></span>"ITC · RUST PROJECTS"</p>
                <h1><span class="hero__title-line">"Independent ideas."</span><br />"Built with "<span class="hero__rust-title">"Rust."</span></h1>
                <p class="hero__lead">
                    "A home for the Rust products and tools we build at Interchouette. Explore each project, see what it does, and find its official home."
                </p>
                <div class="hero__actions">
                    <a class="button button--primary" href="#projects">"Explore projects" <span>"↓"</span></a>
                    <figure class="hero__ferris">
                        <img src="/Ferris.png" alt="Ferris wearing a hard hat" />
                        <figcaption>{move || format!("Ferris says: {}", saying.get())}</figcaption>
                    </figure>
                </div>
                <div class="portfolio-stamp">
                    <span>"PRODUCTS"</span><strong>"01-06"</strong><span>"OPEN SOURCE"</span>
                </div>
            </div>
        </section>
        <section class="projects wrap" id="projects">
            <div class="section-heading">
                <div>
                    <p class="eyebrow">"THE PORTFOLIO"</p>
                    <h2>"Projects with their own purpose."</h2>
                </div>
                <p>"Different problems, different products-connected by a shared commitment to thoughtful Rust software."</p>
            </div>
            <div class="project-grid">
                <ProjectCard
                    number="01"
                    name="Serenade"
                    tag="APPLICATION FRAMEWORK"
                    description="A Symfony-oriented Rust framework built around a kernel, reusable components, and bundles."
                    href="/serenade/"
                    logo="/serenade-brand.png"
                    logo_alt="Serenade · Rust application framework"
                    accent="serenade"
                    wip=false
                />
                <ProjectCard
                    number="02"
                    name="rangular"
                    tag="UI TEMPLATING"
                    description="Angular-shaped HTML and SCSS templates compiled to Leptos views."
                    href="/rangular/"
                    logo="/rangular-mark.png"
                    logo_alt="rangular crab mark"
                    accent="rangular"
                    wip=false
                />
                <ProjectCard
                    number="03"
                    name="rustashop"
                    tag="COMMERCE"
                    description="A Rust-native commerce platform built on Serenade."
                    href="/rustashop/"
                    logo="/rustashop-brand.png"
                    logo_alt="rustashop · Modern commerce, Rust powered"
                    accent="rustashop"
                    wip=true
                />
                <ProjectCard
                    number="04"
                    name="OpenTrading"
                    tag="TRADING WORKSTATION"
                    description="A trading workstation for charting, signal analysis, and AI-assisted workflows."
                    href="/opentrading/"
                    logo="/OpenTrading-brand.png"
                    logo_alt="OpenTrading"
                    accent="opentrading"
                    wip=true
                />
                <ProjectCard
                    number="05"
                    name="InterFire"
                    tag="APPLICATION FIREWALL"
                    description="A Linux-first application firewall in Rust, with process-aware connection rules and network filtering."
                    href="/interfire/"
                    logo="/InterFire-brand.png"
                    logo_alt="InterFire - Firewall, secure, control"
                    accent="interfire"
                    wip=true
                />
                <ProjectCard
                    number="06"
                    name="ITCy"
                    tag="AI OPERATOR"
                    description="An AI operator for Interchouette ITC, helping draft and review social posts before publication."
                    href="/itcy/"
                    logo="/ITCy-brand.png"
                    logo_alt="ITCy owl mascot"
                    accent="itcy"
                    wip=false
                />
            </div>
        </section>
        <section class="contact-section" id="contact">
            <div class="contact-section__inner wrap">
                <header class="contact-section__heading">
                    <div class="contact-section__title">
                        <p class="eyebrow">"START A CONVERSATION"</p>
                        <h2>"Tell us what you're building."</h2>
                    </div>
                    <p>
                        "Share a little context and we can come back with useful questions, a sensible scope, and a quote. "
                        <span class="contact-section__required">"Fields marked * are required."</span>
                    </p>
                </header>
                <form
                    class="contact-form"
                    action="https://formsubmit.co/hi@rust.contact"
                    method="POST"
                    enctype="multipart/form-data"
                >
                    <input type="hidden" name="_subject" value="New Rust.Contact project enquiry" />
                    <input type="hidden" name="_template" value="table" />
                    <label class="contact-form__honeypot">
                        "Leave this field empty"
                        <input type="text" name="_honey" tabindex="-1" autocomplete="off" />
                    </label>
                    <div class="contact-form__grid">
                        <label class="contact-field">
                            <span>"Name *"</span>
                            <input type="text" name="name" autocomplete="name" placeholder="Your name" required maxlength="120" />
                        </label>
                        <label class="contact-field">
                            <span>"Email *"</span>
                            <input type="email" name="email" autocomplete="email" placeholder="you@company.com" required maxlength="254" />
                        </label>
                        <label class="contact-field">
                            <span>"Company or team"</span>
                            <input type="text" name="company" autocomplete="organization" placeholder="Where you work (optional)" maxlength="160" />
                        </label>
                        <label class="contact-field">
                            <span>"Project type *"</span>
                            <select name="project_type" required>
                                <option value="" selected disabled>"Choose a project type"</option>
                                <option value="Product or web application">"Product or web application"</option>
                                <option value="Rust consulting or implementation">"Rust consulting or implementation"</option>
                                <option value="Developer tool or open source">"Developer tool or open source"</option>
                                <option value="Systems integration or migration">"Systems integration or migration"</option>
                                <option value="Other">"Other"</option>
                            </select>
                        </label>
                        <label class="contact-field">
                            <span>"LinkedIn"</span>
                            <input type="url" name="linkedin" autocomplete="url" placeholder="https://linkedin.com/in/you" />
                        </label>
                        <label class="contact-field">
                            <span>"Twitter / X"</span>
                            <input type="url" name="twitter" autocomplete="url" placeholder="https://x.com/you" />
                        </label>
                        <label class="contact-field">
                            <span>"Timeline"</span>
                            <select name="timeline">
                                <option value="" selected>"Choose a timeline (optional)"</option>
                                <option value="As soon as possible">"As soon as possible"</option>
                                <option value="Within 1 month">"Within 1 month"</option>
                                <option value="1-3 months">"1-3 months"</option>
                                <option value="3-6 months">"3-6 months"</option>
                                <option value="Just exploring">"Just exploring"</option>
                            </select>
                        </label>
                        <label class="contact-field">
                            <span>"Estimated budget"</span>
                            <select name="budget">
                                <option value="" selected>"Choose a range (optional)"</option>
                                <option value="Under EUR 5,000">"Under €5,000"</option>
                                <option value="EUR 5,000-15,000">"€5,000-€15,000"</option>
                                <option value="EUR 15,000-40,000">"€15,000-€40,000"</option>
                                <option value="Over EUR 40,000">"Over €40,000"</option>
                                <option value="Not sure yet">"Not sure yet"</option>
                            </select>
                        </label>
                        <label class="contact-field contact-field--wide">
                            <span>"Project details *"</span>
                            <textarea
                                name="message"
                                rows="6"
                                placeholder="What are you building, who is it for, and what would a successful outcome look like?"
                                required
                                minlength="20"
                                maxlength="5000"
                            ></textarea>
                            <small>"A few sentences are plenty to get started."</small>
                        </label>
                        <label class="contact-field contact-field--wide">
                            <span>"Attach a brief or supporting document"</span>
                            <input
                                class="contact-field__file"
                                type="file"
                                name="attachment"
                                accept=".pdf,.doc,.docx,.odt,.txt,.rtf,.zip"
                            />
                            <small>"PDF, DOC, DOCX, ODT, TXT, RTF, or ZIP. 10 MB maximum."</small>
                        </label>
                    </div>
                    <div class="contact-form__footer">
                        <p>"Prefer email? " <a href="mailto:hi@rust.contact">"hi@rust.contact"</a></p>
                        <button class="button button--primary" type="submit">"Send project brief " <span>"→"</span></button>
                    </div>
                </form>
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(
    number: &'static str,
    name: &'static str,
    tag: &'static str,
    description: &'static str,
    href: &'static str,
    logo: &'static str,
    logo_alt: &'static str,
    accent: &'static str,
    wip: bool,
) -> impl IntoView {
    view! {
        <a class=format!("project-card project-card--{accent}") href=href>
            <span class="project-card__top">
                <span>{number}</span>
                <span class="project-card__top-actions">
                    {if wip {
                        view! {
                            <span class="project-card__status" title="Work in progress">
                                <span class="project-card__status-dot" aria-hidden="true"></span>
                                "WIP"
                            </span>
                        }
                        .into_any()
                    } else {
                        ().into_any()
                    }}
                    <span class="project-card__arrow">"↗"</span>
                </span>
            </span>
            <span class="project-card__tag">{tag}</span>
            <span class="project-card__identity"><img src=logo alt=logo_alt loading="lazy" /></span>
            <strong>{name}</strong>
            <span class="project-card__description">{description}</span>
            <span class="project-card__link">"View project " <span>"→"</span></span>
        </a>
    }
}

#[component]
fn ProjectPage(
    name: &'static str,
    category: &'static str,
    headline: &'static str,
    description: &'static str,
    href: &'static str,
    link_label: &'static str,
    repository: &'static str,
    repository_label: &'static str,
    logo: &'static str,
    logo_alt: &'static str,
    accent: &'static str,
) -> impl IntoView {
    view! {
        <section class=format!("project-hero project-hero--{accent}")>
            <div class="wrap project-hero__inner">
                <p class="eyebrow"><span class="eyebrow__dot"></span>{category}</p>
                <img class="project-hero__brand" src=logo alt=logo_alt />
                <p class="project-hero__name">{name}</p>
                <h1>{headline}</h1>
                <p class="hero__lead">{description}</p>
                <div class="hero__actions">
                    <a class="button button--primary" href=href>{link_label}<span>"↗"</span></a>
                    <a class="button button--quiet" href=repository>{repository_label}</a>
                </div>
                <a class="back-link" href="/">"← Back to all projects"</a>
            </div>
        </section>
    }
}

#[cfg(feature = "ssr")]
pub fn render_page(page: &'static str) -> String {
    let owner = Owner::new_root(Some(std::sync::Arc::new(
        hydration_context::SsrSharedContext::new(),
    )));
    owner.with(|| view! { <SitePage page /> }.to_html())
}

#[cfg(any(feature = "hydrate", test))]
fn page_for_path(path: &str) -> &'static str {
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

struct SiteNavHost;

impl Host for SiteNavHost {
    fn get(&self, name: &str) -> Option<Value> {
        let _ = name;
        None
    }

    fn call(&mut self, name: &str, _: &[Value]) -> Result<Value, HostError> {
        Err(HostError::UnknownMember(name.to_owned()))
    }
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::{page_for_path, render_page};

    #[test]
    fn portfolio_routes_resolve() {
        assert_eq!(page_for_path("/"), "home");
        assert_eq!(page_for_path("/serenade/"), "serenade");
        assert_eq!(page_for_path("/rustashop"), "rustashop");
        assert_eq!(page_for_path("/rangular/"), "rangular");
        assert_eq!(page_for_path("/opentrading/"), "opentrading");
        assert_eq!(page_for_path("/interfire/"), "interfire");
        assert_eq!(page_for_path("/itcy/"), "itcy");
    }

    #[test]
    fn each_page_prerenders_distinct_project_content() {
        let cases = [
            ("home", "Projects with their own purpose"),
            ("serenade", "Symfony-shaped. Rust-native."),
            ("rustashop", "Commerce, built in Rust."),
            ("rangular", "Angular-shaped templates for Leptos."),
            ("opentrading", "Charting, signals, and AI-assisted workflows."),
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
        let project_order = [
            "Serenade",
            "rangular",
            "rustashop",
            "OpenTrading",
            "InterFire",
            "ITCy",
        ];
        let positions = project_order.map(|name| {
            html.find(&format!("<strong>{name}</strong>"))
                .unwrap_or_else(|| panic!("missing project card for {name}"))
        });
        assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
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
            assert!(html.contains(expected), "missing {expected} in project form");
        }
    }
}
