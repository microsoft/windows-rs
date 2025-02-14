use windows_bindgen::*;

fn main() {
    let time = std::time::Instant::now();

    bindgen(["--etc", "crates/tools/bindings/src/async.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/collections.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/core_com.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/core.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/metadata.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/numerics.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/registry.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/result.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/strings.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/sys.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/version.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/windows.txt"]);

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
