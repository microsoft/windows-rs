use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use windows_reactor::*;

use crate::generated_surface::{
    CAPABILITY_PROPERTIES, CAPABILITY_PROPERTY_COUNT, EXTENSION_COUNT, EXTENSION_SURFACES,
    PROJECTED_EVENT_COUNT, PROJECTED_EVENTS, PROJECTED_PROPERTIES, PROJECTED_PROPERTY_COUNT,
    STRUCTURAL_COUNT, STRUCTURAL_SURFACES, SURFACE_CASES, SurfaceCase, SurfaceKind,
};

const CASE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SurfaceConfig {
    filter: Option<String>,
    list: bool,
}

impl SurfaceConfig {
    pub(crate) fn from_args() -> Self {
        let mut args = std::env::args().skip(1);
        let mut filter = None;
        let mut list = false;
        while let Some(argument) = args.next() {
            if argument == "--filter" {
                filter = args.next();
            } else if argument == "--list" {
                list = true;
            }
        }
        Self { filter, list }
    }
}

pub(crate) struct SurfaceRunner {
    selected: Vec<usize>,
    current: usize,
    stage: usize,
    generation: u64,
    subscription_baseline: Option<usize>,
    timeout_progress: Arc<Mutex<(u64, Instant)>>,
    timeout: Option<ComponentTask>,
}

pub(crate) enum Message {
    Advance(u64),
    Close,
    SubscriptionCount(u64, Result<usize, String>),
    Timeout(u64),
    TimeoutRejected,
}

impl SurfaceRunner {
    fn case(&self) -> Option<&'static SurfaceCase> {
        self.selected
            .get(self.current)
            .map(|index| &SURFACE_CASES[*index])
    }

    fn selected_count(&self, kind: SurfaceKind) -> usize {
        self.selected
            .iter()
            .filter(|index| SURFACE_CASES[**index].kind == kind)
            .count()
    }

    fn start_timeout(&mut self, context: &ComponentContext<Self>) {
        let progress = Arc::clone(&self.timeout_progress);
        self.timeout = Some(context.spawn_background_with_rejection(
            move |cancellation| {
                loop {
                    if cancellation.is_cancelled() {
                        return Message::Timeout(u64::MAX);
                    }
                    let (generation, started) = *progress.lock().unwrap();
                    let now = Instant::now();
                    let deadline = started + CASE_TIMEOUT;
                    if now >= deadline {
                        return Message::Timeout(generation);
                    }
                    std::thread::sleep((deadline - now).min(Duration::from_millis(10)));
                }
            },
            Message::TimeoutRejected,
        ));
    }

    fn reset_timeout(&self) {
        *self.timeout_progress.lock().unwrap() = (self.generation, Instant::now());
    }

    fn fail(&self, detail: &str) -> ! {
        let name = self.case().map_or("surface suite", |case| case.name);
        eprintln!("not ok {} - {name}", self.current + 1);
        eprintln!("# {detail}");
        std::process::exit(1);
    }

    fn finish_stage(&mut self, context: &ComponentContext<Self>) {
        let case = self.case().unwrap();
        if self.stage + 1 < case.stages {
            self.stage += 1;
            self.generation += 1;
            self.reset_timeout();
            return;
        }
        println!("ok {} - {}", self.current + 1, case.name);
        if self.current + 1 == self.selected.len() {
            println!("1..{}", self.selected.len());
            println!(
                "# controls: {}/{} constructed",
                self.selected_count(SurfaceKind::Control),
                SURFACE_CASES
                    .iter()
                    .filter(|case| case.kind == SurfaceKind::Control)
                    .count()
            );
            println!(
                "# properties: {}/{} set, updated, and cleared",
                self.selected_count(SurfaceKind::Property),
                PROJECTED_PROPERTY_COUNT
            );
            println!(
                "# events: {}/{} subscribed, replaced, and revoked",
                self.selected_count(SurfaceKind::Event),
                PROJECTED_EVENT_COUNT
            );
            println!(
                "# capability properties: {}/{} set, updated, and cleared",
                self.selected_count(SurfaceKind::CapabilityProperty),
                CAPABILITY_PROPERTY_COUNT
            );
            println!(
                "# structural surfaces: {}/{} set, updated, and cleared",
                self.selected_count(SurfaceKind::Structural),
                STRUCTURAL_COUNT
            );
            println!(
                "# extension surfaces: {}/{} attached, updated, and cleared",
                self.selected_count(SurfaceKind::Extension),
                EXTENSION_COUNT
            );
            if let Some(timeout) = self.timeout.take() {
                timeout.cancel();
            }
            if !context.window().request_close() {
                self.fail("surface runner could not close its window");
            }
        } else {
            self.current += 1;
            self.stage = 0;
            self.generation += 1;
            self.subscription_baseline = None;
            self.reset_timeout();
        }
    }

    fn verify_subscription_count(&mut self, count: usize, context: &ComponentContext<Self>) {
        let case = self.case().unwrap();
        let delta = case.subscription_delta.unwrap();
        let baseline = *self.subscription_baseline.get_or_insert(count);
        let expected = if matches!(self.stage, 1 | 2) {
            baseline + delta
        } else {
            baseline
        };
        if count != expected {
            self.fail(&format!(
                "native event subscription count was {count}, expected {expected}"
            ));
        }
        self.finish_stage(context);
    }
}

impl Component for SurfaceRunner {
    type Input = SurfaceConfig;
    type Message = Message;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        if input.list {
            for property in PROJECTED_PROPERTIES {
                println!(
                    "property.{}.{} value={} adapter={} validation={} clearable={} theme_style={}",
                    property.control,
                    property.property,
                    property.value,
                    property.adapter,
                    property.validation.unwrap_or("none"),
                    property.clearable,
                    property.theme_style
                );
            }
            for event in PROJECTED_EVENTS {
                println!(
                    "event.{}.{} payload={} conversion={} subscription={} delivery={} active_property={}",
                    event.control,
                    event.event,
                    event.payload,
                    event.conversion,
                    event.subscription,
                    event.delivery,
                    event.active_property.unwrap_or("none")
                );
            }
            for property in CAPABILITY_PROPERTIES {
                println!("capability.{}.{}", property.capability, property.property);
            }
            for surface in STRUCTURAL_SURFACES {
                println!("structural.{}.{}", surface.control, surface.member);
            }
            for surface in EXTENSION_SURFACES {
                println!("extension.{}", surface.name);
            }
        }
        let selected = SURFACE_CASES
            .iter()
            .enumerate()
            .filter(|(_, case)| {
                !input.list && {
                    input
                        .filter
                        .as_ref()
                        .is_none_or(|filter| case.name.contains(filter))
                }
            })
            .map(|(index, _)| index)
            .collect();
        let timeout_progress = Arc::new(Mutex::new((0, Instant::now())));
        let mut runner = Self {
            selected,
            current: 0,
            stage: 0,
            generation: 0,
            subscription_baseline: None,
            timeout_progress,
            timeout: None,
        };
        if !runner.selected.is_empty() {
            runner.start_timeout(context);
        }
        runner
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            Message::Advance(generation) if generation == self.generation => {
                let diagnostics = take_live_diagnostics();
                if !diagnostics.is_empty() {
                    self.fail(&format!("unexpected diagnostics: {diagnostics:?}"));
                }
                if self.case().unwrap().kind == SurfaceKind::Event {
                    let sender = context.sender();
                    let generation = self.generation;
                    if let Err(error) = schedule_live_event_subscription_count(move |result| {
                        if !sender.send(Message::SubscriptionCount(generation, result)) {
                            eprintln!("surface subscription count was rejected");
                            std::process::exit(1);
                        }
                    }) {
                        self.fail(&format!(
                            "could not schedule native event subscription count: {error}"
                        ));
                    }
                } else {
                    self.finish_stage(context);
                }
            }
            Message::SubscriptionCount(generation, result) if generation == self.generation => {
                match result {
                    Ok(count) => self.verify_subscription_count(count, context),
                    Err(error) => self.fail(&error),
                }
            }
            Message::Timeout(generation) if generation == self.generation => {
                self.fail(&format!("surface case stage {} timed out", self.stage))
            }
            Message::TimeoutRejected => self.fail("surface timeout watchdog was rejected"),
            Message::Close => {
                println!("1..0");
                if !context.window().request_close() {
                    self.fail("surface runner could not close its empty window");
                }
            }
            _ => {}
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("windows-reactor surface tests");
        let Some(case) = self.case() else {
            let sender = context.sender();
            context.use_effect("close-empty-suite", (), move || {
                if !sender.send(Message::Close) {
                    eprintln!("surface suite close was rejected");
                    std::process::exit(1);
                }
                None
            });
            return TextBlock::new().text("No matching surface cases").into();
        };

        let generation = self.generation;
        let sender = context.sender();
        context.use_effect("advance-surface-case", generation, move || {
            if !sender.send(Message::Advance(generation)) {
                eprintln!("surface case completion was rejected");
                std::process::exit(1);
            }
            None
        });
        (case.build)(self.stage)
    }
}
