#![windows_subsystem = "windows"]

use std::rc::Rc;
use std::time::Duration;
use windows_reactor::*;

#[derive(Clone, Copy)]
enum Page {
    Projects,
    Settings,
}

#[derive(Clone, PartialEq)]
struct Project {
    id: usize,
    name: String,
    revision: usize,
}

#[derive(Clone, PartialEq)]
struct ProjectRowInput {
    project: Project,
    selected: bool,
    on_select: Callback<usize>,
    on_synced: Callback<usize>,
}

#[derive(Clone)]
enum RowMessage {
    Select,
    Sync,
    Synced,
}

struct ProjectRow {
    input: ProjectRowInput,
    syncing: bool,
}

impl Component for ProjectRow {
    type Input = ProjectRowInput;
    type Message = RowMessage;

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            input: input.clone(),
            syncing: false,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, message: RowMessage, context: &ComponentContext<Self>) {
        match message {
            RowMessage::Select => self.input.on_select.call(self.input.project.id),
            RowMessage::Sync if !self.syncing => {
                self.syncing = true;
                context.spawn_background(|token| {
                    for _ in 0..10 {
                        if token.is_cancelled() {
                            break;
                        }
                        std::thread::sleep(Duration::from_millis(100));
                    }
                    RowMessage::Synced
                });
            }
            RowMessage::Sync => {}
            RowMessage::Synced => {
                self.syncing = false;
                self.input.on_synced.call(self.input.project.id);
            }
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(8.0)
            .children((
                format!(
                    "{} (revision {})",
                    input.project.name, input.project.revision
                ),
                Button::new()
                    .content(if input.selected { "Selected" } else { "Select" })
                    .on_click(context.message(RowMessage::Select)),
                Button::new()
                    .content(if self.syncing { "Syncing..." } else { "Sync" })
                    .is_enabled(!self.syncing)
                    .on_click(context.message(RowMessage::Sync)),
            ))
            .into()
    }
}

#[derive(Clone)]
enum Message {
    Add,
    Draft(Rc<str>),
    Navigate(Page),
    Reverse,
    Select(usize),
    Synced(usize),
    Dark(bool),
}

struct Workbench {
    draft: Rc<str>,
    dark: bool,
    next_id: usize,
    page: Page,
    projects: Vec<Project>,
    selected: Option<usize>,
}

impl Component for Workbench {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            draft: Rc::from(""),
            dark: false,
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

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Add if !self.draft.trim().is_empty() => {
                self.projects.push(Project {
                    id: self.next_id,
                    name: self.draft.trim().into(),
                    revision: 0,
                });
                self.next_id += 1;
                self.draft = Rc::from("");
            }
            Message::Add => {}
            Message::Draft(value) => self.draft = value,
            Message::Navigate(page) => self.page = page,
            Message::Reverse => self.projects.reverse(),
            Message::Select(id) => self.selected = Some(id),
            Message::Synced(id) => {
                self.projects
                    .iter_mut()
                    .find(|project| project.id == id)
                    .unwrap()
                    .revision += 1;
            }
            Message::Dark(dark) => self.dark = dark,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().theme(if self.dark {
            WindowTheme::Dark
        } else {
            WindowTheme::Light
        }));
        let page: View =
            match self.page {
                Page::Projects => {
                    let rows = self.projects.iter().map(|project| {
                        component::<ProjectRow>(
                            project.id,
                            ProjectRowInput {
                                project: project.clone(),
                                selected: self.selected == Some(project.id),
                                on_select: context.callback(Message::Select),
                                on_synced: context.callback(Message::Synced),
                            },
                        )
                        .keyed()
                    });
                    ScrollViewer::new()
                    .content(StackPanel::new().spacing(12.0).children((
                        TextBox::new()
                            .placeholder_text("Project name")
                            .text(self.draft.clone())
                            .on_text_changed(context.callback(Message::Draft)),
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(8.0)
                            .children((
                                Button::new()
                                    .content("Add project")
                                    .is_enabled(!self.draft.trim().is_empty())
                                    .on_click(context.message(Message::Add)),
                                Button::new()
                                    .content("Reverse projects")
                                    .on_click(context.message(Message::Reverse)),
                            )),
                        TextBlock::new()
                            .text("Sync simulates background work. Reordering keeps each row's \
                                   progress; leaving this page cancels its pending work.")
                            .text_wrapping(TextWrapping::Wrap),
                        StackPanel::new().spacing(8.0).keyed_children(rows),
                    )))
                    .into()
                }
                Page::Settings => ToggleSwitch::new()
                    .header("Dark theme")
                    .is_on(self.dark)
                    .on_toggled(context.callback(Message::Dark))
                    .into(),
            };
        let selection = self
            .selected
            .and_then(|id| self.projects.iter().find(|project| project.id == id));
        context.window_frame(
            "Project workbench",
            Grid::new()
                .margin(24.0)
                .row_spacing(12.0)
                .rows([GridLength::Auto, GridLength::STAR, GridLength::Auto])
                .children((
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children((
                            Button::new()
                                .content("Projects")
                                .on_click(context.message(Message::Navigate(Page::Projects))),
                            Button::new()
                                .content("Settings")
                                .on_click(context.message(Message::Navigate(Page::Settings))),
                        )),
                    Border::new().grid_row(1).content(page),
                    TextBlock::new().grid_row(2).text(match selection {
                        Some(project) => format!(
                            "Selected: {} at revision {}",
                            project.name, project.revision
                        ),
                        None => "No project selected".into(),
                    }),
                )),
        )
    }
}

fn main() {
    App::run_component::<Workbench>(()).unwrap();
}
