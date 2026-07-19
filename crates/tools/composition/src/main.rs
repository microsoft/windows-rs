use windows_bindgen::bindgen;

fn main() {
    let time = std::time::Instant::now();

    // Microsoft.UI.Composition lives in Microsoft.UI.winmd (staged for the
    // reactor tool); Windows.winmd supplies the shared Windows.Foundation /
    // Windows.UI types it references. Flat + minimal keeps the crate's own
    // surface small and namespace-free (see docs/crates/windows-composition.md).
    let args = [
        "--in",
        "crates/tools/reactor/winmd/Microsoft.UI.winmd",
        "crates/libs/bindgen/default/Windows.winmd",
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
