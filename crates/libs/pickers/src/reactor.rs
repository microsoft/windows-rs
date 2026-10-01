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

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    struct LocalMessageComponent;

    impl Component for LocalMessageComponent {
        type Input = ();
        type Message = Rc<()>;

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(
            &self,
            _input: &Self::Input,
            _context: &mut windows_reactor::ViewContext<Self>,
        ) -> windows_reactor::View {
            "picker".into()
        }
    }

    fn request_with_local_messages(context: &ComponentContext<LocalMessageComponent>) {
        _ = OpenFilePicker::new().request(context, |_| Rc::new(()));
        _ = OpenFilePicker::new().request_multiple(context, |_| Rc::new(()));
        _ = FolderPicker::new().request(context, |_| Rc::new(()));
        _ = FolderPicker::new().request_multiple(context, |_| Rc::new(()));
        _ = SaveFilePicker::new().request(context, |_| Rc::new(()));
    }

    #[test]
    fn picker_requests_accept_ui_local_messages() {
        _ = request_with_local_messages as fn(&ComponentContext<LocalMessageComponent>);
    }
}
