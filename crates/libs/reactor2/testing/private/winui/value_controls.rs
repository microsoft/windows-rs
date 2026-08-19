use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::*;
use crate::winui::controlled::tests as controlled_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_VALUE_CONTROLS_FIXTURE";

#[derive(Clone)]
struct EventCounts {
    slider: Rc<Cell<usize>>,
    number: Rc<Cell<usize>>,
    rating: Rc<Cell<usize>>,
    color: Rc<Cell<usize>>,
    date: Rc<Cell<usize>>,
}

impl EventCounts {
    fn new() -> Self {
        Self {
            slider: Rc::new(Cell::new(0)),
            number: Rc::new(Cell::new(0)),
            rating: Rc::new(Cell::new(0)),
            color: Rc::new(Cell::new(0)),
            date: Rc::new(Cell::new(0)),
        }
    }

    fn assert_zero(&self) {
        assert_eq!(self.slider.get(), 0);
        assert_eq!(self.number.get(), 0);
        assert_eq!(self.rating.get(), 0);
        assert_eq!(self.color.get(), 0);
        assert_eq!(self.date.get(), 0);
    }
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn value_controls_update_without_callbacks_and_reset() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::value_controls::value_controls_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn value_controls_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let events = EventCounts::new();
    let events_for_render = events.clone();
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let close = open.clone();
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "Value controls fixture",
                    content(phase.try_value().unwrap(), &events_for_render),
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
        assert_phase(reactor.engine().runtime(), 0);
        events.assert_zero();
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 1);
        events.assert_zero();
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 2);
        events.assert_zero();
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 1);
        events.assert_zero();
        assert_native_events(reactor, &events);
        assert_eq!(events.slider.get(), 1);
        assert_eq!(events.number.get(), 1);
        assert_eq!(events.rating.get(), 0);
        assert_eq!(events.color.get(), 1);
        assert_eq!(events.date.get(), 0);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn count_callback<T>(count: &Rc<Cell<usize>>) -> impl Fn(T) + 'static {
    let count = Rc::clone(count);
    move |_| count.set(count.get() + 1)
}

fn content(phase: usize, events: &EventCounts) -> Element {
    let (slider, number, rating, color, date) = match phase {
        0 => (
            Slider::new(25.0, count_callback(&events.slider))
                .range(0.0, 100.0)
                .orientation(Orientation::Horizontal)
                .build(),
            NumberBox::new(Some(25.0), count_callback(&events.number))
                .range(0.0, 100.0)
                .build(),
            RatingControl::new(Some(3.0), count_callback(&events.rating))
                .max_rating(5)
                .placeholder(4.0)
                .caption("Initial")
                .read_only(false)
                .build(),
            ColorPicker::new(Color::rgb(10, 20, 30), count_callback(&events.color))
                .alpha_enabled(true)
                .hex_input_visible(true)
                .color_slider_visible(true)
                .color_channel_text_input_visible(true)
                .build(),
            DatePicker::new(
                Some(DateTime::from_unix_secs(1_705_276_800)),
                count_callback(&events.date),
            )
            .day_visible(true)
            .month_visible(true)
            .year_visible(true)
            .build(),
        ),
        1 => (
            Slider::new(225.0, count_callback(&events.slider))
                .range(200.0, 300.0)
                .orientation(Orientation::Vertical)
                .build(),
            NumberBox::new(None, count_callback(&events.number))
                .range(200.0, 300.0)
                .build(),
            RatingControl::new(None, count_callback(&events.rating))
                .max_rating(10)
                .placeholder(7.5)
                .caption("Updated")
                .read_only(true)
                .build(),
            ColorPicker::new(Color::argb(128, 40, 50, 60), count_callback(&events.color))
                .alpha_enabled(false)
                .hex_input_visible(false)
                .color_slider_visible(false)
                .color_channel_text_input_visible(false)
                .build(),
            DatePicker::new(None, count_callback(&events.date))
                .day_visible(false)
                .month_visible(false)
                .year_visible(false)
                .build(),
        ),
        _ => (
            Slider::new(0.0, count_callback(&events.slider)).build(),
            NumberBox::new(None, count_callback(&events.number)).build(),
            RatingControl::new(None, count_callback(&events.rating)).build(),
            ColorPicker::new(Color::default(), count_callback(&events.color)).build(),
            DatePicker::new(None, count_callback(&events.date)).build(),
        ),
    };
    StackPanel::new([slider, number, rating, color, date]).build()
}

fn only_node(runtime: &WinUiRuntime, kind: NativeKind) -> NodeId {
    let nodes = RuntimeProbe::new(runtime).nodes(kind);
    assert_eq!(nodes.len(), 1, "expected one {kind:?} node");
    nodes[0]
}

fn assert_phase(runtime: &WinUiRuntime, phase: usize) {
    let slider = only_node(runtime, NativeKind::Slider);
    let number = only_node(runtime, NativeKind::NumberBox);
    let rating = only_node(runtime, NativeKind::RatingControl);
    let color = only_node(runtime, NativeKind::ColorPicker);
    let date = only_node(runtime, NativeKind::DatePicker);

    assert_eq!(
        controlled_probe::slider(runtime, slider).unwrap(),
        match phase {
            0 => (25.0, 0.0, 100.0, Orientation::Horizontal),
            1 => (225.0, 200.0, 300.0, Orientation::Vertical),
            _ => (0.0, 0.0, 100.0, Orientation::Horizontal),
        }
    );
    assert_eq!(
        controlled_probe::number_box(runtime, number).unwrap(),
        match phase {
            0 => (Some(25.0), 0.0, 100.0),
            1 => (None, 200.0, 300.0),
            _ => (None, f64::MIN, f64::MAX),
        }
    );
    assert_eq!(
        controlled_probe::rating_control(runtime, rating).unwrap(),
        match phase {
            0 => (Some(3.0), 5, Some(4.0), "Initial".to_string(), false),
            1 => (None, 10, Some(7.5), "Updated".to_string(), true),
            _ => (None, 5, None, String::new(), false),
        }
    );
    assert_eq!(
        controlled_probe::color_picker(runtime, color).unwrap(),
        match phase {
            0 => (Color::rgb(10, 20, 30), true, true, true, true),
            1 => (Color::argb(128, 40, 50, 60), false, false, false, false),
            _ => (Color::default(), true, true, true, true),
        }
    );
    assert_eq!(
        controlled_probe::date_picker(runtime, date).unwrap(),
        match phase {
            0 => (
                Some(DateTime::from_unix_secs(1_705_276_800)),
                true,
                true,
                true,
            ),
            1 => (None, false, false, false),
            _ => (None, true, true, true),
        }
    );
}

fn assert_native_events(reactor: &mut Reactor<WinUiRuntime>, events: &EventCounts) {
    let runtime = reactor.engine().runtime();
    let slider = only_node(runtime, NativeKind::Slider);
    let number = only_node(runtime, NativeKind::NumberBox);
    let rating = only_node(runtime, NativeKind::RatingControl);
    let color = only_node(runtime, NativeKind::ColorPicker);
    let date = only_node(runtime, NativeKind::DatePicker);
    let native_date = DateTime::from_unix_secs(1_706_054_400);

    controlled_probe::set_slider(runtime, slider, 250.0).unwrap();
    controlled_probe::set_number_box(runtime, number, Some(250.0)).unwrap();
    controlled_probe::set_rating_read_only(runtime, rating, false).unwrap();
    controlled_probe::set_rating_control(runtime, rating, Some(8.0)).unwrap();
    controlled_probe::set_color_picker(runtime, color, Color::rgb(70, 80, 90)).unwrap();
    controlled_probe::set_date_picker(runtime, date, Some(native_date)).unwrap();
    reactor.pump();

    assert_eq!(events.slider.get(), 1);
    assert_eq!(events.number.get(), 1);
    assert_eq!(events.rating.get(), 0);
    assert_eq!(events.color.get(), 1);
    assert_eq!(events.date.get(), 0);
    let runtime = reactor.engine().runtime();
    assert_eq!(controlled_probe::slider(runtime, slider).unwrap().0, 250.0);
    assert_eq!(
        controlled_probe::number_box(runtime, number).unwrap().0,
        Some(250.0)
    );
    assert_eq!(
        controlled_probe::rating_control(runtime, rating).unwrap().0,
        Some(8.0)
    );
    assert_eq!(
        controlled_probe::color_picker(runtime, color).unwrap().0,
        Color::rgb(70, 80, 90)
    );
    assert_eq!(
        controlled_probe::date_picker(runtime, date).unwrap().0,
        Some(native_date)
    );
}
