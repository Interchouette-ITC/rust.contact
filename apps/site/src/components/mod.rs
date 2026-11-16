use leptos::reactive::{signal::RwSignal, traits::Get};
use rangular_host::{Host, HostError, Value};

pub mod interfire_project;
pub mod itcy_project;
pub mod opentrading_project;
pub mod portfolio_home;
pub mod project_page;
pub mod rangular_project;
pub mod rustashop_project;
pub mod serenade_project;
pub mod site_footer;
pub mod site_header;
pub mod site_layout;

/// Base URL baked in at compile time. Override with SITE_BASE_URL env var.
/// Production: `SITE_BASE_URL=https://rust.contact`
/// Default (localhost dev): `http://localhost:4174` (Trunk default port)
pub(crate) const SITE_BASE_URL: &str = match option_env!("SITE_BASE_URL") {
    Some(url) => url,
    None => "http://localhost:4174",
};

/// Returns the canonical URL for a project page or the home page.
/// On production these become subdomain URLs; on localhost path-based URLs.
pub(crate) fn project_url(slug: &str) -> String {
    match SITE_BASE_URL {
        // Production subdomain layout: each project gets its own subdomain.
        "https://rust.contact" => match slug {
            "home" => "https://rust.contact/".to_owned(),
            "opentrading" => "https://open-trading.rust.contact/".to_owned(),
            other => format!("https://{other}.rust.contact/"),
        },
        // Localhost and any other base: use path-based URLs under the base.
        base => match slug {
            "home" => format!("{base}/"),
            other => format!("{base}/{other}/"),
        },
    }
}

#[derive(Default)]
pub(crate) struct SiteHost {
    pub ferris_saying: Option<RwSignal<String>>,
}

const FERRIS_SAYINGS: &[&str] = &[
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

impl Host for SiteHost {
    fn get(&self, name: &str) -> Option<Value> {
        match name {
            "space" => Some(Value::from(" ")),
            "brandCrab" => Some(Value::from("🦀")),
            "siteName" => Some(Value::from("rust.contact")),
            "homeUrl" => Some(Value::Str(project_url("home"))),
            "serenadeUrl" => Some(Value::Str(project_url("serenade"))),
            "rustashopUrl" => Some(Value::Str(project_url("rustashop"))),
            "rangularUrl" => Some(Value::Str(project_url("rangular"))),
            "openTradingUrl" => Some(Value::Str(project_url("opentrading"))),
            "interfireUrl" => Some(Value::Str(project_url("interfire"))),
            "itcyUrl" => Some(Value::Str(project_url("itcy"))),
            "ferrisSaying" => self
                .ferris_saying
                .as_ref()
                .map(|signal| Value::Str(signal.get())),
            _ => None,
        }
    }

    fn call(&mut self, name: &str, _: &[Value]) -> Result<Value, HostError> {
        Err(HostError::UnknownMember(name.to_owned()))
    }
}
