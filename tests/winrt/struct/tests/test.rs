use test_winrt_struct::*;
use Component::Struct::*;
use windows::*;

#[test]
fn blittable() -> Result<()> {
    assert!(Test::SizeOfBlittable()? == std::mem::size_of::<Blittable>() as _);

    let zero = Test::ZeroBlittable()?;
    let default = Blittable::default();
    assert!(zero == default);

    let mut b = Blittable::default();
    b.Bool = true;
    b.Char =  0x42; // L'B'
    b.UInt8 = 8;
    b.UInt16 = 16;
    b.UInt32 = 32;
    b.UInt64 = 64;
    b.Int16 = -16;
    b.Int32 = -32;
    b.Int64 = -64;
    b.Single = 1.23;
    b.Double = 4.56;
    b.Guid = "97FCC68A-30DE-42C0-AD91-0ADB63F4E934".into();
    
    assert_eq!(Test::NonZeroBlittable()?, b);

    Ok(())
}
