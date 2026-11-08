fn main() -> windows_core::Result<()> {
    let fixture = std::env::args().nth(1).unwrap();
    test_reactor::run(&fixture)?;

    // Rust COM objects and delegates must be released at shutdown. Weak reference tear-offs
    // are not checked: WinUI retains a weak reference to the application after App::run returns.
    #[cfg(windows_cast_diagnostics)]
    {
        let live = windows_core::imp::live_objects();
        assert_eq!((live.objects, live.delegates), (0, 0), "{live:?}");
    }

    Ok(())
}
