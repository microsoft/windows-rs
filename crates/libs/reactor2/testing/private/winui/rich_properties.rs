use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::rich_access as rich_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_RICH_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn rich_controls_and_tree_update_natively() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::rich_properties::rich_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn rich_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let text_hits = Rc::new(Cell::new(0usize));
    let text_hits_for_render = Rc::clone(&text_hits);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let current = phase.value();
        let hits = Rc::clone(&text_hits_for_render);
        let rich_edit = RichEditBox::new(format!("edit {current}"), move |_| {
            hits.set(hits.get() + 1);
        })
        .read_only(current == 1)
        .build();
        let rich_text = RichTextBlock::new([RichTextParagraph::new([
            RichTextInline::Run(RichTextRun {
                text: format!("run {current}"),
                bold: current == 1,
                italic: current == 1,
            }),
            RichTextInline::LineBreak,
        ])])
        .font_size(if current == 0 { 12.0 } else { 18.0 })
        .selectable(current == 1)
        .wrap(current == 1)
        .build();
        let nodes = if current == 0 {
            [
                TreeNode::new(1, "First").child(TreeNode::new(2, "Child")),
                TreeNode::new(3, "Second"),
            ]
        } else {
            [
                TreeNode::new(3, "Second updated").child(TreeNode::new(2, "Child")),
                TreeNode::new(1, "First"),
            ]
        };
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new(
                    "Rich fixture",
                    StackPanel::new([rich_edit, rich_text, TreeView::display(nodes).build()])
                        .build(),
                    move || {
                        close.set(false);
                    },
                )
                .build(),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let probe = RuntimeProbe::new(reactor.engine().runtime());
        let edit = probe.nodes(NativeKind::RichEditBox)[0];
        let rich_text = probe.nodes(NativeKind::RichTextBlock)[0];
        let tree = probe.nodes(NativeKind::TreeView)[0];
        assert_eq!(
            rich_probe::rich_edit_box(reactor.engine().runtime(), edit)?,
            ("edit 0".into(), false)
        );
        assert_eq!(
            rich_probe::rich_text_block(reactor.engine().runtime(), rich_text)?,
            (
                12.0,
                false,
                false,
                vec![vec![
                    ("run 0".into(), false, false),
                    ("\n".into(), false, false)
                ]]
            )
        );
        let first = rich_probe::tree_node_identity(reactor.engine().runtime(), tree, 1)?;
        let child = rich_probe::tree_node_identity(reactor.engine().runtime(), tree, 2)?;

        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_eq!(
            rich_probe::rich_edit_box(reactor.engine().runtime(), edit)?,
            ("edit 1".into(), true)
        );
        assert_eq!(text_hits.get(), 0);
        assert_eq!(
            rich_probe::rich_text_block(reactor.engine().runtime(), rich_text)?,
            (
                18.0,
                true,
                true,
                vec![vec![
                    ("run 1".into(), true, true),
                    ("\n".into(), false, false)
                ]]
            )
        );
        assert_eq!(
            rich_probe::tree_node_identity(reactor.engine().runtime(), tree, 1)?,
            first
        );
        assert_eq!(
            rich_probe::tree_node_identity(reactor.engine().runtime(), tree, 2)?,
            child
        );
        assert!(!rich_probe::tree_node_expanded(
            reactor.engine().runtime(),
            tree,
            3
        )?);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}
