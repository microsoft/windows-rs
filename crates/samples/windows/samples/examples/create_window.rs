fn main() -> windows_window::Result<()> {
    use windows_window::{Window, run_with};

    let _window = Window::new("This is a sample window").create()?;

    run_with(|| {
        println!("rendering");
        Ok(false)
    })?;

    println!("window closed");
    Ok(())
}
