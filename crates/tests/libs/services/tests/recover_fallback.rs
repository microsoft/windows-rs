#[test]
fn no_fallback_error() {
    assert!(matches!(
        windows_services::ServiceBuilder::new().run(|_, _| {}),
        Err(windows_services::Error::NotAService)
    ))
}
