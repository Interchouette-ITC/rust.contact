#[cfg(feature = "hydrate")]
fn main() {
    leptos::mount::hydrate_body(rust_contact_site::App);
}

#[cfg(not(feature = "hydrate"))]
fn main() {}
