#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use windows_reactor2 as reactor2;
use windows_reactor2::{App, AppContext};

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Projects,
    Settings,
}

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn name(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }
}

#[derive(Clone, PartialEq)]
struct Project {
    id: usize,
    name: String,
    revision: usize,
}

#[derive(Clone, PartialEq)]
struct NavigationInput {
    on_navigate: reactor2::Callback<Page>,
    page: Page,
}

enum NavigationMessage {
    Projects,
    Settings,
}

struct Navigation(NavigationInput);

impl reactor2::Component for Navigation {
    type Input = NavigationInput;
    type Message = NavigationMessage;

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(input.clone())
    }

    fn input_changed(
        &mut self,
        input: &Self::Input,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0 = input.clone();
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0.on_navigate.call(match message {
            NavigationMessage::Projects => Page::Projects,
            NavigationMessage::Settings => Page::Settings,
        });
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let projects = context.sender();
        let settings = context.sender();
        reactor2::StackPanel::new()
            .orientation(reactor2::Orientation::Horizontal)
            .spacing(8.0)
            .children([
                reactor2::TextBlock::new()
                    .text(match self.0.page {
                        Page::Projects => "Page: Projects",
                        Page::Settings => "Page: Settings",
                    })
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text("Projects"))
                    .on_click(move || {
                        _ = projects.send(NavigationMessage::Projects);
                    })
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text("Settings"))
                    .on_click(move || {
                        _ = settings.send(NavigationMessage::Settings);
                    })
                    .into(),
            ])
            .into()
    }
}

#[derive(Clone, PartialEq)]
struct ProjectRowInput {
    on_select: reactor2::Callback<usize>,
    on_synced: reactor2::Callback<usize>,
    project: Project,
    selected: bool,
}

enum ProjectRowMessage {
    Select,
    Sync,
    Synced,
}

struct ProjectRow {
    input: ProjectRowInput,
    syncing: bool,
}

impl reactor2::Component for ProjectRow {
    type Input = ProjectRowInput;
    type Message = ProjectRowMessage;

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self {
            input: input.clone(),
            syncing: false,
        }
    }

    fn input_changed(
        &mut self,
        input: &Self::Input,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.input = input.clone();
    }

    fn update(
        &mut self,
        message: Self::Message,
        context: &reactor2::ComponentContext<Self::Message>,
    ) {
        match message {
            ProjectRowMessage::Select => self.input.on_select.call(self.input.project.id),
            ProjectRowMessage::Sync if !self.syncing => {
                self.syncing = true;
                _ = context.spawn_background(|token| {
                    for _ in 0..10 {
                        if token.is_cancelled() {
                            return ProjectRowMessage::Synced;
                        }
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    ProjectRowMessage::Synced
                });
            }
            ProjectRowMessage::Sync => {}
            ProjectRowMessage::Synced => {
                self.syncing = false;
                self.input.on_synced.call(self.input.project.id);
            }
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let select = context.sender();
        let sync = context.sender();
        let marker = if self.input.selected { ">" } else { " " };
        let state = if self.syncing { "Syncing..." } else { "Sync" };
        reactor2::StackPanel::new()
            .orientation(reactor2::Orientation::Horizontal)
            .spacing(8.0)
            .children([
                reactor2::TextBlock::new()
                    .text(format!(
                        "{marker} {} (revision {})",
                        self.input.project.name, self.input.project.revision
                    ))
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text("Select"))
                    .on_click(move || {
                        _ = select.send(ProjectRowMessage::Select);
                    })
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text(state))
                    .on_click(move || {
                        _ = sync.send(ProjectRowMessage::Sync);
                    })
                    .into(),
            ])
            .into()
    }
}

#[derive(Clone, PartialEq)]
struct ProjectsInput {
    draft: Rc<str>,
    on_add: reactor2::Callback<()>,
    on_draft: reactor2::Callback<String>,
    on_reverse: reactor2::Callback<()>,
    on_select: reactor2::Callback<usize>,
    on_synced: reactor2::Callback<usize>,
    projects: Vec<Project>,
    selected: Option<usize>,
}

enum ProjectsMessage {
    Add,
    Draft(String),
    Reverse,
}

struct ProjectsPage(ProjectsInput);

impl reactor2::Component for ProjectsPage {
    type Input = ProjectsInput;
    type Message = ProjectsMessage;

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(input.clone())
    }

    fn input_changed(
        &mut self,
        input: &Self::Input,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0 = input.clone();
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        match message {
            ProjectsMessage::Add => self.0.on_add.call(()),
            ProjectsMessage::Draft(value) => self.0.on_draft.call(value),
            ProjectsMessage::Reverse => self.0.on_reverse.call(()),
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let draft = context.sender();
        let add = context.sender();
        let reverse = context.sender();
        let rows = self.0.projects.iter().map(|project| {
            reactor2::component::<ProjectRow>(
                format!("project:{}", project.id),
                ProjectRowInput {
                    on_select: self.0.on_select.clone(),
                    on_synced: self.0.on_synced.clone(),
                    project: project.clone(),
                    selected: self.0.selected == Some(project.id),
                },
            )
            .keyed()
        });
        reactor2::StackPanel::new()
            .spacing(8.0)
            .children([
                reactor2::TextBox::new(Rc::clone(&self.0.draft))
                    .on_text_changed(move |value| {
                        _ = draft.send(ProjectsMessage::Draft(value.to_string()));
                    })
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text("Add project"))
                    .on_click(move || {
                        _ = add.send(ProjectsMessage::Add);
                    })
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text("Reverse projects"))
                    .on_click(move || {
                        _ = reverse.send(ProjectsMessage::Reverse);
                    })
                    .into(),
                reactor2::Grid::new().children(rows).into(),
            ])
            .into()
    }
}

#[derive(Clone)]
struct SettingsInput {
    on_theme: reactor2::Callback<Theme>,
    theme: Rc<reactor2::Context<Theme>>,
}

impl PartialEq for SettingsInput {
    fn eq(&self, other: &Self) -> bool {
        self.on_theme == other.on_theme && Rc::ptr_eq(&self.theme, &other.theme)
    }
}

struct SettingsPage(SettingsInput);

impl reactor2::Component for SettingsPage {
    type Input = SettingsInput;
    type Message = Theme;

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self(input.clone())
    }

    fn input_changed(
        &mut self,
        input: &Self::Input,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0 = input.clone();
    }

    fn update(
        &mut self,
        theme: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        self.0.on_theme.call(theme);
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let theme = context.use_context(&self.0.theme);
        let light = context.sender();
        let dark = context.sender();
        reactor2::StackPanel::new()
            .spacing(8.0)
            .children([
                reactor2::TextBlock::new()
                    .text(format!("Shared theme: {}", theme.name()))
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text("Light"))
                    .on_click(move || {
                        _ = light.send(Theme::Light);
                    })
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new().text("Dark"))
                    .on_click(move || {
                        _ = dark.send(Theme::Dark);
                    })
                    .into(),
            ])
            .into()
    }
}

#[derive(Clone, PartialEq)]
struct DetailsInput {
    project: Option<Project>,
}

struct ProjectDetails;

impl reactor2::Component for ProjectDetails {
    type Input = DetailsInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        reactor2::TextBlock::new()
            .text(match &input.project {
                Some(project) => format!(
                    "Selected project: {} at revision {}",
                    project.name, project.revision
                ),
                None => "No project selected".into(),
            })
            .into()
    }
}

#[derive(Clone)]
struct StatusInput {
    projects: usize,
    theme: Rc<reactor2::Context<Theme>>,
}

impl PartialEq for StatusInput {
    fn eq(&self, other: &Self) -> bool {
        self.projects == other.projects && Rc::ptr_eq(&self.theme, &other.theme)
    }
}

struct StatusBar;

impl reactor2::Component for StatusBar {
    type Input = StatusInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self
    }

    fn view(
        &self,
        input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let theme = context.use_context(&input.theme);
        reactor2::TextBlock::new()
            .text(format!(
                "{} theme - {} projects",
                theme.name(),
                input.projects
            ))
            .into()
    }
}

#[derive(Clone)]
struct WorkbenchInput {
    on_theme: reactor2::Callback<Theme>,
    theme: Rc<reactor2::Context<Theme>>,
}

impl PartialEq for WorkbenchInput {
    fn eq(&self, other: &Self) -> bool {
        self.on_theme == other.on_theme && Rc::ptr_eq(&self.theme, &other.theme)
    }
}

enum WorkbenchMessage {
    Add,
    Draft(String),
    Navigate(Page),
    Reverse,
    Select(usize),
    Synced(usize),
}

struct Workbench {
    draft: Rc<str>,
    input: WorkbenchInput,
    next_id: usize,
    page: Page,
    projects: Vec<Project>,
    selected: Option<usize>,
}

impl reactor2::Component for Workbench {
    type Input = WorkbenchInput;
    type Message = WorkbenchMessage;

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self {
            draft: Rc::from(""),
            input: input.clone(),
            next_id: 3,
            page: Page::Projects,
            projects: vec![
                Project {
                    id: 1,
                    name: "windows".into(),
                    revision: 0,
                },
                Project {
                    id: 2,
                    name: "windows-core".into(),
                    revision: 0,
                },
            ],
            selected: None,
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
        match message {
            WorkbenchMessage::Add if !self.draft.is_empty() => {
                self.projects.push(Project {
                    id: self.next_id,
                    name: self.draft.to_string(),
                    revision: 0,
                });
                self.next_id += 1;
                self.draft = Rc::from("");
            }
            WorkbenchMessage::Add => {}
            WorkbenchMessage::Draft(value) => self.draft = Rc::from(value),
            WorkbenchMessage::Navigate(page) => self.page = page,
            WorkbenchMessage::Reverse => self.projects.reverse(),
            WorkbenchMessage::Select(id) => self.selected = Some(id),
            WorkbenchMessage::Synced(id) => {
                if let Some(project) = self.projects.iter_mut().find(|project| project.id == id) {
                    project.revision += 1;
                }
            }
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let navigate = context.sender();
        let draft = context.sender();
        let add = context.sender();
        let reverse = context.sender();
        let select = context.sender();
        let synced = context.sender();
        let on_navigate = reactor2::Callback::new(move |page| {
            _ = navigate.send(WorkbenchMessage::Navigate(page));
        });
        let page: reactor2::Visual = match self.page {
            Page::Projects => reactor2::component::<ProjectsPage>(
                "projects-page",
                ProjectsInput {
                    draft: Rc::clone(&self.draft),
                    on_add: reactor2::Callback::new(move |()| {
                        _ = add.send(WorkbenchMessage::Add);
                    }),
                    on_draft: reactor2::Callback::new(move |value| {
                        _ = draft.send(WorkbenchMessage::Draft(value));
                    }),
                    on_reverse: reactor2::Callback::new(move |()| {
                        _ = reverse.send(WorkbenchMessage::Reverse);
                    }),
                    on_select: reactor2::Callback::new(move |id| {
                        _ = select.send(WorkbenchMessage::Select(id));
                    }),
                    on_synced: reactor2::Callback::new(move |id| {
                        _ = synced.send(WorkbenchMessage::Synced(id));
                    }),
                    projects: self.projects.clone(),
                    selected: self.selected,
                },
            )
            .into(),
            Page::Settings => reactor2::component::<SettingsPage>(
                "settings-page",
                SettingsInput {
                    on_theme: self.input.on_theme.clone(),
                    theme: Rc::clone(&self.input.theme),
                },
            )
            .into(),
        };
        let selected = self
            .selected
            .and_then(|id| self.projects.iter().find(|project| project.id == id))
            .cloned();
        reactor2::StackPanel::new()
            .spacing(10.0)
            .children([
                reactor2::TextBlock::new()
                    .text("Reactor2 project workbench")
                    .into(),
                reactor2::component::<Navigation>(
                    "navigation",
                    NavigationInput {
                        on_navigate,
                        page: self.page,
                    },
                )
                .into(),
                reactor2::Border::new().content(page).into(),
                reactor2::component::<ProjectDetails>(
                    "details",
                    DetailsInput { project: selected },
                )
                .into(),
                reactor2::component::<StatusBar>(
                    "status",
                    StatusInput {
                        projects: self.projects.len(),
                        theme: Rc::clone(&self.input.theme),
                    },
                )
                .into(),
            ])
            .into()
    }
}

struct Host {
    host: reactor2::ComponentHost<reactor2::native::WinUiAdapter>,
    _window: reactor2::native::NativeWindow,
}

impl Host {
    fn new(context: &AppContext) -> windows_core::Result<Rc<RefCell<Option<Self>>>> {
        let theme = Rc::new(reactor2::Context::new(Theme::Light));
        let pending_theme = Rc::new(RefCell::new(None));
        let request = Rc::clone(&pending_theme);
        let input = WorkbenchInput {
            on_theme: reactor2::Callback::new(move |theme| {
                *request.borrow_mut() = Some(theme);
            }),
            theme: Rc::clone(&theme),
        };
        let state = Rc::new(RefCell::new(None::<Self>));
        let drain_state = Rc::clone(&state);
        let drain_theme = Rc::clone(&theme);
        let drain_pending = Rc::clone(&pending_theme);
        let drain = context.callback(move || {
            let mut state = drain_state.borrow_mut();
            let host = &mut state.as_mut().unwrap().host;
            host.drain(usize::MAX)?;
            if let Some(theme) = drain_pending.borrow_mut().take() {
                host.set_context(&drain_theme, theme)?;
            }
            host.runtime()
                .adapter()
                .validate_graph(host.runtime().graph())
                .map_err(Into::into)
        });
        let mut host = reactor2::ComponentHost::mount_with_services(
            reactor2::native::WinUiAdapter::default(),
            context.component_services(),
            [reactor2::component::<Workbench>("workbench", input)],
        )?;
        let wake = drain.clone();
        host.set_waker(move || {
            _ = wake.invoke();
        });
        let wake = drain;
        host.set_native_event_waker(move || {
            _ = wake.invoke();
        });
        let root = host.runtime().graph().root().unwrap();
        let mut window = host.runtime().adapter().open_window(root)?;
        let application = context.proxy();
        window.set_closed(move || application.exit())?;
        *state.borrow_mut() = Some(Self {
            host,
            _window: window,
        });
        Ok(state)
    }
}

fn main() -> windows_core::Result<()> {
    App::run_with(Host::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn mount() -> (
        reactor2::ComponentHost<reactor2::RecordingAdapter>,
        Rc<reactor2::Context<Theme>>,
    ) {
        let theme = Rc::new(reactor2::Context::new(Theme::Light));
        let host = reactor2::ComponentHost::mount(
            reactor2::RecordingAdapter::default(),
            [reactor2::component::<Workbench>(
                "workbench",
                WorkbenchInput {
                    on_theme: reactor2::Callback::new(|_| {}),
                    theme: Rc::clone(&theme),
                },
            )],
        )
        .unwrap();
        (host, theme)
    }

    fn path(keys: &[&str]) -> Vec<reactor2::Key> {
        keys.iter().map(|key| reactor2::Key::from(*key)).collect()
    }

    fn text(
        host: &reactor2::ComponentHost<reactor2::RecordingAdapter>,
        path: &[reactor2::Key],
    ) -> Rc<str> {
        let root = host.reference_at(path).unwrap().get().unwrap();
        let object = host
            .runtime()
            .graph()
            .properties(root)
            .and_then(|properties| {
                properties
                    .iter()
                    .any(|property| property.id == reactor2::PropertyId::Text)
                    .then_some(root)
            })
            .or_else(|| {
                host.runtime()
                    .adapter()
                    .children(root, reactor2::RelationId::Children)
                    .and_then(|children| children.first().copied())
            })
            .unwrap();
        host.runtime()
            .graph()
            .properties(object)
            .unwrap()
            .iter()
            .find_map(|property| match &property.value {
                reactor2::PropertyValue::String(value)
                    if property.id == reactor2::PropertyId::Text =>
                {
                    Some(Rc::clone(value))
                }
                _ => None,
            })
            .unwrap()
    }

    #[test]
    fn keyed_reorder_preserves_rows_and_row_work_is_isolated() {
        let (mut host, _) = mount();
        let first_path = path(&["workbench", "projects-page", "project:1"]);
        let second_path = path(&["workbench", "projects-page", "project:2"]);
        let first = host.reference_at(&first_path).unwrap().get();
        let second = host.reference_at(&second_path).unwrap().get();

        assert!(
            host.sender::<Workbench>(&reactor2::Key::from("workbench"))
                .unwrap()
                .send(WorkbenchMessage::Reverse)
        );
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 1);
        assert_eq!(host.reference_at(&first_path).unwrap().get(), first);
        assert_eq!(host.reference_at(&second_path).unwrap().get(), second);

        assert!(
            host.sender_at::<ProjectRow>(&first_path)
                .unwrap()
                .send(ProjectRowMessage::Sync)
        );
        let report = host.drain(1).unwrap();
        assert_eq!(report.dispatched, 1);
        assert!(report.mutations > 0);
        assert_eq!(host.reference_at(&first_path).unwrap().get(), first);
        assert_eq!(host.reference_at(&second_path).unwrap().get(), second);
    }

    #[test]
    fn form_adds_a_keyed_project_and_selection_updates_details() {
        let (mut host, _) = mount();
        let page_path = path(&["workbench", "projects-page"]);
        let details_path = path(&["workbench", "details"]);
        let page = host.sender_at::<ProjectsPage>(&page_path).unwrap();

        assert!(page.send(ProjectsMessage::Draft("windows-rdl".into())));
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 2);
        assert!(page.send(ProjectsMessage::Add));
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 2);

        let project_path = path(&["workbench", "projects-page", "project:3"]);
        assert!(host.reference_at(&project_path).unwrap().get().is_some());
        assert!(
            host.sender_at::<ProjectRow>(&project_path)
                .unwrap()
                .send(ProjectRowMessage::Select)
        );
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 2);
        assert_eq!(
            text(&host, &details_path).as_ref(),
            "Selected project: windows-rdl at revision 0"
        );
    }

    #[test]
    fn navigation_retires_page_and_rejects_stale_row_messages() {
        let (mut host, _) = mount();
        let row_path = path(&["workbench", "projects-page", "project:1"]);
        let reference = host.reference_at(&row_path).unwrap();
        let stale = host.sender_at::<ProjectRow>(&row_path).unwrap();

        assert!(
            host.sender::<Workbench>(&reactor2::Key::from("workbench"))
                .unwrap()
                .send(WorkbenchMessage::Navigate(Page::Settings))
        );
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 1);
        assert_eq!(reference.get(), None);

        assert!(stale.send(ProjectRowMessage::Synced));
        assert_eq!(host.drain(usize::MAX).unwrap().dropped, 1);
    }

    #[test]
    fn retiring_a_row_cancels_its_background_completion() {
        let completed = Arc::new(AtomicUsize::new(0));
        let callback = Arc::clone(&completed);
        let mut host = reactor2::ComponentHost::mount(
            reactor2::RecordingAdapter::default(),
            [reactor2::component::<ProjectRow>(
                "project",
                ProjectRowInput {
                    on_select: reactor2::Callback::new(|_| {}),
                    on_synced: reactor2::Callback::new(move |_| {
                        callback.fetch_add(1, Ordering::Relaxed);
                    }),
                    project: Project {
                        id: 1,
                        name: "windows".into(),
                        revision: 0,
                    },
                    selected: false,
                },
            )],
        )
        .unwrap();
        assert!(
            host.sender::<ProjectRow>(&reactor2::Key::from("project"))
                .unwrap()
                .send(ProjectRowMessage::Sync)
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);

        host.remove(&reactor2::Key::from("project")).unwrap();
        std::thread::sleep(Duration::from_millis(150));
        assert_eq!(host.drain(usize::MAX).unwrap(), Default::default());
        assert_eq!(completed.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn context_change_updates_only_the_two_subscribers() {
        let (mut host, theme) = mount();
        assert!(
            host.sender::<Workbench>(&reactor2::Key::from("workbench"))
                .unwrap()
                .send(WorkbenchMessage::Navigate(Page::Settings))
        );
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 1);

        let report = host.set_context(&theme, Theme::Dark).unwrap();
        assert_eq!(report.dispatched, 2);
        assert_eq!(
            text(&host, &path(&["workbench", "settings-page"])).as_ref(),
            "Shared theme: Dark"
        );
        assert_eq!(
            text(&host, &path(&["workbench", "status"])).as_ref(),
            "Dark theme - 2 projects"
        );
    }
}
