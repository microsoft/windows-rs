//! Named-slot mounting and reconciliation contracts.

use super::super::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;

fn navigation(content: Option<View>, header: Option<View>) -> View {
    let mut slots = Vec::new();
    if let Some(content) = content {
        slots.push(SlotView::new(NavigationViewSlot::Content, content));
    }
    if let Some(header) = header {
        slots.push(SlotView::new(NavigationViewSlot::Header, header));
    }
    NavigationView::new().slots(slots)
}

#[test]
fn named_slots_mount_update_replace_and_clear_independently() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(navigation(
        Some(View::native(TextBlock::new().text("content-1"))),
        Some(View::native(TextBlock::new().text("header-1"))),
    ))
    .unwrap();

    let root = pump.root().unwrap();
    let content = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot(SlotId::NavigationViewContent)
        .unwrap();
    let header = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot(SlotId::NavigationViewHeader)
        .unwrap();

    pump.update_view(navigation(
        Some(View::native(TextBlock::new().text("content-2"))),
        Some(View::native(Slider::new())),
    ))
    .unwrap();

    let recorded = pump.runtime().node(root).unwrap();
    assert_eq!(recorded.slot(SlotId::NavigationViewContent), Some(content));
    assert_ne!(recorded.slot(SlotId::NavigationViewHeader), Some(header));
    assert_eq!(
        pump.runtime()
            .node(content)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("content-2".into()))
    );

    let header = recorded.slot(SlotId::NavigationViewHeader).unwrap();
    let header_slot = pump.tree.children(root).unwrap()[1];
    assert_eq!(
        pump.tree.kind(header_slot),
        Ok(NodeKind::NamedSlot(SlotId::NavigationViewHeader))
    );
    assert_eq!(pump.tree.children(header_slot).unwrap(), &[header]);
    assert_eq!(
        pump.tree.kind(header),
        Ok(NodeKind::Native(MountedKind::Slider))
    );
    pump.update_view(navigation(
        None,
        Some(View::native(Slider::new().is_enabled(false))),
    ))
    .unwrap();

    let recorded = pump.runtime().node(root).unwrap();
    assert_eq!(recorded.slot(SlotId::NavigationViewContent), None);
    assert_eq!(recorded.slot(SlotId::NavigationViewHeader), Some(header));
    assert_eq!(
        pump.runtime()
            .node(header)
            .unwrap()
            .property(PropertyId::SliderIsEnabled),
        Some(&PropertyValue::Bool(false))
    );
}

#[test]
fn named_slot_rejects_duplicate_assignments_and_multiple_native_roots() {
    let duplicate = NavigationView::new().slots([
        SlotView::new(NavigationViewSlot::Content, View::native(TextBlock::new())),
        SlotView::new(NavigationViewSlot::Content, View::native(Button::new())),
    ]);
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(duplicate),
        Err(PumpError::StructureUnsupported)
    );
    assert!(pump.root().is_none());

    let multiple = View::fragment([TextBlock::new().into(), Button::new().into()]);
    assert_eq!(
        pump.mount_view(navigation(Some(multiple), None)),
        Err(PumpError::StructureUnsupported)
    );
    assert!(pump.root().is_none());
}

#[test]
fn named_slots_preserve_context_and_component_effect_lifecycle() {
    #[derive(Clone)]
    struct Props {
        context: Rc<Context<String>>,
        log: Rc<RefCell<Vec<String>>>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.context, &other.context) && Rc::ptr_eq(&self.log, &other.log)
        }
    }

    struct Consumer(Props);

    impl Component for Consumer {
        type Message = ();
        type Props = Props;

        fn create(props: &Props, _context: &mut ComponentContext<Self>) -> Self {
            Self(props.clone())
        }

        fn changed(&mut self, props: &Props, _context: &mut ComponentContext<Self>) {
            self.0 = props.clone();
        }

        fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

        fn view(&self, _props: &Self::Props, context: &mut ViewContext<Self>) -> View {
            let value = context.use_context(&self.0.context);
            let log = Rc::clone(&self.0.log);
            let effect_value = value.clone();
            context.use_effect(effect_value.clone(), move || {
                log.borrow_mut().push(format!("setup {effect_value}"));
                Some(Box::new(move || {
                    log.borrow_mut().push(format!("cleanup {effect_value}"));
                }))
            });
            View::native(TextBlock::new().text(value))
        }
    }

    let context = Rc::new(Context::new("default".to_string()));
    let log = Rc::new(RefCell::new(Vec::new()));
    let content = View::component::<Consumer>(Props {
        context: Rc::clone(&context),
        log: Rc::clone(&log),
    });
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::provide(
        &context,
        "provided".to_string(),
        navigation(Some(content), None),
    ))
    .unwrap();

    assert_eq!(&*log.borrow(), &["setup provided"]);
    let navigation_node = pump.tree.children(pump.root().unwrap()).unwrap()[0];
    let content = pump
        .runtime()
        .node(navigation_node)
        .unwrap()
        .slot(SlotId::NavigationViewContent)
        .unwrap();
    assert_eq!(
        pump.runtime()
            .node(content)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("provided".into()))
    );

    pump.update_view(View::provide(
        &context,
        "provided".to_string(),
        navigation(None, None),
    ))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup provided", "cleanup provided"]);
}
