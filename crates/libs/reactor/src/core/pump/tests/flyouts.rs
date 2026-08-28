use super::super::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

fn target(pump: &Pump<RecordingRuntime>) -> NodeId {
    Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap()
}

fn flyout_text(runtime: &RecordingRuntime, content: NodeId) -> &str {
    let PropertyValue::Str(text) = runtime
        .node(content)
        .unwrap()
        .property(PropertyId::TextBlockText)
        .unwrap()
    else {
        panic!("expected flyout text");
    };
    text
}

#[test]
fn mounts_and_updates_text_without_target_or_attachment_churn() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        Button::new()
            .content(TextBlock::new().text("Owner"))
            .flyout("First"),
    )
    .unwrap();

    let owner = target(&pump);
    let (content, placement) = pump.runtime().flyout(owner).unwrap();
    assert_eq!(placement, FlyoutPlacement::Top);
    assert_eq!(flyout_text(pump.runtime(), content), "First");

    pump.update_view(
        Button::new()
            .content(TextBlock::new().text("Owner"))
            .flyout("Second"),
    )
    .unwrap();

    assert_eq!(target(&pump), owner);
    assert_eq!(
        pump.runtime().flyout(owner),
        Some((content, FlyoutPlacement::Top))
    );
    assert_eq!(flyout_text(pump.runtime(), content), "Second");
    assert!(
        !pump
            .runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(command, Command::SetFlyout { .. }))
    );
}

#[test]
fn applies_bottom_placement_and_replaces_or_removes_attachment() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        Button::new()
            .content(TextBlock::new().text("Owner"))
            .flyout_with(Flyout::text("First").placement(FlyoutPlacement::Bottom)),
    )
    .unwrap();
    let first_owner = target(&pump);
    let first_content = pump.runtime().flyout(first_owner).unwrap().0;
    assert_eq!(
        pump.runtime().flyout(first_owner),
        Some((first_content, FlyoutPlacement::Bottom))
    );

    pump.update_view(
        Button::new()
            .content(TextBlock::new().text("Replacement"))
            .flyout_with(Flyout::rich(
                Border::new().content(TextBlock::new().text("Rich")),
            )),
    )
    .unwrap();
    assert_eq!(target(&pump), first_owner);
    let replacement = pump.runtime().flyout(first_owner).unwrap().0;
    assert_ne!(replacement, first_content);
    assert!(pump.runtime().node(first_content).is_none());

    pump.update_view(Button::new().content(TextBlock::new().text("Replacement")))
        .unwrap();
    assert_eq!(pump.runtime().flyout(first_owner), None);
    assert!(pump.runtime().node(replacement).is_none());
}

#[test]
fn component_target_replacement_detaches_before_destroy_and_reattaches() {
    #[derive(Clone)]
    struct Input(Rc<RefCell<Option<LocalSender<bool>>>>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct SwitchingButton(bool);

    struct FirstButton;
    struct SecondButton;

    macro_rules! button_component {
        ($name:ident, $label:literal) => {
            impl Component for $name {
                type Input = ();
                type Message = ();

                fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
                    Self
                }

                fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

                fn view(&self, _input: &(), _context: &mut ViewContext<Self>) -> View {
                    Button::new().content(TextBlock::new().text($label))
                }
            }
        };
    }

    button_component!(FirstButton, "First");
    button_component!(SecondButton, "Second");

    impl Component for SwitchingButton {
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
            if self.0 {
                View::component::<SecondButton>(())
            } else {
                View::component::<FirstButton>(())
            }
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        View::component::<SwitchingButton>(Input(Rc::clone(&sender))).flyout("Content"),
    )
    .unwrap();
    let old_target = target(&pump);
    let content = pump.runtime().flyout(old_target).unwrap().0;

    assert!(sender.borrow().as_ref().unwrap().send(true));
    pump.dispatch_components(1).unwrap();

    let new_target = target(&pump);
    assert_ne!(new_target, old_target);
    assert_eq!(pump.runtime().flyout(old_target), None);
    assert_eq!(
        pump.runtime().flyout(new_target),
        Some((content, FlyoutPlacement::Top))
    );
    let commands = pump.runtime().commands().last().unwrap();
    let clear = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetFlyout {
                    target,
                    content: None,
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
                Command::SetFlyout {
                    target,
                    content: Some(current),
                    ..
                } if *target == new_target && *current == content
            )
        })
        .unwrap();
    assert!(clear < destroy);
    assert!(destroy < attach);
}

#[test]
fn keyed_parent_reconciliation_preserves_attachment() {
    let view = |label: &str| {
        StackPanel::new().keyed_children([
            KeyedView::new(
                "flyout",
                Button::new()
                    .content(TextBlock::new().text(label))
                    .flyout("Content"),
            ),
            KeyedView::new("plain", TextBlock::new().text("Plain")),
        ])
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view("First")).unwrap();
    let panel = pump.root().unwrap();
    let logical = pump.tree.children(panel).unwrap().to_vec();
    let owner = pump.runtime().node(panel).unwrap().children()[0];
    let content = pump.runtime().flyout(owner).unwrap().0;

    pump.update_view(view("Second")).unwrap();

    assert_eq!(pump.tree.children(panel).unwrap(), logical);
    assert_eq!(pump.runtime().node(panel).unwrap().children()[0], owner);
    assert_eq!(
        pump.runtime().flyout(owner),
        Some((content, FlyoutPlacement::Top))
    );
}

#[test]
fn rejects_non_button_and_nested_owned_attachments() {
    let mut non_button = Pump::new(RecordingRuntime::default());
    assert_eq!(
        non_button.mount_view(TextBlock::new().text("Owner").flyout("Content")),
        Err(PumpError::StructureUnsupported)
    );

    let mut nested = Pump::new(RecordingRuntime::default());
    assert_eq!(
        nested.mount_view(
            Button::new()
                .content(TextBlock::new().text("Owner"))
                .flyout("Inner")
                .tooltip("Outer"),
        ),
        Err(PumpError::StructureUnsupported)
    );
}

#[test]
fn split_button_supports_rich_flyout_content() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        SplitButton::new()
            .content(TextBlock::new().text("Paste"))
            .flyout_with(Flyout::rich(
                Button::new().content(TextBlock::new().text("Keep text only")),
            )),
    )
    .unwrap();

    let owner = target(&pump);
    assert_eq!(
        pump.runtime().node(owner).unwrap().kind(),
        Some(MountedKind::SplitButton)
    );
    let (content, placement) = pump.runtime().flyout(owner).unwrap();
    assert_eq!(placement, FlyoutPlacement::Top);
    assert_eq!(
        pump.runtime().node(content).unwrap().kind(),
        Some(MountedKind::Button)
    );
}
