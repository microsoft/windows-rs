use super::*;

/// How a widget exposes its children to the child reconciler.
pub(crate) enum Children<'a> {
    None,
    PositionalSingle(&'a Element),
    Keyed(&'a [Element]),
    Tabs(&'a [TabItem]),
    PivotItems(&'a [PivotItem]),
}

/// Adapter implemented by every built-in [`Element`] widget variant; lets
/// the reconciler drive backend prop / child operations uniformly.
pub(crate) trait Widget {
    fn kind(&self) -> ControlKind;
    fn key(&self) -> Option<&str>;
    fn modifiers(&self) -> &Modifiers;
    fn attached(&self) -> Option<&AttachedProps> {
        self.modifiers().attached.as_ref()
    }
    fn bindings(&self) -> PropBindings;
    fn children(&self) -> Children<'_> {
        Children::None
    }
    /// Optional element tree for a header slot (e.g. Expander complex header).
    fn header_element(&self) -> Option<&Element> {
        None
    }
    /// Optional element tree for a pane slot (e.g. SplitView pane).
    fn pane_element(&self) -> Option<&Element> {
        None
    }
    /// Optional post-mount callback. When present, the reconciler invokes it
    /// with the native element (`IInspectable`) immediately after creation.
    fn on_mounted_callback(
        &self,
    ) -> Option<&crate::core::callback::Callback<windows_core::IInspectable>> {
        None
    }
}
