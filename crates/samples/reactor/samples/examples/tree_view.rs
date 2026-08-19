#![windows_subsystem = "windows"]

use std::collections::BTreeSet;

use windows_reactor::{Element, RenderCx, StackPanel, TextBlock, Thickness, TreeNode, TreeView};

fn node_name(key: u64) -> &'static str {
    match key {
        1 => "Documents",
        2 => "Work",
        3 => "report.docx",
        4 => "Personal",
        5 => "budget.xlsx",
        6 => "notes.txt",
        7 => "Pictures",
        8 => "vacation.jpg",
        9 => "family.png",
        10 => "Music",
        11 => "playlist.m3u",
        _ => unreachable!(),
    }
}

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let last_invoked = cx.use_state(|| String::from("(none)"));
    let expanded = cx.use_state(|| BTreeSet::from([1]));
    let current_expanded = expanded.value();
    let nodes = [
        TreeNode::new(1, "Documents")
            .expanded(current_expanded.contains(&1))
            .children([
                TreeNode::new(2, "Work")
                    .expanded(current_expanded.contains(&2))
                    .child(TreeNode::new(3, "report.docx")),
                TreeNode::new(4, "Personal")
                    .expanded(current_expanded.contains(&4))
                    .children([
                        TreeNode::new(5, "budget.xlsx"),
                        TreeNode::new(6, "notes.txt"),
                    ]),
            ]),
        TreeNode::new(7, "Pictures")
            .expanded(current_expanded.contains(&7))
            .children([
                TreeNode::new(8, "vacation.jpg"),
                TreeNode::new(9, "family.png"),
            ]),
        TreeNode::new(10, "Music")
            .expanded(current_expanded.contains(&10))
            .child(TreeNode::new(11, "playlist.m3u")),
    ];

    StackPanel::new([
        TreeView::new(nodes, move |key, value| {
            expanded.update(|keys| {
                if value {
                    keys.insert(key);
                } else {
                    keys.remove(&key);
                }
            });
        })
        .on_item_invoked({
            let last_invoked = last_invoked.clone();
            move |key| {
                last_invoked.set(node_name(key).into());
            }
        })
        .automation_id("tree")
        .build(),
        TextBlock::new(format!("Last invoked: {}", last_invoked.value()))
            .automation_id("last-invoked")
            .build(),
    ])
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TreeView", app)
}
