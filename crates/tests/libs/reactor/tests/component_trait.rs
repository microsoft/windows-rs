use windows_reactor::core::component::Component;
use windows_reactor::core::element::{Element, TextBlock};
use windows_reactor::core::render_context::RenderCx;

struct Noop;
impl Component for Noop {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        Element::Empty
    }
}

#[test]
fn propless_component_compiles_and_renders() {
    let c = Noop;
    let mut cx = RenderCx::for_test();
    assert_eq!(c.render(&(), &mut cx), Element::Empty);
}

#[derive(Clone, PartialEq)]
struct Greeting {
    who: String,
}

struct GreetingView;
impl Component<Greeting> for GreetingView {
    fn render(&self, props: &Greeting, _cx: &mut RenderCx) -> Element {
        TextBlock::new(format!("hi {}", props.who)).into()
    }
}

#[test]
fn props_component_receives_props() {
    let c = GreetingView;
    let mut cx = RenderCx::for_test();
    let out = c.render(&Greeting { who: "x".into() }, &mut cx);
    match out {
        Element::TextBlock(t) => assert_eq!(t.text, "hi x"),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn default_should_update_is_partial_eq() {
    let c = GreetingView;
    let a = Greeting { who: "a".into() };
    let b = Greeting { who: "b".into() };
    assert!(!c.should_update(&a, &a));
    assert!(c.should_update(&a, &b));
}
