# rust.contact

The ITC portfolio for Rust projects. A single Serenade SSG build produces the
portfolio homepage and project microsites as static HTML. The shared navigation
template and styles are compiled with rangular; pages use Leptos and hydrate in
the browser.

The SEO head is a Rangular template with interpolated page metadata and ordinary
Open Graph attributes. Its Rust module is SSR-only; browser hydration mounts the
page body without compiling the SEO component.

## Requirements

- Rust toolchain compatible with the Serenade workspace
- `wasm32-unknown-unknown` Rust target
- `serenade-ssg` (`make tools`), which installs Serenade's pinned Trunk release.
  The Render build (`scripts/render-build.sh`) installs prebuilt binaries from the latest
  Serenade release, or from the rolling pre-release with `SERENADE_CLI_CHANNEL=pre-release`

## Build and preview

```sh
rustup target add wasm32-unknown-unknown
make tools
make test
make build
make preview
```

`make build` runs `serenade-ssg --release --app-dir apps/site`. Trunk builds the
hydration bundle and writes its asset manifest; the `prerender` binary then
renders each route as a complete Serenade document from the rangular document
and head templates, with Trunk's hashed stylesheet and WASM loader markup taken
from the manifest. Prerendering fails unless the manifest has exactly one main
WASM bundle whose generated loader is present, so a page cannot ship without
hydration. `serenade-ssg` minifies the written HTML.

The static output is `apps/site/dist/`. The preview is available at
`http://127.0.0.1:4174/`.

The project pages are available at `/serenade/`, `/rustashop/`, and `/rangular/`.
Custom-domain mapping is intentionally handled as a deployment concern; this
project does not run an application server or contain rustashop product logic.
