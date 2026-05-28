use super::*;

/// Stateless render unit parameterised by props `P`. Implementors return
/// an [`Element`] tree from `render`; the host re-invokes `render` when
/// `should_update` returns `true` for the latest props.
pub trait Component<P = ()>: 'static
where
    P: Clone + PartialEq + 'static,
{
    fn render(&self, props: &P, cx: &mut RenderCx) -> Element;

    fn should_update(&self, old_props: &P, new_props: &P) -> bool {
        old_props != new_props
    }

    fn has_on_appeared(&self) -> bool {
        false
    }

    fn has_on_disappeared(&self) -> bool {
        false
    }

    fn on_appeared(&self, _props: &P, _cx: &mut RenderCx) {}

    fn on_disappeared(&self, _props: &P, _cx: &mut RenderCx) {}
}

/// Blanket impl: any `Fn(&P, &mut RenderCx) -> impl Into<Element>` is a [`Component<P>`].
///
/// ```ignore
/// fn greeting(props: &GreetingProps, _cx: &mut RenderCx) -> impl Into<Element> {
///     text_block(format!("Hello, {}!", props.name))
/// }
/// ```
///
/// For unit-props, use `fn(_: &(), cx: &mut RenderCx) -> impl Into<Element>`.
impl<F, P, R> Component<P> for F
where
    F: Fn(&P, &mut RenderCx) -> R + 'static,
    R: Into<Element>,
    P: Clone + PartialEq + 'static,
{
    fn render(&self, props: &P, cx: &mut RenderCx) -> Element {
        (self)(props, cx).into()
    }
}
