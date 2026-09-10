#![windows_subsystem = "console"]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use windows::Win32::{
    GetForegroundWindow, HWND, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    SendInput, SetForegroundWindow,
};
use windows_reactor::test::{
    LiveInputProbe, LiveInputProbeStage, clear_live_performance_times, schedule_live_input_probe,
    schedule_live_test_exit, take_live_performance_times,
};
use windows_reactor::*;

const F13: u16 = 0x7C;
const WAIT_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, PartialEq)]
struct Options {
    warmup: usize,
    samples: usize,
    batch: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            warmup: 200,
            samples: 2_000,
            batch: 256,
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
                "--batch" => &mut options.batch,
                "--help" | "-h" => {
                    println!(
                        "reactor-live-input\n\
                         \n\
                         Options:\n\
                           --warmup N   Sequential warmup keys (default: 200)\n\
                           --samples N  Sequential measured keys (default: 2000)\n\
                           --batch N    Keys in the throughput burst (default: 256)\n\
                           -h, --help   Print this help"
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
        if options.samples == 0 || options.batch == 0 {
            return Err("--samples and --batch must be greater than zero".to_string());
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
    raw: Stage,
    source: Stage,
    routed: Stage,
    component: Stage,
    completion: Condvar,
    completed: Mutex<usize>,
}

impl Measurements {
    fn new(capacity: usize) -> Self {
        Self {
            origin: Instant::now(),
            starts: (0..capacity).map(|_| AtomicU64::new(0)).collect(),
            raw: Stage::new(capacity),
            source: Stage::new(capacity),
            routed: Stage::new(capacity),
            component: Stage::new(capacity),
            completion: Condvar::new(),
            completed: Mutex::new(0),
        }
    }

    fn now(&self) -> u64 {
        self.origin.elapsed().as_nanos() as u64
    }

    fn mark(&self, stage: LiveInputProbeStage) {
        let timestamp = self.now();
        match stage {
            LiveInputProbeStage::RawWindowMessage => self.raw.mark(timestamp),
            LiveInputProbeStage::InputKeyboardSource => self.source.mark(timestamp),
        }
    }

    fn mark_component(&self) {
        self.component.mark(self.now());
        let mut completed = self.completed.lock().unwrap();
        *completed += 1;
        self.completion.notify_one();
    }

    fn wait_for_component(&self, expected: usize) -> Result<(), String> {
        let completed = self.completed.lock().unwrap();
        let (completed, timeout) = self
            .completion
            .wait_timeout_while(completed, WAIT_TIMEOUT, |completed| *completed < expected)
            .unwrap();
        if timeout.timed_out() {
            Err(format!(
                "timed out waiting for key {expected}; observed {completed} component updates"
            ))
        } else {
            Ok(())
        }
    }
}

struct Report {
    samples: usize,
    raw: Distribution,
    source: Distribution,
    routed: Distribution,
    reactor: Distribution,
    end_to_end: Distribution,
    batch: usize,
    batch_duration: Duration,
    dispatch: Option<RuntimeDistribution>,
    native_apply: Option<RuntimeDistribution>,
}

struct RuntimeDistribution {
    count: usize,
    distribution: Distribution,
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

fn percentile(values: &[u64], percentile: usize) -> u64 {
    values[(values.len() - 1) * percentile / 100]
}

enum Message {
    Focused(Result<(bool, isize), String>),
    Key,
    Report(Box<Result<Report, String>>),
}

struct LiveInput {
    options: Options,
    measurements: Arc<Measurements>,
    target: ElementRef<Border>,
    probe: Rc<RefCell<Option<LiveInputProbe>>>,
}

impl Component for LiveInput {
    type Input = Options;
    type Message = Message;

    fn create(options: &Options, _context: &ComponentContext<Self>) -> Self {
        let capacity = options.warmup + options.samples + options.batch;
        Self {
            options: *options,
            measurements: Arc::new(Measurements::new(capacity)),
            target: ElementRef::new(),
            probe: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Focused(Ok((true, hwnd))) => {
                clear_live_performance_times();
                let measurements = Arc::clone(&self.measurements);
                let options = self.options;
                context.spawn_background(move |_| {
                    Message::Report(Box::new(run_measurement(options, hwnd, measurements)))
                });
            }
            Message::Focused(Ok((false, _))) => {
                eprintln!("WinUI rejected the keyboard target focus request");
                schedule_live_test_exit(false).unwrap();
            }
            Message::Focused(Err(error)) => {
                eprintln!("{error}");
                schedule_live_test_exit(false).unwrap();
            }
            Message::Key => self.measurements.mark_component(),
            Message::Report(report) => match *report {
                Ok(mut report) => {
                    let (dispatch, native_apply) = take_live_performance_times();
                    report.dispatch = runtime_distribution(dispatch);
                    report.native_apply = runtime_distribution(native_apply);
                    print_report(&report);
                    schedule_live_test_exit(true).unwrap();
                }
                Err(error) => {
                    eprintln!("{error}");
                    schedule_live_test_exit(false).unwrap();
                }
            },
        }
    }

    fn view(&self, _options: &Options, context: &mut ViewContext<Self>) -> View {
        context.window_title("windows-reactor live input measurement");

        let probe = Rc::clone(&self.probe);
        let target = self.target.clone();
        let probe_measurements = Arc::clone(&self.measurements);
        let sender = context.sender();
        context.use_effect("setup", (), move || {
            let completion = sender.clone();
            let result = schedule_live_input_probe(
                {
                    let measurements = Arc::clone(&probe_measurements);
                    move |stage| measurements.mark(stage)
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
                                "keyboard target was not published".to_string(),
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
        Border::new()
            .width(320.0)
            .height(160.0)
            .is_tab_stop(true)
            .element_ref(&self.target)
            .on_preview_key_down(context.routed_callback(move |_| {
                measurements.routed.mark(measurements.now());
                RoutedMessage::handled(Message::Key)
            }))
            .content("Focused keyboard measurement target")
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

    let sequential = options.warmup + options.samples;
    for index in 0..sequential {
        measurements.starts[index].store(measurements.now(), Ordering::Release);
        inject_keys(1)?;
        measurements.wait_for_component(index + 1)?;
    }

    let batch_start = Instant::now();
    inject_keys(options.batch)?;
    let expected = sequential + options.batch;
    measurements.wait_for_component(expected)?;
    let batch_duration = batch_start.elapsed();

    for (name, stage) in [
        ("raw window message", &measurements.raw),
        ("InputKeyboardSource", &measurements.source),
        ("WinUI PreviewKeyDown", &measurements.routed),
        ("component update", &measurements.component),
    ] {
        let actual = stage.count.load(Ordering::Acquire);
        if actual != expected {
            return Err(format!(
                "{name} observed {actual} keys; expected {expected}; stage correlation is invalid"
            ));
        }
    }

    let range = options.warmup..sequential;

    Ok(Report {
        samples: options.samples,
        raw: Distribution::from(differences(
            range.clone(),
            "SendInput",
            "raw window message",
            &|index| measurements.starts[index].load(Ordering::Acquire),
            &|index| measurements.raw.get(index),
        )?),
        source: Distribution::from(differences(
            range.clone(),
            "raw window message",
            "InputKeyboardSource",
            &|index| measurements.raw.get(index),
            &|index| measurements.source.get(index),
        )?),
        routed: Distribution::from(differences(
            range.clone(),
            "raw window message",
            "WinUI PreviewKeyDown",
            &|index| measurements.raw.get(index),
            &|index| measurements.routed.get(index),
        )?),
        reactor: Distribution::from(differences(
            range.clone(),
            "WinUI PreviewKeyDown",
            "component update",
            &|index| measurements.routed.get(index),
            &|index| measurements.component.get(index),
        )?),
        end_to_end: Distribution::from(differences(
            range,
            "SendInput",
            "component update",
            &|index| measurements.starts[index].load(Ordering::Acquire),
            &|index| measurements.component.get(index),
        )?),
        batch: options.batch,
        batch_duration,
        dispatch: None,
        native_apply: None,
    })
}

fn runtime_distribution(values_us: Vec<f64>) -> Option<RuntimeDistribution> {
    if values_us.is_empty() {
        return None;
    }
    Some(RuntimeDistribution {
        count: values_us.len(),
        distribution: Distribution::from(
            values_us
                .into_iter()
                .map(|value| (value * 1_000.0) as u64)
                .collect(),
        ),
    })
}

fn differences(
    range: std::ops::Range<usize>,
    first_name: &str,
    second_name: &str,
    first: &dyn Fn(usize) -> u64,
    second: &dyn Fn(usize) -> u64,
) -> Result<Vec<u64>, String> {
    range
        .map(|index| {
            second(index).checked_sub(first(index)).ok_or_else(|| {
                format!(
                    "{second_name} preceded {first_name} for key {index}; stage correlation is invalid"
                )
            })
        })
        .collect()
}

fn inject_keys(count: usize) -> Result<(), String> {
    let down = INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: F13,
                ..Default::default()
            },
        },
    };
    let up = INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: F13,
                dwFlags: KEYEVENTF_KEYUP as u32,
                ..Default::default()
            },
        },
    };
    let inputs: Vec<_> = std::iter::repeat_n([down, up], count).flatten().collect();
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
    println!("samples: {}", report.samples);
    println!("stage                                  median       p95       p99");
    print_distribution("SendInput -> raw window message", &report.raw);
    print_distribution("raw -> InputKeyboardSource", &report.source);
    print_distribution("raw -> WinUI PreviewKeyDown", &report.routed);
    print_distribution("WinUI callback -> component update", &report.reactor);
    print_distribution("SendInput -> component update", &report.end_to_end);
    if let Some(dispatch) = &report.dispatch {
        print_distribution(
            &format!("Reactor dispatch (n={})", dispatch.count),
            &dispatch.distribution,
        );
    }
    if let Some(native_apply) = &report.native_apply {
        print_distribution(
            &format!("native apply (n={})", native_apply.count),
            &native_apply.distribution,
        );
    }
    println!(
        "throughput: {} keys in {:.3} ms ({:.0} keys/s)",
        report.batch,
        report.batch_duration.as_secs_f64() * 1_000.0,
        report.batch as f64 / report.batch_duration.as_secs_f64()
    );
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

    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("live input measurement timed out");
        std::process::exit(1);
    });

    App::run_component::<LiveInput>(options)
}
