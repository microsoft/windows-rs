use std::time::Duration;

use windows_reactor::test::{LiveProbe, take_live_diagnostics};
use windows_reactor::*;

#[cfg(feature = "self-contained")]
use crate::fixtures::WebViewLifecycle;
use crate::fixtures::{
    CompositionLifecycle, EncodedImageLifecycle, FixtureInput, FixtureResult, FocusPublication,
    ImageSourceLifecycle, KeyedNativeMutations, NestedWindowOperation, PointerInjection,
    ProbeFixture, ProbeInput, SwapChainLifecycle, ThemeResources, TimerLifecycle, WindowLifecycle,
};

const FIXTURE_TIMEOUT: Duration = Duration::from_secs(15);
const WEBVIEW_FIXTURE_TIMEOUT: Duration = Duration::from_secs(45);
pub(crate) const SUITE_TIMEOUT: Duration = Duration::from_secs(
    FIXTURE_TIMEOUT.as_secs() * FIXTURES.len() as u64
        + if cfg!(feature = "self-contained") {
            WEBVIEW_FIXTURE_TIMEOUT.as_secs() - FIXTURE_TIMEOUT.as_secs()
        } else {
            0
        }
        + 10,
);

#[derive(Clone, Copy)]
enum FixtureKind {
    ContentDialogLifecycle,
    FocusPublication,
    EventDelivery,
    EventRevokers,
    ControlledFeedback,
    NestedWindowOperation,
    WindowLifecycle,
    #[cfg(feature = "self-contained")]
    WebViewLifecycle,
    EncodedImageLifecycle,
    ImageSourceLifecycle,
    CompositionLifecycle,
    TimerLifecycle,
    SwapChainLifecycle,
    ThemeResources,
    PointerInjection,
    KeyedNativeMutations,
}

struct Fixture {
    name: &'static str,
    kind: FixtureKind,
}

impl Fixture {
    fn timeout(&self) -> Duration {
        match self.kind {
            #[cfg(feature = "self-contained")]
            FixtureKind::WebViewLifecycle => WEBVIEW_FIXTURE_TIMEOUT,
            _ => FIXTURE_TIMEOUT,
        }
    }
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
        name: "Window_NestedOperationRearming",
        kind: FixtureKind::NestedWindowOperation,
    },
    Fixture {
        name: "Window_ClosureTaskAndEffectCleanup",
        kind: FixtureKind::WindowLifecycle,
    },
    #[cfg(feature = "self-contained")]
    Fixture {
        name: "WebView_InitializeBridgeAndScript",
        kind: FixtureKind::WebViewLifecycle,
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
        name: "Timer_FireCancelAndDrop",
        kind: FixtureKind::TimerLifecycle,
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

pub(crate) fn select_fixtures(filter: Option<&str>) -> Result<Vec<usize>, String> {
    let selected = FIXTURES
        .iter()
        .enumerate()
        .filter_map(|(index, fixture)| {
            filter
                .is_none_or(|filter| fixture.name.contains(filter))
                .then_some(index)
        })
        .collect::<Vec<_>>();
    if selected.is_empty() {
        Err(format!("no fixture matched {filter:?}"))
    } else {
        Ok(selected)
    }
}

pub(crate) struct FixtureRunner {
    current: usize,
    generation: u64,
    selected: Vec<usize>,
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
    fn fixture(&self) -> &'static Fixture {
        &FIXTURES[self.selected[self.current]]
    }

    fn start_timeout(&mut self, context: &ComponentContext<Self>) {
        let generation = self.generation;
        let timeout = self.fixture().timeout();
        self.timeout = Some(context.spawn_background(move |cancellation| {
            std::thread::sleep(timeout);
            if cancellation.is_cancelled() {
                Message::Timeout(u64::MAX)
            } else {
                Message::Timeout(generation)
            }
        }));
    }

    fn fail(&self, detail: &str) -> ! {
        eprintln!("not ok {} - {}", self.current + 1, self.fixture().name);
        eprintln!("# {detail}");
        std::process::exit(1);
    }

    fn open_probe(&self, context: &ComponentContext<Self>) {
        let probe = match self.fixture().kind {
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
    type Input = Vec<usize>;
    type Message = Message;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let mut runner = Self {
            current: 0,
            generation: 0,
            selected: input.clone(),
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
                println!("ok {} - {}", self.current + 1, self.fixture().name);
                self.current += 1;
                self.generation += 1;
                if self.current == self.selected.len() {
                    println!("1..{}", self.selected.len());
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
        match self
            .selected
            .get(self.current)
            .map(|index| FIXTURES[*index].kind)
        {
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
            Some(FixtureKind::NestedWindowOperation) => {
                View::component::<NestedWindowOperation>(input)
            }
            Some(FixtureKind::WindowLifecycle) => View::component::<WindowLifecycle>(input),
            #[cfg(feature = "self-contained")]
            Some(FixtureKind::WebViewLifecycle) => View::component::<WebViewLifecycle>(input),
            Some(FixtureKind::ImageSourceLifecycle) => {
                View::component::<ImageSourceLifecycle>(input)
            }
            Some(FixtureKind::EncodedImageLifecycle) => {
                View::component::<EncodedImageLifecycle>(input)
            }
            Some(FixtureKind::CompositionLifecycle) => {
                View::component::<CompositionLifecycle>(input)
            }
            Some(FixtureKind::TimerLifecycle) => View::component::<TimerLifecycle>(input),
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
