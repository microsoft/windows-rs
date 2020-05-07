winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui.xaml"
);

#[test]
fn xaml() -> winrt::Result<()> {
    Ok(())
}
