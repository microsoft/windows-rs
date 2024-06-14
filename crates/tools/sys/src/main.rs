use windows_bindgen::*;

fn main() -> Result<()> {
    bindgen(["--etc", "crates/tools/sys/src/bindings.txt"])?;
    Ok(())
}
