use leptos::prelude::*;
use rangular_aot::HostCell;
use rangular_host::{Host, HostError, Value};

include!(concat!(env!("OUT_DIR"), "/rangular/site_layout_view.rs"));

pub(crate) fn site_layout(page: &'static str, children: Children) -> impl IntoView {
    site_layout_view(HostCell::new(SiteLayoutHost { page }), children)
}

struct SiteLayoutHost {
    page: &'static str,
}

impl Host for SiteLayoutHost {
    fn get(&self, name: &str) -> Option<Value> {
        let page = self.page;
        match name {
            "isSerenade" => Some(Value::Bool(page == "serenade")),
            "isRustashop" => Some(Value::Bool(page == "rustashop")),
            "isRangular" => Some(Value::Bool(page == "rangular")),
            "isOpenTrading" => Some(Value::Bool(page == "opentrading")),
            "isInterfire" => Some(Value::Bool(page == "interfire")),
            "isItcy" => Some(Value::Bool(page == "itcy")),
            _ => None,
        }
    }

    fn call(&mut self, name: &str, _: &[Value]) -> Result<Value, HostError> {
        Err(HostError::UnknownMember(name.to_owned()))
    }
}
