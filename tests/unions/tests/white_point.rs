use test_unions::Windows::Win32::WindowsColorSystem::{WhitePoint, WhitePoint_0, XYYPoint};

#[test]
fn test() {
    assert_eq!(std::mem::size_of::<WhitePoint>(), 16);

    let mut wp = WhitePoint {
        r#type: WhitePoint::TEMPERATURE,
        Anonymous: WhitePoint_0 { CCT: 1.2 },
    };

    assert_eq!(wp.r#type, 1);
    wp.r#type = WhitePoint::D65;
    assert_eq!(wp.r#type, 2);
    wp.r#type = WhitePoint::CHROMATICITY;
    assert_eq!(wp.r#type, 0);

    unsafe {
        assert_eq!(wp.Anonymous.CCT, 1.2);
    }

    wp.Anonymous.CCT = 3.4;

    unsafe {
        assert_eq!(wp.Anonymous.CCT, 3.4);
    }

    wp.Anonymous.xyY.x = 1.0;
    wp.Anonymous.xyY.y = 2.0;
    wp.Anonymous.xyY.Y = 3.0;

    unsafe {
        assert_eq!(wp.Anonymous.xyY.x, 1.0);
        assert_eq!(wp.Anonymous.xyY.y, 2.0);
        assert_eq!(wp.Anonymous.xyY.Y, 3.0);
    }

    wp.Anonymous.xyY = XYYPoint {
        x: 10.0,
        y: 11.0,
        Y: 12.0,
    };

    unsafe {
        assert_eq!(wp.Anonymous.xyY.x, 10.0);
        assert_eq!(wp.Anonymous.xyY.y, 11.0);
        assert_eq!(wp.Anonymous.xyY.Y, 12.0);
    }
}
