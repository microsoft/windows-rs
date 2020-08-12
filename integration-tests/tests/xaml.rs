use tests::windows::ui::xaml::*;
use winrt::ComInterface;

#[test]
fn xaml() -> winrt::Result<()> {
    let app = Application::default();
    assert_eq!(app.is_null(), true);

    let app = Application::new()?;
    assert_eq!(app.is_null(), false);

    // The DataTemplateKey "constructors" fail because they're not
    // satisfying preconditions, but they still validate that the
    // composable constructors are being generated correctly.

    assert_eq!(
        DataTemplateKey::new().unwrap_err().code(),
        winrt::ErrorCode(0x8001_010E) // wrong thread
    );

    assert_eq!(
        DataTemplateKey::create_instance_with_type(winrt::Object::default())
            .unwrap_err()
            .code(),
        winrt::ErrorCode(0x8007_0057) // invalid argument
    );

    Ok(())
}
