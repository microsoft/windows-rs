#![windows_subsystem = "console"]

mod allocator;

use std::cell::RefCell;
use std::mem::size_of;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use windows::Win32::{
    GetForegroundWindow, HWND, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, SendInput, SetForegroundWindow,
};
use windows_reactor::{App, AppContext, AppProxy};
use windows_reactor2 as reactor2;

const WAIT_TIMEOUT: Duration = Duration::from_secs(5);
const END_KEY: u16 = 0x23;

#[derive(Clone, Copy)]
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
                        "reactor2-live-notepad\n\
                         \n\
                         Measures real TextBox input through Reactor2's controlled text path.\n\
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
    callback: Stage,
    reconcile: Stage,
    mutations: AtomicUsize,
    final_text_size: AtomicUsize,
    native_sets: AtomicUsize,
    completion: Condvar,
    completed: Mutex<usize>,
}

impl Measurements {
    fn new(capacity: usize) -> Self {
        Self {
            origin: Instant::now(),
            starts: (0..capacity).map(|_| AtomicU64::new(0)).collect(),
            callback: Stage::new(capacity),
            reconcile: Stage::new(capacity),
            mutations: AtomicUsize::new(0),
            final_text_size: AtomicUsize::new(0),
            native_sets: AtomicUsize::new(0),
            completion: Condvar::new(),
            completed: Mutex::new(0),
        }
    }

    fn now(&self) -> u64 {
        self.origin.elapsed().as_nanos() as u64
    }

    fn complete(&self, mutation_count: usize) {
        self.mutations.fetch_add(mutation_count, Ordering::Relaxed);
        self.reconcile.mark(self.now());
        let mut completed = self.completed.lock().unwrap();
        *completed += 1;
        self.completion.notify_one();
    }

    fn wait_for_reconcile(&self, expected: usize) -> Result<(), String> {
        let completed = self.completed.lock().unwrap();
        let (completed, timeout) = self
            .completion
            .wait_timeout_while(completed, WAIT_TIMEOUT, |completed| *completed < expected)
            .unwrap();
        if timeout.timed_out() {
            Err(format!(
                "timed out waiting for input {expected}; observed {completed} reconciliations"
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
    mutations_per_input: f64,
    native_sets: usize,
    callback: Distribution,
    reconcile: Distribution,
    end_to_end: Distribution,
}

struct Harness {
    runtime: reactor2::Runtime<reactor2::native::WinUiAdapter>,
    callback: reactor2::Callback<Rc<str>>,
    pending_text: Rc<RefCell<Option<Rc<str>>>>,
    events: Vec<reactor2::EventDispatch>,
    measurements: Arc<Measurements>,
    root: reactor2::ObjectId,
    window: reactor2::native::NativeWindow,
}

fn create_harness(
    app: &AppContext,
    options: Options,
    measurements: Arc<Measurements>,
) -> windows_core::Result<Rc<RefCell<Option<Harness>>>> {
    let holder = Rc::new(RefCell::new(None::<Harness>));
    let pending_text = Rc::new(RefCell::new(None));
    let callback_pending_text = Rc::clone(&pending_text);
    let callback_measurements = Arc::clone(&measurements);
    let callback = reactor2::Callback::new(move |text: Rc<str>| {
        callback_measurements
            .callback
            .mark(callback_measurements.now());
        *callback_pending_text.borrow_mut() = Some(text);
    });

    let mut runtime = reactor2::Runtime::new(reactor2::native::WinUiAdapter::default());
    runtime
        .update(
            reactor2::TextBox::new("a".repeat(options.text_size))
                .on_text_changed_callback(callback.clone()),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let window = runtime.adapter().create_window(root).unwrap();
    let initial_native_sets = runtime.adapter().text_box_set_count(root).unwrap();

    *holder.borrow_mut() = Some(Harness {
        runtime,
        callback,
        pending_text,
        events: Vec::new(),
        measurements: Arc::clone(&measurements),
        root,
        window,
    });

    let event_holder = Rc::downgrade(&holder);
    holder
        .borrow_mut()
        .as_mut()
        .unwrap()
        .runtime
        .adapter_mut()
        .set_event_waker(move || process_events(&event_holder));

    let window = holder.borrow().as_ref().unwrap().window.clone();
    window.activate().unwrap();
    schedule_focus(
        Rc::downgrade(&holder),
        app.proxy(),
        options,
        measurements,
        initial_native_sets,
    );

    Ok(holder)
}

fn process_events(holder: &Weak<RefCell<Option<Harness>>>) {
    let holder = holder.upgrade().unwrap();
    let mut events = {
        let mut holder = holder.borrow_mut();
        let harness = holder.as_mut().unwrap();
        let mut events = std::mem::take(&mut harness.events);
        harness.runtime.drain_events(&mut events).unwrap();
        events
    };
    for event in events.drain(..) {
        event.invoke();
    }
    let mut holder = holder.borrow_mut();
    let harness = holder.as_mut().unwrap();
    harness.events = events;
    let Some(text) = harness.pending_text.borrow_mut().take() else {
        return;
    };
    let text_size = text.len();
    let callback = harness.callback.clone();
    let mutations = harness
        .runtime
        .update(reactor2::TextBox::new(text).on_text_changed_callback(callback))
        .unwrap();
    harness
        .measurements
        .final_text_size
        .store(text_size, Ordering::Relaxed);
    harness.measurements.native_sets.store(
        harness
            .runtime
            .adapter()
            .text_box_set_count(harness.root)
            .unwrap(),
        Ordering::Relaxed,
    );
    harness.measurements.complete(mutations.len());
}

fn schedule_focus(
    holder: Weak<RefCell<Option<Harness>>>,
    proxy: AppProxy,
    options: Options,
    measurements: Arc<Measurements>,
    initial_native_sets: usize,
) {
    let holder_value = holder.upgrade().unwrap();
    let holder_value = holder_value.borrow();
    let harness = holder_value.as_ref().unwrap();
    harness
        .runtime
        .adapter()
        .focus_text_box_deferred(harness.root, move |focused| {
            if !focused {
                eprintln!("WinUI rejected the Reactor2 TextBox focus request");
                std::process::exit(1);
            }
            let hwnd = unsafe { GetForegroundWindow() };
            if hwnd == HWND::default() {
                eprintln!("Reactor2 measurement window is not foreground");
                std::process::exit(1);
            }
            let hwnd = hwnd as isize;
            let measurements = Arc::clone(&measurements);
            let proxy = proxy.clone();
            std::thread::spawn(move || {
                let report = run_measurement(options, hwnd, measurements, initial_native_sets);
                proxy
                    .dispatch(move |context| {
                        match report {
                            Ok(report) => {
                                print_report(&report);
                            }
                            Err(error) => {
                                eprintln!("{error}");
                                std::process::exit(1);
                            }
                        }
                        _ = context.exit();
                    })
                    .unwrap();
            });
        })
        .unwrap();
}

fn run_measurement(
    options: Options,
    hwnd: isize,
    measurements: Arc<Measurements>,
    initial_native_sets: usize,
) -> Result<Report, String> {
    let hwnd = hwnd as HWND;
    unsafe {
        let _ = SetForegroundWindow(hwnd);
        if GetForegroundWindow() != hwnd {
            return Err("Reactor2 measurement window is not foreground".to_string());
        }
    }
    std::thread::sleep(Duration::from_millis(100));
    inject_virtual_key(END_KEY)?;
    std::thread::sleep(Duration::from_millis(100));

    let total = options.warmup + options.samples;
    for index in 0..options.warmup {
        measurements.starts[index].store(measurements.now(), Ordering::Release);
        inject_character('x')?;
        measurements.wait_for_reconcile(index + 1)?;
    }
    let mutations_started = measurements.mutations.load(Ordering::Relaxed);
    let allocations_started = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let allocated_bytes_started = allocator::allocated_bytes();
    for index in options.warmup..total {
        measurements.starts[index].store(measurements.now(), Ordering::Release);
        inject_character('x')?;
        measurements.wait_for_reconcile(index + 1)?;
    }
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations_started;
    let allocated_bytes = allocator::allocated_bytes() - allocated_bytes_started;
    let mutations = measurements.mutations.load(Ordering::Relaxed) - mutations_started;

    for (name, stage) in [
        ("Reactor2 event callback", &measurements.callback),
        ("Reactor2 reconcile", &measurements.reconcile),
    ] {
        let actual = stage.count.load(Ordering::Acquire);
        if actual != total {
            return Err(format!(
                "{name} observed {actual} inputs; expected {total}; stage correlation is invalid"
            ));
        }
    }

    let range = options.warmup..total;
    let final_text_size = measurements.final_text_size.load(Ordering::Relaxed);
    let expected_text_size = options.text_size + total;
    if final_text_size != expected_text_size {
        return Err(format!(
            "TextBox ended with {final_text_size} bytes; expected {expected_text_size}"
        ));
    }
    Ok(Report {
        text_size: options.text_size,
        final_text_size,
        samples: options.samples,
        allocations_per_input: allocations as f64 / options.samples as f64,
        allocated_bytes_per_input: allocated_bytes as f64 / options.samples as f64,
        mutations_per_input: mutations as f64 / options.samples as f64,
        native_sets: measurements
            .native_sets
            .load(Ordering::Relaxed)
            .saturating_sub(initial_native_sets),
        callback: distribution(
            range.clone(),
            &measurements.starts,
            &measurements.callback,
            "SendInput",
            "Reactor2 event callback",
        )?,
        reconcile: distribution(
            range.clone(),
            &measurements.callback.timestamps,
            &measurements.reconcile,
            "Reactor2 event callback",
            "Reactor2 reconcile",
        )?,
        end_to_end: distribution(
            range,
            &measurements.starts,
            &measurements.reconcile,
            "SendInput",
            "Reactor2 reconcile",
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
    inject(&[down, up])
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
    inject(&[down, up])
}

fn inject(inputs: &[INPUT]) -> Result<(), String> {
    let inserted = unsafe { SendInput(inputs, size_of::<INPUT>() as i32) };
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
    println!(
        "mutations/input: {:.2}, native SetText calls: {}",
        report.mutations_per_input, report.native_sets
    );
    println!("stage                                  median       p95       p99");
    print_distribution("SendInput -> Reactor2 callback", &report.callback);
    print_distribution("callback + reconcile", &report.reconcile);
    print_distribution("SendInput -> reconcile complete", &report.end_to_end);
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

    let total = options.warmup + options.samples;
    let measurements = Arc::new(Measurements::new(total));
    let timeout = Duration::from_secs(30 + (options.text_size.max(1_024) * total) as u64 / 500_000);
    std::thread::spawn(move || {
        std::thread::sleep(timeout);
        eprintln!("Reactor2 live Notepad measurement timed out");
        std::process::exit(1);
    });

    App::run_with(move |app| create_harness(app, options, measurements))
}
