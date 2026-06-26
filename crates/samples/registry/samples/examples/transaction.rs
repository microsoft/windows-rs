use windows_registry::*;

fn main() -> Result<()> {
    let tx = Transaction::new()?;

    let key = CURRENT_USER
        .options()
        .read()
        .write()
        .create()
        .transaction(&tx)
        .open(r"software\windows-rs\sample")?;

    key.set_u32("number", 123)?;
    tx.commit()?;

    println!("number = {}", key.get_u32("number")?);

    CURRENT_USER.remove_tree(r"software\windows-rs\sample")?;
    Ok(())
}
