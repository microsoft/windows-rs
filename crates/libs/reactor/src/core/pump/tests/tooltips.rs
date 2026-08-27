use super::super::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

fn target(pump: &Pump<RecordingRuntime>) -> NodeId {
    Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap()
}

fn tooltip_text(runtime: &RecordingRuntime, tooltip: NodeId) -> &str {
    let content = runtime.node(tooltip).unwrap().children()[0];
    let PropertyValue::Str(text) = runtime
        .node(content)
        .unwrap()
        .property(PropertyId::TextBlockText)
        .unwrap()
    else {
        panic!("expected tooltip text");
    };
    text
}

#[test]
fn mounts_and_replaces_text_tooltips_without_replacing_the_owner() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        Button::new()
            .content(TextBlock::new().text("Owner"))
            .tooltip("First"),
    )
    .unwrap();

    let owner = target(&pump);
    let (tooltip, placement) = pump.runtime().tooltip(owner).unwrap();
    assert_eq!(placement, TooltipPlacement::Top);
    assert_eq!(tooltip_text(pump.runtime(), tooltip), "First");

    pump.update_view(
        Button::new()
            .content(TextBlock::new().text("Owner"))
            .tooltip("Second"),
    )
    .unwrap();

    assert_eq!(target(&pump), owner);
    assert_eq!(
        pump.runtime().tooltip(owner),
        Some((tooltip, TooltipPlacement::Top))
    );
    assert_eq!(tooltip_text(pump.runtime(), tooltip), "Second");
}

#[test]
fn updates_all_supported_placements_on_the_stable_attachment() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        TextBlock::new()
            .text("Owner")
            .tooltip_with(Tooltip::text("Tip").placement(TooltipPlacement::Bottom)),
    )
    .unwrap();

    let owner = target(&pump);
    let tooltip = pump.runtime().tooltip(owner).unwrap().0;
    for placement in [
        TooltipPlacement::Left,
        TooltipPlacement::Right,
        TooltipPlacement::Mouse,
        TooltipPlacement::Top,
    ] {
        pump.update_view(
            TextBlock::new()
                .text("Owner")
                .tooltip_with(Tooltip::text("Tip").placement(placement)),
        )
        .unwrap();
        assert_eq!(pump.runtime().tooltip(owner), Some((tooltip, placement)));
    }
}

#[test]
fn owns_and_reconciles_rich_tooltip_children() {
    let rich = |detail| {
        Tooltip::rich(StackPanel::new().spacing(4.0).children((
            TextBlock::new().text("Action: Save"),
            TextBlock::new().text(detail),
        )))
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        Button::new()
            .content(TextBlock::new().text("Save"))
            .tooltip_with(rich("Writes the document.")),
    )
    .unwrap();

    let owner = target(&pump);
    let tooltip = pump.runtime().tooltip(owner).unwrap().0;
    let panel = pump.runtime().node(tooltip).unwrap().children()[0];
    let children = pump.runtime().node(panel).unwrap().children();
    assert_eq!(children.len(), 2);

    pump.update_view(
        Button::new()
            .content(TextBlock::new().text("Save"))
            .tooltip_with(rich("Writes the current document to disk.")),
    )
    .unwrap();

    assert_eq!(pump.runtime().tooltip(owner).unwrap().0, tooltip);
    let detail = pump.runtime().node(panel).unwrap().children()[1];
    assert_eq!(
        pump.runtime()
            .node(detail)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str(
            "Writes the current document to disk.".to_string()
        ))
    );
}

#[test]
fn clears_on_removal_and_moves_ownership_on_target_replacement() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        Button::new()
            .content(TextBlock::new().text("Owner"))
            .tooltip("Tip"),
    )
    .unwrap();
    let first_owner = target(&pump);
    let first_tooltip = pump.runtime().tooltip(first_owner).unwrap().0;

    pump.update_view(TextBlock::new().text("Replacement").tooltip("Tip"))
        .unwrap();
    let second_owner = target(&pump);
    assert_ne!(second_owner, first_owner);
    assert_eq!(pump.runtime().tooltip(first_owner), None);
    assert!(pump.runtime().tooltip(second_owner).is_some());

    pump.update_view(TextBlock::new().text("Replacement").into())
        .unwrap();
    assert_eq!(pump.runtime().tooltip(second_owner), None);
    assert!(pump.runtime().node(first_tooltip).is_none());
}

#[test]
fn keyed_tooltip_wrappers_reconcile_in_child_lists_across_updates() {
    let view = |first: &str, second: &str| {
        StackPanel::new().keyed_children([
            KeyedView::new("tip", TextBlock::new().text(first).tooltip("Help")),
            KeyedView::new("plain", TextBlock::new().text(second)),
        ])
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view("First", "Second")).unwrap();
    let panel = pump.root().unwrap();
    let logical = pump.tree.children(panel).unwrap().to_vec();
    let owner = pump.runtime().node(panel).unwrap().children()[0];

    pump.update_view(view("Updated", "Second")).unwrap();
    pump.update_view(view("Updated again", "Changed")).unwrap();

    assert_eq!(pump.tree.children(panel).unwrap(), logical);
    assert_eq!(pump.runtime().node(panel).unwrap().children()[0], owner);
    assert!(pump.runtime().tooltip(owner).is_some());
}

#[test]
fn component_local_target_replacement_refreshes_before_destroy() {
    #[derive(Clone)]
    struct Input(Rc<RefCell<Option<LocalSender<bool>>>>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct SwitchingTarget(bool);

    impl Component for SwitchingTarget {
        type Input = Input;
        type Message = bool;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.0.borrow_mut() = Some(context.sender());
            Self(false)
        }

        fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self>) {}

        fn update(&mut self, button: bool, _context: &ComponentContext<Self>) {
            self.0 = button;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            if self.0 {
                Button::new().content(TextBlock::new().text("Button"))
            } else {
                View::native(TextBlock::new().text("Text"))
            }
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<SwitchingTarget>(Input(Rc::clone(&sender))).tooltip("Help"))
        .unwrap();
    let old_target = target(&pump);
    let tooltip = pump.runtime().tooltip(old_target).unwrap().0;

    assert!(sender.borrow().as_ref().unwrap().send(true));
    pump.dispatch_components(1).unwrap();

    let new_target = target(&pump);
    assert_ne!(new_target, old_target);
    assert_eq!(pump.runtime().tooltip(old_target), None);
    assert_eq!(
        pump.runtime().tooltip(new_target),
        Some((tooltip, TooltipPlacement::Top))
    );
    let commands = pump.runtime().commands().last().unwrap();
    let clear = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetTooltip {
                    target,
                    tooltip: None,
                    ..
                } if *target == old_target
            )
        })
        .unwrap();
    let destroy = commands
        .iter()
        .position(|command| *command == Command::Destroy { node: old_target })
        .unwrap();
    let attach = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetTooltip {
                    target,
                    tooltip: Some(current),
                    ..
                } if *target == new_target && *current == tooltip
            )
        })
        .unwrap();
    assert!(clear < destroy);
    assert!(destroy < attach);
}

#[test]
fn stable_target_and_tooltip_updates_do_not_churn_attachment() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TextBlock::new().text("Owner").tooltip("First"))
        .unwrap();
    let owner = target(&pump);
    let tooltip = pump.runtime().tooltip(owner).unwrap().0;

    pump.update_view(TextBlock::new().text("Changed").tooltip("Second"))
        .unwrap();

    assert_eq!(target(&pump), owner);
    assert_eq!(
        pump.runtime().tooltip(owner),
        Some((tooltip, TooltipPlacement::Top))
    );
    assert!(
        !pump
            .runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(command, Command::SetTooltip { .. }))
    );
}

#[test]
fn component_content_replacement_does_not_churn_stable_target_attachment() {
    #[derive(Clone)]
    struct Input(Rc<RefCell<Option<LocalSender<bool>>>>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct StableTarget(bool);

    impl Component for StableTarget {
        type Input = Input;
        type Message = bool;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.0.borrow_mut() = Some(context.sender());
            Self(false)
        }

        fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self>) {}

        fn update(&mut self, changed: bool, _context: &ComponentContext<Self>) {
            self.0 = changed;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            let content = if self.0 {
                Border::new().content(TextBlock::new().text("Changed"))
            } else {
                View::from(TextBlock::new().text("Initial"))
            };
            Button::new().content(content)
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<StableTarget>(Input(Rc::clone(&sender))).tooltip("Help"))
        .unwrap();
    let owner = target(&pump);
    let tooltip = pump.runtime().tooltip(owner).unwrap().0;

    assert!(sender.borrow().as_ref().unwrap().send(true));
    pump.dispatch_components(1).unwrap();

    assert_eq!(target(&pump), owner);
    assert_eq!(
        pump.runtime().tooltip(owner),
        Some((tooltip, TooltipPlacement::Top))
    );
    assert!(
        !pump
            .runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(command, Command::SetTooltip { .. }))
    );
}

#[test]
fn wrapping_and_unwrapping_preserves_the_native_target() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TextBlock::new().text("Owner").into())
        .unwrap();
    let owner = target(&pump);

    pump.update_view(TextBlock::new().text("Wrapped").tooltip("Help"))
        .unwrap();
    assert_eq!(target(&pump), owner);
    assert!(pump.runtime().tooltip(owner).is_some());

    pump.update_view(TextBlock::new().text("Unwrapped").into())
        .unwrap();
    assert_eq!(target(&pump), owner);
    assert_eq!(pump.runtime().tooltip(owner), None);
}

#[test]
fn nested_and_direct_native_tooltips_are_rejected() {
    let mut nested = Pump::new(RecordingRuntime::default());
    assert_eq!(
        nested.mount_view(TextBlock::new().tooltip("Inner").tooltip("Outer")),
        Err(PumpError::StructureUnsupported)
    );

    let mut direct = Pump::new(RecordingRuntime::default());
    assert_eq!(
        direct.mount_view(ToolTip::new().content(TextBlock::new().text("Direct"))),
        Err(PumpError::StructureUnsupported)
    );
}
