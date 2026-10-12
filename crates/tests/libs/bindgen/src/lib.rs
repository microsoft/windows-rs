#![expect(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]

include!(concat!(env!("OUT_DIR"), "/compile_fixtures.rs"));

#[test]
fn narrow_event_delegate_invoke() {
    let handler = narrow_event::Handler::new(|value| {
        assert_eq!(value.Number, 42);
        Err(windows_core::HRESULT(-1).into())
    });
    let error = handler
        .Invoke(narrow_event::Value { Number: 42 })
        .unwrap_err();
    assert_eq!(error.code(), windows_core::HRESULT(-1));
}

#[test]
fn narrow_generic_event_delegate_invoke() {
    let handler =
        narrow_event_generic::Handler::<narrow_event_generic::ISource>::new(|_, value| {
            assert_eq!(value, 42);
            Err(windows_core::HRESULT(-1).into())
        });
    let error = handler
        .Invoke(None::<&narrow_event_generic::ISource>, 42)
        .unwrap_err();
    assert_eq!(error.code(), windows_core::HRESULT(-1));
}

#[test]
fn narrow_collection_iteration_traits() {
    fn assert_into_iterator<T: IntoIterator<Item = i32>>() {}
    fn assert_iterator<T: Iterator<Item = i32>>() {}

    assert_into_iterator::<narrow_iterable_interface_full::IItems<i32>>();
    assert_into_iterator::<&narrow_iterable_interface_full::IItems<i32>>();
    assert_into_iterator::<narrow_iterable_names::IItems<i32>>();
    assert_into_iterator::<&narrow_iterable_names::IItems<i32>>();
    assert_into_iterator::<narrow_iterable_names::Items>();
    assert_into_iterator::<&narrow_iterable_names::Items>();
    assert_into_iterator::<narrow_iterable_first::IIterable<i32>>();
    assert_into_iterator::<&narrow_iterable_first::IIterable<i32>>();
    assert_iterator::<narrow_iterator_full::IIterator<i32>>();
}
