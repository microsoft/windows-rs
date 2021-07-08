fn main() {
    windows::build! {
        Microsoft::Web::WebView2::Win32::CompareBrowserVersions,
    };

    // WebView2LoaderStatic.lib has references to these libraries which WebView2Loader.dll would have linked internally.
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=shell32");
}
