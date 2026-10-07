use super::*;
use crate::Application;
use crate::component::ApplicationMessages;
use crate::native::notifyicon::{NotifyIcon as NativeIcon, NotifyIconEvent};

struct ApplicationHost<A: Application> {
    state: Rc<RefCell<Option<ApplicationState<A>>>>,
    drain: AppCallback,
}

struct ApplicationState<A: Application> {
    model: A,
    input: A::Input,
    context: ApplicationContext<A>,
    messages: ApplicationMessages<A::Message>,
    application: LiveApplication,
    windows: HashMap<Key, WindowSlot>,
    icons: HashMap<Key, IconSlot>,
    pending_show: Vec<Key>,
    events: Rc<RefCell<VecDeque<IconEvent>>>,
    next_generation: u64,
    menu_host: Option<TransientMenuHost>,
    menu_owner: Option<Key>,
    exiting: bool,
}

struct WindowSlot {
    root: ComponentNode,
    instance: Option<u64>,
}

struct IconSlot {
    native: NativeIcon,
    declaration: NotifyIcon,
    generation: u64,
}

struct IconEvent {
    key: Key,
    generation: u64,
    event: NotifyIconEvent,
}

impl App {
    /// Runs application state with independently owned windows and notification icons.
    ///
    /// Window declarations are reopenable slots, not open windows. The application exits when
    /// no user windows or notification icons remain, after pending lifecycle work has settled.
    /// Native creation, update, and Shell recovery failures are returned as application errors.
    pub fn run_application<A: Application>(input: A::Input) -> windows_core::Result<()> {
        Self::run_with(move |context| ApplicationHost::<A>::new(context, input))
    }
}

impl<A: Application> ApplicationHost<A> {
    fn new(app: &AppContext, input: A::Input) -> windows_core::Result<Self> {
        let state: Rc<RefCell<Option<ApplicationState<A>>>> = Rc::new(RefCell::new(None));
        let weak = Rc::downgrade(&state);
        let application = LiveApplication::new(app, false)?;
        let drain = application.services.dispatch.callback(app, move || {
            if let Some(state) = weak.upgrade() {
                let mut state = state.borrow_mut();
                if let Some(state) = state.as_mut()
                    && let Err(error) = state.drain()
                {
                    state.exiting = true;
                    return Err(error);
                }
            }
            Ok(())
        });
        let host = Self { state, drain };
        let wake = host.drain.clone();
        let messages = ApplicationMessages::new(move || {
            if let Err(error) = wake.invoke() {
                report_error(error);
            }
        });
        let context = ApplicationContext {
            sender: messages.sender(),
            commands: RefCell::new(Vec::new()),
        };
        application.state.borrow_mut().changed = Some(host.drain.clone());
        let model = A::create(&input, &context);
        let mut state = ApplicationState {
            model,
            input,
            context,
            messages,
            application,
            windows: HashMap::new(),
            icons: HashMap::new(),
            pending_show: Vec::new(),
            events: Rc::new(RefCell::new(VecDeque::new())),
            next_generation: 1,
            menu_host: None,
            menu_owner: None,
            exiting: false,
        };
        state.render()?;
        *host.state.borrow_mut() = Some(state);
        host.drain.invoke()?;
        Ok(host)
    }
}

impl<A: Application> Drop for ApplicationHost<A> {
    fn drop(&mut self) {
        self.drain.clone().cancel();
        self.state.borrow_mut().take();
    }
}

impl<A: Application> Drop for ApplicationState<A> {
    fn drop(&mut self) {
        self.menu_host.take();
        self.icons.clear();
        self.application.state.borrow_mut().changed = None;
    }
}

impl<A: Application> ApplicationState<A> {
    fn render(&mut self) -> windows_core::Result<()> {
        if self
            .context
            .commands
            .borrow()
            .iter()
            .any(|command| matches!(command, ApplicationCommand::Exit))
        {
            return Ok(());
        }
        let view = self.model.view(
            &self.input,
            &ApplicationViewContext {
                sender: self.messages.sender(),
            },
        );
        view.validate()?;
        for (_, icon) in &view.icons {
            if let Some(menu) = &icon.menu {
                transient_menu::validate_menu_items(&menu.items, &mut HashSet::new())?;
            }
        }

        let window_keys: HashSet<_> = view.windows.iter().map(|(key, _)| key.clone()).collect();
        let removed: Vec<_> = self
            .windows
            .keys()
            .filter(|key| !window_keys.contains(*key))
            .cloned()
            .collect();
        for key in removed {
            let slot = self.windows.remove(&key).unwrap();
            self.pending_show.retain(|pending| pending != &key);
            if let Some(id) = slot.instance {
                self.application
                    .services
                    .push_critical(WindowWork::Close(id));
            }
        }
        for (key, root) in view.windows {
            if let Some(slot) = self.windows.get_mut(&key) {
                slot.root = root.clone();
                if let Some(id) = slot.instance {
                    self.application.services.update_input(root, id);
                }
            } else {
                self.windows.insert(
                    key,
                    WindowSlot {
                        root,
                        instance: None,
                    },
                );
            }
        }

        let icon_keys: HashSet<_> = view.icons.iter().map(|(key, _)| key.clone()).collect();
        let removed: Vec<_> = self
            .icons
            .keys()
            .filter(|key| !icon_keys.contains(*key))
            .cloned()
            .collect();
        for key in removed {
            self.dismiss_menu(&key)?;
            self.icons.remove(&key);
        }
        for (key, declaration) in view.icons {
            if self
                .icons
                .get(&key)
                .is_some_and(|icon| icon.declaration.menu != declaration.menu)
            {
                self.dismiss_menu(&key)?;
            }
            if let Some(icon) = self.icons.get_mut(&key) {
                if icon.declaration.path != declaration.path {
                    icon.native.set_icon(&declaration.path)?;
                }
                if icon.declaration.tooltip != declaration.tooltip {
                    icon.native.set_tooltip(declaration.tooltip.as_deref())?;
                }
                icon.declaration = declaration;
            } else {
                let generation = self.next_generation;
                self.next_generation = self.next_generation.checked_add(1).unwrap();
                let events = Rc::clone(&self.events);
                let event_key = key.clone();
                let wake = self.application.state.borrow().changed.clone().unwrap();
                let mut builder = NativeIcon::new(&declaration.path).on_event(move |event| {
                    let mut events = events.borrow_mut();
                    if events.len() >= WINDOW_WORK_CAPACITY {
                        drop(events);
                        report_error(application_error("notification event queue is full"));
                        return;
                    }
                    events.push_back(IconEvent {
                        key: event_key.clone(),
                        generation,
                        event,
                    });
                    drop(events);
                    if let Err(error) = wake.invoke() {
                        report_error(error);
                    }
                });
                if let Some(tooltip) = &declaration.tooltip {
                    builder = builder.tooltip(tooltip);
                }
                let native = builder.build()?;
                self.icons.insert(
                    key,
                    IconSlot {
                        native,
                        declaration,
                        generation,
                    },
                );
            }
        }
        Ok(())
    }

    fn dismiss_menu(&mut self, owner: &Key) -> windows_core::Result<()> {
        if self.menu_owner.as_ref() == Some(owner) {
            if let Some(host) = &self.menu_host {
                host.handle().hide()?;
            }
            self.menu_owner = None;
        }
        Ok(())
    }

    fn dispatch_icon_event(&mut self, event: IconEvent) -> windows_core::Result<()> {
        let Some(icon) = self
            .icons
            .get(&event.key)
            .filter(|icon| icon.generation == event.generation)
        else {
            return Ok(());
        };
        match event.event {
            NotifyIconEvent::Activate { position } => {
                if let Some(callback) = &icon.declaration.on_activate {
                    callback.call(ScreenPoint::new(position.x, position.y));
                }
            }
            NotifyIconEvent::ContextMenu { position } => {
                let Some(menu) = &icon.declaration.menu else {
                    return Ok(());
                };
                let menu = menu.clone();
                if self
                    .menu_host
                    .as_ref()
                    .is_some_and(|host| host.handle().is_open())
                {
                    if self.menu_owner.as_ref() == Some(&event.key) {
                        return Ok(());
                    }
                    if let Some(owner) = self.menu_owner.clone() {
                        self.dismiss_menu(&owner)?;
                    }
                }
                if self.menu_host.is_none() {
                    self.menu_host = Some(TransientMenuHost::new(
                        self.application.state.borrow().context.dispatcher.clone(),
                    )?);
                }
                self.menu_host
                    .as_ref()
                    .unwrap()
                    .handle()
                    .show(ScreenPoint::new(position.x, position.y), menu)?;
                self.menu_owner = Some(event.key);
            }
            NotifyIconEvent::Unavailable => {
                return Err(windows_core::Error::new(
                    E_FAIL,
                    "the Windows Shell could not restore the notification icon",
                ));
            }
        }
        Ok(())
    }

    fn drain(&mut self) -> windows_core::Result<()> {
        if self.exiting {
            return Ok(());
        }
        for _ in 0..COMPONENT_DRAIN_BUDGET {
            let event = self.events.borrow_mut().pop_front();
            let Some(event) = event else { break };
            self.dispatch_icon_event(event)?;
        }
        let mut changed = false;
        for _ in 0..COMPONENT_DRAIN_BUDGET {
            let Some(message) = self.messages.pop() else {
                break;
            };
            self.model.update(message, &self.context);
            changed = true;
            if self
                .context
                .commands
                .borrow()
                .iter()
                .any(|command| matches!(command, ApplicationCommand::Exit))
            {
                break;
            }
        }
        if changed {
            self.render()?;
        }
        let commands = std::mem::take(&mut *self.context.commands.borrow_mut());
        if commands
            .iter()
            .any(|command| matches!(command, ApplicationCommand::Exit))
        {
            self.exiting = true;
            return self.application.state.borrow().application.exit();
        }
        for command in commands {
            if let ApplicationCommand::Show(key) = command {
                if !self.windows.contains_key(&key) {
                    return Err(application_error("application window key is not declared"));
                }
                if !self.pending_show.contains(&key) {
                    self.pending_show.push(key);
                }
            }
        }
        let pending = self.pending_show.clone();
        for key in pending {
            let slot = self.windows.get_mut(&key).unwrap();
            if slot.instance.is_some_and(|id| {
                self.application
                    .services
                    .pending
                    .borrow()
                    .iter()
                    .any(|work| {
                        matches!(work, WindowWork::Close(current) | WindowWork::Closed(current)
                        | WindowWork::Input { window: current, .. }
                        | WindowWork::Publish { window: current, .. } if *current == id)
                    })
            }) {
                continue;
            }
            let instance = slot.instance.and_then(|id| {
                self.application
                    .state
                    .borrow()
                    .windows
                    .get(&id)
                    .map(|window| {
                        let state = window.state.borrow();
                        let state = state.as_ref().unwrap();
                        (
                            state.lifecycle.get(),
                            state.window.clone(),
                            state.host.has_pending_input(),
                        )
                    })
            });
            match instance {
                Some((ComponentWindowLifecycle::Open, window, false)) => {
                    window.restore_and_activate()?;
                    self.pending_show.retain(|pending| pending != &key);
                }
                Some(_) => {}
                None => {
                    slot.instance = Some(open_component_window(
                        &self.application.state,
                        &self.application.services,
                        slot.root.clone(),
                        WindowPolicy::new(),
                    )?);
                    self.pending_show.retain(|pending| pending != &key);
                }
            }
        }
        let pending_messages = !self.messages.is_empty() || !self.events.borrow().is_empty();
        if pending_messages {
            self.application
                .state
                .borrow()
                .changed
                .as_ref()
                .unwrap()
                .invoke()?;
        } else if self.icons.is_empty()
            && self.pending_show.is_empty()
            && !self.application.services.has_pending()
            && self.application.state.borrow().windows.is_empty()
        {
            self.exiting = true;
            self.application.state.borrow().application.exit()?;
        }
        Ok(())
    }
}
