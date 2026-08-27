use windows_reactor::*;

struct TreeViewSample {
    last_invoked: String,
}

impl Component for TreeViewSample {
    type Message = String;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_invoked: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: String, _context: &ComponentContext<Self>) {
        self.last_invoked = message;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("TreeView");
        let nodes = [
            TreeNode::new("documents", "Documents")
                .expanded(true)
                .children([
                    TreeNode::new("work", "Work").child(TreeNode::new("report", "report.docx")),
                    TreeNode::new("personal", "Personal").children([
                        TreeNode::new("budget", "budget.xlsx"),
                        TreeNode::new("notes", "notes.txt"),
                    ]),
                ]),
            TreeNode::new("pictures", "Pictures").children([
                TreeNode::new("vacation", "vacation.jpg"),
                TreeNode::new("family", "family.png"),
            ]),
            TreeNode::new("music", "Music").child(TreeNode::new("playlist", "playlist.m3u")),
        ];

        StackPanel::new().spacing(12.0).children((
            TreeView::new()
                .on_item_invoked(context.callback(std::convert::identity))
                .nodes(nodes),
            format!("Last invoked: {}", self.last_invoked),
        ))
    }
}

fn main() {
    App::run_component::<TreeViewSample>(()).unwrap();
}
