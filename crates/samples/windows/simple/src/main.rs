#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
mod imp {
    pub fn main() -> windows::core::Result<()> {
        use windows::UI::Colors;

        let red = Colors::Red()?;
        println!("Red: {red:?}");

        Ok(())
    }
}

#[cfg(windows)]
fn main() -> impl std::process::Termination {
    imp::main()
}
