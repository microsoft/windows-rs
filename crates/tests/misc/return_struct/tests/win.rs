// This test validates that free functions are not affected by the Visual C++ quirk affecting member functions
// returning structs where the return value is effectively a trailing out parameter.
#[test]
fn test() {
    use windows::Win32::Graphics::Direct2D::{Common::*, *};

    let before = D2D1_COLOR_F {
        r: 1.0,
        g: 2.0,
        b: 3.0,
        a: 4.0,
    };

    let after =
        unsafe { D2D1ConvertColorSpace(D2D1_COLOR_SPACE_SRGB, D2D1_COLOR_SPACE_SCRGB, &before) };

    let expected = D2D1_COLOR_F {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 4.0,
    };

    assert!(after == expected);
}
