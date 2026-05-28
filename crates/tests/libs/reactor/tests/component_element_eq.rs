use windows_reactor::core::component::Component;
use windows_reactor::core::component_element::{component, memo};
use windows_reactor::core::element::{Element, TextBlock};
use windows_reactor::core::render_context::RenderCx;

#[derive(Clone, PartialEq, Debug)]
struct Greeting {
    who: String,
}

struct GreetingView;
impl Component<Greeting> for GreetingView {
    fn render(&self, props: &Greeting, _cx: &mut RenderCx) -> Element {
        TextBlock::new(format!("hi {}", props.who)).into()
    }
}

struct OtherView;
impl Component<Greeting> for OtherView {
    fn render(&self, props: &Greeting, _cx: &mut RenderCx) -> Element {
        TextBlock::new(format!("yo {}", props.who)).into()
    }
}

#[test]
fn component_factory_produces_component_variant() {
    let e = component(GreetingView, Greeting { who: "x".into() });
    assert!(matches!(e, Element::Component(_)));
}

#[test]
fn equivalent_components_compare_equal() {
    let a = component(GreetingView, Greeting { who: "x".into() });
    let b = component(GreetingView, Greeting { who: "x".into() });
    assert_eq!(a, b);
}

#[test]
fn different_props_compare_unequal() {
    let a = component(GreetingView, Greeting { who: "x".into() });
    let b = component(GreetingView, Greeting { who: "y".into() });
    assert_ne!(a, b);
}

#[test]
fn different_component_types_compare_unequal() {
    let a = component(GreetingView, Greeting { who: "x".into() });
    let b = component(OtherView, Greeting { who: "x".into() });
    assert_ne!(a, b);
}

#[test]
fn memo_flag_distinguishes_from_plain_component() {
    let a = component(GreetingView, Greeting { who: "x".into() });
    let b = memo(GreetingView, Greeting { who: "x".into() });
    assert_ne!(a, b);
}
