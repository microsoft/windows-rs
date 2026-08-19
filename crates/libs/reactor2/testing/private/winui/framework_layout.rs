use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::framework_access as framework_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_FRAMEWORK_LAYOUT_FIXTURE";

#[derive(Clone, Copy)]
enum Case {
    Framework,
    StackPanel,
    Grid,
    Attached,
}

fn run_case(case: &str) {
    let output = test_reactor_support::run_test_process(
        "winui::tests::framework_layout::framework_layout_fixture",
        &[(FIXTURE_ENV, case)],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn framework_properties_update_and_reset() {
    run_case("framework");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn stack_panel_layout_updates_and_resets() {
    run_case("stack-panel");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn grid_definitions_and_placement_update_and_reset() {
    run_case("grid");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn canvas_and_relative_panel_placement_update_and_reset() {
    run_case("attached");
}

#[test]
fn framework_layout_fixture() {
    let Some(case) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };
    let case = match case.to_str().unwrap() {
        "framework" => Case::Framework,
        "stack-panel" => Case::StackPanel,
        "grid" => Case::Grid,
        "attached" => Case::Attached,
        case => panic!("unknown framework layout fixture: {case}"),
    };

    bootstrap().unwrap();
    run_fixture(case);
}

fn run_fixture(case: Case) {
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let close = open.clone();
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "Framework layout fixture",
                    content(case, phase.try_value().unwrap()),
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
        let mut grid = None;
        assert_phase(reactor.engine().runtime(), case, 0, &mut grid);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), case, 1, &mut grid);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), case, 2, &mut grid);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn content(case: Case, phase: usize) -> Element {
    match case {
        Case::Framework => match phase {
            0 => TextBlock::new("framework")
                .width(120.0)
                .height(24.0)
                .min_width(50.0)
                .max_width(400.0)
                .min_height(20.0)
                .max_height(200.0)
                .margin(Thickness::xy(8.0, 4.0))
                .horizontal_alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Top)
                .visibility(Visibility::Visible)
                .opacity(0.8)
                .build(),
            1 => TextBlock::new("framework")
                .width(160.0)
                .height(48.0)
                .min_width(60.0)
                .max_width(500.0)
                .min_height(30.0)
                .max_height(240.0)
                .margin(Thickness::uniform(12.0))
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Bottom)
                .visibility(Visibility::Collapsed)
                .opacity(0.4)
                .build(),
            _ => TextBlock::new("framework").build(),
        },
        Case::StackPanel => {
            let panel = StackPanel::new([Button::new("child").on_click(|| {}).build()]);
            match phase {
                0 => panel
                    .orientation(Orientation::Horizontal)
                    .spacing(12.0)
                    .padding(Thickness::xy(8.0, 4.0))
                    .build(),
                1 => panel
                    .orientation(Orientation::Vertical)
                    .spacing(6.0)
                    .padding(Thickness::uniform(10.0))
                    .build(),
                _ => panel.build(),
            }
        }
        Case::Grid => {
            let child = grid_child(TextBlock::new("grid child").build());
            let (child, columns, rows) = match phase {
                0 => (
                    child.row(2).column(1).row_span(3).column_span(2),
                    vec![GridLength::Pixel(40.0), GridLength::STAR],
                    vec![GridLength::Auto, GridLength::Star(2.0)],
                ),
                1 => (
                    child.row(1).column(0).row_span(2).column_span(3),
                    vec![GridLength::Star(3.0), GridLength::Auto],
                    vec![GridLength::Pixel(24.0), GridLength::STAR],
                ),
                _ => (child, Vec::new(), Vec::new()),
            };
            Grid::new([child]).columns(columns).rows(rows).build()
        }
        Case::Attached => {
            let canvas_child = canvas_child(Button::new("canvas child").on_click(|| {}).build());
            let relative_child =
                relative_panel_child(CheckBox::new("relative child", false, |_| {}).build());
            let (canvas_child, relative_child) = match phase {
                0 => (
                    canvas_child.left(12.5).top(-8.0).z_index(77),
                    relative_child
                        .align_left(true)
                        .align_top(true)
                        .align_horizontal_center(true),
                ),
                1 => (
                    canvas_child.left(-30.75).top(40.5).z_index(1_000_000),
                    relative_child
                        .align_right(true)
                        .align_bottom(true)
                        .align_vertical_center(true),
                ),
                _ => (canvas_child, relative_child),
            };
            StackPanel::new([
                Canvas::new([canvas_child]).build(),
                RelativePanel::new([relative_child]).build(),
            ])
            .build()
        }
    }
}

fn only_node(runtime: &WinUiRuntime, kind: NativeKind) -> NodeId {
    let nodes = RuntimeProbe::new(runtime).nodes(kind);
    assert_eq!(nodes.len(), 1, "expected one {kind:?} node");
    nodes[0]
}

fn assert_phase(runtime: &WinUiRuntime, case: Case, phase: usize, grid: &mut Option<NodeId>) {
    match case {
        Case::Framework => assert_framework(runtime, phase),
        Case::StackPanel => assert_stack_panel(runtime, phase),
        Case::Grid => assert_grid(runtime, phase, grid),
        Case::Attached => assert_attached(runtime, phase),
    }
}

fn assert_framework(runtime: &WinUiRuntime, phase: usize) {
    let id = only_node(runtime, NativeKind::TextBlock);
    let actual = (
        framework_probe::width(runtime, id).unwrap(),
        framework_probe::height(runtime, id).unwrap(),
        framework_probe::min_width(runtime, id).unwrap(),
        framework_probe::max_width(runtime, id).unwrap(),
        framework_probe::min_height(runtime, id).unwrap(),
        framework_probe::max_height(runtime, id).unwrap(),
        framework_probe::margin(runtime, id).unwrap(),
        framework_probe::alignment(runtime, id).unwrap(),
        framework_probe::visibility(runtime, id).unwrap(),
        framework_probe::opacity(runtime, id).unwrap(),
    );
    match phase {
        0 => assert_eq!(
            actual,
            (
                120.0,
                24.0,
                50.0,
                400.0,
                20.0,
                200.0,
                Thickness::xy(8.0, 4.0),
                (HorizontalAlignment::Left, VerticalAlignment::Top),
                Visibility::Visible,
                0.8_f32 as f64,
            )
        ),
        1 => assert_eq!(
            actual,
            (
                160.0,
                48.0,
                60.0,
                500.0,
                30.0,
                240.0,
                Thickness::uniform(12.0),
                (HorizontalAlignment::Center, VerticalAlignment::Bottom),
                Visibility::Collapsed,
                0.4_f32 as f64,
            )
        ),
        _ => {
            assert!(actual.0.is_nan());
            assert!(actual.1.is_nan());
            assert_eq!(actual.2, 0.0);
            assert!(actual.3.is_infinite());
            assert_eq!(actual.4, 0.0);
            assert!(actual.5.is_infinite());
            assert_eq!(actual.6, Thickness::default());
            assert_eq!(
                actual.7,
                (HorizontalAlignment::Stretch, VerticalAlignment::Stretch)
            );
            assert_eq!(actual.8, Visibility::Visible);
            assert_eq!(actual.9, 1.0);
        }
    }
}

fn assert_stack_panel(runtime: &WinUiRuntime, phase: usize) {
    let id = only_node(runtime, NativeKind::StackPanel);
    let actual = (
        framework_probe::stack_layout(runtime, id).unwrap(),
        framework_probe::padding(runtime, id).unwrap(),
    );
    assert_eq!(
        actual,
        match phase {
            0 => ((Orientation::Horizontal, 12.0), Thickness::xy(8.0, 4.0),),
            1 => ((Orientation::Vertical, 6.0), Thickness::uniform(10.0),),
            _ => ((Orientation::Vertical, 0.0), Thickness::default()),
        }
    );
}

fn assert_grid(runtime: &WinUiRuntime, phase: usize, grid: &mut Option<NodeId>) {
    let grid = *grid.get_or_insert_with(|| {
        RuntimeProbe::new(runtime)
            .nodes(NativeKind::Grid)
            .into_iter()
            .find(|id| {
                let (columns, _) = framework_probe::grid_definitions(runtime, *id).unwrap();
                columns == [GridLength::Pixel(40.0), GridLength::STAR]
            })
            .unwrap()
    });
    let child = only_node(runtime, NativeKind::TextBlock);
    assert_eq!(
        framework_probe::grid_definitions(runtime, grid).unwrap(),
        match phase {
            0 => (
                vec![GridLength::Pixel(40.0), GridLength::STAR],
                vec![GridLength::Auto, GridLength::Star(2.0)],
            ),
            1 => (
                vec![GridLength::Star(3.0), GridLength::Auto],
                vec![GridLength::Pixel(24.0), GridLength::STAR],
            ),
            _ => (Vec::new(), Vec::new()),
        }
    );
    assert_eq!(
        framework_probe::grid_placement(runtime, child).unwrap(),
        match phase {
            0 => (2, 1, 3, 2),
            1 => (1, 0, 2, 3),
            _ => (0, 0, 1, 1),
        }
    );
}

fn assert_attached(runtime: &WinUiRuntime, phase: usize) {
    let canvas_child = only_node(runtime, NativeKind::Button);
    let relative_child = only_node(runtime, NativeKind::CheckBox);
    assert_eq!(
        framework_probe::canvas_placement(runtime, canvas_child).unwrap(),
        match phase {
            0 => (12.5, -8.0, 77),
            1 => (-30.75, 40.5, 1_000_000),
            _ => (0.0, 0.0, 0),
        }
    );
    assert_eq!(
        framework_probe::relative_placement(runtime, relative_child).unwrap(),
        match phase {
            0 => (true, false, true, false, true, false),
            1 => (false, true, false, true, false, true),
            _ => (false, false, false, false, false, false),
        }
    );
}
