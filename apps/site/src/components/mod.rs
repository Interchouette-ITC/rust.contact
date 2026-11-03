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

pub(crate) struct SiteHost;

impl Host for SiteHost {
    fn get(&self, name: &str) -> Option<Value> {
        match name {
            "space" => Some(Value::from(" ")),
            "ferrisSaying" => {
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

                #[cfg(all(feature = "hydrate", not(feature = "ssr")))]
                {
                    let index = (js_sys::Math::random() * sayings.len() as f64) as usize;
                    Some(Value::Str(format!("Ferris says: {}", sayings[index])))
                }
                #[cfg(not(all(feature = "hydrate", not(feature = "ssr"))))]
                Some(Value::Str(format!("Ferris says: {}", sayings[0])))
            }
            _ => None,
        }
    }

    fn call(&mut self, name: &str, _: &[Value]) -> Result<Value, HostError> {
        Err(HostError::UnknownMember(name.to_owned()))
    }
}
