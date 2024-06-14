use windows_bindgen::*;

fn main() -> Result<()> {
    bindgen(["--etc", "crates/tools/result/src/bindings.txt"])?;
    Ok(())
}
