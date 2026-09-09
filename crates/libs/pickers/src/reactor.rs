use super::*;

impl OpenFilePicker {
    /// Queues this picker for the component's owning Reactor window.
    ///
    /// `map` converts the picker result into a component message. A `true` return means the
    /// request was staged; Reactor can still discard it if publication fails, the component
    /// retires, or the window starts closing.
    #[must_use = "false means the picker request was not staged"]
    pub fn request<C, F>(self, context: &ComponentContext<C>, map: F) -> bool
    where
        C: Component,
        F: FnOnce(Result<Option<PathBuf>>) -> C::Message + 'static,
    {
        context.run_window(move |window| map(self.show_for_hwnd(window.as_raw())))
    }

    /// Queues this multiple-selection picker for the component's owning Reactor window.
    ///
    /// An empty vector means the user cancelled.
    #[must_use = "false means the picker request was not staged"]
    pub fn request_multiple<C, F>(self, context: &ComponentContext<C>, map: F) -> bool
    where
        C: Component,
        F: FnOnce(Result<Vec<PathBuf>>) -> C::Message + 'static,
    {
        context.run_window(move |window| map(self.show_multiple_for_hwnd(window.as_raw())))
    }
}

impl FolderPicker {
    /// Queues this picker for the component's owning Reactor window.
    ///
    /// `map` converts the picker result into a component message. A `true` return means the
    /// request was staged; Reactor can still discard it if publication fails, the component
    /// retires, or the window starts closing.
    #[must_use = "false means the picker request was not staged"]
    pub fn request<C, F>(self, context: &ComponentContext<C>, map: F) -> bool
    where
        C: Component,
        F: FnOnce(Result<Option<PathBuf>>) -> C::Message + 'static,
    {
        context.run_window(move |window| map(self.show_for_hwnd(window.as_raw())))
    }

    /// Queues this multiple-selection picker for the component's owning Reactor window.
    ///
    /// An empty vector means the user cancelled.
    #[must_use = "false means the picker request was not staged"]
    pub fn request_multiple<C, F>(self, context: &ComponentContext<C>, map: F) -> bool
    where
        C: Component,
        F: FnOnce(Result<Vec<PathBuf>>) -> C::Message + 'static,
    {
        context.run_window(move |window| map(self.show_multiple_for_hwnd(window.as_raw())))
    }
}

impl SaveFilePicker {
    /// Queues this picker for the component's owning Reactor window.
    ///
    /// `map` converts the picker result into a component message. A `true` return means the
    /// request was staged; Reactor can still discard it if publication fails, the component
    /// retires, or the window starts closing.
    #[must_use = "false means the picker request was not staged"]
    pub fn request<C, F>(self, context: &ComponentContext<C>, map: F) -> bool
    where
        C: Component,
        F: FnOnce(Result<Option<PathBuf>>) -> C::Message + 'static,
    {
        context.run_window(move |window| map(self.show_for_hwnd(window.as_raw())))
    }
}
