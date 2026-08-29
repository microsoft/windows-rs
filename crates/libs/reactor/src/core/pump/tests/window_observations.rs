use super::super::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
struct ObservationInput {
    color_scheme: Rc<Cell<ColorScheme>>,
    observe: bool,
    size: Rc<Cell<WindowSize>>,
}

impl PartialEq for ObservationInput {
    fn eq(&self, other: &Self) -> bool {
        self.observe == other.observe
            && Rc::ptr_eq(&self.color_scheme, &other.color_scheme)
            && Rc::ptr_eq(&self.size, &other.size)
    }
}

enum ObservationMessage {
    ColorScheme(ColorScheme),
    WindowSize(WindowSize),
}

struct ObservationComponent {
    color_scheme: Rc<Cell<ColorScheme>>,
    size: Rc<Cell<WindowSize>>,
}

impl Component for ObservationComponent {
    type Message = ObservationMessage;
    type Input = ObservationInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            color_scheme: Rc::clone(&input.color_scheme),
            size: Rc::clone(&input.size),
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            ObservationMessage::ColorScheme(value) => self.color_scheme.set(value),
            ObservationMessage::WindowSize(value) => self.size.set(value),
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        if input.observe {
            let color_scheme = context.callback(ObservationMessage::ColorScheme);
            let window_size = context.callback(ObservationMessage::WindowSize);
            context.on_color_scheme(color_scheme);
            context.on_window_size(window_size);
        }
        TextBlock::new().text("host").into()
    }
}

fn observations(pump: &Pump<RecordingRuntime>) -> WindowObservationFlags {
    pump.runtime()
        .commands()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::SetWindowObservations { observations, .. } => Some(*observations),
            _ => None,
        })
        .unwrap()
}

#[test]
fn host_observations_dispatch_without_grid_events() {
    let color_scheme = Rc::new(Cell::new(ColorScheme::Light));
    let size = Rc::new(Cell::new(WindowSize::default()));
    let input = ObservationInput {
        color_scheme: Rc::clone(&color_scheme),
        observe: true,
        size: Rc::clone(&size),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ObservationComponent>(input.clone()))
        .unwrap();

    assert!(pump.runtime().commands()[0].iter().any(|command| matches!(
        command,
        Command::SetWindowObservations {
            observations: WindowObservationFlags {
                window_size: Some(_),
                color_scheme: Some(_),
            },
            ..
        }
    )));

    let observations = observations(&pump);
    pump.runtime_mut().queue_host_event(HostEvent::WindowSize {
        observation: observations.window_size.unwrap(),
        size: WindowSize {
            width: 640.0,
            height: 480.0,
        },
    });
    pump.runtime_mut().queue_host_event(HostEvent::ColorScheme {
        observation: observations.color_scheme.unwrap(),
        scheme: ColorScheme::Dark,
    });
    assert_eq!(pump.dispatch_events(), Ok(2));
    assert_eq!(pump.dispatch_components(2), Ok(2));
    assert_eq!(
        size.get(),
        WindowSize {
            width: 640.0,
            height: 480.0,
        }
    );
    assert_eq!(color_scheme.get(), ColorScheme::Dark);

    pump.update_view(View::component::<ObservationComponent>(ObservationInput {
        observe: false,
        ..input
    }))
    .unwrap();
    assert!(
        pump.runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| {
                matches!(
                    command,
                    Command::SetWindowObservations {
                        observations: WindowObservationFlags {
                            window_size: None,
                            color_scheme: None,
                        },
                        ..
                    }
                )
            })
    );
}

#[test]
fn stale_host_observation_is_rejected_after_resubscription() {
    let color_scheme = Rc::new(Cell::new(ColorScheme::Light));
    let size = Rc::new(Cell::new(WindowSize::default()));
    let input = ObservationInput {
        color_scheme,
        observe: true,
        size: Rc::clone(&size),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ObservationComponent>(input.clone()))
        .unwrap();
    let stale = observations(&pump).window_size.unwrap();

    pump.update_view(View::component::<ObservationComponent>(ObservationInput {
        observe: false,
        ..input.clone()
    }))
    .unwrap();
    pump.update_view(View::component::<ObservationComponent>(input))
        .unwrap();
    let current = observations(&pump).window_size.unwrap();
    assert_ne!(stale, current);

    pump.runtime_mut()
        .queue_host_event(HostEvent::ObservationError {
            observation: stale,
            error: RuntimeError::UnsupportedKind,
        });
    pump.runtime_mut().queue_host_event(HostEvent::WindowSize {
        observation: stale,
        size: WindowSize {
            width: 320.0,
            height: 200.0,
        },
    });
    pump.runtime_mut().queue_host_event(HostEvent::WindowSize {
        observation: current,
        size: WindowSize {
            width: 800.0,
            height: 600.0,
        },
    });
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(
        size.get(),
        WindowSize {
            width: 800.0,
            height: 600.0,
        }
    );
}

struct DuplicateObservation;

impl Component for DuplicateObservation {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let first = context.callback(|_| ());
        let second = context.callback(|_| ());
        context.on_window_size(first);
        context.on_window_size(second);
        View::empty()
    }
}

#[test]
fn duplicate_host_observation_is_rejected() {
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(View::component::<DuplicateObservation>(())),
        Err(PumpError::DuplicateWindowSizeObservation)
    );
}

struct ObservationChild;

impl Component for ObservationChild {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let callback = context.callback(|_| ());
        context.on_window_size(callback);
        TextBlock::new().into()
    }
}

struct ObservationParent;

impl Component for ObservationParent {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().children((
            View::component::<ObservationChild>(()),
            View::component::<ObservationChild>(()),
        ))
    }
}

#[test]
fn multiple_component_observers_are_rejected_transactionally() {
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(View::component::<ObservationParent>(())),
        Err(PumpError::DuplicateWindowSizeObservation)
    );
    assert!(pump.runtime().is_empty());
}
