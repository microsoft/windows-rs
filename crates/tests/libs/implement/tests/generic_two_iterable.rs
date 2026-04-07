use windows::core::*;
use windows_collections::*;

#[implement(
    IIterable<i32>,
    IIterable<HSTRING>,
)]
struct Thing();

impl IIterable_Impl<i32> for Thing_Impl {
    fn First(&self) -> Result<IIterator<i32>> {
        panic!();
    }
}

impl IIterable_Impl<HSTRING> for Thing_Impl {
    fn First(&self) -> Result<IIterator<HSTRING>> {
        panic!();
    }
}

#[test]
fn test_implement() -> Result<()> {
    let a: IIterable<i32> = Thing().into();
    let _: IIterable<HSTRING> = a.cast()?;

    Ok(())
}
