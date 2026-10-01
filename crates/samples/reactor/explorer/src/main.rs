#![windows_subsystem = "windows"]

use std::rc::Rc;
use std::time::Duration;
use windows_reactor as reactor;
use windows_reactor::App;

#[derive(Clone, PartialEq)]
struct Node {
    children: Vec<Self>,
    expanded: bool,
    key: String,
    label: String,
    loaded: bool,
}

impl Node {
    fn branch(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            children: Vec::new(),
            expanded: false,
            key: key.into(),
            label: label.into(),
            loaded: false,
        }
    }

    fn leaf(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            children: Vec::new(),
            expanded: false,
            key: key.into(),
            label: label.into(),
            loaded: true,
        }
    }
}

fn find_node_mut<'a>(nodes: &'a mut [Node], key: &str) -> Option<&'a mut Node> {
    for node in nodes {
        if node.key == key {
            return Some(node);
        }
        if let Some(node) = find_node_mut(&mut node.children, key) {
            return Some(node);
        }
    }
    None
}

#[derive(Clone, PartialEq)]
struct LoadResult {
    children: Vec<Node>,
    key: String,
}

#[derive(Clone, PartialEq)]
struct RowInput {
    expanded: bool,
    key: String,
    label: String,
    loaded: bool,
    on_loaded: reactor::Callback<LoadResult>,
    on_select: reactor::Callback<String>,
    on_toggle: reactor::Callback<String>,
    selected: bool,
}

enum RowMessage {
    Cancelled,
    Load,
    Loaded(Vec<Node>),
    Select,
    Toggle,
}

struct Row {
    input: RowInput,
    loading: bool,
}

impl reactor::Component for Row {
    type Input = RowInput;
    type Message = RowMessage;

    fn create(input: &Self::Input, _context: &reactor::ComponentContext<Self>) -> Self {
        Self {
            input: input.clone(),
            loading: false,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &reactor::ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, message: Self::Message, context: &reactor::ComponentContext<Self>) {
        match message {
            RowMessage::Cancelled => self.loading = false,
            RowMessage::Load => {
                if self.input.loaded || self.loading {
                    return;
                }
                self.loading = true;
                let key = self.input.key.clone();
                _ = context.spawn_background(move |token| {
                    for _ in 0..10 {
                        if token.is_cancelled() {
                            return RowMessage::Cancelled;
                        }
                        std::thread::sleep(Duration::from_millis(20));
                    }
                    RowMessage::Loaded(
                        (0..3)
                            .map(|index| {
                                Node::leaf(
                                    format!("{key}/{index}"),
                                    format!("Loaded child {}", index + 1),
                                )
                            })
                            .collect(),
                    )
                });
            }
            RowMessage::Loaded(children) => {
                self.loading = false;
                self.input.on_loaded.call(LoadResult {
                    children,
                    key: self.input.key.clone(),
                });
            }
            RowMessage::Select => self.input.on_select.call(self.input.key.clone()),
            RowMessage::Toggle => {
                self.input.on_toggle.call(self.input.key.clone());
                if !self.input.loaded && !self.loading {
                    _ = context.sender().send(RowMessage::Load);
                }
            }
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        let select = context.sender();
        let toggle = context.sender();
        let marker = if self.input.selected { ">" } else { " " };
        let state = if self.loading {
            "loading"
        } else if self.input.loaded {
            if self.input.expanded {
                "collapse"
            } else {
                "expand"
            }
        } else {
            "load"
        };
        reactor::StackPanel::new()
            .orientation(reactor::Orientation::Horizontal)
            .spacing(6.0)
            .children([
                reactor::TextBlock::new()
                    .text(format!("{marker} {}", self.input.label))
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Select"))
                    .on_click(move || {
                        _ = select.send(RowMessage::Select);
                    })
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text(state))
                    .on_click(move || {
                        _ = toggle.send(RowMessage::Toggle);
                    })
                    .into(),
            ])
            .into()
    }
}

struct Details;

impl reactor::Component for Details {
    type Input = Option<String>;
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
            .text(
                input
                    .as_deref()
                    .map_or("No node selected".to_string(), |key| {
                        format!("Selected node: {key}")
                    }),
            )
            .into()
    }
}

enum ExplorerMessage {
    Filter(String),
    Loaded(LoadResult),
    Reverse,
    Select(String),
    Toggle(String),
}

struct Explorer {
    filter: Rc<str>,
    nodes: Vec<Node>,
    on_loaded: reactor::Callback<LoadResult>,
    on_select: reactor::Callback<String>,
    on_toggle: reactor::Callback<String>,
    selected: Option<String>,
}

impl Explorer {
    fn declaration(&self, node: &Node) -> Option<reactor::TreeNode> {
        let children = node
            .children
            .iter()
            .filter_map(|child| self.declaration(child))
            .collect::<Vec<_>>();
        let matches = self.filter.is_empty()
            || node
                .label
                .to_ascii_lowercase()
                .contains(&self.filter.to_ascii_lowercase());
        if !matches && children.is_empty() {
            return None;
        }
        let row = RowInput {
            expanded: node.expanded,
            key: node.key.clone(),
            label: node.label.clone(),
            loaded: node.loaded,
            on_loaded: self.on_loaded.clone(),
            on_select: self.on_select.clone(),
            on_toggle: self.on_toggle.clone(),
            selected: self.selected.as_ref() == Some(&node.key),
        };
        Some(
            reactor::TreeNode::new(node.key.clone(), node.label.clone())
                .expanded(node.expanded)
                .content(reactor::component::<Row>(format!("row:{}", node.key), row))
                .children(children),
        )
    }
}

impl reactor::Component for Explorer {
    type Input = ();
    type Message = ExplorerMessage;

    fn create(_input: &Self::Input, context: &reactor::ComponentContext<Self>) -> Self {
        let loaded = context.sender();
        let select = context.sender();
        let toggle = context.sender();
        Self {
            filter: Rc::from(""),
            nodes: vec![
                Node::branch("projects", "Projects"),
                Node::branch("references", "References"),
                Node::leaf("readme", "README.md"),
            ],
            on_loaded: reactor::Callback::new(move |result| {
                _ = loaded.send(ExplorerMessage::Loaded(result));
            }),
            on_select: reactor::Callback::new(move |key| {
                _ = select.send(ExplorerMessage::Select(key));
            }),
            on_toggle: reactor::Callback::new(move |key| {
                _ = toggle.send(ExplorerMessage::Toggle(key));
            }),
            selected: None,
        }
    }

    fn update(&mut self, message: Self::Message, _context: &reactor::ComponentContext<Self>) {
        match message {
            ExplorerMessage::Filter(value) => self.filter = Rc::from(value),
            ExplorerMessage::Loaded(result) => {
                if let Some(node) = find_node_mut(&mut self.nodes, &result.key) {
                    node.children = result.children;
                    node.expanded = true;
                    node.loaded = true;
                }
            }
            ExplorerMessage::Reverse => self.nodes.reverse(),
            ExplorerMessage::Select(key) => self.selected = Some(key),
            ExplorerMessage::Toggle(key) => {
                if let Some(node) = find_node_mut(&mut self.nodes, &key) {
                    node.expanded = !node.expanded;
                }
            }
        }
    }

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        let filter = context.sender();
        let reverse = context.sender();
        let nodes = self
            .nodes
            .iter()
            .filter_map(|node| self.declaration(node))
            .collect::<Vec<_>>();
        reactor::StackPanel::new()
            .spacing(8.0)
            .children([
                reactor::TextBlock::new()
                    .text("Reactor TreeView explorer")
                    .into(),
                reactor::TextBlock::new()
                    .text(
                        "Selection uses row actions until native TreeView selection is projected.",
                    )
                    .into(),
                reactor::TextBox::new(Rc::clone(&self.filter))
                    .on_text_changed(move |value: Rc<str>| {
                        _ = filter.send(ExplorerMessage::Filter(value.to_string()));
                    })
                    .into(),
                reactor::Button::new()
                    .content(reactor::TextBlock::new().text("Reverse roots"))
                    .on_click(move || {
                        _ = reverse.send(ExplorerMessage::Reverse);
                    })
                    .into(),
                reactor::TreeView::new().nodes(nodes).into(),
                reactor::Border::new()
                    .content(reactor::component::<Details>(
                        "details",
                        self.selected.clone(),
                    ))
                    .into(),
            ])
            .into()
    }
}

fn main() -> windows_core::Result<()> {
    App::run_component::<Explorer>(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(parent: &str, child: &str) -> [reactor::Key; 2] {
        [reactor::Key::from(parent), reactor::Key::from(child)]
    }

    #[test]
    fn reorder_preserves_row_identity_and_selection_updates_details() {
        let mut host = reactor::ComponentHost::mount(
            reactor::RecordingAdapter::default(),
            [reactor::component::<Explorer>("explorer", ())],
        )
        .unwrap();
        let projects_path = path("explorer", "row:projects");
        let references_path = path("explorer", "row:references");
        let details_path = path("explorer", "details");
        let projects = host.reference_at(&projects_path).unwrap().get();
        let references = host.reference_at(&references_path).unwrap().get();
        let details = host.reference_at(&details_path).unwrap().get().unwrap();

        assert!(
            host.sender::<Explorer>(&reactor::Key::from("explorer"))
                .unwrap()
                .send(ExplorerMessage::Reverse)
        );
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 1);
        assert_eq!(host.reference_at(&projects_path).unwrap().get(), projects);
        assert_eq!(
            host.reference_at(&references_path).unwrap().get(),
            references
        );

        assert!(
            host.sender_at::<Row>(&projects_path)
                .unwrap()
                .send(RowMessage::Select)
        );
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 2);
        assert_eq!(
            host.runtime()
                .graph()
                .properties(details)
                .unwrap()
                .iter()
                .find(|property| property.id == reactor::PropertyId::Text)
                .map(|property| &property.value),
            Some(&reactor::PropertyValue::String(Rc::from(
                "Selected node: projects"
            )))
        );
    }

    #[test]
    fn filtering_retires_rows_and_rejects_stale_messages() {
        let mut host = reactor::ComponentHost::mount(
            reactor::RecordingAdapter::default(),
            [reactor::component::<Explorer>("explorer", ())],
        )
        .unwrap();
        let projects_path = path("explorer", "row:projects");
        let reference = host.reference_at(&projects_path).unwrap();
        let original = reference.get();
        let stale = host.sender_at::<Row>(&projects_path).unwrap();
        let explorer = host
            .sender::<Explorer>(&reactor::Key::from("explorer"))
            .unwrap();

        assert!(explorer.send(ExplorerMessage::Filter("README".into())));
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 1);
        assert_eq!(reference.get(), None);
        assert!(host.sender_at::<Row>(&projects_path).is_none());

        assert!(stale.send(RowMessage::Toggle));
        assert_eq!(host.drain(usize::MAX).unwrap().dropped, 1);

        assert!(explorer.send(ExplorerMessage::Filter(String::new())));
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 1);
        let replacement = host.reference_at(&projects_path).unwrap().get();
        assert!(replacement.is_some());
        assert_ne!(replacement, original);

        assert!(stale.send(RowMessage::Toggle));
        assert_eq!(host.drain(usize::MAX).unwrap().dropped, 1);
        assert!(
            host.sender_at::<Row>(&projects_path)
                .unwrap()
                .send(RowMessage::Toggle)
        );
        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 3);
    }
}
