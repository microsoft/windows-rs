use super::*;
use state::{BAR, HEADER, ROW};
use std::cell::Cell;

#[derive(Clone, PartialEq)]
struct TestRows {
    keys: Rc<Vec<usize>>,
    values: u32,
    reads: Rc<Cell<usize>>,
}

impl Rows for TestRows {
    fn len(&self) -> usize {
        self.keys.len()
    }
    fn key(&self, index: usize) -> Key {
        self.keys[index].into()
    }
    fn cell(&self, index: usize, column: usize) -> String {
        self.reads.set(self.reads.get() + 1);
        format!("{}:{column}:{}", self.keys[index], self.values)
    }
}

fn input(count: usize) -> Input<TestRows> {
    Input {
        automation_id: Rc::from("details"),
        name: Rc::from("Details"),
        rows: TestRows {
            keys: Rc::new((0..count).collect()),
            values: 0,
            reads: Rc::new(Cell::new(0)),
        },
        order_revision: 0,
        columns: Rc::from([
            Column {
                title: "Name",
                width: 150.0,
                right_aligned: false,
            },
            Column {
                title: "Value",
                width: 150.0,
                right_aligned: true,
            },
        ]),
        selected: None,
        sort: None,
        scheme: ColorScheme::Light,
        on_select: Callback::new(|_| {}),
        on_sort: Callback::new(|_| {}),
    }
}

#[test]
fn identity_navigation_survives_reorder_removal_and_empty() {
    let mut state = State::new(input(100));
    state.resize(314.0, HEADER + BAR + ROW * 4.0);
    state.key(VirtualKey::END);
    assert_eq!(state.input.selected, Some(99usize.into()));
    assert!(state.visible().contains(&99));
    let mut next = state.input.clone();
    next.rows.keys = Rc::new((0..100).rev().collect());
    next.order_revision += 1;
    state.set_input(next);
    assert_eq!(state.input.selected, Some(99usize.into()));
    state.key(VirtualKey::DOWN);
    assert_eq!(state.input.selected, Some(98usize.into()));
    assert!(state.visible().contains(&1));
    let mut next = state.input.clone();
    next.rows.keys = Rc::new(vec![100, 101]);
    next.order_revision += 1;
    next.selected = None;
    state.set_input(next);
    state.key(VirtualKey::DOWN);
    assert_eq!(state.input.selected, Some(100usize.into()));
    let mut next = state.input.clone();
    next.rows.keys = Rc::new(Vec::new());
    next.order_revision += 1;
    next.selected = None;
    state.set_input(next);
    assert!(state.key(VirtualKey::END).is_none());
    assert!(state.visible().is_empty());
    assert_eq!(state.scroll[1], 0.0);
}

#[test]
fn fractional_wheel_and_capture_loss_preserve_viewport_contract() {
    let mut state = State::new(input(100));
    state.resize(200.0, 200.0);
    state.wheel(
        PointerEventInfo {
            wheel_delta: -30,
            ..Default::default()
        },
        f64::from(ROW * 3.0),
    );
    assert_eq!(state.scroll[1], f64::from(ROW) * 0.75);
    let pointer = PointerEventInfo {
        x: 148.0,
        y: 12.0,
        is_left_button_pressed: true,
        is_captured: true,
        ..Default::default()
    };
    let mut frame = state::Frame::default();
    frame.record(&state);
    assert!(state.press(pointer, &frame).is_none());
    state.move_pointer(
        PointerEventInfo {
            x: 198.0,
            ..pointer
        },
        &frame,
    );
    assert_eq!(state.widths[0], 200.0);
    state.cancel(pointer.pointer_id);
    state.move_pointer(
        PointerEventInfo {
            x: 298.0,
            ..pointer
        },
        &frame,
    );
    assert_eq!(state.widths[0], 200.0);
    state.scroll = [1_000_000.0; 2];
    state.resize(200.0, 200.0);
    assert_eq!(state.scroll, [state.maximum(0), state.maximum(1)]);
}

#[test]
fn column_dividers_resize_from_both_sides_after_horizontal_scroll() {
    for scroll in [0.0, 40.0] {
        for distance in [-4.0, 0.0, 4.0] {
            let mut state = State::new(input(100));
            state.resize(200.0, 200.0);
            state.scroll[0] = scroll;
            let pointer = PointerEventInfo {
                pointer_id: 7,
                x: 150.0 - scroll + distance,
                y: 12.0,
                is_left_button_pressed: true,
                is_captured: true,
                ..Default::default()
            };
            let mut frame = state::Frame::default();
            frame.record(&state);
            assert!(
                state.press(pointer, &frame).is_none(),
                "divider must not sort"
            );
            state.move_pointer(
                PointerEventInfo {
                    x: pointer.x + 60.0,
                    ..pointer
                },
                &frame,
            );
            assert_eq!(state.widths, [210.0, 150.0]);
            let mut next = state.input.clone();
            next.rows.values += 1;
            state.set_input(next);
            state.resize(220.0, 220.0);
            assert_eq!(state.widths, [210.0, 150.0]);
        }
    }
}

#[test]
fn scrollbar_gutters_share_the_body_background() {
    let device = windows_canvas::GpuDevice::new_warp().unwrap();
    let target = device.create_render_target(314, 150).unwrap();
    for scheme in [ColorScheme::Light, ColorScheme::Dark] {
        let mut input = input(1);
        input.scheme = scheme;
        let mut state = State::new(input);
        state.resize(314.0, 150.0);
        let mut renderer = Renderer::new();
        target
            .draw(|session| renderer.draw(session, &state, 1.0, true))
            .unwrap();
        let pixels = target.read_pixels().unwrap();
        let pixel = |x: usize, y: usize| &pixels[(y * 314 + x) * 4..(y * 314 + x + 1) * 4];
        assert_eq!(pixel(10, 100), pixel(305, 100));
        assert_eq!(pixel(10, 100), pixel(10, 145));
        assert_eq!(pixel(10, 100), pixel(305, 145));
    }
}

#[test]
fn only_the_initiating_pointer_can_move_or_end_a_drag() {
    for (x, y) in [(150.0, 12.0), (193.0, f64::from(HEADER + 5.0))] {
        let mut state = State::new(input(100));
        state.resize(200.0, 200.0);
        let pointer = PointerEventInfo {
            pointer_id: 7,
            x,
            y,
            is_left_button_pressed: true,
            is_captured: true,
            ..Default::default()
        };
        let mut frame = state::Frame::default();
        frame.record(&state);
        state.press(pointer, &frame);
        let other = PointerEventInfo {
            pointer_id: 8,
            x: x + 30.0,
            y: y + 30.0,
            ..pointer
        };
        assert!(state.press(other, &frame).is_none());
        state.move_pointer(other, &frame);
        state.cancel(other.pointer_id);
        assert_eq!(state.widths, [150.0, 150.0]);
        assert_eq!(state.scroll, [0.0; 2]);
        state.move_pointer(
            PointerEventInfo {
                x: other.x,
                y: other.y,
                ..pointer
            },
            &frame,
        );
        assert!(state.widths[0] > 150.0 || state.scroll[1] > 0.0);
        let widths = state.widths.clone();
        let scroll = state.scroll;
        state.cancel(pointer.pointer_id);
        state.move_pointer(
            PointerEventInfo {
                x: x + 60.0,
                y: y + 60.0,
                ..pointer
            },
            &frame,
        );
        assert_eq!(state.widths, widths);
        assert_eq!(state.scroll, scroll);
    }
}

#[test]
fn row_hits_keep_submitted_identity_across_model_and_frame_changes() {
    let mut state = State::new(input(3));
    state.resize(314.0, 200.0);
    let mut frames = Frames::default();
    let pointer = PointerEventInfo {
        x: 10.0,
        y: f64::from(HEADER + 10.0),
        is_left_button_pressed: true,
        ..Default::default()
    };
    frames.pending.record(&state);
    assert!(state.press(pointer, &frames.current).is_none());
    frames.present();
    let mut next = state.input.clone();
    next.rows.keys = Rc::new(vec![2, 1, 0]);
    next.order_revision += 1;
    state.set_input(next);
    frames.pending.record(&state);

    let action = state.press(pointer, &frames.current).unwrap();
    assert!(matches!(&action, Action::Select(key) if *key == Key::from(0usize)));
    frames.present();
    assert!(
        matches!(state.apply_action(action.clone()), Some(Action::Select(key)) if key == Key::from(0usize))
    );

    let mut next = state.input.clone();
    next.rows.keys = Rc::new(vec![2, 1]);
    next.order_revision += 1;
    next.selected = None;
    state.set_input(next);
    assert!(state.apply_action(action).is_none());
    assert!(state.input.selected.is_none());
}

#[test]
fn row_hits_use_submitted_scroll_and_reject_retired_columns() {
    let mut state = State::new(input(100));
    state.resize(314.0, 200.0);
    let mut frame = state::Frame::default();
    frame.record(&state);
    state.key(VirtualKey::END);
    let action = state
        .press(
            PointerEventInfo {
                x: 10.0,
                y: f64::from(HEADER + 10.0),
                is_left_button_pressed: true,
                ..Default::default()
            },
            &frame,
        )
        .unwrap();
    assert!(matches!(action, Action::Select(key) if key == Key::from(0usize)));
    let header = PointerEventInfo {
        x: 10.0,
        y: 10.0,
        is_left_button_pressed: true,
        ..Default::default()
    };
    let sort = state.press(header, &frame).unwrap();
    let mut next = state.input.clone();
    next.columns = Rc::from([Column {
        title: "Other",
        width: 100.0,
        right_aligned: false,
    }]);
    state.set_input(next);
    assert!(state.apply_action(sort).is_none());
    assert!(state.press(header, &frame).is_none());
}

#[test]
fn rendering_formats_only_visible_cells_and_reuses_unchanged_layouts() {
    let input = input(100_000);
    let reads = Rc::clone(&input.rows.reads);
    let mut state = State::new(input);
    state.resize(314.0, HEADER + BAR + ROW * 4.0);
    let device = windows_canvas::GpuDevice::new_warp().unwrap();
    let target = device
        .create_render_target(314, state.size[1] as u32)
        .unwrap();
    let mut renderer = Renderer::new();
    target
        .draw(|session| renderer.draw(session, &state, 1.0, true))
        .unwrap();
    assert_eq!(reads.get(), 8);
    target
        .draw(|session| renderer.draw(session, &state, 1.0, false))
        .unwrap();
    assert_eq!(reads.get(), 8);
    state.key(VirtualKey::END);
    target
        .draw(|session| renderer.draw(session, &state, 1.0, false))
        .unwrap();
    assert_eq!(reads.get(), 16);
    assert_eq!(renderer.cached_cells(), 10);
    let mut next = state.input.clone();
    next.rows.values = 1;
    state.set_input(next);
    target
        .draw(|session| renderer.draw(session, &state, 1.0, false))
        .unwrap();
    assert_eq!(reads.get(), 24);
    assert_eq!(renderer.cached_cells(), 10);
}
