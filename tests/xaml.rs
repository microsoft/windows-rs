winrt::import!(
    dependencies
        os
    types
        windows::ui::xaml::*
);
#[test]
fn xaml() -> winrt::Result<()> {
    use winrt::ComInterface;
    let app = windows::ui::xaml::Application::default();
    assert!(app.is_null());

    Ok(())
}
