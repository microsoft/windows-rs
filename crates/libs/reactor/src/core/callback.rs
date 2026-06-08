#[cfg(test)]
use std::rc::Rc;

crate::impl_rc_fn_wrapper! {
    /// Cheap-to-clone reference-counted callback. Two clones of the same
    /// `Callback` compare equal (`Rc` pointer equality), letting the
    /// reconciler skip rebinding when the same handler is re-rendered.
    pub struct Callback<T>(dyn Fn(T));
}

impl<T> Callback<T> {
    pub fn invoke(&self, arg: T) {
        (self.inner)(arg);
    }

    /// Construct a `Callback` from a raw `Rc<dyn Fn(T)>`. Used internally
    /// to bridge `SetState` / `Dispatch` into `Callback` without cloning.
    pub(crate) fn from_rc(inner: std::rc::Rc<dyn Fn(T)>) -> Self {
        Self { inner }
    }

    #[cfg(test)]
    pub(crate) fn strong_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

/// Trait for types that can be converted into a [`Callback<T>`].
///
/// Implemented for closures (`Fn(T) + 'static`) and state setters
/// ([`SetState<T>`](super::render_context::SetState),
/// [`Dispatch<A>`](super::render_context::Dispatch)), allowing them
/// to be passed directly to event handler methods without wrapping
/// in a manual closure.
///
/// # Examples
/// ```ignore
/// // Before: manual adapter closure
/// text_box(name).on_text_changed(move |v| set_name.call(v))
///
/// // After: pass the setter directly
/// text_box(name).on_text_changed(set_name)
/// ```
pub trait IntoCallback<T> {
    fn into_callback(self) -> Callback<T>;
}

impl<T, F> IntoCallback<T> for F
where
    F: Fn(T) + 'static,
{
    fn into_callback(self) -> Callback<T> {
        Callback::new(self)
    }
}

impl<T: 'static> IntoCallback<T> for Callback<T> {
    fn into_callback(self) -> Callback<T> {
        self
    }
}

impl<T: 'static> IntoCallback<T> for super::render_context::SetState<T> {
    fn into_callback(self) -> Callback<T> {
        self.into()
    }
}

impl<T: 'static> IntoCallback<T> for super::render_context::Dispatch<T> {
    fn into_callback(self) -> Callback<T> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

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
        let seen_c = Rc::clone(&seen);
        let cb = Callback::<i32>::new(move |v| seen_c.set(v));
        cb.invoke(7);
        assert_eq!(seen.get(), 7);
        cb.invoke(-3);
        assert_eq!(seen.get(), -3);
    }

    #[test]
    fn invoke_through_clone_touches_same_state() {
        let seen = Rc::new(Cell::new(0_i32));
        let seen_c = Rc::clone(&seen);
        let a = Callback::<i32>::new(move |v| seen_c.set(seen_c.get() + v));
        let b = a.clone();
        a.invoke(2);
        b.invoke(5);
        assert_eq!(seen.get(), 7);
    }

    #[test]
    fn debug_renders_pointer() {
        let cb = Callback::<()>::new(|()| {});
        let s = format!("{cb:?}");
        assert!(
            s.starts_with("Callback(") && s.ends_with(')'),
            "unexpected debug output: {s}"
        );
    }
}
