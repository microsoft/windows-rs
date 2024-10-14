use windows_bindgen2::*;

fn main() {
    //run("crates/tools/bindings/src/core_com.txt")?;
    run("crates/tools/bindings/src/core.txt");
    //run("crates/tools/bindings/src/metadata.txt")?;
    //run("crates/tools/bindings/src/registry.txt")?;
    //run("crates/tools/bindings/src/result.txt")?;
    //run("crates/tools/bindings/src/strings.txt")?;
    //run("crates/tools/bindings/src/sys.txt")?;
    //run("crates/tools/bindings/src/version.txt")?;
    //run("crates/tools/bindings/src/windows.txt")?;
}

fn run(path: &str) {
    println!("{path}");
    bindgen(["--etc", path]);
}
