winrt::include_bindings!();

fn main() -> winrt::Result<()> {
    use windows::ui::Colors;

    let red = Colors::red()?;
    println!("Red: {:?}", red);

    Ok(())
}
