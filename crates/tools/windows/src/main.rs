use windows_bindgen::*;

fn main() -> Result<()> {
    bindgen(["--etc", "crates/tools/windows/src/bindings.txt"])?;
    Ok(())
}
