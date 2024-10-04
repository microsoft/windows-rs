use windows::{core::*, Foundation::*};

#[test]
fn test() -> Result<()> {
    let class: MemoryBuffer = MemoryBuffer::Create(10)?;

    call_class(&class)?;
    call_interface((&class).into())?;
    as_class(&class)?;
    as_interface(&class)?;

    // `IMemoryBuffer` is `MemoryBuffer`'s default interface, even though it is not an exclusive interface.
    // So this `into` cast should succeed without requiring a call to `QueryInterface`.
    let interface: IMemoryBuffer = class.into();

    call_interface(&interface)?;
    as_interface(&interface)?;

    Ok(())
}

fn call_class(b: &MemoryBuffer) -> Result<()> {
    assert_eq!(10, b.CreateReference()?.Capacity()?);
    Ok(())
}

fn call_interface(b: &IMemoryBuffer) -> Result<()> {
    assert_eq!(10, b.CreateReference()?.Capacity()?);
    Ok(())
}

fn as_class<P: Param<MemoryBuffer>>(b: P) -> Result<()> {
    unsafe { call_class(b.param().borrow().as_ref().unwrap()) }
}

fn as_interface<P: Param<IMemoryBuffer>>(b: P) -> Result<()> {
    unsafe { call_interface(b.param().borrow().as_ref().unwrap()) }
}
