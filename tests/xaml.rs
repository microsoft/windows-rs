winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui.xaml.controls"
);

#[test]
fn xaml() -> winrt::Result<()> {
    Ok(())
}
