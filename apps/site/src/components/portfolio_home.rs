use leptos::prelude::*;
use rangular_aot::HostCell;

use super::{FERRIS_SAYINGS, SiteHost};

include!(concat!(env!("OUT_DIR"), "/rangular/portfolio_home_view.rs"));

#[component]
pub fn PortfolioHome() -> impl IntoView {
    let ferris_saying = RwSignal::new(format!("Ferris says: {}", FERRIS_SAYINGS[0]));

    Effect::new(move |_| {
        #[cfg(all(feature = "hydrate", not(feature = "ssr")))]
        {
            let index = (js_sys::Math::random() * FERRIS_SAYINGS.len() as f64) as usize;

            ferris_saying.set(format!("Ferris says: {}", FERRIS_SAYINGS[index]));
        }
    });

    portfolio_home_view(HostCell::new(SiteHost {
        ferris_saying: Some(ferris_saying),
    }))
}
