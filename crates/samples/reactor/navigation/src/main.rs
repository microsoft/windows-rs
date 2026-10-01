#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use windows_reactor::*;

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
struct WorkspaceInput {
    role: WindowRole,
    shared: Rc<SharedApp>,
}

impl PartialEq for WorkspaceInput {
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
struct HomeInput {
    count: u32,
    increment: Callback<()>,
    role: WindowRole,
}

struct HomePage;

#[derive(Clone, PartialEq)]
struct EditorInput {
    changed: Callback<Rc<str>>,
    editor_ref: ElementRef<TextBox>,
    note: String,
    role: WindowRole,
}

struct EditorPage;

#[derive(Clone)]
struct ThemeInput {
    context: Rc<Context<bool>>,
    role: WindowRole,
}

impl PartialEq for ThemeInput {
    fn eq(&self, other: &Self) -> bool {
        self.role == other.role && Rc::ptr_eq(&self.context, &other.context)
    }
}

struct ThemeBanner;

impl Component for Workspace {
    type Message = Message;
    type Input = WorkspaceInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            count: 0,
            editor_ref: ElementRef::new(),
            note: format!("{} window note", input.role.label()),
            page: Page::Home,
            role: input.role,
            secondary_open: false,
            sender: context.sender(),
            shared: Rc::clone(&input.shared),
            status: "Ready".to_string(),
            working: false,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::CloseWindow => {
                if !context.close_window() {
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
                let opened = context.open_window::<Self>(WorkspaceInput {
                    role: WindowRole::Secondary,
                    shared: Rc::clone(&self.shared),
                });
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

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
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
            Page::Home => View::component::<HomePage>(HomeInput {
                count: self.count,
                increment: context.message(Message::Increment),
                role: self.role,
            }),
            Page::Editor => View::component::<EditorPage>(EditorInput {
                changed: context.callback(|note: Rc<str>| Message::NoteChanged(note.to_string())),
                editor_ref: self.editor_ref.clone(),
                note: self.note.clone(),
                role: self.role,
            }),
        };
        let mut header = vec![
            View::component::<ThemeBanner>(ThemeInput {
                context: Rc::clone(&self.shared.theme),
                role: self.role,
            }),
            self.status.clone().into(),
            Button::new()
                .on_click(context.message(Message::Navigate(Page::Home)))
                .content("Home")
                .into(),
            Button::new()
                .on_click(context.message(Message::Navigate(Page::Editor)))
                .content("Editor")
                .into(),
            Button::new()
                .on_click(context.message(Message::ToggleTheme))
                .content("Toggle shared theme")
                .into(),
        ];
        if self.role == WindowRole::Primary {
            header.push(
                Button::new()
                    .is_enabled(!self.secondary_open)
                    .on_click(context.message(Message::OpenSecondary))
                    .content("Open secondary window")
                    .into(),
            );
        }
        header.extend([
            Button::new()
                .is_enabled(!self.working)
                .on_click(context.message(Message::StartWork))
                .content("Start background work")
                .into(),
            Button::new()
                .on_click(context.message(Message::CloseWindow))
                .content("Close this window")
                .into(),
        ]);
        let header = StackPanel::new().spacing(4.0).children(header);

        provide(
            &self.shared.theme,
            self.shared.dark.get(),
            SplitView::new()
                .open_pane_length(280.0)
                .compact_pane_length(48.0)
                .display_mode(SplitViewDisplayMode::CompactInline)
                .is_pane_open(true)
                .pane(header)
                .content(page),
        )
    }
}

impl Component for HomePage {
    type Message = ();
    type Input = HomeInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .spacing(4.0)
            .children((
                format!("{} home count: {}", input.role.label(), input.count),
                Button::new()
                    .on_click(input.increment.clone())
                    .content("Increment local count"),
            ))
            .into()
    }
}

impl Component for EditorPage {
    type Message = ();
    type Input = EditorInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .spacing(4.0)
            .children((
                format!("{} editor", input.role.label()),
                TextBox::new(input.note.clone())
                    .element_ref(&input.editor_ref)
                    .on_text_changed(input.changed.clone()),
            ))
            .into()
    }
}

impl Component for ThemeBanner {
    type Message = ();
    type Input = ThemeInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let dark = context.use_context(&input.context);
        format!(
            "{} workspace - {} theme",
            input.role.label(),
            if dark { "dark" } else { "light" }
        )
        .into()
    }
}

fn main() {
    let shared = SharedApp::new();
    App::run_component::<Workspace>(WorkspaceInput {
        role: WindowRole::Primary,
        shared,
    })
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn host(shared: &Rc<SharedApp>, role: WindowRole) -> ComponentHost<RecordingAdapter> {
        ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<Workspace>(
                role.label(),
                WorkspaceInput {
                    role,
                    shared: Rc::clone(shared),
                },
            )],
        )
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
    fn navigation_and_component_lifetimes_remain_isolated() {
        let shared = SharedApp::new();
        let mut primary = host(&shared, WindowRole::Primary);
        let mut secondary = host(&shared, WindowRole::Secondary);
        let primary_sender = primary
            .sender::<Workspace>(&Key::from(WindowRole::Primary.label()))
            .unwrap();
        let secondary_sender = secondary
            .sender::<Workspace>(&Key::from(WindowRole::Secondary.label()))
            .unwrap();

        assert!(primary_sender.send(Message::Navigate(Page::Editor)));
        assert_eq!(primary.drain(1).unwrap().dispatched, 1);
        assert!(primary_sender.send(Message::NoteChanged("retained primary draft".to_string())));
        assert_eq!(primary.drain(1).unwrap().dispatched, 1);
        assert!(primary_sender.send(Message::Navigate(Page::Home)));
        assert_eq!(primary.drain(1).unwrap().dispatched, 1);
        assert!(primary_sender.send(Message::Navigate(Page::Editor)));
        assert_eq!(primary.drain(1).unwrap().dispatched, 1);

        assert!(primary_sender.send(Message::ToggleTheme));
        assert_eq!(primary.drain(2).unwrap().dispatched, 2);
        assert_eq!(secondary.drain(1).unwrap().dispatched, 1);
        assert!(shared.dark.get());

        assert!(secondary_sender.send(Message::StartWork));
        assert_eq!(secondary.drain(1).unwrap().dispatched, 1);
        drop(secondary);
        assert!(!secondary_sender.send(Message::Increment));
        wait_until(|| shared.cancellations.load(Ordering::Acquire) == 1);
        assert_eq!(shared.lifecycle[&WindowRole::Secondary].cleanups.get(), 1);
        assert_eq!(primary.drain(1).unwrap().dispatched, 1);
        assert!(primary_sender.send(Message::Increment));
        assert_eq!(primary.drain(1).unwrap().dispatched, 1);

        drop(primary);
        assert_eq!(shared.lifecycle[&WindowRole::Primary].cleanups.get(), 1);
    }
}
