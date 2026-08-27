use crate::controls::*;
use windows_reactor::*;

pub struct TreeViewPage {
    last_invoked: String,
}

#[derive(Clone)]
pub enum Message {
    Invoked(String),
}

impl Component for TreeViewPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_invoked: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Invoked(text) => self.last_invoked = text,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let file_system = [
            TreeNode::new("documents", "Documents")
                .expanded(true)
                .children([
                    TreeNode::new("work", "Work").children([
                        TreeNode::new("report", "Report.docx"),
                        TreeNode::new("budget", "Budget.xlsx"),
                    ]),
                    TreeNode::new("personal", "Personal")
                        .child(TreeNode::new("resume", "Resume.pdf")),
                ]),
            TreeNode::new("pictures", "Pictures").children([
                TreeNode::new("vacation", "Vacation.jpg"),
                TreeNode::new("family", "Family.png"),
            ]),
            TreeNode::new("music", "Music").children([
                TreeNode::new("song1", "Song1.mp3"),
                TreeNode::new("song2", "Song2.mp3"),
            ]),
        ];
        let flat = [
            TreeNode::new("item-a", "Item A"),
            TreeNode::new("item-b", "Item B"),
            TreeNode::new("item-c", "Item C"),
        ];

        page_content(
            "TreeView",
            "A hierarchical list with expanding and collapsing nodes.",
            [
                KeyedView::new(
                    "file-explorer-tree-view",
                    sample_card(
                        "File Explorer TreeView",
                        StackPanel::new().spacing(8.0).children((
                            TreeView::new()
                                .on_item_invoked(context.callback(Message::Invoked))
                                .nodes(file_system),
                            TextBlock::new()
                                .text(format!("Last invoked: {}", self.last_invoked))
                                .opacity(0.6),
                        )),
                        r#"TreeView::new().on_item_invoked(...).nodes(nodes)"#,
                    ),
                ),
                KeyedView::new(
                    "flat-tree-view",
                    sample_card(
                        "Flat TreeView",
                        TreeView::new().nodes(flat),
                        r#"TreeView::new().nodes([TreeNode::new("a", "Item A"), ...])"#,
                    ),
                ),
            ],
        )
    }
}
