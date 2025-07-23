fn main() -> Result<(), windows::core::HRESULT> {
    use windows::UI::Colors;

    let red = Colors::Red()?;
    println!("Red: {red:?}");

    Ok(())
}
