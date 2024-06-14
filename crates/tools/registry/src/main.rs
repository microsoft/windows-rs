use windows_bindgen::*;

fn main() -> Result<()> {
    bindgen(["--etc", "crates/tools/registry/src/bindings.txt"])?;
    Ok(())
}
