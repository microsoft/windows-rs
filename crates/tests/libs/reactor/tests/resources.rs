use std::collections::HashMap;
use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::{
    Color, CornerRadius, Element, Prop, PropValue, Reconciler, ResourceExt, ResourceValue,
    Thickness, button,
};

fn rr() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn resource_updates(reconciler: &Reconciler<RecordingBackend>) -> Vec<PropValue> {
    reconciler
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetProp {
                prop: Prop::Resources,
                value,
                ..
            } => Some(value.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn resources_support_heterogeneous_typed_values() {
    let element: Element = button("Styled")
        .resource_overrides(|resources| {
            resources
                .set("ButtonBackground", Color::rgb(178, 34, 34))
                .set("ButtonBorderThemeThickness", Thickness::uniform(0.0))
                .set("ReactorScalar", 12.0)
                .set("ControlCornerRadius", CornerRadius::uniform(6.0))
                .set("Label", "Destructive")
        })
        .into();

    let mut reconciler = Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, &element, None, rr());

    let expected = HashMap::from([
        (
            "ButtonBackground".into(),
            ResourceValue::SolidColorBrush(Color::rgb(178, 34, 34)),
        ),
        (
            "ButtonBorderThemeThickness".into(),
            ResourceValue::Thickness(Thickness::uniform(0.0)),
        ),
        ("ReactorScalar".into(), ResourceValue::F64(12.0)),
        (
            "ControlCornerRadius".into(),
            ResourceValue::CornerRadius(CornerRadius::uniform(6.0)),
        ),
        ("Label".into(), ResourceValue::String("Destructive".into())),
    ]);
    assert_eq!(
        resource_updates(&reconciler),
        vec![PropValue::Resources(expected)]
    );
}

#[test]
fn resources_preserves_the_iterator_api() {
    let element: Element = button("Styled")
        .resources([("Label", "Destructive")])
        .into();

    let mut reconciler = Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, &element, None, rr());

    assert_eq!(
        resource_updates(&reconciler),
        vec![PropValue::Resources(HashMap::from([(
            "Label".into(),
            ResourceValue::String("Destructive".into()),
        )]))]
    );
}

#[test]
fn clearing_resources_emits_an_empty_replacement() {
    let old: Element = button("Styled")
        .resource_overrides(|resources| resources.set("ButtonBackground", Color::rgb(178, 34, 34)))
        .into();
    let new: Element = button("Styled").into();

    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let id = reconciler.reconcile(None, &old, None, rr()).unwrap();
    reconciler.backend.clear_ops();
    reconciler.reconcile(Some(&old), &new, Some(id), rr());

    assert_eq!(
        resource_updates(&reconciler),
        vec![PropValue::Resources(HashMap::new())]
    );
}
