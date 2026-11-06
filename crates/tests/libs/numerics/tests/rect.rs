use windows_numerics::Rect;

#[test]
fn rect_from_xywh_and_accessors() {
    let rect = Rect::from_xywh(10.0, 20.0, 30.0, 40.0);
    assert_eq!(rect, Rect::new(10.0, 20.0, 40.0, 60.0));
    assert_eq!(rect.width(), 30.0);
    assert_eq!(rect.height(), 40.0);

    let moved = rect.offset(5.0, -8.0);
    assert_eq!(moved, Rect::new(15.0, 12.0, 45.0, 52.0));
}
