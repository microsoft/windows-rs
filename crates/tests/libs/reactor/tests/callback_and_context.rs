use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::{Callback, Context, ContextId, ContextStack};

#[test]
fn clone_of_same_callback_compares_equal() {
    let a = Callback::<i32>::new(|_| {});
    let b = a.clone();
    assert_eq!(a, b);
    assert_eq!(a.strong_count(), 2);
}

#[test]
fn independently_constructed_callbacks_are_not_equal() {
    let a = Callback::<i32>::new(|_| {});
    let b = Callback::<i32>::new(|_| {});
    assert_ne!(a, b);
}

#[test]
fn invoke_delivers_argument() {
    let seen = Rc::new(Cell::new(0_i32));
    let seen_clone = Rc::clone(&seen);
    let callback = Callback::<i32>::new(move |value| seen_clone.set(value));
    callback.invoke(7);
    assert_eq!(seen.get(), 7);
    callback.invoke(-3);
    assert_eq!(seen.get(), -3);
}

#[test]
fn invoke_through_clone_touches_same_state() {
    let seen = Rc::new(Cell::new(0_i32));
    let seen_clone = Rc::clone(&seen);
    let a = Callback::<i32>::new(move |value| seen_clone.set(seen_clone.get() + value));
    let b = a.clone();
    a.invoke(2);
    b.invoke(5);
    assert_eq!(seen.get(), 7);
}

#[test]
fn debug_renders_pointer() {
    let callback = Callback::<()>::new(|()| {});
    let output = format!("{callback:?}");
    assert!(
        output.starts_with("Callback(") && output.ends_with(')'),
        "unexpected debug output: {output}"
    );
}

#[test]
fn context_ids_are_unique() {
    let a = ContextId::new();
    let b = ContextId::new();
    assert_ne!(a, b);
}

#[test]
fn context_new_assigns_unique_id() {
    let a = Context::<i32>::new(0);
    let b = Context::<i32>::new(0);
    assert_ne!(a.id, b.id);
}

#[test]
fn stack_get_returns_none_when_empty() {
    let stack = ContextStack::new();
    let id = ContextId::new();
    assert!(stack.get::<i32>(id).is_none());
}

#[test]
fn stack_push_pop_shadows_lookup() {
    let stack = ContextStack::new();
    let id = ContextId::new();
    assert!(stack.get::<i32>(id).is_none());
    {
        let _guard = stack.push(id, 42_i32);
        assert_eq!(stack.get::<i32>(id), Some(42));
    }
    assert!(stack.get::<i32>(id).is_none());
}

#[test]
fn nested_provides_return_innermost() {
    let stack = ContextStack::new();
    let id = ContextId::new();
    let _outer = stack.push(id, "outer".to_string());
    {
        let _inner = stack.push(id, "inner".to_string());
        assert_eq!(stack.get::<String>(id), Some("inner".into()));
    }
    assert_eq!(stack.get::<String>(id), Some("outer".into()));
}

#[test]
fn distinct_ids_dont_shadow() {
    let stack = ContextStack::new();
    let id_a = ContextId::new();
    let id_b = ContextId::new();
    let _a = stack.push(id_a, 1_i32);
    let _b = stack.push(id_b, 2_i32);
    assert_eq!(stack.get::<i32>(id_a), Some(1));
    assert_eq!(stack.get::<i32>(id_b), Some(2));
}

#[test]
#[should_panic(expected = "context type mismatch")]
fn mismatched_types_panic() {
    let stack = ContextStack::new();
    let id = ContextId::new();
    let _guard = stack.push(id, 1_i32);
    let _ = stack.get::<String>(id);
}
