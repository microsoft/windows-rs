use super::super::*;
use crate::native::*;

#[derive(Clone, PartialEq)]
struct VisualInput {
    duplicate: bool,
    visuals: Option<WindowVisuals>,
}

struct VisualComponent;
struct EmptyVisualComponent;

impl Component for VisualComponent {
    type Message = ();
    type Input = VisualInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if let Some(visuals) = input.visuals {
            context.window_visuals(visuals);
            if input.duplicate {
                context.window_visuals(visuals);
            }
        }
        TextBlock::new().text("content").into()
    }
}

impl Component for EmptyVisualComponent {
    type Message = ();
    type Input = WindowVisuals;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_visuals(*input);
        View::empty()
    }
}

fn input(visuals: Option<WindowVisuals>) -> VisualInput {
    VisualInput {
        duplicate: false,
        visuals,
    }
}

fn command_count(runtime: &RecordingRuntime) -> usize {
    runtime
        .commands()
        .iter()
        .flatten()
        .filter(|command| matches!(command, Command::SetWindowVisuals { .. }))
        .count()
}

#[test]
fn initial_visuals_apply_before_activation() {
    let visuals = WindowVisuals::new()
        .theme(WindowTheme::Dark)
        .backdrop(WindowBackdrop::Mica)
        .icon("sample.ico")
        .client_size(1400.0, 900.0);
    let mut pump = Pump::new(RecordingRuntime::default());

    pump.mount_view(View::component::<VisualComponent>(input(Some(visuals))))
        .unwrap();
    let window = pump.window.unwrap();

    assert_eq!(pump.runtime().window_visuals(window), Some(visuals));
    let commands = &pump.runtime().commands()[0];
    let visual_command = commands
        .iter()
        .position(|command| matches!(command, Command::SetWindowVisuals { .. }))
        .unwrap();
    let activation = commands
        .iter()
        .position(|command| matches!(command, Command::ActivateWindow { .. }))
        .unwrap();
    assert!(visual_command < activation);
}

#[test]
fn empty_window_accepts_window_level_visuals() {
    let visuals = WindowVisuals::new()
        .backdrop(WindowBackdrop::MicaAlt)
        .client_size(800.0, 600.0);
    let mut pump = Pump::new(RecordingRuntime::default());

    pump.mount_view(View::component::<EmptyVisualComponent>(visuals))
        .unwrap();

    assert_eq!(
        pump.runtime().window_visuals(pump.window.unwrap()),
        Some(visuals)
    );
}

#[test]
fn visuals_update_clear_and_skip_unchanged_commands() {
    let first = WindowVisuals::new().backdrop(WindowBackdrop::Mica);
    let second = WindowVisuals::new()
        .theme(WindowTheme::Light)
        .backdrop(WindowBackdrop::Acrylic);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<VisualComponent>(input(Some(first))))
        .unwrap();
    let window = pump.window.unwrap();

    pump.update_view(View::component::<VisualComponent>(input(Some(second))))
        .unwrap();
    assert_eq!(pump.runtime().window_visuals(window), Some(second));
    let commands = command_count(pump.runtime());

    pump.update_view(View::component::<VisualComponent>(input(Some(second))))
        .unwrap();
    assert_eq!(command_count(pump.runtime()), commands);

    pump.update_view(View::component::<VisualComponent>(input(None)))
        .unwrap();
    assert_eq!(
        pump.runtime().window_visuals(window),
        Some(WindowVisuals::default())
    );
    assert!(pump.tree.window_visuals().is_none());
}

#[test]
fn constraints_clear_without_changing_client_size() {
    let constrained =
        WindowVisuals::new()
            .client_size(800.0, 600.0)
            .constraints(WindowConstraints {
                min_width: Some(400.0),
                ..Default::default()
            });
    let cleared = WindowVisuals::new().client_size(800.0, 600.0);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<VisualComponent>(input(Some(constrained))))
        .unwrap();
    let window = pump.window.unwrap();

    pump.update_view(View::component::<VisualComponent>(input(Some(cleared))))
        .unwrap();

    assert_eq!(pump.runtime().window_visuals(window), Some(cleared));
    assert_eq!(command_count(pump.runtime()), 2);
}

#[test]
fn duplicate_visual_declarations_are_rejected_transactionally() {
    let visuals = WindowVisuals::new().backdrop(WindowBackdrop::Mica);
    let mut duplicate = input(Some(visuals));
    duplicate.duplicate = true;
    let mut pump = Pump::new(RecordingRuntime::default());

    assert_eq!(
        pump.mount_view(View::component::<VisualComponent>(duplicate)),
        Err(PumpError::DuplicateWindowVisuals)
    );
    assert!(pump.runtime().is_empty());
}

#[test]
fn failed_native_apply_does_not_publish_visual_candidate() {
    let published = WindowVisuals::new().backdrop(WindowBackdrop::Mica);
    let candidate = WindowVisuals::new().backdrop(WindowBackdrop::Acrylic);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<VisualComponent>(input(Some(published))))
        .unwrap();
    let window = pump.window.unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update_view(View::component::<VisualComponent>(input(Some(candidate)))),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(pump.runtime().window_visuals(window), Some(published));
    assert_eq!(pump.tree.window_visuals().unwrap().visuals, published);
}
