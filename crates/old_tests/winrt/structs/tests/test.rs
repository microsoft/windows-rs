use test_winrt_structs::*;
use windows::core::*;
use Component::Structs::*;
use Windows::Foundation::IReference;

#[test]
fn size() -> Result<()> {
    assert!(Test::SizeOfBlittable()? == core::mem::size_of::<Blittable>() as _);
    assert!(Test::SizeOfNonBlittable()? == core::mem::size_of::<NonBlittable>() as _);
    assert!(Test::SizeOfNested()? == core::mem::size_of::<Nested>() as _);
    assert_eq!(Test::GuidOfRefNested()?, IReference::<Nested>::IID);

    Ok(())
}
