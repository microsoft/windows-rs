mod bindings {
    ::windows::include_bindings!();
}

fn main() -> windows::Result<()> {
    use bindings::windows::ui::Colors;

    let red = Colors::red()?;
    println!("Red: {:?}", red);

    Ok(())
}
