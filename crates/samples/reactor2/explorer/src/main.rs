#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use windows_reactor2 as reactor2;
use windows_reactor2::{App, AppContext};

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
    on_loaded: reactor2::Callback<LoadResult>,
    on_select: reactor2::Callback<String>,
    on_toggle: reactor2::Callback<String>,
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

impl reactor2::Component for Row {
    type Input = RowInput;
    type Message = RowMessage;

    fn create(input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self {
            input: input.clone(),
            loading: false,
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
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
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
        reactor2::StackPanel::new()
            .orientation(reactor2::Orientation::Horizontal)
            .spacing(6.0)
            .children([
                reactor2::TextBlock::new(format!("{marker} {}", self.input.label)).into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new("Select"))
                    .on_click(move || {
                        _ = select.send(RowMessage::Select);
                    })
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new(state))
                    .on_click(move || {
                        _ = toggle.send(RowMessage::Toggle);
                    })
                    .into(),
            ])
            .into()
    }
}

struct Details;

impl reactor2::Component for Details {
    type Input = Option<String>;
    type Message = ();

    fn create(_input: &Self::Input, _context: &reactor2::ComponentContext<Self::Message>) -> Self {
        Self
    }

    fn view(
        &self,
        input: &Self::Input,
        _context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        reactor2::TextBlock::new(
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
    on_loaded: reactor2::Callback<LoadResult>,
    on_select: reactor2::Callback<String>,
    on_toggle: reactor2::Callback<String>,
    selected: Option<String>,
}

impl Explorer {
    fn declaration(&self, node: &Node) -> Option<reactor2::TreeNode> {
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
            reactor2::TreeNode::new(node.key.clone(), node.label.clone())
                .expanded(node.expanded)
                .content(reactor2::component::<Row>(format!("row:{}", node.key), row))
                .children(children),
        )
    }
}

impl reactor2::Component for Explorer {
    type Input = ();
    type Message = ExplorerMessage;

    fn create(_input: &Self::Input, context: &reactor2::ComponentContext<Self::Message>) -> Self {
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
            on_loaded: reactor2::Callback::new(move |result| {
                _ = loaded.send(ExplorerMessage::Loaded(result));
            }),
            on_select: reactor2::Callback::new(move |key| {
                _ = select.send(ExplorerMessage::Select(key));
            }),
            on_toggle: reactor2::Callback::new(move |key| {
                _ = toggle.send(ExplorerMessage::Toggle(key));
            }),
            selected: None,
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _context: &reactor2::ComponentContext<Self::Message>,
    ) {
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
        context: &mut reactor2::ComponentViewContext<'_, Self::Message>,
    ) -> reactor2::Visual {
        let filter = context.sender();
        let reverse = context.sender();
        let nodes = self
            .nodes
            .iter()
            .filter_map(|node| self.declaration(node))
            .collect::<Vec<_>>();
        reactor2::StackPanel::new()
            .spacing(8.0)
            .children([
                reactor2::TextBlock::new("Reactor2 TreeView explorer").into(),
                reactor2::TextBlock::new(
                    "Selection uses row actions until native TreeView selection is projected.",
                )
                .into(),
                reactor2::TextBox::new(Rc::clone(&self.filter))
                    .on_text_changed(move |value| {
                        _ = filter.send(ExplorerMessage::Filter(value.to_string()));
                    })
                    .into(),
                reactor2::Button::new()
                    .content(reactor2::TextBlock::new("Reverse roots"))
                    .on_click(move || {
                        _ = reverse.send(ExplorerMessage::Reverse);
                    })
                    .into(),
                reactor2::TreeView::new().nodes(nodes).into(),
                reactor2::Border::new()
                    .content(reactor2::component::<Details>(
                        "details",
                        self.selected.clone(),
                    ))
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
        let state = Rc::new(RefCell::new(None::<Self>));
        let drain_state = Rc::clone(&state);
        let drain = context.callback(move || {
            let mut state = drain_state.borrow_mut();
            let host = &mut state.as_mut().unwrap().host;
            host.drain(usize::MAX)?;
            host.runtime()
                .adapter()
                .validate_graph(host.runtime().graph())
                .map_err(Into::into)
        });
        let mut host = reactor2::ComponentHost::mount_with_services(
            reactor2::native::WinUiAdapter::default(),
            context.component_services(),
            [reactor2::component::<Explorer>("explorer", ())],
        )?;
        let wake = drain.clone();
        host.set_waker(move || {
            _ = wake.invoke();
        });
        let wake = drain;
        host.runtime_mut().adapter_mut().set_event_waker(move || {
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

    fn path(parent: &str, child: &str) -> [reactor2::Key; 2] {
        [reactor2::Key::from(parent), reactor2::Key::from(child)]
    }

    #[test]
    fn reorder_preserves_row_identity_and_selection_updates_details() {
        let mut host = reactor2::ComponentHost::mount(
            reactor2::RecordingAdapter::default(),
            [reactor2::component::<Explorer>("explorer", ())],
        )
        .unwrap();
        let projects_path = path("explorer", "row:projects");
        let references_path = path("explorer", "row:references");
        let details_path = path("explorer", "details");
        let projects = host.reference_at(&projects_path).unwrap().get();
        let references = host.reference_at(&references_path).unwrap().get();
        let details = host.reference_at(&details_path).unwrap().get().unwrap();

        assert!(
            host.sender::<Explorer>(&reactor2::Key::from("explorer"))
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
                .find(|property| property.id == reactor2::PropertyId::Text)
                .map(|property| &property.value),
            Some(&reactor2::PropertyValue::String(Rc::from(
                "Selected node: projects"
            )))
        );
    }

    #[test]
    fn filtering_retires_rows_and_rejects_stale_messages() {
        let mut host = reactor2::ComponentHost::mount(
            reactor2::RecordingAdapter::default(),
            [reactor2::component::<Explorer>("explorer", ())],
        )
        .unwrap();
        let projects_path = path("explorer", "row:projects");
        let reference = host.reference_at(&projects_path).unwrap();
        let original = reference.get();
        let stale = host.sender_at::<Row>(&projects_path).unwrap();
        let explorer = host
            .sender::<Explorer>(&reactor2::Key::from("explorer"))
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
