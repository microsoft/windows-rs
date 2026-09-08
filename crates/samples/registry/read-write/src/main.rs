use windows_registry::*;

fn main() -> Result<()> {
    let key = CURRENT_USER.create(r"software\windows-rs\sample")?;

    key.set_u32("number", 123)?;
    key.set_string("name", "Rust")?;

    println!("number = {}", key.get_u32("number")?);
    println!("name   = {}", key.get_string("name")?);

    CURRENT_USER.remove_tree(r"software\windows-rs\sample")?;
    Ok(())
}
