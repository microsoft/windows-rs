use super::*;
use crate::Element;

#[test]
fn framework_height_mounts_updates_skips_and_clears() {
    let height = Rc::new(RefCell::new(None::<State<Option<f64>>>));
    let height_for_render = Rc::clone(&height);
    let root = component(move |cx| {
        let state = cx.use_state(|| Some(24.0));
        *height_for_render.borrow_mut() = Some(state.clone());
        TextBlock::new("value").height(state.get().unwrap()).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| height_update(command) == Some((target, Dimension::Pixels(24.0))))
    );

    let state = height.borrow().as_ref().unwrap().clone();
    let batches = reactor.engine().runtime().batches().len();
    assert!(state.try_set(Some(24.0)));
    reactor.pump();
    assert!(
        reactor.engine().runtime().batches()[batches..]
            .iter()
            .flatten()
            .all(|command| height_update(command).is_none())
    );

    assert!(state.try_set(Some(48.0)));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| height_update(command) == Some((target, Dimension::Pixels(48.0))))
    );

    assert!(state.try_set(None));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| height_update(command) == Some((target, Dimension::Default)))
    );

    let batches = reactor.engine().runtime().batches().len();
    assert!(state.try_set(Some(f64::NAN)));
    reactor.pump();
    assert!(
        reactor.engine().runtime().batches()[batches..]
            .iter()
            .flatten()
            .all(|command| height_update(command).is_none())
    );
}

#[derive(Clone)]
struct SizeValues {
    width: Option<f64>,
    height: Option<f64>,
    min_width: Option<f64>,
    max_width: Option<f64>,
    min_height: Option<f64>,
    max_height: Option<f64>,
}

#[test]
fn framework_size_constraints_mount_update_skip_and_clear_together() {
    let sizes = Rc::new(RefCell::new(None::<State<SizeValues>>));
    let sizes_for_render = Rc::clone(&sizes);
    let root = component(move |cx| {
        let state = cx.use_state(|| SizeValues {
            width: Some(120.0),
            height: Some(24.0),
            min_width: Some(50.0),
            max_width: Some(400.0),
            min_height: Some(20.0),
            max_height: Some(200.0),
        });
        *sizes_for_render.borrow_mut() = Some(state.clone());
        let values = state.get().unwrap();
        TextBlock::new("value")
            .width(values.width)
            .height(values.height)
            .min_width(values.min_width)
            .max_width(values.max_width)
            .min_height(values.min_height)
            .max_height(values.max_height)
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert_eq!(
        framework_updates(&reactor, target),
        [
            FrameworkUpdate::Width(Dimension::Pixels(120.0)),
            FrameworkUpdate::Height(Dimension::Pixels(24.0)),
            FrameworkUpdate::MinWidth(Dimension::Pixels(50.0)),
            FrameworkUpdate::MaxWidth(Dimension::Pixels(400.0)),
            FrameworkUpdate::MinHeight(Dimension::Pixels(20.0)),
            FrameworkUpdate::MaxHeight(Dimension::Pixels(200.0)),
        ]
    );

    let state = sizes.borrow().as_ref().unwrap().clone();
    let batches = reactor.engine().runtime().batches().len();
    assert!(state.try_set(state.get().unwrap()));
    reactor.pump();
    assert!(
        reactor.engine().runtime().batches()[batches..]
            .iter()
            .flatten()
            .all(|command| !matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Framework(_),
                    ..
                }
            ))
    );

    assert!(state.try_set(SizeValues {
        width: Some(160.0),
        height: Some(48.0),
        min_width: Some(60.0),
        max_width: Some(500.0),
        min_height: Some(30.0),
        max_height: Some(240.0),
    }));
    reactor.pump();
    assert_eq!(
        framework_updates_in_last_batch(&reactor, target),
        [
            FrameworkUpdate::Width(Dimension::Pixels(160.0)),
            FrameworkUpdate::Height(Dimension::Pixels(48.0)),
            FrameworkUpdate::MinWidth(Dimension::Pixels(60.0)),
            FrameworkUpdate::MaxWidth(Dimension::Pixels(500.0)),
            FrameworkUpdate::MinHeight(Dimension::Pixels(30.0)),
            FrameworkUpdate::MaxHeight(Dimension::Pixels(240.0)),
        ]
    );

    assert!(state.try_set(SizeValues {
        width: None,
        height: None,
        min_width: None,
        max_width: None,
        min_height: None,
        max_height: None,
    }));
    reactor.pump();
    assert_eq!(
        framework_updates_in_last_batch(&reactor, target),
        [
            FrameworkUpdate::Width(Dimension::Default),
            FrameworkUpdate::Height(Dimension::Default),
            FrameworkUpdate::MinWidth(Dimension::Default),
            FrameworkUpdate::MaxWidth(Dimension::Default),
            FrameworkUpdate::MinHeight(Dimension::Default),
            FrameworkUpdate::MaxHeight(Dimension::Default),
        ]
    );
}

#[derive(Clone)]
struct LayoutValues {
    margin: Option<Thickness>,
    horizontal: Option<HorizontalAlignment>,
    vertical: Option<VerticalAlignment>,
}

#[test]
fn framework_layout_mounts_updates_skips_and_clears_together() {
    let layout = Rc::new(RefCell::new(None::<State<LayoutValues>>));
    let layout_for_render = Rc::clone(&layout);
    let root = component(move |cx| {
        let state = cx.use_state(|| LayoutValues {
            margin: Some(Thickness::xy(8.0, 4.0)),
            horizontal: Some(HorizontalAlignment::Left),
            vertical: Some(VerticalAlignment::Top),
        });
        *layout_for_render.borrow_mut() = Some(state.clone());
        let values = state.get().unwrap();
        TextBlock::new("value")
            .margin(values.margin)
            .horizontal_alignment(values.horizontal)
            .vertical_alignment(values.vertical)
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert_eq!(
        framework_updates(&reactor, target),
        [
            FrameworkUpdate::Margin(Some(Thickness::xy(8.0, 4.0))),
            FrameworkUpdate::HorizontalAlignment(Some(HorizontalAlignment::Left)),
            FrameworkUpdate::VerticalAlignment(Some(VerticalAlignment::Top)),
        ]
    );

    let state = layout.borrow().as_ref().unwrap().clone();
    let batches = reactor.engine().runtime().batches().len();
    assert!(state.try_set(state.get().unwrap()));
    reactor.pump();
    assert!(
        reactor.engine().runtime().batches()[batches..]
            .iter()
            .flatten()
            .all(|command| !matches!(
                command,
                Command::Update {
                    update: NativeUpdate::Framework(_),
                    ..
                }
            ))
    );

    assert!(state.try_set(LayoutValues {
        margin: Some(Thickness::uniform(12.0)),
        horizontal: Some(HorizontalAlignment::Center),
        vertical: Some(VerticalAlignment::Bottom),
    }));
    reactor.pump();
    assert_eq!(
        framework_updates_in_last_batch(&reactor, target),
        [
            FrameworkUpdate::Margin(Some(Thickness::uniform(12.0))),
            FrameworkUpdate::HorizontalAlignment(Some(HorizontalAlignment::Center)),
            FrameworkUpdate::VerticalAlignment(Some(VerticalAlignment::Bottom)),
        ]
    );

    assert!(state.try_set(LayoutValues {
        margin: None,
        horizontal: None,
        vertical: None,
    }));
    reactor.pump();
    assert_eq!(
        framework_updates_in_last_batch(&reactor, target),
        [
            FrameworkUpdate::Margin(None),
            FrameworkUpdate::HorizontalAlignment(None),
            FrameworkUpdate::VerticalAlignment(None),
        ]
    );
}

#[test]
fn framework_height_targets_dissimilar_controls_without_control_specific_paths() {
    let root = stack_panel([
        TextBlock::new("text").height(24.0).build(),
        TextBox::new("input", |_| {}).height(32.0).build(),
        StackPanel::new([text_block("child")]).height(40.0).build(),
    ]);
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    let heights = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(height_update)
        .map(|(_, value)| value)
        .collect::<Vec<_>>();
    assert_eq!(
        heights,
        [
            Dimension::Pixels(24.0),
            Dimension::Pixels(32.0),
            Dimension::Pixels(40.0)
        ]
    );
}

#[test]
fn visibility_targets_dissimilar_controls_and_clears_to_visible() {
    let collapsed = Rc::new(RefCell::new(None::<State<bool>>));
    let collapsed_for_render = Rc::clone(&collapsed);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *collapsed_for_render.borrow_mut() = Some(state.clone());
        let visibility = state.get().unwrap().then_some(Visibility::Collapsed);
        stack_panel([
            TextBlock::new("text").visibility(visibility).build(),
            Button::new("button")
                .on_click(|| {})
                .visibility(visibility)
                .build(),
            CheckBox::new("check", false, |_| {})
                .visibility(visibility)
                .build(),
            TextBox::new("input", |_| {}).visibility(visibility).build(),
        ])
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let visibility_updates = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(visibility_update)
        .map(|(_, value)| value)
        .collect::<Vec<_>>();
    assert_eq!(visibility_updates, [Some(Visibility::Collapsed); 4]);

    assert!(collapsed.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(visibility_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [None; 4]
    );
}

#[test]
fn opacity_targets_dissimilar_controls_and_clears_to_opaque() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let opacity = match state.get().unwrap() {
            0 => Some(0.25),
            1 => Some(0.75),
            2 => None,
            _ => Some(f32::NAN),
        };
        StackPanel::new([
            TextBlock::new("text").opacity(opacity).build(),
            Button::new("button")
                .on_click(|| {})
                .opacity(opacity)
                .build(),
            CheckBox::new("check", false, |_| {})
                .opacity(opacity)
                .build(),
            TextBox::new("input", |_| {}).opacity(opacity).build(),
        ])
        .opacity(opacity)
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(opacity_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(0.25); 5]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(opacity_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(0.75); 5]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(opacity_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [None; 5]
    );

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(opacity_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(opacity_update)
            .count(),
        update_count
    );
}

#[test]
fn text_style_targets_text_controls_and_clears_local_values() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let font_size = match state.get().unwrap() {
            0 => Some(16.0),
            1 => Some(24.0),
            2 => None,
            _ => Some(f32::NAN),
        };
        let character_spacing = match state.get().unwrap() {
            0 => Some(i32::MIN),
            1 => Some(100),
            _ => None,
        };
        let font_weight = match state.get().unwrap() {
            0 => Some(FontWeight::BOLD),
            1 => Some(FontWeight::LIGHT),
            _ => None,
        };
        let font_style = match state.get().unwrap() {
            0 => Some(FontStyle::Italic),
            1 => Some(FontStyle::Oblique),
            _ => None,
        };
        let font_stretch = match state.get().unwrap() {
            0 => Some(FontStretch::Condensed),
            1 => Some(FontStretch::Expanded),
            _ => None,
        };
        let font_family = match state.get().unwrap() {
            0 => Some("Arial".to_string()),
            1 => Some("Consolas".to_string()),
            2 => None,
            _ => Some(String::new()),
        };
        StackPanel::new([
            TextBlock::new("text")
                .font_size(font_size)
                .character_spacing(character_spacing)
                .font_weight(font_weight)
                .font_style(font_style)
                .font_stretch(font_stretch)
                .font_family(font_family.clone())
                .build(),
            Button::new("button")
                .on_click(|| {})
                .font_size(font_size)
                .character_spacing(character_spacing)
                .font_weight(font_weight)
                .font_style(font_style)
                .font_stretch(font_stretch)
                .font_family(font_family.clone())
                .build(),
            CheckBox::new("check", false, |_| {})
                .font_size(font_size)
                .character_spacing(character_spacing)
                .font_weight(font_weight)
                .font_style(font_style)
                .font_stretch(font_stretch)
                .font_family(font_family.clone())
                .build(),
            TextBox::new("input", |_| {})
                .font_size(font_size)
                .character_spacing(character_spacing)
                .font_weight(font_weight)
                .font_style(font_style)
                .font_stretch(font_stretch)
                .font_family(font_family)
                .build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_size_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(16.0); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(character_spacing_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(i32::MIN); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_weight_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(FontWeight::BOLD); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_style_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(FontStyle::Italic); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_stretch_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(FontStretch::Condensed); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_family_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some("Arial"); 4]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_size_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(24.0); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(character_spacing_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(100); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_weight_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(FontWeight::LIGHT); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_style_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(FontStyle::Oblique); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_stretch_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(FontStretch::Expanded); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_family_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some("Consolas"); 4]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_size_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [None; 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_family_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [None; 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(character_spacing_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [None; 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_style_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [None; 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_stretch_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [None; 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_weight_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [None; 4]
    );

    let font_size_update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(font_size_update)
        .count();
    let character_spacing_update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(character_spacing_update)
        .count();
    let font_weight_update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(font_weight_update)
        .count();
    let font_style_update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(font_style_update)
        .count();
    let font_stretch_update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(font_stretch_update)
        .count();
    let font_family_update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(font_family_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_size_update)
            .count(),
        font_size_update_count
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(character_spacing_update)
            .count(),
        character_spacing_update_count
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_weight_update)
            .count(),
        font_weight_update_count
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_style_update)
            .count(),
        font_style_update_count
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_stretch_update)
            .count(),
        font_stretch_update_count
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(font_family_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [Some(""); 4]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_family_update)
            .count(),
        font_family_update_count + 4
    );
    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(font_family_update)
            .count(),
        font_family_update_count + 4
    );
}

#[test]
fn foreground_targets_text_controls_and_clears_local_values() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let foreground = match state.get().unwrap() {
            0 => Some(Color::rgb(10, 20, 30)),
            1 => Some(Color::argb(128, 40, 50, 60)),
            _ => None,
        };
        StackPanel::new([
            TextBlock::new("text").foreground(foreground).build(),
            Button::new("button")
                .on_click(|| {})
                .foreground(foreground)
                .build(),
            CheckBox::new("check", false, |_| {})
                .foreground(foreground)
                .build(),
            TextBox::new("input", |_| {}).foreground(foreground).build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(foreground_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        vec![Some(Color::rgb(10, 20, 30).into()); 4]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(foreground_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        vec![Some(Color::argb(128, 40, 50, 60).into()); 4]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(foreground_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        vec![None; 4]
    );

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(foreground_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(foreground_update)
            .count(),
        update_count
    );
}

#[test]
fn grid_placement_updates_clear_and_precede_attachment() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        let element = if matches!(version, 2..=4) {
            Button::new("cell").on_click(|| {}).build()
        } else {
            TextBlock::new("cell").build()
        };
        let child = grid_child(element.key(7));
        let child = match version {
            0 => child.row(1).column(2).row_span(3).column_span(4),
            1 | 4 => child.row(5).column(2).row_span(3).column_span(4),
            _ => child,
        };
        Grid::new([child]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    let grid = native_node(&reactor, NativeKind::Grid);
    let first = reactor.engine().runtime().batches().first().unwrap();
    let updates = first
        .iter()
        .filter_map(grid_update)
        .filter(|(id, _)| *id == target)
        .map(|(_, update)| update)
        .collect::<Vec<_>>();
    assert_eq!(
        updates,
        [
            AttachedUpdate::Row(Some(1)),
            AttachedUpdate::Column(Some(2)),
            AttachedUpdate::RowSpan(Some(3)),
            AttachedUpdate::ColumnSpan(Some(4)),
        ]
    );
    let update_position = first
        .iter()
        .position(|command| grid_update(command).is_some())
        .unwrap();
    let attach_position = first
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Attach {
                    parent,
                    child,
                    ..
                } if *parent == grid && *child == target
            )
        })
        .unwrap();
    assert!(update_position < attach_position);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [(target, AttachedUpdate::Row(Some(5)))]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    let target = native_node(&reactor, NativeKind::Button);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::Row(Some(5))),
            (target, AttachedUpdate::Column(Some(2))),
            (target, AttachedUpdate::RowSpan(Some(3))),
            (target, AttachedUpdate::ColumnSpan(Some(4))),
        ]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::Row(None)),
            (target, AttachedUpdate::Column(None)),
            (target, AttachedUpdate::RowSpan(None)),
            (target, AttachedUpdate::ColumnSpan(None)),
        ]
    );

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(grid_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(grid_update)
            .count(),
        update_count
    );
}

#[test]
fn grid_definitions_mount_update_clear_and_skip_unchanged_values() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        match state.get().unwrap() {
            0 | 1 => Grid::new([text_block("cell")])
                .columns([
                    GridLength::Auto,
                    GridLength::Pixel(64.0),
                    GridLength::Star(2.0),
                ])
                .rows([GridLength::Pixel(18.0)])
                .build(),
            _ => Grid::new([text_block("cell")])
                .columns([GridLength::Pixel(32.0)])
                .build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let grid = native_node(&reactor, NativeKind::Grid);
    let definitions = reactor
        .engine()
        .runtime()
        .batches()
        .first()
        .unwrap()
        .iter()
        .filter_map(grid_definition_update)
        .collect::<Vec<_>>();
    assert_eq!(
        definitions,
        [
            (
                grid,
                GridUpdate::Columns(
                    vec![
                        GridLength::Auto,
                        GridLength::Pixel(64.0),
                        GridLength::Star(2.0),
                    ]
                    .into_boxed_slice(),
                ),
            ),
            (
                grid,
                GridUpdate::Rows(vec![GridLength::Pixel(18.0)].into_boxed_slice()),
            ),
        ]
    );

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(grid_definition_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(grid_definition_update)
            .count(),
        update_count
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_definition_update)
            .collect::<Vec<_>>(),
        [
            (
                grid,
                GridUpdate::Columns(vec![GridLength::Pixel(32.0)].into_boxed_slice()),
            ),
            (grid, GridUpdate::Rows(Box::default())),
        ]
    );
}

#[test]
fn native_style_props_mount_update_clear_and_skip_unchanged_values() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let styled = state.get().unwrap() < 2;
        StackPanel::new([
            Button::new("primary")
                .on_click(|| {})
                .emphasis(if styled {
                    ButtonEmphasis::Accent
                } else {
                    ButtonEmphasis::Standard
                })
                .build(),
            Border::new(text_block("border"))
                .background(styled.then_some(Color::rgb(1, 2, 3)))
                .border_brush(styled.then_some(Color::rgb(4, 5, 6)))
                .border_thickness(styled.then_some(Thickness::uniform(2.0)))
                .corner_radius(styled.then_some(CornerRadius::uniform(8.0)))
                .padding(styled.then_some(Thickness::uniform(4.0)))
                .build(),
            Grid::new([text_block("grid")])
                .column_spacing(if styled { 6.0 } else { 0.0 })
                .row_spacing(if styled { 8.0 } else { 0.0 })
                .build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let button = native_node(&reactor, NativeKind::Button);
    let border = native_node(&reactor, NativeKind::Border);
    let grid = native_node(&reactor, NativeKind::Grid);

    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(button_emphasis_update)
            .collect::<Vec<_>>(),
        [(button, ButtonEmphasis::Accent)]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(border_update)
            .collect::<Vec<_>>(),
        [
            (
                border,
                BorderUpdate::Background(Some(Color::rgb(1, 2, 3).into())),
            ),
            (
                border,
                BorderUpdate::BorderBrush(Some(Color::rgb(4, 5, 6).into())),
            ),
            (
                border,
                BorderUpdate::BorderThickness(Some(Thickness::uniform(2.0))),
            ),
            (
                border,
                BorderUpdate::CornerRadius(Some(CornerRadius::uniform(8.0))),
            ),
            (border, BorderUpdate::Padding(Some(Thickness::uniform(4.0))),),
        ]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(grid_definition_update)
            .collect::<Vec<_>>(),
        [
            (grid, GridUpdate::ColumnSpacing(6.0)),
            (grid, GridUpdate::RowSpacing(8.0)),
        ]
    );

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter(|command| {
            button_emphasis_update(command).is_some()
                || border_update(command).is_some()
                || grid_definition_update(command).is_some()
        })
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter(|command| {
                button_emphasis_update(command).is_some()
                    || border_update(command).is_some()
                    || grid_definition_update(command).is_some()
            })
            .count(),
        update_count
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let commands = reactor.engine().runtime().batches().last().unwrap();
    assert_eq!(
        commands
            .iter()
            .filter_map(button_emphasis_update)
            .collect::<Vec<_>>(),
        [(button, ButtonEmphasis::Standard)]
    );
    assert_eq!(
        commands
            .iter()
            .filter_map(border_update)
            .collect::<Vec<_>>(),
        [
            (border, BorderUpdate::Background(None)),
            (border, BorderUpdate::BorderBrush(None)),
            (border, BorderUpdate::BorderThickness(None)),
            (border, BorderUpdate::CornerRadius(None)),
            (border, BorderUpdate::Padding(None)),
        ]
    );
    assert_eq!(
        commands
            .iter()
            .filter_map(grid_definition_update)
            .collect::<Vec<_>>(),
        [
            (grid, GridUpdate::ColumnSpacing(0.0)),
            (grid, GridUpdate::RowSpacing(0.0)),
        ]
    );
}

#[test]
fn element_resources_mount_replace_clear_and_skip_unchanged_values() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let resources = match state.get().unwrap() {
            0 | 1 => ElementResources::new([
                (
                    "ButtonBackground",
                    ApplicationResource::from(Color::rgb(178, 34, 34)),
                ),
                (
                    "ButtonBorderThemeThickness",
                    ApplicationResource::from(Thickness::uniform(0.0)),
                ),
            ]),
            2 => ElementResources::new([
                (
                    "ButtonBackground",
                    ApplicationResource::from(Color::rgb(0, 90, 158)),
                ),
                (
                    "ControlCornerRadius",
                    ApplicationResource::from(CornerRadius::uniform(8.0)),
                ),
            ]),
            _ => ElementResources::default(),
        };
        Button::new("Delete")
            .resources(resources.entries().map(|(key, value)| (key, value.clone())))
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let button = native_node(&reactor, NativeKind::Button);
    macro_rules! updates {
        () => {
            reactor
                .engine()
                .runtime()
                .batches()
                .iter()
                .flatten()
                .filter_map(resource_update)
                .collect::<Vec<_>>()
        };
    }
    assert_eq!(updates!().len(), 1);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(updates!().len(), 1);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(updates!().len(), 2);
    assert_eq!(updates!()[1].0, button);
    assert_eq!(updates!()[1].1.len(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(updates!().len(), 3);
    assert!(updates!()[2].1.is_empty());
}

#[test]
fn implicit_transitions_and_scale_mount_update_clear_and_skip_unchanged_values() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        Border::new(text_block("animated"))
            .opacity_transition((current < 2).then_some(Duration::from_millis(400)))
            .scale_transition((current < 2).then_some(Duration::from_millis(600)))
            .scale((current < 2).then_some(if current == 0 { 1.0 } else { 1.25 }))
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let border = native_node(&reactor, NativeKind::Border);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();

    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(visual_update)
            .collect::<Vec<_>>(),
        [
            (
                border,
                VisualUpdate::ImplicitTransitions(ImplicitTransitions {
                    opacity: Some(Duration::from_millis(400)),
                    scale: Some(Duration::from_millis(600)),
                }),
            ),
            (border, VisualUpdate::Scale(Some(1.0))),
            (border, VisualUpdate::Scale(Some(1.25))),
            (
                border,
                VisualUpdate::ImplicitTransitions(ImplicitTransitions::default()),
            ),
            (border, VisualUpdate::Scale(None)),
        ]
    );
}

#[test]
fn fade_transition_enters_and_delays_exit_removal_until_completion() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        StackPanel::new([fade_transition(
            if state.get().unwrap() {
                text_block("animated")
            } else {
                fragment([])
            },
            Some(Duration::from_millis(100)),
            Some(Duration::from_millis(200)),
        )])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| visual_update(command)
                == Some((
                    target,
                    VisualUpdate::FadeTo {
                        opacity: 1.0,
                        duration: Duration::from_millis(100),
                    },
                )))
    );

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert!(reactor.engine().runtime().contains(target));
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| visual_update(command)
                == Some((
                    target,
                    VisualUpdate::FadeTo {
                        opacity: 0.0,
                        duration: Duration::from_millis(200),
                    },
                )))
    );
    let timer = *reactor.engine().runtime().timers().values().next().unwrap();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TimerFired {
            owner: timer.owner,
            slot: timer.slot,
            revision: timer.revision,
        });
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(target));
    assert!(reactor.engine().runtime().timers().is_empty());
}

#[test]
fn fade_transition_exit_can_be_interrupted_and_rejects_stale_completion() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        StackPanel::new([fade_transition(
            if state.get().unwrap() {
                text_block("animated")
            } else {
                fragment([])
            },
            Some(Duration::from_millis(100)),
            Some(Duration::from_millis(200)),
        )])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    let stale = *reactor.engine().runtime().timers().values().next().unwrap();
    assert!(visible.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    assert!(reactor.engine().runtime().timers().is_empty());
    assert!(reactor.engine().runtime().contains(target));
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| visual_update(command)
                == Some((
                    target,
                    VisualUpdate::FadeTo {
                        opacity: 1.0,
                        duration: Duration::from_millis(100),
                    },
                )))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TimerFired {
            owner: stale.owner,
            slot: stale.slot,
            revision: stale.revision,
        });
    reactor.pump();
    assert!(reactor.engine().runtime().contains(target));
}

#[test]
fn fade_transition_uses_updated_exit_duration_and_stops_on_parent_removal() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        match state.get().unwrap() {
            3 => text_block("removed"),
            current => fade_transition(
                if current == 0 {
                    text_block("animated")
                } else {
                    fragment([])
                },
                None,
                Some(if current == 1 {
                    Duration::from_millis(200)
                } else {
                    Duration::from_millis(400)
                }),
            ),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let stale = *reactor.engine().runtime().timers().values().next().unwrap();
    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let current = *reactor.engine().runtime().timers().values().next().unwrap();
    assert_eq!(current.interval, Duration::from_millis(400));
    assert_ne!(current.revision, stale.revision);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TimerFired {
            owner: stale.owner,
            slot: stale.slot,
            revision: stale.revision,
        });
    reactor.pump();
    assert!(reactor.engine().runtime().contains(target));

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(target));
    assert!(reactor.engine().runtime().timers().is_empty());
}

#[test]
fn fade_transition_without_exit_duration_removes_immediately() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        StackPanel::new([fade_transition(
            if state.get().unwrap() {
                text_block("animated")
            } else {
                fragment([])
            },
            None,
            None,
        )])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(target));
    assert!(reactor.engine().runtime().timers().is_empty());
}

#[test]
fn stack_panel_layout_and_text_padding_update_clear_and_skip_unchanged_values() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() < 2 {
            StackPanel::new([TextBlock::new("cell")
                .padding(Thickness::xy(2.0, 1.0))
                .build()])
            .orientation(Orientation::Horizontal)
            .spacing(12.0)
            .padding(Thickness::uniform(8.0))
            .build()
        } else {
            StackPanel::new([TextBlock::new("cell").build()]).build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stack = native_node(&reactor, NativeKind::StackPanel);
    let text = native_node(&reactor, NativeKind::TextBlock);
    let first = reactor.engine().runtime().batches().first().unwrap();
    assert_eq!(
        first
            .iter()
            .filter_map(stack_panel_update)
            .collect::<Vec<_>>(),
        [
            (
                stack,
                StackPanelUpdate::Orientation(Orientation::Horizontal),
            ),
            (stack, StackPanelUpdate::Spacing(12.0)),
        ]
    );
    assert_eq!(
        first.iter().filter_map(padding_update).collect::<Vec<_>>(),
        [
            (stack, Thickness::uniform(8.0)),
            (text, Thickness::xy(2.0, 1.0)),
        ]
    );

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter(|command| {
            stack_panel_update(command).is_some() || padding_update(command).is_some()
        })
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter(|command| {
                stack_panel_update(command).is_some() || padding_update(command).is_some()
            })
            .count(),
        update_count
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let last = reactor.engine().runtime().batches().last().unwrap();
    assert_eq!(
        last.iter()
            .filter_map(stack_panel_update)
            .collect::<Vec<_>>(),
        [
            (stack, StackPanelUpdate::Orientation(Orientation::Vertical),),
            (stack, StackPanelUpdate::Spacing(0.0)),
        ]
    );
    let padding = last.iter().filter_map(padding_update).collect::<Vec<_>>();
    assert_eq!(padding.len(), 2);
    assert_eq!(padding[0].0, stack);
    assert!(padding[0].1.left.is_nan());
    assert_eq!(padding[1].0, text);
    assert!(padding[1].1.left.is_nan());
}

#[test]
fn render_metrics_report_build_diff_skip_and_creation_counts() {
    let value = Rc::new(RefCell::new(None::<State<usize>>));
    let value_for_render = Rc::clone(&value);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *value_for_render.borrow_mut() = Some(state.clone());
        StackPanel::new([
            TextBlock::new(format!("value {}", state.get().unwrap())).build(),
            TextBlock::new("stable").build(),
        ])
        .build()
    });
    let metrics = Rc::new(RefCell::new(Vec::new()));
    let metrics_for_callback = Rc::clone(&metrics);
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.set_render_complete(move |value| {
        metrics_for_callback.borrow_mut().push(*value);
    });

    reactor.pump();
    let initial = metrics.borrow()[0];
    assert!(initial.tree_build_ms >= 0.0);
    assert!(initial.reconcile_ms >= 0.0);
    assert!(initial.effects_ms >= 0.0);
    assert_eq!(initial.elements_diffed, 0);
    assert_eq!(initial.elements_created, 4);

    assert!(value.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let update = metrics.borrow()[1];
    assert!(update.elements_diffed >= 3);
    assert!(update.elements_skipped >= 1);
    assert_eq!(update.elements_created, 0);
}

#[test]
fn memoized_components_skip_equal_dependencies_and_keep_the_latest_render_closure() {
    let parent = Rc::new(RefCell::new(None::<State<usize>>));
    let child = Rc::new(RefCell::new(None::<State<usize>>));
    let renders = Rc::new(Cell::new(0));
    let parent_for_render = Rc::clone(&parent);
    let child_for_render = Rc::clone(&child);
    let renders_for_child = Rc::clone(&renders);
    let root = component(move |cx| {
        let parent_state = cx.use_state(|| 0usize);
        *parent_for_render.borrow_mut() = Some(parent_state.clone());
        let parent_value = parent_state.get().unwrap();
        let child_for_render = Rc::clone(&child_for_render);
        let renders_for_child = Rc::clone(&renders_for_child);
        memo_component(parent_value / 2, move |cx| {
            renders_for_child.set(renders_for_child.get() + 1);
            let child_state = cx.use_state(|| 0usize);
            *child_for_render.borrow_mut() = Some(child_state.clone());
            text_block(format!("{parent_value}:{}", child_state.get().unwrap()))
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(renders.get(), 1);

    assert!(parent.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(renders.get(), 1);

    assert!(parent.borrow().as_ref().unwrap().try_set(0));
    assert!(child.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(renders.get(), 2);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(text_update)
            .map(|(_, text)| text)
            .collect::<Vec<_>>(),
        ["0:1"]
    );

    assert!(parent.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(renders.get(), 3);
}

#[test]
#[should_panic(expected = "AttachedChildNativeRootCount")]
fn grid_child_rejects_multiple_projected_native_roots() {
    let mut reactor = Reactor::new(
        RecordingRuntime::default(),
        Grid::new([grid_child(fragment([
            text_block("first"),
            text_block("second"),
        ]))])
        .build(),
    );
    reactor.pump();
}

#[test]
fn canvas_placement_updates_clear_and_preserve_native_identity() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let child = canvas_child(TextBlock::new("cell").build().key(9));
        let child = match state.get().unwrap() {
            0 => child.left(10.5).top(-20.25).z_index(-99),
            1 => child.left(-30.75).top(40.5).z_index(1_000_000),
            4 | 5 => child.left(f64::NAN),
            _ => child,
        };
        Canvas::new([child]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::CanvasLeft(Some(10.5))),
            (target, AttachedUpdate::CanvasTop(Some(-20.25))),
            (target, AttachedUpdate::CanvasZIndex(Some(-99))),
        ]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::CanvasLeft(Some(-30.75))),
            (target, AttachedUpdate::CanvasTop(Some(40.5))),
            (target, AttachedUpdate::CanvasZIndex(Some(1_000_000))),
        ]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::CanvasLeft(None)),
            (target, AttachedUpdate::CanvasTop(None)),
            (target, AttachedUpdate::CanvasZIndex(None)),
        ]
    );

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(grid_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(grid_update)
            .count(),
        update_count
    );

    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(grid_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(5));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(grid_update)
            .count(),
        update_count
    );
}

#[test]
fn relative_panel_placement_preserves_tristate_values_and_edge_ordering() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        let element = if matches!(version, 2..=4) {
            Button::new("cell").on_click(|| {}).build()
        } else {
            TextBlock::new("cell").build()
        };
        let child = relative_panel_child(element.key(11));
        let child = match version {
            0 => child
                .align_left(true)
                .align_right(false)
                .align_top(true)
                .align_bottom(false)
                .align_horizontal_center(true)
                .align_vertical_center(false),
            1 | 2 => child
                .align_left(false)
                .align_right(true)
                .align_top(false)
                .align_bottom(true)
                .align_horizontal_center(false)
                .align_vertical_center(true),
            _ => child,
        };
        RelativePanel::new([child]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    let panel = native_node(&reactor, NativeKind::RelativePanel);
    let first = reactor.engine().runtime().batches().first().unwrap();
    assert_eq!(
        first
            .iter()
            .filter_map(grid_update)
            .filter(|(id, _)| *id == target)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::RelativeAlignLeft(Some(true))),
            (target, AttachedUpdate::RelativeAlignRight(Some(false))),
            (target, AttachedUpdate::RelativeAlignTop(Some(true))),
            (target, AttachedUpdate::RelativeAlignBottom(Some(false))),
            (
                target,
                AttachedUpdate::RelativeAlignHorizontalCenter(Some(true))
            ),
            (
                target,
                AttachedUpdate::RelativeAlignVerticalCenter(Some(false))
            ),
        ]
    );
    let update_position = first
        .iter()
        .position(|command| grid_update(command).is_some())
        .unwrap();
    let attach_position = first
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Attach {
                    parent,
                    child,
                    ..
                } if *parent == panel && *child == target
            )
        })
        .unwrap();
    assert!(update_position < attach_position);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::RelativeAlignLeft(Some(false))),
            (target, AttachedUpdate::RelativeAlignRight(Some(true))),
            (target, AttachedUpdate::RelativeAlignTop(Some(false))),
            (target, AttachedUpdate::RelativeAlignBottom(Some(true))),
            (
                target,
                AttachedUpdate::RelativeAlignHorizontalCenter(Some(false))
            ),
            (
                target,
                AttachedUpdate::RelativeAlignVerticalCenter(Some(true))
            ),
        ]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let target = native_node(&reactor, NativeKind::Button);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::RelativeAlignLeft(Some(false))),
            (target, AttachedUpdate::RelativeAlignRight(Some(true))),
            (target, AttachedUpdate::RelativeAlignTop(Some(false))),
            (target, AttachedUpdate::RelativeAlignBottom(Some(true))),
            (
                target,
                AttachedUpdate::RelativeAlignHorizontalCenter(Some(false))
            ),
            (
                target,
                AttachedUpdate::RelativeAlignVerticalCenter(Some(true))
            ),
        ]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(grid_update)
            .collect::<Vec<_>>(),
        [
            (target, AttachedUpdate::RelativeAlignLeft(None)),
            (target, AttachedUpdate::RelativeAlignRight(None)),
            (target, AttachedUpdate::RelativeAlignTop(None)),
            (target, AttachedUpdate::RelativeAlignBottom(None)),
            (target, AttachedUpdate::RelativeAlignHorizontalCenter(None)),
            (target, AttachedUpdate::RelativeAlignVerticalCenter(None)),
        ]
    );

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(grid_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(grid_update)
            .count(),
        update_count
    );
}

#[test]
fn text_block_flow_updates_clear_and_retain_native_identity() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let (wrapping, trimming, selection) = match state.get().unwrap() {
            0 => (
                Some(TextWrapping::Wrap),
                Some(TextTrimming::CharacterEllipsis),
                Some(true),
            ),
            1 => (
                Some(TextWrapping::WrapWholeWords),
                Some(TextTrimming::Clip),
                Some(false),
            ),
            _ => (None, None, None),
        };
        TextBlock::new("flow")
            .text_wrapping(wrapping)
            .text_trimming(trimming)
            .text_selection_enabled(selection)
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(text_wrapping_update)
            .collect::<Vec<_>>(),
        [(target, Some(TextWrapping::Wrap))]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(text_trimming_update)
            .collect::<Vec<_>>(),
        [(target, Some(TextTrimming::CharacterEllipsis))]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(text_selection_enabled_update)
            .collect::<Vec<_>>(),
        [(target, Some(true))]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(text_wrapping_update)
            .collect::<Vec<_>>(),
        [(target, Some(TextWrapping::WrapWholeWords))]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(text_trimming_update)
            .collect::<Vec<_>>(),
        [(target, Some(TextTrimming::Clip))]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(text_selection_enabled_update)
            .collect::<Vec<_>>(),
        [(target, Some(false))]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(text_wrapping_update)
            .collect::<Vec<_>>(),
        [(target, None)]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(text_selection_enabled_update)
            .collect::<Vec<_>>(),
        [(target, None)]
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(text_trimming_update)
            .collect::<Vec<_>>(),
        [(target, None)]
    );

    let batches = reactor.engine().runtime().batches().len();
    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().batches().len(), batches);
}

#[test]
fn enabled_targets_control_builders_and_clears_to_true() {
    let disabled = Rc::new(RefCell::new(None::<State<bool>>));
    let disabled_for_render = Rc::clone(&disabled);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *disabled_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            stack_panel([
                Button::new("button")
                    .on_click(|| {})
                    .enabled(false)
                    .visibility(Visibility::Visible)
                    .build(),
                CheckBox::new("check", false, |_| {})
                    .visibility(Visibility::Visible)
                    .enabled(false)
                    .build(),
                TextBox::new("input", |_| {})
                    .enabled(false)
                    .height(None)
                    .build(),
            ])
        } else {
            stack_panel([
                Button::new("button")
                    .on_click(|| {})
                    .visibility(Visibility::Visible)
                    .build(),
                CheckBox::new("check", false, |_| {})
                    .visibility(Visibility::Visible)
                    .build(),
                TextBox::new("input", |_| {}).height(None).build(),
            ])
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(enabled_update)
            .map(|(_, value)| value)
            .filter(|value| !value)
            .count(),
        3
    );

    assert!(disabled.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(enabled_update)
            .map(|(_, value)| value)
            .collect::<Vec<_>>(),
        [true, true, true]
    );
}

#[test]
fn accessibility_targets_dissimilar_builders() {
    let root = StackPanel::new([
        TextBlock::new("text")
            .automation_name("text name")
            .help_text("text help")
            .build(),
        Button::new("button")
            .on_click(|| {})
            .automation_name("button name")
            .help_text("button help")
            .build(),
        CheckBox::new("check", false, |_| {})
            .automation_name("check name")
            .help_text("check help")
            .build(),
        TextBox::new("input", |_| {})
            .automation_name("input name")
            .help_text("input help")
            .build(),
    ])
    .automation_name("panel name")
    .help_text("panel help")
    .build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    let updates = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(accessibility_update)
        .collect::<Vec<_>>();
    assert_eq!(updates.len(), 10);
    assert_eq!(
        updates
            .iter()
            .filter(|(_, update)| matches!(update, AccessibilityUpdate::AutomationName(_)))
            .count(),
        5
    );
    assert_eq!(
        updates
            .iter()
            .filter(|(_, update)| matches!(update, AccessibilityUpdate::HelpText(_)))
            .count(),
        5
    );
}

#[test]
fn accessibility_updates_and_clears_without_replacing_the_native_node() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        match state.get().unwrap() {
            0 => TextBlock::new("text")
                .automation_name("first name")
                .automation_id("first-id")
                .heading_level(AutomationHeadingLevel::Level1)
                .help_text("first help")
                .build(),
            1 => TextBlock::new("text")
                .automation_name("second name")
                .automation_id("second-id")
                .heading_level(AutomationHeadingLevel::Level2)
                .help_text("second help")
                .build(),
            _ => TextBlock::new("text").build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(accessibility_update)
            .map(|(_, update)| update)
            .collect::<Vec<_>>(),
        [
            AccessibilityUpdate::AutomationName("second name".to_string()),
            AccessibilityUpdate::AutomationId("second-id".to_string()),
            AccessibilityUpdate::HeadingLevel(Some(AutomationHeadingLevel::Level2)),
            AccessibilityUpdate::HelpText("second help".to_string()),
        ]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(accessibility_update)
            .map(|(_, update)| update)
            .collect::<Vec<_>>(),
        [
            AccessibilityUpdate::AutomationName(String::new()),
            AccessibilityUpdate::AutomationId(String::new()),
            AccessibilityUpdate::HeadingLevel(None),
            AccessibilityUpdate::HelpText(String::new()),
        ]
    );
}

#[test]
fn automation_id_independently_sets_changes_and_clears() {
    assert_independent_accessibility_updates(
        |revision| match revision {
            0 => TextBlock::new("text").build(),
            1 => TextBlock::new("text").automation_id("first-id").build(),
            2 => TextBlock::new("text").automation_id("second-id").build(),
            _ => TextBlock::new("text").build(),
        },
        [
            AccessibilityUpdate::AutomationId("first-id".to_string()),
            AccessibilityUpdate::AutomationId("second-id".to_string()),
            AccessibilityUpdate::AutomationId(String::new()),
        ],
    );
}

#[test]
fn automation_name_independently_sets_changes_and_clears() {
    assert_independent_accessibility_updates(
        |revision| match revision {
            0 => TextBlock::new("text").build(),
            1 => TextBlock::new("text").automation_name("first name").build(),
            2 => TextBlock::new("text")
                .automation_name("second name")
                .build(),
            _ => TextBlock::new("text").build(),
        },
        [
            AccessibilityUpdate::AutomationName("first name".to_string()),
            AccessibilityUpdate::AutomationName("second name".to_string()),
            AccessibilityUpdate::AutomationName(String::new()),
        ],
    );
}

#[test]
fn help_text_independently_sets_changes_and_clears() {
    assert_independent_accessibility_updates(
        |revision| match revision {
            0 => TextBlock::new("text").build(),
            1 => TextBlock::new("text").help_text("first help").build(),
            2 => TextBlock::new("text").help_text("second help").build(),
            _ => TextBlock::new("text").build(),
        },
        [
            AccessibilityUpdate::HelpText("first help".to_string()),
            AccessibilityUpdate::HelpText("second help".to_string()),
            AccessibilityUpdate::HelpText(String::new()),
        ],
    );
}

#[test]
fn heading_level_independently_sets_changes_and_clears() {
    assert_independent_accessibility_updates(
        |revision| match revision {
            0 => TextBlock::new("text").build(),
            1 => TextBlock::new("text")
                .heading_level(AutomationHeadingLevel::Level1)
                .build(),
            2 => TextBlock::new("text")
                .heading_level(AutomationHeadingLevel::Level2)
                .build(),
            _ => TextBlock::new("text").build(),
        },
        [
            AccessibilityUpdate::HeadingLevel(Some(AutomationHeadingLevel::Level1)),
            AccessibilityUpdate::HeadingLevel(Some(AutomationHeadingLevel::Level2)),
            AccessibilityUpdate::HeadingLevel(None),
        ],
    );
}

fn assert_independent_accessibility_updates(
    render: impl Fn(usize) -> Element + 'static,
    expected: [AccessibilityUpdate; 3],
) {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        render(state.get().unwrap())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);

    for (revision, expected) in (1..).zip(expected) {
        assert!(version.borrow().as_ref().unwrap().try_set(revision));
        reactor.pump();
        assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
        assert_eq!(
            reactor
                .engine()
                .runtime()
                .batches()
                .last()
                .unwrap()
                .iter()
                .filter_map(accessibility_update)
                .map(|(_, update)| update)
                .collect::<Vec<_>>(),
            [expected]
        );
    }
}

#[test]
fn virtual_list_accessibility_updates_and_clears_on_the_host() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let list = VirtualList::new(10, 300.0, |index| text_block(format!("row {index}")));
        match state.get().unwrap() {
            0 => list
                .automation_name("first list")
                .help_text("first help")
                .build(),
            1 => list
                .automation_name("second list")
                .help_text("second help")
                .build(),
            _ => list.build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::ListView);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ListView), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(accessibility_update)
            .map(|(_, update)| update)
            .collect::<Vec<_>>(),
        [
            AccessibilityUpdate::AutomationName("second list".to_string()),
            AccessibilityUpdate::HelpText("second help".to_string()),
        ]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ListView), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(accessibility_update)
            .map(|(_, update)| update)
            .collect::<Vec<_>>(),
        [
            AccessibilityUpdate::AutomationName(String::new()),
            AccessibilityUpdate::HelpText(String::new()),
        ]
    );
}

#[test]
fn framework_height_is_applied_to_a_replacement_target() {
    let replace = Rc::new(RefCell::new(None::<State<bool>>));
    let replace_for_render = Rc::clone(&replace);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *replace_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            TextBox::new("input", |_| {}).height(36.0).build()
        } else {
            TextBlock::new("text").height(36.0).build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let first = native_node(&reactor, NativeKind::TextBlock);

    assert!(replace.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    let second = native_node(&reactor, NativeKind::TextBox);

    assert_ne!(first, second);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| height_update(command) == Some((second, Dimension::Pixels(36.0))))
    );
}
