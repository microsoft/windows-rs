use windows_bindgen::*;

fn main() -> Result<()> {
    bindgen(["--etc", "crates/tools/metadata/src/bindings.txt"])?;
    Ok(())
}
