#![windows_subsystem = "console"]

mod allocator;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use windows::Win32::{
    GetForegroundWindow, HWND, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, SendInput, SetForegroundWindow,
};
use windows_reactor::test::{
    LiveInputProbe, LiveInputProbeStage, clear_live_performance_times, schedule_live_input_probe,
    take_live_performance_times,
};
use windows_reactor::*;

const WAIT_TIMEOUT: Duration = Duration::from_secs(5);
const END_KEY: u16 = 0x23;

#[derive(Clone, Copy, PartialEq)]
struct Options {
    warmup: usize,
    samples: usize,
    text_size: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            warmup: 100,
            samples: 1_000,
            text_size: 0,
        }
    }
}

impl Options {
    fn parse() -> Result<Option<Self>, String> {
        let mut options = Self::default();
        let mut arguments = std::env::args().skip(1);
        while let Some(argument) = arguments.next() {
            let target = match argument.as_str() {
                "--warmup" => &mut options.warmup,
                "--samples" => &mut options.samples,
                "--text-size" => &mut options.text_size,
                "--help" | "-h" => {
                    println!(
                        "reactor-live-notepad\n\
                         \n\
                         Measures real TextBox input through the controlled reactor-notepad path.\n\
                         \n\
                         Options:\n\
                           --warmup N     Warmup characters (default: 100)\n\
                           --samples N    Measured characters (default: 1000)\n\
                           --text-size N  Initial UTF-8/UTF-16 ASCII text length (default: 0)\n\
                           -h, --help     Print this help"
                    );
                    return Ok(None);
                }
                _ => return Err(format!("unknown argument: {argument}")),
            };
            let value = arguments
                .next()
                .ok_or_else(|| format!("{argument} requires a value"))?;
            *target = value
                .parse()
                .map_err(|_| format!("invalid {argument} value: {value}"))?;
        }
        if options.samples == 0 {
            return Err("--samples must be greater than zero".to_string());
        }
        Ok(Some(options))
    }
}

struct Stage {
    count: AtomicUsize,
    timestamps: Vec<AtomicU64>,
}

impl Stage {
    fn new(capacity: usize) -> Self {
        Self {
            count: AtomicUsize::new(0),
            timestamps: (0..capacity).map(|_| AtomicU64::new(0)).collect(),
        }
    }

    fn mark(&self, timestamp: u64) {
        let index = self.count.fetch_add(1, Ordering::AcqRel);
        if let Some(slot) = self.timestamps.get(index) {
            slot.store(timestamp, Ordering::Release);
        }
    }

    fn get(&self, index: usize) -> u64 {
        self.timestamps[index].load(Ordering::Acquire)
    }
}

struct Measurements {
    origin: Instant,
    starts: Vec<AtomicU64>,
    raw_character: Stage,
    native_changed: Stage,
    native_ready: Stage,
    event_callback: Stage,
    component: Stage,
    dispatch_complete: Stage,
    completion: Condvar,
    completed: Mutex<usize>,
}

impl Measurements {
    fn new(capacity: usize) -> Self {
        Self {
            origin: Instant::now(),
            starts: (0..capacity).map(|_| AtomicU64::new(0)).collect(),
            raw_character: Stage::new(capacity),
            native_changed: Stage::new(capacity),
            native_ready: Stage::new(capacity),
            event_callback: Stage::new(capacity),
            component: Stage::new(capacity),
            dispatch_complete: Stage::new(capacity),
            completion: Condvar::new(),
            completed: Mutex::new(0),
        }
    }

    fn now(&self) -> u64 {
        self.origin.elapsed().as_nanos() as u64
    }

    fn mark_probe(&self, stage: LiveInputProbeStage) {
        let timestamp = self.now();
        match stage {
            LiveInputProbeStage::RawCharacter => self.raw_character.mark(timestamp),
            LiveInputProbeStage::NativeTextChanged => self.native_changed.mark(timestamp),
            LiveInputProbeStage::NativeTextReady => self.native_ready.mark(timestamp),
            LiveInputProbeStage::ReactorDispatchComplete => {
                self.dispatch_complete.mark(timestamp);
                let mut completed = self.completed.lock().unwrap();
                *completed += 1;
                self.completion.notify_one();
            }
            LiveInputProbeStage::RawWindowMessage | LiveInputProbeStage::InputKeyboardSource => {}
        }
    }

    fn mark_event_callback(&self) {
        self.event_callback.mark(self.now());
    }

    fn mark_component(&self) {
        self.component.mark(self.now());
    }

    fn wait_for_dispatch(&self, expected: usize) -> Result<(), String> {
        let completed = self.completed.lock().unwrap();
        let (completed, timeout) = self
            .completion
            .wait_timeout_while(completed, WAIT_TIMEOUT, |completed| *completed < expected)
            .unwrap();
        if timeout.timed_out() {
            Err(format!(
                "timed out waiting for input {expected}; observed {completed} completed dispatches"
            ))
        } else {
            Ok(())
        }
    }
}

struct Distribution {
    median_ns: u64,
    p95_ns: u64,
    p99_ns: u64,
}

impl Distribution {
    fn from(mut values: Vec<u64>) -> Self {
        values.sort_unstable();
        Self {
            median_ns: percentile(&values, 50),
            p95_ns: percentile(&values, 95),
            p99_ns: percentile(&values, 99),
        }
    }
}

struct Report {
    text_size: usize,
    final_text_size: usize,
    samples: usize,
    allocations_per_input: f64,
    allocated_bytes_per_input: f64,
    raw: Distribution,
    winui: Distribution,
    text_read: Distribution,
    queue: Distribution,
    update: Distribution,
    reconcile: Distribution,
    reactor_total: Distribution,
    end_to_end: Distribution,
}

enum Message {
    Focused(Result<(bool, isize), String>),
    Text(String),
    Report(Box<Result<Report, String>>),
}

struct LiveNotepad {
    options: Options,
    text: String,
    measurements: Arc<Measurements>,
    target: ElementRef<TextBox>,
    probe: Rc<RefCell<Option<LiveInputProbe>>>,
}

impl Component for LiveNotepad {
    type Input = Options;
    type Message = Message;

    fn create(options: &Options, _context: &ComponentContext<Self>) -> Self {
        Self {
            options: *options,
            text: "a".repeat(options.text_size),
            measurements: Arc::new(Measurements::new(options.warmup + options.samples)),
            target: ElementRef::new(),
            probe: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Focused(Ok((true, hwnd))) => {
                clear_live_performance_times();
                let options = self.options;
                let measurements = Arc::clone(&self.measurements);
                context.spawn_background(move |_| {
                    Message::Report(Box::new(run_measurement(options, hwnd, measurements)))
                });
            }
            Message::Focused(Ok((false, _))) => {
                eprintln!("WinUI rejected the TextBox focus request");
                std::process::exit(1);
            }
            Message::Focused(Err(error)) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
            Message::Text(text) => {
                self.measurements.mark_component();
                self.text = text;
            }
            Message::Report(report) => match *report {
                Ok(mut report) => {
                    report.final_text_size = self.text.len();
                    let expected =
                        self.options.text_size + self.options.warmup + self.options.samples;
                    if report.final_text_size != expected {
                        eprintln!(
                            "TextBox ended with {} bytes; expected {expected}; the large-text \
                             scenario is invalid",
                            report.final_text_size
                        );
                        std::process::exit(1);
                    }
                    let (_, native_apply) = take_live_performance_times();
                    if !native_apply.is_empty() {
                        eprintln!(
                            "controlled TextBox input unexpectedly produced {} native apply batches",
                            native_apply.len()
                        );
                        std::process::exit(1);
                    }
                    print_report(&report);
                    std::process::exit(0);
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            },
        }
    }

    fn view(&self, _options: &Options, context: &mut ViewContext<Self>) -> View {
        context.window_title("windows-reactor live Notepad measurement");

        let probe = Rc::clone(&self.probe);
        let target = self.target.clone();
        let probe_measurements = Arc::clone(&self.measurements);
        let sender = context.sender();
        context.use_effect("setup", (), move || {
            let completion = sender.clone();
            let result = schedule_live_input_probe(
                {
                    let measurements = Arc::clone(&probe_measurements);
                    move |stage| measurements.mark_probe(stage)
                },
                move |result| match result {
                    Ok(live_probe) => {
                        let hwnd = live_probe.window_handle();
                        *probe.borrow_mut() = Some(live_probe);
                        let focus_completion = completion.clone();
                        let focused_probe = Rc::clone(&probe);
                        if !target.request_focus_result(move |result| {
                            let result = result
                                .map_err(|error| format!("focus request failed: {error:?}"))
                                .and_then(|focused| {
                                    if focused {
                                        focused_probe
                                            .borrow_mut()
                                            .as_mut()
                                            .unwrap()
                                            .retarget_raw_input()
                                            .map_err(|error| {
                                                format!(
                                                    "failed to hook focused input window: {error}"
                                                )
                                            })?;
                                    }
                                    Ok((focused, hwnd))
                                });
                            focus_completion.send(Message::Focused(result));
                        }) {
                            completion.send(Message::Focused(Err(
                                "TextBox target was not published".to_string(),
                            )));
                        }
                    }
                    Err(error) => {
                        completion.send(Message::Focused(Err(format!(
                            "failed to install live input probe: {error}"
                        ))));
                    }
                },
            );
            if let Err(error) = result {
                sender.send(Message::Focused(Err(format!(
                    "failed to schedule live input probe: {error}"
                ))));
            }
            None
        });

        let measurements = Arc::clone(&self.measurements);
        TextBox::new()
            .text(self.text.clone())
            .accepts_return(true)
            .text_wrapping(TextWrapping::Wrap)
            .element_ref(&self.target)
            .on_text_changed(context.callback(move |text| {
                measurements.mark_event_callback();
                Message::Text(text)
            }))
            .into()
    }
}

fn run_measurement(
    options: Options,
    hwnd: isize,
    measurements: Arc<Measurements>,
) -> Result<Report, String> {
    unsafe {
        let _ = SetForegroundWindow(HWND(hwnd as *mut _));
        if GetForegroundWindow() != HWND(hwnd as *mut _) {
            return Err("measurement window is not foreground".to_string());
        }
    }
    std::thread::sleep(Duration::from_millis(100));
    inject_virtual_key(END_KEY)?;
    std::thread::sleep(Duration::from_millis(100));

    let total = options.warmup + options.samples;
    for index in 0..options.warmup {
        measurements.starts[index].store(measurements.now(), Ordering::Release);
        inject_character('x')?;
        measurements.wait_for_dispatch(index + 1)?;
    }
    let allocations_started = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let allocated_bytes_started = allocator::allocated_bytes();
    for index in options.warmup..total {
        measurements.starts[index].store(measurements.now(), Ordering::Release);
        inject_character('x')?;
        measurements.wait_for_dispatch(index + 1)?;
    }
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations_started;
    let allocated_bytes = allocator::allocated_bytes() - allocated_bytes_started;

    for (name, stage) in [
        ("raw WM_CHAR", &measurements.raw_character),
        ("native TextChanged", &measurements.native_changed),
        ("native text ready", &measurements.native_ready),
        ("Reactor event callback", &measurements.event_callback),
        ("component update", &measurements.component),
        ("dispatch completion", &measurements.dispatch_complete),
    ] {
        let actual = stage.count.load(Ordering::Acquire);
        if actual != total {
            return Err(format!(
                "{name} observed {actual} inputs; expected {total}; stage correlation is invalid"
            ));
        }
    }

    let range = options.warmup..total;
    Ok(Report {
        text_size: options.text_size,
        final_text_size: 0,
        samples: options.samples,
        allocations_per_input: allocations as f64 / options.samples as f64,
        allocated_bytes_per_input: allocated_bytes as f64 / options.samples as f64,
        raw: distribution(
            range.clone(),
            &measurements.starts,
            &measurements.raw_character,
            "SendInput",
            "raw WM_CHAR",
        )?,
        winui: distribution(
            range.clone(),
            &measurements.raw_character.timestamps,
            &measurements.native_changed,
            "raw WM_CHAR",
            "native TextChanged",
        )?,
        text_read: distribution(
            range.clone(),
            &measurements.native_changed.timestamps,
            &measurements.native_ready,
            "native TextChanged",
            "native text ready",
        )?,
        queue: distribution(
            range.clone(),
            &measurements.native_ready.timestamps,
            &measurements.event_callback,
            "native text ready",
            "Reactor event callback",
        )?,
        update: distribution(
            range.clone(),
            &measurements.event_callback.timestamps,
            &measurements.component,
            "Reactor event callback",
            "component update",
        )?,
        reconcile: distribution(
            range.clone(),
            &measurements.component.timestamps,
            &measurements.dispatch_complete,
            "component update",
            "dispatch completion",
        )?,
        reactor_total: distribution(
            range.clone(),
            &measurements.native_ready.timestamps,
            &measurements.dispatch_complete,
            "native text ready",
            "dispatch completion",
        )?,
        end_to_end: distribution(
            range,
            &measurements.starts,
            &measurements.dispatch_complete,
            "SendInput",
            "dispatch completion",
        )?,
    })
}

fn distribution(
    range: std::ops::Range<usize>,
    first: &[AtomicU64],
    second: &Stage,
    first_name: &str,
    second_name: &str,
) -> Result<Distribution, String> {
    let values = range
        .map(|index| {
            second
                .get(index)
                .checked_sub(first[index].load(Ordering::Acquire))
                .ok_or_else(|| {
                    format!(
                        "{second_name} preceded {first_name} for input {index}; correlation is invalid"
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Distribution::from(values))
}

fn percentile(values: &[u64], percentile: usize) -> u64 {
    values[(values.len() - 1) * percentile / 100]
}

fn inject_character(character: char) -> Result<(), String> {
    let down = INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wScan: character as u16,
                dwFlags: KEYEVENTF_UNICODE as u32,
                ..Default::default()
            },
        },
    };
    let up = INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wScan: character as u16,
                dwFlags: (KEYEVENTF_UNICODE | KEYEVENTF_KEYUP) as u32,
                ..Default::default()
            },
        },
    };
    let inputs = [down, up];
    let inserted = unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };
    if inserted == inputs.len() as u32 {
        Ok(())
    } else {
        Err(format!(
            "SendInput inserted {inserted} of {} events",
            inputs.len()
        ))
    }
}

fn inject_virtual_key(key: u16) -> Result<(), String> {
    let down = INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                ..Default::default()
            },
        },
    };
    let up = INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                dwFlags: KEYEVENTF_KEYUP as u32,
                ..Default::default()
            },
        },
    };
    let inputs = [down, up];
    let inserted = unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };
    if inserted == inputs.len() as u32 {
        Ok(())
    } else {
        Err(format!(
            "SendInput inserted {inserted} of {} events",
            inputs.len()
        ))
    }
}

fn print_report(report: &Report) {
    println!(
        "initial text: {} bytes, final text: {} bytes, samples: {}",
        report.text_size, report.final_text_size, report.samples
    );
    println!(
        "Rust allocations/input: {:.2}, allocated bytes/input: {:.0}",
        report.allocations_per_input, report.allocated_bytes_per_input
    );
    println!("stage                                  median       p95       p99");
    print_distribution("SendInput -> raw WM_CHAR", &report.raw);
    print_distribution("raw -> native TextChanged", &report.winui);
    print_distribution("Text() retrieval", &report.text_read);
    print_distribution("native queue -> event callback", &report.queue);
    print_distribution("component message queue", &report.update);
    print_distribution("update + reconcile", &report.reconcile);
    print_distribution("Reactor total after Text()", &report.reactor_total);
    print_distribution("SendInput -> dispatch complete", &report.end_to_end);
    println!("native apply: none (controlled feedback was suppressed)");
}

fn print_distribution(name: &str, distribution: &Distribution) {
    println!(
        "{name:<36} {:>7.2} us {:>7.2} us {:>7.2} us",
        distribution.median_ns as f64 / 1_000.0,
        distribution.p95_ns as f64 / 1_000.0,
        distribution.p99_ns as f64 / 1_000.0,
    );
}

fn main() -> windows_core::Result<()> {
    let Some(options) = Options::parse().unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2);
    }) else {
        return Ok(());
    };

    let text_work = options
        .text_size
        .max(1_024)
        .saturating_mul(options.warmup + options.samples) as u64;
    let timeout = Duration::from_secs(30 + text_work / 500_000);
    std::thread::spawn(move || {
        std::thread::sleep(timeout);
        eprintln!("live Notepad measurement timed out");
        std::process::exit(1);
    });

    App::run_component::<LiveNotepad>(options)
}
