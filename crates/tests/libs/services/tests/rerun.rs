use windows_services::ServiceBuilder;

#[test]
fn rerun() {
    let mut service = ServiceBuilder::new();
    assert!(matches!(
        service.run(|_, _| {}),
        Err(windows_services::Error::NotAService)
    ));
    assert!(matches!(
        service.run(|_, _| {}),
        Err(windows_services::Error::Running)
    ))
}
