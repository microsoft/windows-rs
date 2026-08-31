use std::time::Duration;

use windows_reactor::test::{LiveProbe, take_live_diagnostics};
use windows_reactor::*;

use crate::fixtures::{
    CompositionLifecycle, EncodedImageLifecycle, FixtureInput, FixtureResult, FocusPublication,
    ImageSourceLifecycle, KeyedNativeMutations, PointerInjection, ProbeFixture, ProbeInput,
    SwapChainLifecycle, ThemeResources, WindowLifecycle,
};

const FIXTURE_TIMEOUT: Duration = Duration::from_secs(15);
pub(crate) const SUITE_TIMEOUT: Duration =
    Duration::from_secs(FIXTURE_TIMEOUT.as_secs() * FIXTURES.len() as u64 + 10);

#[derive(Clone, Copy)]
enum FixtureKind {
    ContentDialogLifecycle,
    FocusPublication,
    EventDelivery,
    EventRevokers,
    ControlledFeedback,
    WindowLifecycle,
    EncodedImageLifecycle,
    ImageSourceLifecycle,
    CompositionLifecycle,
    SwapChainLifecycle,
    ThemeResources,
    PointerInjection,
    KeyedNativeMutations,
}

struct Fixture {
    name: &'static str,
    kind: FixtureKind,
}

const FIXTURES: &[Fixture] = &[
    Fixture {
        name: "Focus_PublicationAndRetirement",
        kind: FixtureKind::FocusPublication,
    },
    Fixture {
        name: "ContentDialog_QueuedReopenLifecycle",
        kind: FixtureKind::ContentDialogLifecycle,
    },
    Fixture {
        name: "Events_NativePayloadDelivery",
        kind: FixtureKind::EventDelivery,
    },
    Fixture {
        name: "Events_ReplacementAndRevocation",
        kind: FixtureKind::EventRevokers,
    },
    Fixture {
        name: "Controlled_NativeFeedback",
        kind: FixtureKind::ControlledFeedback,
    },
    Fixture {
        name: "Window_ClosureTaskAndEffectCleanup",
        kind: FixtureKind::WindowLifecycle,
    },
    Fixture {
        name: "ImageSource_DpiAttachClearRetire",
        kind: FixtureKind::ImageSourceLifecycle,
    },
    Fixture {
        name: "Image_EncodedLoadReplaceAndFailure",
        kind: FixtureKind::EncodedImageLifecycle,
    },
    Fixture {
        name: "Composition_AttachReplaceClearRetire",
        kind: FixtureKind::CompositionLifecycle,
    },
    Fixture {
        name: "SwapChain_MetricsRenderClearRetire",
        kind: FixtureKind::SwapChainLifecycle,
    },
    Fixture {
        name: "Theme_AndResourceUpdates",
        kind: FixtureKind::ThemeResources,
    },
    Fixture {
        name: "Pointer_RealInputGesture",
        kind: FixtureKind::PointerInjection,
    },
    Fixture {
        name: "Reconcile_KeyedNativeMutations",
        kind: FixtureKind::KeyedNativeMutations,
    },
];

pub(crate) struct FixtureRunner {
    current: usize,
    generation: u64,
    timeout: Option<ComponentTask>,
}

pub(crate) enum Message {
    Complete {
        generation: u64,
        result: FixtureResult,
    },
    Timeout(u64),
}

impl FixtureRunner {
    fn start_timeout(&mut self, context: &ComponentContext<Self>) {
        let generation = self.generation;
        self.timeout = Some(context.spawn_background(move |cancellation| {
            std::thread::sleep(FIXTURE_TIMEOUT);
            if cancellation.is_cancelled() {
                Message::Timeout(u64::MAX)
            } else {
                Message::Timeout(generation)
            }
        }));
    }

    fn fail(&self, detail: &str) -> ! {
        eprintln!(
            "not ok {} - {}",
            self.current + 1,
            FIXTURES[self.current].name
        );
        eprintln!("# {detail}");
        std::process::exit(1);
    }

    fn open_probe(&self, context: &ComponentContext<Self>) {
        let probe = match FIXTURES[self.current].kind {
            FixtureKind::ContentDialogLifecycle => LiveProbe::ContentDialogLifecycle,
            FixtureKind::EventDelivery => LiveProbe::EventDelivery,
            FixtureKind::EventRevokers => LiveProbe::EventRevokers,
            FixtureKind::ControlledFeedback => LiveProbe::ControlledFeedback,
            _ => return,
        };
        let generation = self.generation;
        let complete = context
            .sender()
            .callback(move |result| Message::Complete { generation, result });
        if !context.open_window(View::component::<ProbeFixture>(ProbeInput {
            complete,
            probe,
        })) {
            self.fail(&format!("{probe:?} probe window was rejected"));
        }
    }
}

impl Component for FixtureRunner {
    type Input = ();
    type Message = Message;

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let mut runner = Self {
            current: 0,
            generation: 0,
            timeout: None,
        };
        runner.start_timeout(context);
        runner
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            Message::Complete { generation, result } if generation == self.generation => {
                if let Some(timeout) = self.timeout.take() {
                    timeout.cancel();
                }
                if let Err(detail) = result {
                    self.fail(&detail);
                }
                let diagnostics = take_live_diagnostics();
                if !diagnostics.is_empty() {
                    self.fail(&format!("unexpected diagnostics: {diagnostics:?}"));
                }
                println!("ok {} - {}", self.current + 1, FIXTURES[self.current].name);
                self.current += 1;
                self.generation += 1;
                if self.current == FIXTURES.len() {
                    println!("1..{}", FIXTURES.len());
                    if !context.window().request_close() {
                        self.fail("fixture runner could not close its window");
                    }
                } else {
                    self.start_timeout(context);
                    self.open_probe(context);
                }
            }
            Message::Timeout(generation) if generation == self.generation => {
                self.fail("fixture timed out")
            }
            _ => {}
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("windows-reactor selftest");
        let generation = self.generation;
        let input = FixtureInput {
            complete: context.callback(move |result| Message::Complete { generation, result }),
        };
        match FIXTURES.get(self.current).map(|fixture| fixture.kind) {
            Some(FixtureKind::ContentDialogLifecycle) => TextBlock::new()
                .text("ContentDialog lifecycle probe")
                .into(),
            Some(FixtureKind::FocusPublication) => View::component::<FocusPublication>(input),
            Some(FixtureKind::EventDelivery) => {
                TextBlock::new().text("event delivery probe").into()
            }
            Some(FixtureKind::EventRevokers) => TextBlock::new().text("event probe").into(),
            Some(FixtureKind::ControlledFeedback) => {
                TextBlock::new().text("controlled probe").into()
            }
            Some(FixtureKind::WindowLifecycle) => View::component::<WindowLifecycle>(input),
            Some(FixtureKind::ImageSourceLifecycle) => {
                View::component::<ImageSourceLifecycle>(input)
            }
            Some(FixtureKind::EncodedImageLifecycle) => {
                View::component::<EncodedImageLifecycle>(input)
            }
            Some(FixtureKind::CompositionLifecycle) => {
                View::component::<CompositionLifecycle>(input)
            }
            Some(FixtureKind::SwapChainLifecycle) => View::component::<SwapChainLifecycle>(input),
            Some(FixtureKind::ThemeResources) => View::component::<ThemeResources>(input),
            Some(FixtureKind::PointerInjection) => View::component::<PointerInjection>(input),
            Some(FixtureKind::KeyedNativeMutations) => {
                View::component::<KeyedNativeMutations>(input)
            }
            None => TextBlock::new().text("fixtures complete").into(),
        }
    }
}
