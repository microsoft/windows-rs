#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use windows_reactor_next::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum WindowRole {
    Primary,
    Secondary,
}

impl WindowRole {
    fn label(self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Secondary => "Secondary",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Editor,
}

impl Page {
    fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Editor => "Editor",
        }
    }
}

#[derive(Default)]
struct LifecycleMetrics {
    cleanups: Cell<usize>,
    setups: Cell<usize>,
}

struct SharedApp {
    cancellations: Arc<AtomicUsize>,
    dark: Cell<bool>,
    lifecycle: HashMap<WindowRole, LifecycleMetrics>,
    senders: RefCell<HashMap<WindowRole, LocalSender<Message>>>,
    theme: Rc<Context<bool>>,
}

impl SharedApp {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            cancellations: Arc::new(AtomicUsize::new(0)),
            dark: Cell::new(false),
            lifecycle: HashMap::from([
                (WindowRole::Primary, LifecycleMetrics::default()),
                (WindowRole::Secondary, LifecycleMetrics::default()),
            ]),
            senders: RefCell::new(HashMap::new()),
            theme: Rc::new(Context::new(false)),
        })
    }

    fn broadcast(&self, message: Message) {
        for sender in self.senders.borrow().values() {
            _ = sender.send(message.clone());
        }
    }

    fn register(&self, role: WindowRole, sender: LocalSender<Message>) {
        assert!(self.senders.borrow_mut().insert(role, sender).is_none());
        self.lifecycle[&role]
            .setups
            .set(self.lifecycle[&role].setups.get() + 1);
    }

    fn unregister(&self, role: WindowRole) {
        assert!(self.senders.borrow_mut().remove(&role).is_some());
        self.lifecycle[&role]
            .cleanups
            .set(self.lifecycle[&role].cleanups.get() + 1);
        self.broadcast(Message::PeerClosed(role));
    }
}

#[derive(Clone)]
struct WorkspaceProps {
    role: WindowRole,
    shared: Rc<SharedApp>,
}

impl PartialEq for WorkspaceProps {
    fn eq(&self, other: &Self) -> bool {
        self.role == other.role && Rc::ptr_eq(&self.shared, &other.shared)
    }
}

struct Workspace {
    count: u32,
    editor_ref: ElementRef<TextBox>,
    note: String,
    page: Page,
    role: WindowRole,
    secondary_open: bool,
    sender: LocalSender<Message>,
    shared: Rc<SharedApp>,
    status: String,
    working: bool,
}

#[derive(Clone)]
enum Message {
    CloseWindow,
    Increment,
    Navigate(Page),
    NoteChanged(String),
    OpenSecondary,
    PeerClosed(WindowRole),
    SharedChanged,
    StartWork,
    ToggleTheme,
    WorkCancelled,
    WorkFinished,
}

#[derive(Clone, PartialEq)]
struct HomeProps {
    count: u32,
    increment: Callback<()>,
    role: WindowRole,
}

struct HomePage;

#[derive(Clone, PartialEq)]
struct EditorProps {
    changed: Callback<String>,
    editor_ref: ElementRef<TextBox>,
    note: String,
    role: WindowRole,
}

struct EditorPage;

#[derive(Clone)]
struct ThemeProps {
    context: Rc<Context<bool>>,
    role: WindowRole,
}

impl PartialEq for ThemeProps {
    fn eq(&self, other: &Self) -> bool {
        self.role == other.role && Rc::ptr_eq(&self.context, &other.context)
    }
}

struct ThemeBanner;

impl Component for Workspace {
    type Message = Message;
    type Props = WorkspaceProps;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        Self {
            count: 0,
            editor_ref: ElementRef::new(),
            note: format!("{} window note", props.role.label()),
            page: Page::Home,
            role: props.role,
            secondary_open: false,
            sender: context.sender(),
            shared: Rc::clone(&props.shared),
            status: "Ready".to_string(),
            working: false,
        }
    }

    fn update(&mut self, message: Message, context: &mut ComponentContext<Self>) {
        match message {
            Message::CloseWindow => {
                if !context.window().request_close() {
                    self.status = "Window close request was rejected".to_string();
                }
            }
            Message::Increment => self.count += 1,
            Message::Navigate(page) => self.page = page,
            Message::NoteChanged(note) => self.note = note,
            Message::PeerClosed(role) if role != self.role => {
                if role == WindowRole::Secondary {
                    self.secondary_open = false;
                }
                self.status = format!("{} window closed", role.label());
            }
            Message::OpenSecondary if self.role == WindowRole::Primary && !self.secondary_open => {
                let opened = context.open_window(View::component::<Self>(WorkspaceProps {
                    role: WindowRole::Secondary,
                    shared: Rc::clone(&self.shared),
                }));
                self.secondary_open = opened;
                if !opened {
                    self.status = "Secondary window open request was rejected".to_string();
                }
            }
            Message::SharedChanged => {}
            Message::StartWork if !self.working => {
                self.working = true;
                self.status = "Background work running...".to_string();
                let cancellations = Arc::clone(&self.shared.cancellations);
                context.spawn_background(move |cancellation| {
                    for _ in 0..50 {
                        std::thread::sleep(Duration::from_millis(10));
                        if cancellation.is_cancelled() {
                            cancellations.fetch_add(1, Ordering::AcqRel);
                            return Message::WorkCancelled;
                        }
                    }
                    Message::WorkFinished
                });
            }
            Message::ToggleTheme => {
                self.shared.dark.set(!self.shared.dark.get());
                self.shared.broadcast(Message::SharedChanged);
            }
            Message::WorkCancelled => {
                self.working = false;
                self.status = "Background work cancelled".to_string();
            }
            Message::WorkFinished => {
                self.working = false;
                self.status = "Background work finished".to_string();
            }
            Message::OpenSecondary | Message::PeerClosed(_) | Message::StartWork => {}
        }
    }

    fn view(&self, _props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        context.window_title(format!(
            "{} workspace - {}",
            self.role.label(),
            self.page.label()
        ));
        let shared = Rc::clone(&self.shared);
        let role = self.role;
        let sender = self.sender.clone();
        context.use_effect("window-registration", (), move || {
            shared.register(role, sender);
            Some(Box::new(move || shared.unregister(role)))
        });

        let editor_active = self.page == Page::Editor;
        let editor_ref = self.editor_ref.clone();
        context.use_effect("editor-focus", editor_active, move || {
            if editor_active {
                _ = editor_ref.request_focus();
            }
            None
        });

        let page = match self.page {
            Page::Home => View::component::<HomePage>(HomeProps {
                count: self.count,
                increment: context.message(Message::Increment),
                role: self.role,
            }),
            Page::Editor => View::component::<EditorPage>(EditorProps {
                changed: context.callback(Message::NoteChanged),
                editor_ref: self.editor_ref.clone(),
                note: self.note.clone(),
                role: self.role,
            }),
        };
        let header = StackPanel::new().spacing(4.0).children((
            View::component::<ThemeBanner>(ThemeProps {
                context: Rc::clone(&self.shared.theme),
                role: self.role,
            }),
            TextBlock::new().text(self.status.clone()),
            Button::new()
                .on_click(context.message(Message::Navigate(Page::Home)))
                .content(TextBlock::new().text("Home")),
            Button::new()
                .on_click(context.message(Message::Navigate(Page::Editor)))
                .content(TextBlock::new().text("Editor")),
            Button::new()
                .on_click(context.message(Message::ToggleTheme))
                .content(TextBlock::new().text("Toggle shared theme")),
            if self.role == WindowRole::Primary {
                Button::new()
                    .is_enabled(!self.secondary_open)
                    .on_click(context.message(Message::OpenSecondary))
                    .content(TextBlock::new().text("Open secondary window"))
            } else {
                View::empty()
            },
            Button::new()
                .is_enabled(!self.working)
                .on_click(context.message(Message::StartWork))
                .content(TextBlock::new().text("Start background work")),
            Button::new()
                .on_click(context.message(Message::CloseWindow))
                .content(TextBlock::new().text("Close this window")),
        ));

        View::provide(
            &self.shared.theme,
            self.shared.dark.get(),
            NavigationView::new().slots([
                SlotView::new(NavigationViewSlot::Header, header),
                SlotView::new(NavigationViewSlot::Content, page),
            ]),
        )
    }
}

impl Component for HomePage {
    type Message = ();
    type Props = HomeProps;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(4.0).children((
            TextBlock::new().text(format!(
                "{} home count: {}",
                props.role.label(),
                props.count
            )),
            Button::new()
                .on_click(props.increment.clone())
                .content(TextBlock::new().text("Increment local count")),
        ))
    }
}

impl Component for EditorPage {
    type Message = ();
    type Props = EditorProps;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(4.0).children((
            TextBlock::new().text(format!("{} editor", props.role.label())),
            TextBox::new()
                .element_ref(&props.editor_ref)
                .text(props.note.clone())
                .on_text_changed(props.changed.clone()),
        ))
    }
}

impl Component for ThemeBanner {
    type Message = ();
    type Props = ThemeProps;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        let dark = context.use_context(&props.context);
        TextBlock::new()
            .text(format!(
                "{} workspace - {} theme",
                props.role.label(),
                if dark { "dark" } else { "light" }
            ))
            .into()
    }
}

fn main() {
    bootstrap().unwrap();
    let shared = SharedApp::new();
    App::run_component::<Workspace>(WorkspaceProps {
        role: WindowRole::Primary,
        shared,
    })
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn sender(shared: &SharedApp, role: WindowRole) -> LocalSender<Message> {
        shared.senders.borrow()[&role].clone()
    }

    fn live_text(pump: &Pump<RecordingRuntime>, expected: &str) -> bool {
        pump.runtime().commands().iter().flatten().any(|command| {
            let Command::SetProperty {
                node,
                property: PropertyId::TextBlockText,
                ..
            } = command
            else {
                return false;
            };
            pump.runtime()
                .node(*node)
                .and_then(|node| node.property(PropertyId::TextBlockText))
                == Some(&PropertyValue::Str(expected.into()))
        })
    }

    fn editor_input(pump: &Pump<RecordingRuntime>) -> NodeId {
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .find_map(|command| match command {
                Command::SubscribeEvent {
                    node,
                    event: EventId::TextBoxTextChanged,
                    ..
                } if pump.runtime().node(*node).is_some() => Some(*node),
                _ => None,
            })
            .unwrap()
    }

    fn wait_until(mut condition: impl FnMut() -> bool) {
        let deadline = Instant::now() + Duration::from_secs(2);
        while !condition() {
            assert!(
                Instant::now() < deadline,
                "background cancellation timed out"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn navigation_and_window_lifetimes_remain_isolated() {
        let shared = SharedApp::new();
        let mut primary = Pump::new(RecordingRuntime::default());
        let mut secondary = Pump::new(RecordingRuntime::default());
        primary
            .mount_view(View::component::<Workspace>(WorkspaceProps {
                role: WindowRole::Primary,
                shared: Rc::clone(&shared),
            }))
            .unwrap();
        secondary
            .mount_view(View::component::<Workspace>(WorkspaceProps {
                role: WindowRole::Secondary,
                shared: Rc::clone(&shared),
            }))
            .unwrap();

        let primary_sender = sender(&shared, WindowRole::Primary);
        let secondary_sender = sender(&shared, WindowRole::Secondary);
        assert!(primary_sender.send(Message::OpenSecondary));
        assert_eq!(primary.dispatch_components(1), Ok(1));
        assert_eq!(primary.runtime().opened_windows().len(), 1);
        assert_eq!(
            primary.runtime().window_title(primary.window().unwrap()),
            Some("Primary workspace - Home")
        );
        assert_eq!(
            secondary
                .runtime()
                .window_title(secondary.window().unwrap()),
            Some("Secondary workspace - Home")
        );
        assert!(primary_sender.send(Message::Navigate(Page::Editor)));
        assert_eq!(primary.dispatch_components(1), Ok(1));
        assert_eq!(
            primary.runtime().window_title(primary.window().unwrap()),
            Some("Primary workspace - Editor")
        );
        let input = editor_input(&primary);
        assert_eq!(primary.process_imperatives(), Ok(1));

        let revision = primary
            .event_revision(input, EventId::TextBoxTextChanged)
            .unwrap();
        primary.queue_event(QueuedEvent::new(
            input,
            EventId::TextBoxTextChanged,
            revision,
            EventPayload::Str("retained primary draft".into()),
        ));
        assert_eq!(primary.dispatch_events(), Ok(1));
        assert_eq!(primary.dispatch_components(1), Ok(1));

        assert!(primary_sender.send(Message::Navigate(Page::Home)));
        assert_eq!(primary.dispatch_components(1), Ok(1));
        assert!(primary_sender.send(Message::Navigate(Page::Editor)));
        assert_eq!(primary.dispatch_components(1), Ok(1));
        assert!(
            primary
                .runtime()
                .commands()
                .iter()
                .flatten()
                .any(|command| {
                    matches!(
                        command,
                        Command::SetProperty {
                            node,
                            property: PropertyId::TextBoxText,
                            ..
                        } if primary
                            .runtime()
                            .node(*node)
                            .and_then(|node| node.property(PropertyId::TextBoxText))
                            == Some(&PropertyValue::Str("retained primary draft".into()))
                    )
                })
        );
        assert!(live_text(&secondary, "Secondary home count: 0"));

        assert!(primary_sender.send(Message::ToggleTheme));
        assert_eq!(primary.dispatch_components(1), Ok(1));
        assert_eq!(primary.dispatch_components(1), Ok(1));
        assert_eq!(secondary.dispatch_components(1), Ok(1));
        assert!(live_text(&primary, "Primary workspace - dark theme"));
        assert!(live_text(&secondary, "Secondary workspace - dark theme"));

        assert!(secondary_sender.send(Message::StartWork));
        assert_eq!(secondary.dispatch_components(1), Ok(1));
        assert!(secondary_sender.send(Message::CloseWindow));
        assert_eq!(secondary.dispatch_components(1), Ok(1));
        assert_eq!(secondary.runtime().close_requests().len(), 1);
        secondary.shutdown();
        assert!(!secondary_sender.send(Message::Increment));
        wait_until(|| shared.cancellations.load(Ordering::Acquire) == 1);
        assert_eq!(shared.lifecycle[&WindowRole::Secondary].cleanups.get(), 1);
        assert_eq!(primary.dispatch_components(1), Ok(1));
        assert!(live_text(&primary, "Secondary window closed"));
        assert!(primary_sender.send(Message::Increment));

        primary.shutdown();
        assert_eq!(shared.lifecycle[&WindowRole::Primary].cleanups.get(), 1);
    }
}
