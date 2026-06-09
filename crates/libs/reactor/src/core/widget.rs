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
    /// immediately after creation with the native element (`IInspectable`), or
    /// `None` if the backend exposes no native element for the control.
    fn on_mounted_callback(
        &self,
    ) -> Option<&crate::core::callback::Callback<Option<windows_core::IInspectable>>> {
        None
    }
    /// Optional pre-unmount callback. When present, the reconciler invokes it
    /// just before the control is destroyed, while the control still exists,
    /// with the native element (`IInspectable`), or `None` if the backend
    /// exposes no native element. Owners use this to tear down external
    /// resources bound to the control (e.g. join a render thread that presents
    /// into the control's swap chain) — teardown runs regardless of whether a
    /// native element is present.
    fn on_unmounted_callback(
        &self,
    ) -> Option<&crate::core::callback::Callback<Option<windows_core::IInspectable>>> {
        None
    }
}
