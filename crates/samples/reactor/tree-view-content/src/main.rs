#![windows_subsystem = "windows"]

use windows_reactor::*;

struct Project {
    key: &'static str,
    label: &'static str,
    files: &'static [&'static str],
    changes: i32,
}

#[derive(Clone)]
enum Message {
    Update,
    Invoked(String),
}

struct TreeViewContentSample {
    projects: Vec<Project>,
    invoked: String,
}

impl Component for TreeViewContentSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            projects: vec![
                Project {
                    key: "windows",
                    label: "windows-rs",
                    files: &["Cargo.toml", "readme.md"],
                    changes: 3,
                },
                Project {
                    key: "samples",
                    label: "samples",
                    files: &["counter.rs", "notepad.rs"],
                    changes: 1,
                },
            ],
            invoked: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Update => {
                self.projects.rotate_left(1);
                for project in &mut self.projects {
                    project.changes = (project.changes + 2) % 6;
                }
            }
            Message::Invoked(label) => self.invoked = label,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TreeView custom content");
        let nodes = self.projects.iter().map(|project| {
            TreeNode::new(project.key, project.label)
                .content(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children((
                            SymbolIcon::new().symbol(Symbol::Folder),
                            TextBlock::new().text(project.label),
                            TextBlock::new()
                                .text(format!("{} changes", project.changes))
                                .opacity(0.6),
                        )),
                )
                .children(project.files.iter().map(|file| TreeNode::new(*file, *file)))
        });

        Border::new().padding(16.0).content(
            StackPanel::new().spacing(12.0).children((
                "Expand or collapse a project, then update. Its state follows its key.",
                Button::new()
                    .on_click(context.message(Message::Update))
                    .content("Update status and reorder"),
                TreeView::new()
                    .on_item_invoked(context.callback(Message::Invoked))
                    .nodes(nodes),
                TextBlock::new().text(format!("Invoked: {}", self.invoked)),
            )),
        )
    }
}

fn main() {
    App::run_component::<TreeViewContentSample>(()).unwrap();
}
