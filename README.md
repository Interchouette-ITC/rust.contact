# Rust.Contact

The ITC portfolio for Rust projects. A single Serenade SSG build produces the
portfolio homepage and project microsites as static HTML. The shared navigation
template and styles are compiled with rangular; pages use Leptos and hydrate in
the browser.

## Requirements

- Rust toolchain compatible with the Serenade workspace
- `wasm32-unknown-unknown` Rust target
- Trunk

## Build and preview

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
make test
make build
make preview
```

The static output is `apps/site/dist/`. The preview is available at
`http://127.0.0.1:4174/`.

The project pages are available at `/serenade/`, `/rustashop/`, and `/rangular/`.
Custom-domain mapping is intentionally handled as a deployment concern; this
project does not run an application server or contain rustashop product logic.
