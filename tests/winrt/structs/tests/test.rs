use test_winrt_structs::*;
use windows::*;
use Component::Structs::*;

#[test]
fn size() -> Result<()> {
    assert!(Test::SizeOfBlittable()? == std::mem::size_of::<Blittable>() as _);
    assert!(Test::SizeOfNonBlittable()? == std::mem::size_of::<NonBlittable>() as _);
    assert!(Test::SizeOfNested()? == std::mem::size_of::<Nested>() as _);

    Ok(())
}
