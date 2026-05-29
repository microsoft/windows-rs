use crate::controls::*;
use windows_reactor::*;

pub fn tree_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (invoked, set_invoked) = cx.use_state(String::from("(none)"));

    let file_system = vec![
        tree_node("Documents").expanded().children(vec![
            tree_node("Work").children(vec![tree_node("Report.docx"), tree_node("Budget.xlsx")]),
            tree_node("Personal").children(vec![tree_node("Resume.pdf")]),
        ]),
        tree_node("Pictures").children(vec![tree_node("Vacation.jpg"), tree_node("Family.png")]),
        tree_node("Music").children(vec![tree_node("Song1.mp3"), tree_node("Song2.mp3")]),
    ];

    page_content(
        "TreeView",
        "A hierarchical list with expanding and collapsing nodes.",
        vec![
            sample_card(
                "File Explorer TreeView",
                vstack((
                    tree_view(file_system).on_item_invoked(move |s| set_invoked.call(s)),
                    text_block(format!("Last invoked: {invoked}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"tree_view(nodes).on_item_invoked(|s| set(s))"#,
            ),
            sample_card(
                "Flat TreeView",
                tree_view(vec![
                    tree_node("Item A"),
                    tree_node("Item B"),
                    tree_node("Item C"),
                ]),
                r#"tree_view(vec![tree_node("A"), tree_node("B")])"#,
            ),
        ],
    )
    .into()
}
