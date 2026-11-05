use crate::controls::*;
use std::rc::Rc;
use windows_reactor::*;

pub struct TreeViewPage {
    last_invoked: Rc<str>,
}

impl Component for TreeViewPage {
    type Message = Rc<str>;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_invoked: "(none)".into(),
        }
    }

    fn update(&mut self, text: Rc<str>, _context: &ComponentContext<Self>) {
        self.last_invoked = text;
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
                        .children([TreeNode::new("resume", "Resume.pdf")]),
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
                                .on_item_invoked(context.forward())
                                .nodes(file_system),
                            TextBlock::new()
                                .text(format!("Last invoked: {}", self.last_invoked))
                                .opacity(0.6),
                        )),
                    ),
                ),
                KeyedView::new(
                    "flat-tree-view",
                    sample_card("Flat TreeView", TreeView::new().nodes(flat)),
                ),
            ],
        )
    }
}
