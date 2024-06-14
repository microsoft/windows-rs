use windows_bindgen::*;

fn main() -> Result<()> {
    bindgen(["--etc", "crates/tools/core/src/bindings.txt"])?;
    bindgen(["--etc", "crates/tools/core/src/com_bindings.txt"])?;
    Ok(())
}
