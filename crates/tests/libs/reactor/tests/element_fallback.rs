use std::any::Any;

use windows_reactor::{Element, Fallback, TextBlock, error_boundary, panic_message};

#[test]
fn fallback_invoke_returns_element() {
    let fallback = Fallback::new(|msg| TextBlock::new(format!("oops: {msg}")).into());
    let element = fallback.invoke("boom");
    match element {
        Element::TextBlock(text) => assert_eq!(text.text, "oops: boom"),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn fallback_clone_ptr_eq() {
    let fallback1 = Fallback::new(|_| Element::Empty);
    let fallback2 = fallback1.clone();
    let fallback3 = Fallback::new(|_| Element::Empty);
    assert_eq!(fallback1, fallback2);
    assert_ne!(fallback1, fallback3);
}

#[test]
fn error_boundary_factory_produces_variant() {
    let error_boundary = error_boundary(TextBlock::new("child"), |_| Element::Empty);
    assert!(matches!(error_boundary, Element::ErrorBoundary(_)));
}

#[test]
fn panic_message_handles_known_payload_shapes() {
    let payload: Box<dyn Any + Send> = Box::new("literal");
    assert_eq!(panic_message(payload), "literal");

    let payload: Box<dyn Any + Send> = Box::new(String::from("owned"));
    assert_eq!(panic_message(payload), "owned");

    let payload: Box<dyn Any + Send> = Box::new(42_i32);
    assert_eq!(panic_message(payload), "<non-string panic>");
}
