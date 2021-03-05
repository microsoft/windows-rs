use test_unions::windows::win32::windows_color_system::{WhitePoint, WhitePoint_0, XYYPoint};

#[test]
fn test() {
    assert!(std::mem::size_of::<WhitePoint>() == 16);

    let mut wp = WhitePoint {
        r#type: WhitePoint::TEMPERATURE,
        anonymous: WhitePoint_0 { cct: 1.2 },
    };

    assert_eq!(wp.r#type, 1);
    wp.r#type = WhitePoint::D65;
    assert_eq!(wp.r#type, 2);
    wp.r#type = WhitePoint::CHROMATICITY;
    assert_eq!(wp.r#type, 0);

    unsafe {
        assert_eq!(wp.anonymous.cct, 1.2);
    }

    wp.anonymous.cct = 3.4;

    unsafe {
        assert_eq!(wp.anonymous.cct, 3.4);
    }

    wp.anonymous.xyy.x = 1.0;
    wp.anonymous.xyy.y = 2.0;
    wp.anonymous.xyy.y2 = 3.0;

    unsafe {
        assert_eq!(wp.anonymous.xyy.x, 1.0);
        assert_eq!(wp.anonymous.xyy.y, 2.0);
        assert_eq!(wp.anonymous.xyy.y2, 3.0);
    }

    wp.anonymous.xyy = XYYPoint {
        x: 10.0,
        y: 11.0,
        y2: 12.0,
    };

    unsafe {
        assert_eq!(wp.anonymous.xyy.x, 10.0);
        assert_eq!(wp.anonymous.xyy.y, 11.0);
        assert_eq!(wp.anonymous.xyy.y2, 12.0);
    }
}
