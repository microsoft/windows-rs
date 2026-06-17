use windows_reactor::{Binding, Event, Prop, PropValue, find_event, find_prop};

#[test]
fn find_prop_returns_first_match() {
    let bindings = vec![
        Binding::Prop(Prop::Text, PropValue::Str("hi".into())),
        Binding::Prop(Prop::FontSize, PropValue::F64(12.0)),
    ];
    assert_eq!(
        find_prop(&bindings, Prop::Text),
        Some(&PropValue::Str("hi".into()))
    );
    assert_eq!(
        find_prop(&bindings, Prop::FontSize),
        Some(&PropValue::F64(12.0))
    );
    assert_eq!(find_prop(&bindings, Prop::Background), None);
}

#[test]
fn find_event_distinguishes_some_and_none() {
    let bindings = vec![Binding::Event(Event::Click, None)];
    assert!(matches!(find_event(&bindings, Event::Click), Some(None)));
    assert!(find_event(&bindings, Event::Toggled).is_none());
}
