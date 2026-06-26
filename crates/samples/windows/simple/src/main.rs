fn main() -> windows::core::Result<()> {
    use windows::UI::Colors;

    let red = Colors::Red()?;
    println!("Red: {red:?}");

    Ok(())
}
