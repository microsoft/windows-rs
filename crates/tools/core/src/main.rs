use windows_bindgen::*;

fn main() -> Result<()> {
    bindgen(["--etc", "crates/tools/core/bindings.txt"])?;
    bindgen(["--etc", "crates/tools/core/com_bindings.txt"])?;
    Ok(())
}
