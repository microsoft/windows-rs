import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
        nuget: KennyKerr.Windows.TestWinRT
    types
        test_component::Composable
);

use test_component::Composable;
use winrt::*;

#[test]
fn composable() -> Result<()> {
    Ok(())
}
