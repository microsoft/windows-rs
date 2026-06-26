//! Sample for the `TreeView` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (last_invoked, set_last_invoked) = cx.use_state(String::from("(none)"));

    let nodes = vec![
        tree_node("Documents").expanded().children(vec![
            tree_node("Work").child(tree_node("report.docx")),
            tree_node("Personal").children(vec![tree_node("budget.xlsx"), tree_node("notes.txt")]),
        ]),
        tree_node("Pictures").children(vec![tree_node("vacation.jpg"), tree_node("family.png")]),
        tree_node("Music").child(tree_node("playlist.m3u")),
    ];

    vstack((
        tree_view(nodes).on_item_invoked(set_last_invoked),
        text_block(format!("Last invoked: {last_invoked}")),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("TreeView", app)
}
