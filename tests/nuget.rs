winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    modules
        "windows.ui.composition"
);
use windows::ui::composition::LayerVisual;

#[test]
fn nuget() -> winrt::Result<()> {
    LayerVisual::default();
    Ok(())
}
