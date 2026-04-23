fn main() {
    let h = "src/WebView2.h";
    let library = "WebView2Loader.dll";
    let namespace = "WebView2";
    let rdl = "src/WebView2.rdl";
    let reference = "../../../libs/bindgen/default";
    let winmd = "WebView2.winmd";

    println!("cargo:rerun-if-changed={h}");

    windows_rdl::clang()
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .input(h)
        .input(reference)
        .output(rdl)
        .namespace(namespace)
        .library(library)
        .write()
        .unwrap();

    windows_rdl::reader()
        .input(rdl)
        .input(reference)
        .output(winmd)
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        winmd,
        reference,
        "--out",
        "src/bindings.rs",
        "--filter",
        namespace,
        "--flat",
        "--typedef",
        "--reference",
        "windows",
    ])
    .unwrap();
}
