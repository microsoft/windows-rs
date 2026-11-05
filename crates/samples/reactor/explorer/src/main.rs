#![windows_subsystem = "windows"]

use std::rc::Rc;
use std::time::Duration;
use windows_reactor::*;

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
            loaded: true,
            ..Self::branch(key, label)
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
    can_expand: bool,
    on_loaded: Callback<LoadResult>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
    selected: bool,
}

#[derive(Clone)]
enum RowMessage {
    Cancelled,
    Loaded(Vec<Node>),
    Select,
    Toggle,
}

struct Row {
    input: RowInput,
    loading: bool,
}

impl Component for Row {
    type Input = RowInput;
    type Message = RowMessage;

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            input: input.clone(),
            loading: false,
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, message: RowMessage, context: &ComponentContext<Self>) {
        match message {
            RowMessage::Cancelled => self.loading = false,
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
                    self.loading = true;
                    let key = self.input.key.clone();
                    context.spawn_background(move |token| {
                        for _ in 0..10 {
                            if token.is_cancelled() {
                                return RowMessage::Cancelled;
                            }
                            std::thread::sleep(Duration::from_millis(100));
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
            }
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let action: View = if input.can_expand {
            Button::new()
                .content(if self.loading {
                    "Loading..."
                } else if !input.loaded {
                    "Load"
                } else if input.expanded {
                    "Collapse"
                } else {
                    "Expand"
                })
                .is_enabled(!self.loading)
                .on_click(context.message(RowMessage::Toggle))
                .into()
        } else {
            "File".into()
        };
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(6.0)
            .children((
                input.label.clone(),
                Button::new()
                    .content(if input.selected { "Selected" } else { "Select" })
                    .on_click(context.message(RowMessage::Select)),
                action,
            ))
            .into()
    }
}

#[derive(Clone)]
enum Message {
    Filter(Rc<str>),
    Loaded(LoadResult),
    Reverse,
    Select(String),
    Toggle(String),
}

struct Explorer {
    filter: Rc<str>,
    nodes: Vec<Node>,
    on_loaded: Callback<LoadResult>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
    selected: Option<String>,
}

impl Explorer {
    fn declaration(&self, node: &Node, filter: &str) -> Option<TreeNode> {
        let children: Vec<_> = node
            .children
            .iter()
            .filter_map(|child| self.declaration(child, filter))
            .collect();
        if !node.label.to_ascii_lowercase().contains(filter) && children.is_empty() {
            return None;
        }
        let row = RowInput {
            expanded: node.expanded,
            key: node.key.clone(),
            label: node.label.clone(),
            loaded: node.loaded,
            can_expand: !node.loaded || !node.children.is_empty(),
            on_loaded: self.on_loaded.clone(),
            on_select: self.on_select.clone(),
            on_toggle: self.on_toggle.clone(),
            selected: self.selected.as_ref() == Some(&node.key),
        };
        Some(
            TreeNode::new(node.key.as_str(), &node.label)
                .expanded(node.expanded)
                .content(component::<Row>(format!("row:{}", node.key), row))
                .children(children),
        )
    }
}

impl Component for Explorer {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        Self {
            filter: Rc::from(""),
            nodes: vec![
                Node::branch("projects", "Projects"),
                Node::branch("references", "References"),
                Node::leaf("readme", "README.md"),
            ],
            on_loaded: context.callback(Message::Loaded),
            on_select: context.callback(Message::Select),
            on_toggle: context.callback(Message::Toggle),
            selected: None,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Filter(value) => self.filter = value,
            Message::Loaded(result) => {
                let node = find_node_mut(&mut self.nodes, &result.key).unwrap();
                node.children = result.children;
                node.loaded = true;
            }
            Message::Reverse => self.nodes.reverse(),
            Message::Select(key) => self.selected = Some(key),
            Message::Toggle(key) => {
                let node = find_node_mut(&mut self.nodes, &key).unwrap();
                node.expanded = !node.loaded || !node.expanded;
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let filter = self.filter.to_ascii_lowercase();
        let nodes = self
            .nodes
            .iter()
            .filter_map(|node| self.declaration(node, &filter));
        context.window_frame(
            "Tree explorer",
            Grid::new()
                .margin(24.0)
                .row_spacing(8.0)
                .rows([
                    GridLength::Auto,
                    GridLength::Auto,
                    GridLength::Auto,
                    GridLength::STAR,
                    GridLength::Auto,
                ])
                .children((
                    TextBlock::new()
                        .text(
                            "Load branches and filter the tree. Reversing roots keeps each row's \
                               state; filtering a row out cancels its pending load.",
                        )
                        .text_wrapping(TextWrapping::Wrap),
                    TextBox::new()
                        .grid_row(1)
                        .placeholder_text("Filter loaded nodes")
                        .text(self.filter.clone())
                        .on_text_changed(context.callback(Message::Filter)),
                    Button::new()
                        .grid_row(2)
                        .content("Reverse roots")
                        .on_click(context.message(Message::Reverse)),
                    TreeView::new().grid_row(3).nodes(nodes),
                    TextBlock::new().grid_row(4).text(match &self.selected {
                        Some(key) => format!("Selected node: {key}"),
                        None => "No node selected".into(),
                    }),
                )),
        )
    }
}

fn main() {
    App::run_component::<Explorer>(()).unwrap();
}
