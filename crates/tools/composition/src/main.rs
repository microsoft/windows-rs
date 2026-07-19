use windows_bindgen::bindgen;

fn main() {
    let time = std::time::Instant::now();

    // Windows.UI.Composition (the system stack) lives in Windows.winmd;
    // Windows.Win32.winmd supplies ICompositorDesktopInterop and the HWND/BOOL
    // types used to host a visual tree in a plain window. Flat + minimal keeps
    // the crate's own surface small and namespace-free
    // (see docs/crates/windows-composition.md).
    let args = [
        "--in",
        "crates/libs/bindgen/default/Windows.winmd",
        "crates/libs/bindgen/default/Windows.Win32.winmd",
        "--out",
        "crates/libs/composition/src/bindings.rs",
        "--minimal",
        "--dead-code",
        "--flat",
        "--filter",
        "--etc",
        "crates/tools/composition/src/composition.txt",
    ];
    bindgen(args);

    println!(
        "tool_composition: generated bindings in {:.2}s",
        time.elapsed().as_secs_f32()
    );
}
