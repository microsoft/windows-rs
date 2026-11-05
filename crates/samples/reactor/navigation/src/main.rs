use std::rc::Rc;
use windows_reactor::*;

struct Workspaces {
    dark: bool,
}

#[derive(Clone)]
enum ApplicationMessage {
    Open(&'static str),
    ToggleTheme,
}

impl Application for Workspaces {
    type Input = ();
    type Message = ApplicationMessage;

    fn create(_: &(), context: &ApplicationContext<Self>) -> Self {
        context.show_window("primary");
        Self { dark: false }
    }

    fn update(&mut self, message: ApplicationMessage, context: &ApplicationContext<Self>) {
        match message {
            ApplicationMessage::Open(key) => context.show_window(key),
            ApplicationMessage::ToggleTheme => self.dark = !self.dark,
        }
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        let input = |name, other| WorkspaceInput {
            name,
            theme: if self.dark {
                WindowTheme::Dark
            } else {
                WindowTheme::Light
            },
            open_other: context.message(ApplicationMessage::Open(other)),
            toggle_theme: context.message(ApplicationMessage::ToggleTheme),
        };
        ApplicationView::new()
            .window::<Workspace>("primary", input("Primary workspace", "secondary"))
            .window::<Workspace>("secondary", input("Secondary workspace", "primary"))
    }
}

#[derive(Clone, PartialEq)]
struct WorkspaceInput {
    name: &'static str,
    theme: WindowTheme,
    open_other: Callback<()>,
    toggle_theme: Callback<()>,
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Editor,
}

struct Workspace {
    page: Page,
    count: u32,
    note: Rc<str>,
}

#[derive(Clone)]
enum Message {
    Navigate(Option<Rc<str>>),
    Increment,
    Note(Rc<str>),
}

impl Component for Workspace {
    type Input = WorkspaceInput;
    type Message = Message;

    fn create(_: &WorkspaceInput, _: &ComponentContext<Self>) -> Self {
        Self {
            page: Page::Home,
            count: 0,
            note: Rc::from(""),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Navigate(tag) => match tag.as_deref() {
                Some("home") => self.page = Page::Home,
                Some("editor") => self.page = Page::Editor,
                None => {}
                Some(_) => unreachable!(),
            },
            Message::Increment => self.count += 1,
            Message::Note(note) => self.note = note,
        }
    }

    fn view(&self, input: &WorkspaceInput, context: &mut ViewContext<Self>) -> View {
        context.window_visuals(
            WindowVisuals::new()
                .client_size(760.0, 480.0)
                .theme(input.theme),
        );
        let page: View = match self.page {
            Page::Home => StackPanel::new()
                .margin(24.0)
                .spacing(12.0)
                .children((
                    format!("Local count: {}", self.count),
                    Button::new()
                        .content("Increment")
                        .on_click(context.message(Message::Increment)),
                    "Each workspace keeps its own count, page, and note.",
                ))
                .into(),
            Page::Editor => TextBox::new()
                .margin(24.0)
                .text(self.note.clone())
                .accepts_return(true)
                .placeholder_text("This note survives navigation, but not closing the window.")
                .on_text_changed(context.callback(Message::Note))
                .into(),
        };
        context.window_frame(
            input.name,
            NavigationView::new()
                .is_settings_visible(false)
                .menu_items((
                    NavigationViewItem::new()
                        .tag("home")
                        .content("Home")
                        .is_selected(self.page == Page::Home),
                    NavigationViewItem::new()
                        .tag("editor")
                        .content("Editor")
                        .is_selected(self.page == Page::Editor),
                ))
                .on_selected_tag_changed(context.callback(Message::Navigate))
                .header(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children((
                            Button::new()
                                .content("Open other workspace")
                                .on_click(input.open_other.clone()),
                            Button::new()
                                .content(if input.theme == WindowTheme::Dark {
                                    "Use light theme"
                                } else {
                                    "Use dark theme"
                                })
                                .on_click(input.toggle_theme.clone()),
                        )),
                )
                .content(page),
        )
    }
}

fn main() {
    App::run_application::<Workspaces>(()).unwrap();
}
