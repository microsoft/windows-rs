winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    modules
        "windows.ui.composition"
);

#[test]
fn nuget() -> winrt::Result<()> {
    Ok(())
}
