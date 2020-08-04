winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::ui::Colors
);

fn main() -> winrt::Result<()> {
    use windows::ui::Colors;

    let red = Colors::red()?;
    println!("Red: {:?}", red);

    Ok(())
}
