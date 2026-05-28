use super::*;

/// One entry of a widget's flat declarative binding list: either a
/// property value or an event handler slot.
#[derive(Debug, Clone, PartialEq)]
pub enum Binding {
    Prop(Prop, PropValue),
    Event(Event, Option<EventHandler>),
}

pub type PropBindings = Vec<Binding>;

pub(crate) fn find_prop(bindings: &[Binding], prop: Prop) -> Option<&PropValue> {
    bindings.iter().find_map(|b| match b {
        Binding::Prop(p, v) if *p == prop => Some(v),
        _ => None,
    })
}

pub(crate) fn find_event(bindings: &[Binding], event: Event) -> Option<&Option<EventHandler>> {
    bindings.iter().find_map(|b| match b {
        Binding::Event(e, h) if *e == event => Some(h),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_prop_returns_first_match() {
        let bs = vec![
            Binding::Prop(Prop::Text, PropValue::Str("hi".into())),
            Binding::Prop(Prop::FontSize, PropValue::F64(12.0)),
        ];
        assert_eq!(
            find_prop(&bs, Prop::Text),
            Some(&PropValue::Str("hi".into()))
        );
        assert_eq!(find_prop(&bs, Prop::FontSize), Some(&PropValue::F64(12.0)));
        assert_eq!(find_prop(&bs, Prop::Background), None);
    }

    #[test]
    fn find_event_distinguishes_some_and_none() {
        let bs = vec![Binding::Event(Event::Click, None)];
        assert!(matches!(find_event(&bs, Event::Click), Some(None)));
        assert!(find_event(&bs, Event::Toggled).is_none());
    }
}
