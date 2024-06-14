use windows_bindgen::*;

fn main() -> Result<()> {
    bindgen(["--etc", "crates/tools/version/src/bindings.txt"])?;
    Ok(())
}
