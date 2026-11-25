fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .sys()
        .filters([
            "TaskDialogIndirect",
            "TDN_BUTTON_CLICKED",
            "TDF_USE_COMMAND_LINKS",
            "TDF_ALLOW_DIALOG_CANCELLATION",
        ])
        .write();

    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
        println!("cargo:rerun-if-changed=manifest.xml");
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");

        println!(
            "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
            std::path::Path::new("manifest.xml")
                .canonicalize()
                .unwrap()
                .display()
        );
    }
}
