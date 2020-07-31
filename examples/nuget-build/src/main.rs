include!(concat!(env!("OUT_DIR"), "/winrt.rs"));

fn main() -> winrt::Result<()> {
    use windows::ui::Colors;

    let red = Colors::red()?;
    println!("Red: {:?}", red);

    Ok(())
}
