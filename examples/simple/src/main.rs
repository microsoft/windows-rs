mod bindings {
    windows::include_bindings!();
}

fn main() -> windows::Result<()> {
    use bindings::Windows::UI::Colors;

    let red = Colors::Red()?;
    println!("Red: {:?}", red);

    Ok(())
}
