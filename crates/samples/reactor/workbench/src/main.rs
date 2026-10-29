#![windows_subsystem = "windows"]

use std::rc::Rc;
use std::time::Duration;
use windows_reactor as reactor;
use windows_reactor::App;

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
    on_navigate: reactor::Callback<Page>,
    page: Page,
}

enum NavigationMessage {
    Projects,
    Settings,
}

struct Navigation(NavigationInput);

impl reactor::Component for Navigation {
    type Input = NavigationInput;
    type Message = NavigationMessage;

    fn create(input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &reactor::ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, message: Self::Message, _context: &reactor::ComponentContext<Self>) {
        self.0.on_navigate.call(match message {
            NavigationMessage::Projects => Page::Projects,
            NavigationMessage::Settings => Page::Settings,
        });
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        let projects = context.sender();
        let settings = context.sender();
        reactor::StackPanel::new()
            .orientation(reactor::Orientation::Horizontal)
            .spacing(8.0)
            .children([
                reactor::TextBlock::new()
                    .text(match self.0.page {
                        Page::Projects => "Page: Projects",
                        Page::Settings => "Page: Settings",
                    })
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Projects"))
                    .on_click(move || {
                        _ = projects.send(NavigationMessage::Projects);
                    })
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Settings"))
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
    on_select: reactor::Callback<usize>,
    on_synced: reactor::Callback<usize>,
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

impl reactor::Component for ProjectRow {
    type Input = ProjectRowInput;
    type Message = ProjectRowMessage;

    fn create(input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self {
            input: input.clone(),
            syncing: false,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &reactor::ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, message: Self::Message, context: &reactor::ComponentContext<Self>) {
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
        context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        let select = context.sender();
        let sync = context.sender();
        let marker = if self.input.selected { ">" } else { " " };
        let state = if self.syncing { "Syncing..." } else { "Sync" };
        reactor::StackPanel::new()
            .orientation(reactor::Orientation::Horizontal)
            .spacing(8.0)
            .children([
                reactor::TextBlock::new()
                    .text(format!(
                        "{marker} {} (revision {})",
                        self.input.project.name, self.input.project.revision
                    ))
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Select"))
                    .on_click(move || {
                        _ = select.send(ProjectRowMessage::Select);
                    })
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text(state))
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
    on_add: reactor::Callback<()>,
    on_draft: reactor::Callback<String>,
    on_reverse: reactor::Callback<()>,
    on_select: reactor::Callback<usize>,
    on_synced: reactor::Callback<usize>,
    projects: Vec<Project>,
    selected: Option<usize>,
}

enum ProjectsMessage {
    Add,
    Draft(String),
    Reverse,
}

struct ProjectsPage(ProjectsInput);

impl reactor::Component for ProjectsPage {
    type Input = ProjectsInput;
    type Message = ProjectsMessage;

    fn create(input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &reactor::ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, message: Self::Message, _context: &reactor::ComponentContext<Self>) {
        match message {
            ProjectsMessage::Add => self.0.on_add.call(()),
            ProjectsMessage::Draft(value) => self.0.on_draft.call(value),
            ProjectsMessage::Reverse => self.0.on_reverse.call(()),
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        let draft = context.sender();
        let add = context.sender();
        let reverse = context.sender();
        let rows = self.0.projects.iter().map(|project| {
            reactor::component::<ProjectRow>(
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
        reactor::StackPanel::new()
            .spacing(8.0)
            .children([
                reactor::TextBox::new()
                    .text(Rc::clone(&self.0.draft))
                    .on_text_changed(move |value: Rc<str>| {
                        _ = draft.send(ProjectsMessage::Draft(value.to_string()));
                    })
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Add project"))
                    .on_click(move || {
                        _ = add.send(ProjectsMessage::Add);
                    })
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Reverse projects"))
                    .on_click(move || {
                        _ = reverse.send(ProjectsMessage::Reverse);
                    })
                    .into(),
                reactor::Grid::new().keyed_children(rows).into(),
            ])
            .into()
    }
}

#[derive(Clone)]
struct SettingsInput {
    on_theme: reactor::Callback<Theme>,
    theme: Rc<reactor::Context<Theme>>,
}

impl PartialEq for SettingsInput {
    fn eq(&self, other: &Self) -> bool {
        self.on_theme == other.on_theme && Rc::ptr_eq(&self.theme, &other.theme)
    }
}

struct SettingsPage(SettingsInput);

impl reactor::Component for SettingsPage {
    type Input = SettingsInput;
    type Message = Theme;

    fn create(input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &reactor::ComponentContext<Self>) {
        self.0 = input.clone();
    }

    fn update(&mut self, theme: Self::Message, _context: &reactor::ComponentContext<Self>) {
        self.0.on_theme.call(theme);
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        let theme = context.use_context(&self.0.theme);
        let light = context.sender();
        let dark = context.sender();
        reactor::StackPanel::new()
            .spacing(8.0)
            .children([
                reactor::TextBlock::new()
                    .text(format!("Shared theme: {}", theme.name()))
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Light"))
                    .on_click(move || {
                        _ = light.send(Theme::Light);
                    })
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Dark"))
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

impl reactor::Component for ProjectDetails {
    type Input = DetailsInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        reactor::TextBlock::new()
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
    theme: Rc<reactor::Context<Theme>>,
}

impl PartialEq for StatusInput {
    fn eq(&self, other: &Self) -> bool {
        self.projects == other.projects && Rc::ptr_eq(&self.theme, &other.theme)
    }
}

struct StatusBar;

impl reactor::Component for StatusBar {
    type Input = StatusInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut reactor::ViewContext<Self>) -> reactor::View {
        let theme = context.use_context(&input.theme);
        reactor::TextBlock::new()
            .text(format!(
                "{} theme - {} projects",
                theme.name(),
                input.projects
            ))
            .into()
    }
}

enum WorkbenchMessage {
    Add,
    Draft(String),
    Navigate(Page),
    Reverse,
    Select(usize),
    Synced(usize),
    Theme(Theme),
}

struct Workbench {
    draft: Rc<str>,
    theme: Rc<reactor::Context<Theme>>,
    current_theme: Theme,
    next_id: usize,
    page: Page,
    projects: Vec<Project>,
    selected: Option<usize>,
}

impl reactor::Component for Workbench {
    type Input = ();
    type Message = WorkbenchMessage;

    fn create(_input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self {
            draft: Rc::from(""),
            theme: Rc::new(reactor::Context::new(Theme::Light)),
            current_theme: Theme::Light,
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

    fn update(&mut self, message: Self::Message, _context: &reactor::ComponentContext<Self>) {
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
            WorkbenchMessage::Theme(theme) => self.current_theme = theme,
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        let navigate = context.sender();
        let draft = context.sender();
        let add = context.sender();
        let reverse = context.sender();
        let select = context.sender();
        let synced = context.sender();
        let on_navigate = reactor::Callback::new(move |page| {
            _ = navigate.send(WorkbenchMessage::Navigate(page));
        });
        let page: reactor::View = match self.page {
            Page::Projects => reactor::component::<ProjectsPage>(
                "projects-page",
                ProjectsInput {
                    draft: Rc::clone(&self.draft),
                    on_add: reactor::Callback::new(move |()| {
                        _ = add.send(WorkbenchMessage::Add);
                    }),
                    on_draft: reactor::Callback::new(move |value| {
                        _ = draft.send(WorkbenchMessage::Draft(value));
                    }),
                    on_reverse: reactor::Callback::new(move |()| {
                        _ = reverse.send(WorkbenchMessage::Reverse);
                    }),
                    on_select: reactor::Callback::new(move |id| {
                        _ = select.send(WorkbenchMessage::Select(id));
                    }),
                    on_synced: reactor::Callback::new(move |id| {
                        _ = synced.send(WorkbenchMessage::Synced(id));
                    }),
                    projects: self.projects.clone(),
                    selected: self.selected,
                },
            )
            .into(),
            Page::Settings => reactor::component::<SettingsPage>(
                "settings-page",
                SettingsInput {
                    on_theme: context.callback(WorkbenchMessage::Theme),
                    theme: Rc::clone(&self.theme),
                },
            )
            .into(),
        };
        let selected = self
            .selected
            .and_then(|id| self.projects.iter().find(|project| project.id == id))
            .cloned();
        reactor::provide(
            &self.theme,
            self.current_theme,
            reactor::StackPanel::new().spacing(10.0).children([
                reactor::TextBlock::new()
                    .text("Reactor project workbench")
                    .into(),
                reactor::component::<Navigation>(
                    "navigation",
                    NavigationInput {
                        on_navigate,
                        page: self.page,
                    },
                )
                .into(),
                reactor::Border::new().content(page).into(),
                reactor::component::<ProjectDetails>("details", DetailsInput { project: selected })
                    .into(),
                reactor::component::<StatusBar>(
                    "status",
                    StatusInput {
                        projects: self.projects.len(),
                        theme: Rc::clone(&self.theme),
                    },
                )
                .into(),
            ]),
        )
    }
}

fn main() -> windows_core::Result<()> {
    App::run_component::<Workbench>(())
}
