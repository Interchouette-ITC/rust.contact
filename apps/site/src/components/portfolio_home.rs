use leptos::prelude::*;
use rangular_aot::HostCell;

use super::SiteHost;

include!(concat!(env!("OUT_DIR"), "/rangular/portfolio_home_view.rs"));

#[component]
pub fn PortfolioHome() -> impl IntoView {
    portfolio_home_view(HostCell::new(SiteHost::default()))
}
