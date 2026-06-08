use std::any::Any;

use super::*;

crate::impl_rc_fn_wrapper! {
    /// Renders a fallback subtree given a panic message string.
    pub struct Fallback(dyn Fn(&str) -> Element);
}

impl Fallback {
    pub fn invoke(&self, message: &str) -> Element {
        (self.inner)(message)
    }
}

/// Subtree wrapper that catches panics from descendants and renders the
/// `fallback` element instead. Carried by [`Element::ErrorBoundary`].
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct ErrorBoundaryElement {
    pub key: Option<String>,
    pub child: Box<Element>,
    pub fallback: Fallback,
}

/// Wrap `child` in an [`ErrorBoundaryElement`] using `fallback` for recovery.
pub fn error_boundary<F>(child: impl Into<Element>, fallback: F) -> Element
where
    F: Fn(&str) -> Element + 'static,
{
    Element::ErrorBoundary(ErrorBoundaryElement {
        key: None,
        child: Box::new(child.into()),
        fallback: Fallback::new(fallback),
    })
}

pub(crate) fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic>".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_invoke_returns_element() {
        let fb = Fallback::new(|msg| TextBlock::new(format!("oops: {msg}")).into());
        let e = fb.invoke("boom");
        match e {
            Element::TextBlock(t) => assert_eq!(t.text, "oops: boom"),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn fallback_clone_ptr_eq() {
        let fb1 = Fallback::new(|_| Element::Empty);
        let fb2 = fb1.clone();
        let fb3 = Fallback::new(|_| Element::Empty);
        assert_eq!(fb1, fb2);
        assert_ne!(fb1, fb3);
    }

    #[test]
    fn error_boundary_factory_produces_variant() {
        let eb = error_boundary(TextBlock::new("child"), |_| Element::Empty);
        assert!(matches!(eb, Element::ErrorBoundary(_)));
    }

    #[test]
    fn panic_message_handles_known_payload_shapes() {
        let e: Box<dyn Any + Send> = Box::new("literal");
        assert_eq!(panic_message(e), "literal");
        let e: Box<dyn Any + Send> = Box::new(String::from("owned"));
        assert_eq!(panic_message(e), "owned");
        let e: Box<dyn Any + Send> = Box::new(42_i32);
        assert_eq!(panic_message(e), "<non-string panic>");
    }
}
