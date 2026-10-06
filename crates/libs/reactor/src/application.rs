use super::*;

/// Application state that outlives individual windows and notification icons.
///
/// The application exits when no user windows or declared notification icons remain and pending
/// lifecycle work has finished. Use [`ApplicationContext::exit`] to exit explicitly.
pub trait Application: Sized + 'static {
    type Input: 'static;
    type Message: 'static;

    fn create(input: &Self::Input, context: &ApplicationContext<Self>) -> Self;
    fn update(&mut self, message: Self::Message, context: &ApplicationContext<Self>);
    fn view(&self, input: &Self::Input, context: &ApplicationViewContext<Self>) -> ApplicationView;
}

/// Commands and typed messages for an application root.
pub struct ApplicationContext<A: Application> {
    pub(crate) sender: LocalSender<A::Message>,
    pub(crate) commands: RefCell<Vec<ApplicationCommand>>,
}

pub(crate) enum ApplicationCommand {
    Show(Key),
    Exit,
}

impl<A: Application> ApplicationContext<A> {
    pub fn sender(&self) -> LocalSender<A::Message> {
        self.sender.clone()
    }

    pub fn callback<T>(&self, map: impl Fn(T) -> A::Message + 'static) -> Callback<T> {
        self.sender.callback(map)
    }

    pub fn message(&self, message: A::Message) -> Callback<()>
    where
        A::Message: Clone,
    {
        self.sender.message(message)
    }

    pub fn forward(&self) -> Callback<A::Message> {
        self.sender.forward()
    }

    /// Opens, restores, or activates the declared window without creating duplicate instances.
    ///
    /// Requests are applied after the next application declaration is committed. An unknown key
    /// is an application error. A request during closure reopens after the old instance closes.
    pub fn show_window(&self, key: impl Into<Key>) {
        let key = key.into();
        let mut commands = self.commands.borrow_mut();
        if !commands
            .iter()
            .any(|command| matches!(command, ApplicationCommand::Show(current) if *current == key))
        {
            commands.push(ApplicationCommand::Show(key));
        }
    }

    /// Exits the application, closing its windows, menus, and notification icons.
    pub fn exit(&self) {
        let mut commands = self.commands.borrow_mut();
        if !commands
            .iter()
            .any(|command| matches!(command, ApplicationCommand::Exit))
        {
            commands.push(ApplicationCommand::Exit);
        }
    }
}

/// Typed callbacks used while describing application resources.
pub struct ApplicationViewContext<A: Application> {
    pub(crate) sender: LocalSender<A::Message>,
}

impl<A: Application> ApplicationViewContext<A> {
    pub fn callback<T>(&self, map: impl Fn(T) -> A::Message + 'static) -> Callback<T> {
        self.sender.callback(map)
    }

    pub fn message(&self, message: A::Message) -> Callback<()>
    where
        A::Message: Clone,
    {
        self.sender.message(message)
    }

    pub fn forward(&self) -> Callback<A::Message> {
        self.sender.forward()
    }
}

/// Keyed application resources, independent of any visual tree.
#[derive(Default)]
pub struct ApplicationView {
    pub(crate) windows: Vec<(Key, ComponentNode)>,
    pub(crate) icons: Vec<(Key, NotifyIcon)>,
}

impl ApplicationView {
    pub fn new() -> Self {
        Self::default()
    }

    /// Declares a reopenable window. Declaration alone does not open it.
    ///
    /// An open window receives updated inputs without recreating its component. Closing the
    /// window drops its component state; showing it again creates a new component.
    pub fn window<C: Component>(mut self, key: impl Into<Key>, input: C::Input) -> Self {
        let key = key.into();
        self.windows.push((key.clone(), component::<C>(key, input)));
        self
    }

    /// Declares a notification icon. Omit its key from the next view to remove it.
    pub fn notify_icon(mut self, key: impl Into<Key>, icon: NotifyIcon) -> Self {
        self.icons.push((key.into(), icon));
        self
    }

    pub(crate) fn validate(&self) -> windows_core::Result<()> {
        let mut keys = HashSet::new();
        for (key, _) in &self.windows {
            if !keys.insert(key) {
                return Err(application_error("duplicate application window key"));
            }
        }
        keys.clear();
        for (key, _) in &self.icons {
            if !keys.insert(key) {
                return Err(application_error("duplicate notification icon key"));
            }
        }
        Ok(())
    }
}

/// A notification-area icon owned by an application declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct NotifyIcon {
    pub(crate) path: std::path::PathBuf,
    pub(crate) tooltip: Option<String>,
    pub(crate) on_activate: Option<Callback<ScreenPoint>>,
    pub(crate) menu: Option<Menu>,
}

impl NotifyIcon {
    /// Uses an `.ico` file. Loading failures are returned by [`App::run_application`].
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            path: path.into(),
            tooltip: None,
            on_activate: None,
            menu: None,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn on_activate(mut self, callback: impl IntoPayloadCallback<ScreenPoint>) -> Self {
        self.on_activate = Some(callback.into_payload_callback());
        self
    }

    /// Attaches an application menu positioned from mouse or keyboard Shell invocation.
    pub fn menu(mut self, menu: Menu) -> Self {
        self.menu = Some(menu);
        self
    }
}

pub(crate) fn application_error(message: &str) -> windows_core::Error {
    windows_core::Error::new(windows_core::HRESULT(0x80070057u32 as i32), message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::ApplicationMessages;

    struct TestApp;

    impl Application for TestApp {
        type Input = ();
        type Message = bool;

        fn create(_: &(), _: &ApplicationContext<Self>) -> Self {
            Self
        }
        fn update(&mut self, _: bool, _: &ApplicationContext<Self>) {}
        fn view(&self, _: &(), _: &ApplicationViewContext<Self>) -> ApplicationView {
            ApplicationView::new()
        }
    }

    struct TestWindow;

    impl Component for TestWindow {
        type Input = ();
        type Message = ();

        fn create(_: &(), _: &ComponentContext<Self>) -> Self {
            Self
        }
        fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
            TextBlock::new().into()
        }
    }

    #[test]
    fn application_resource_keys_are_unique_within_each_kind() {
        assert!(
            ApplicationView::new()
                .window::<TestWindow>("main", ())
                .notify_icon("main", NotifyIcon::new("icon.ico"))
                .validate()
                .is_ok()
        );
        assert!(
            ApplicationView::new()
                .window::<TestWindow>("main", ())
                .window::<TestWindow>("main", ())
                .validate()
                .is_err()
        );
        assert!(
            ApplicationView::new()
                .notify_icon("tray", NotifyIcon::new("icon.ico"))
                .notify_icon("tray", NotifyIcon::new("icon.ico"))
                .validate()
                .is_err()
        );
    }

    #[test]
    fn application_commands_coalesce_without_mutating_declarations() {
        let messages = ApplicationMessages::new(|| {});
        let context = ApplicationContext::<TestApp> {
            sender: messages.sender(),
            commands: RefCell::new(Vec::new()),
        };
        context.show_window("main");
        context.show_window("other");
        context.show_window("main");
        context.exit();
        context.exit();
        let commands = context.commands.borrow();
        assert_eq!(commands.len(), 3);
        assert!(matches!(&commands[0], ApplicationCommand::Show(key) if *key == Key::from("main")));
        assert!(
            matches!(&commands[1], ApplicationCommand::Show(key) if *key == Key::from("other"))
        );
        assert!(matches!(&commands[2], ApplicationCommand::Exit));
    }

    #[test]
    fn application_callbacks_preserve_identity_and_retire_with_the_root() {
        let messages = ApplicationMessages::new(|| {});
        let context = ApplicationViewContext::<TestApp> {
            sender: messages.sender(),
        };
        fn callback(context: &ApplicationViewContext<TestApp>) -> Callback<()> {
            context.callback(|()| true)
        }
        let first = callback(&context);
        assert_eq!(first, callback(&context));
        first.call(());
        assert_eq!(messages.pop(), Some(true));
        assert!(messages.is_empty());
        let sender = messages.sender();
        drop(messages);
        assert!(!sender.send(false));
        first.call(());
    }

    #[test]
    fn independent_application_roots_do_not_share_callback_identity() {
        fn callback(messages: &ApplicationMessages<bool>) -> Callback<()> {
            messages.sender().callback(|()| true)
        }
        let first = ApplicationMessages::new(|| {});
        let second = ApplicationMessages::new(|| {});
        assert_ne!(callback(&first), callback(&second));
        callback(&first).call(());
        assert_eq!(first.pop(), Some(true));
        assert!(second.is_empty());
    }

    struct ObservedWindow(Callback<&'static str>);

    impl Component for ObservedWindow {
        type Input = (u32, Callback<&'static str>);
        type Message = ();

        fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
            input.1.call("create");
            Self(input.1.clone())
        }

        fn input_changed(&mut self, _: &Self::Input, _: &ComponentContext<Self>) {
            self.0.call("input");
        }

        fn view(&self, input: &Self::Input, _: &mut ViewContext<Self>) -> View {
            self.0.call("view");
            TextBlock::new().text(input.0.to_string()).into()
        }
    }

    impl Drop for ObservedWindow {
        fn drop(&mut self) {
            self.0.call("drop");
        }
    }

    #[test]
    fn live_root_input_updates_preserve_component_state() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&events);
        let callback = Callback::new(move |event| record.borrow_mut().push(event));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<ObservedWindow>("main", (1, callback.clone()))],
        )
        .unwrap();
        host.update_root(component::<ObservedWindow>("main", (1, callback.clone())))
            .unwrap();
        assert_eq!(*events.borrow(), ["create", "view"]);
        host.update_root(component::<ObservedWindow>("main", (2, callback)))
            .unwrap();
        assert_eq!(*events.borrow(), ["create", "view", "input", "view"]);
        assert!(matches!(
            host.update_root(component::<TestWindow>("main", ())),
            Err(ComponentError::ComponentType(_))
        ));
        drop(host);
        assert_eq!(
            *events.borrow(),
            ["create", "view", "input", "view", "drop"]
        );
    }
}
