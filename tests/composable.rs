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
    let c = Composable::new()?;
    assert_eq!(c.value()?, 0);

    let c = Composable::create_with_value(123)?;
    assert_eq!(c.value()?, 123);

    Ok(())
}
