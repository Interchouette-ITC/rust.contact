fn main() {
    // Rerun if the base URL changes between builds.
    println!("cargo:rerun-if-env-changed=SITE_BASE_URL");

    rangular_build::build(
        rangular_build::Config::new("src/components", "style/rangular.generated.css")
            .global_scss("style/site.scss"),
    );
}
