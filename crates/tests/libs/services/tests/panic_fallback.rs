#[test]
#[should_panic(
    expected = "Service was invoked as a console application without a fallback callback."
)]
fn panic_fallback() {
    windows_services::ServiceBuilder::new()
        .run(|_, _| {})
        .unwrap();
}
