use windows_bindgen2::*;

fn main() {
    bindgen(["--etc", "crates/tools/bindings/src/core_com.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/core.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/bindgen.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/registry.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/result.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/strings.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/sys.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/version.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/windows.txt"]);
}
