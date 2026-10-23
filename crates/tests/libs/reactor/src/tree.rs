use super::*;

pub(super) struct TreeFixture {
    roots: Vec<&'static str>,
    fruit: Vec<&'static str>,
    fruit_expanded: bool,
    renamed: bool,
    invoked: Rc<str>,
    calls: u32,
}

#[derive(Clone)]
pub(super) enum Message {
    InsertChild,
    RemoveChild,
    ReverseChildren,
    ReverseRoots,
    Rename,
    Collapse,
    Expand,
    RemoveRoot,
    RestoreRoot,
    Invoked(Rc<str>),
}

impl Component for TreeFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            roots: vec!["fruit", "veg"],
            fruit: vec!["apple", "banana"],
            fruit_expanded: true,
            renamed: false,
            invoked: Rc::from("none"),
            calls: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::InsertChild => self.fruit.insert(1, "cherry"),
            Message::RemoveChild => self.fruit.retain(|key| *key != "banana"),
            Message::ReverseChildren => self.fruit.reverse(),
            Message::ReverseRoots => self.roots.reverse(),
            Message::Rename => self.renamed = true,
            Message::Collapse => self.fruit_expanded = false,
            Message::Expand => self.fruit_expanded = true,
            Message::RemoveRoot => self.roots.retain(|key| *key != "fruit"),
            Message::RestoreRoot => self.roots.insert(0, "fruit"),
            Message::Invoked(text) => {
                self.invoked = text;
                self.calls += 1;
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let roots = self.roots.iter().map(|&key| match key {
            "fruit" => TreeNode::new(key, if self.renamed { "Fruits" } else { "Fruit" })
                .expanded(self.fruit_expanded)
                .children(self.fruit.iter().map(|&key| TreeNode::new(key, title(key)))),
            _ => TreeNode::new(key, "Vegetables").children([TreeNode::new("carrot", "Carrot")
                .content(
                    TextBlock::new()
                        .automation_id("carrot-content")
                        .text("Carrot content"),
                )]),
        });
        context.window_title("Reactor UI fixture: tree");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 700.0));
        let button = |id: &'static str, message| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(message))
        };
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                TreeView::new()
                    .automation_id("tree")
                    .height(300.0)
                    .nodes(roots)
                    .on_item_invoked(context.callback(Message::Invoked)),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(4.0)
                    .children((
                        button("insert-child", Message::InsertChild),
                        button("remove-child", Message::RemoveChild),
                        button("reverse-children", Message::ReverseChildren),
                        button("reverse-roots", Message::ReverseRoots),
                        button("rename", Message::Rename),
                    )),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(4.0)
                    .children((
                        button("collapse", Message::Collapse),
                        button("expand", Message::Expand),
                        button("remove-root", Message::RemoveRoot),
                        button("restore-root", Message::RestoreRoot),
                    )),
                TextBlock::new()
                    .automation_id("invoked")
                    .text(format!("Invoked {}; calls {}", self.invoked, self.calls)),
            ))
            .into()
    }
}

fn title(key: &str) -> String {
    let mut title = key.to_string();
    title[..1].make_ascii_uppercase();
    title
}
