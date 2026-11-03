use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let component = manifest.join("src/site_nav");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is set"));

    println!("cargo:rerun-if-changed=src/site_nav/site_nav.html");
    println!("cargo:rerun-if-changed=src/site_nav/site_nav.scss");

    let html_path = component.join("site_nav.html");
    let html = std::fs::read_to_string(&html_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", html_path.display()));
    let compiled = rangular_aot::compile_named(&html, "src/site_nav/site_nav.html", "site_nav");
    assert!(compiled.ok(), "site_nav.html: {:?}", compiled.issues);
    std::fs::write(out_dir.join("site_nav.rs"), compiled.code)
        .expect("write generated rangular navigation component");

    let scss_path = component.join("site_nav.scss");
    let scss = std::fs::read_to_string(&scss_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", scss_path.display()));
    let compiled = rangular_css::compile_scss(&scss);
    assert!(compiled.ok(), "site_nav.scss: {:?}", compiled.issues);
    let css_path = manifest.join("style/rangular.generated.css");
    std::fs::write(&css_path, compiled.css)
        .unwrap_or_else(|error| panic!("write {}: {error}", css_path.display()));
}
