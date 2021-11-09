// This test validates that free functions are not affected by the Visual C++ quirk affecting member functions
// returning structs where the return value is effectively a trailing out parameter.
//
// TODO: D2D1ConvertColorSpace is exported by ordinal rather than by name and Rust doesn't appear to support
// this at the moment. It happens to work on x64 but not on x86, hence the cfg check below.
#[test]
#[cfg(target_pointer_width = "64")]
fn test() {
    use test_return_struct::Windows::Win32::Graphics::Direct2D::{Common::*, *};

    let before = D2D1_COLOR_F { r: 1.0, g: 2.0, b: 3.0, a: 4.0 };

    let after = unsafe { D2D1ConvertColorSpace(D2D1_COLOR_SPACE_SRGB, D2D1_COLOR_SPACE_SCRGB, &before) };

    let expected = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 4.0 };

    assert!(after == expected);
}
