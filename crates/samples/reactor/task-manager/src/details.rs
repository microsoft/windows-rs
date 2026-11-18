use crate::inspect::{self, Counters, Identity};
use crate::model::{Process, ProcessId};
use std::rc::Rc;
use windows_reactor::*;

#[derive(Clone, PartialEq)]
pub struct DetailInput {
    pub process: Option<Rc<Process>>,
    pub live: bool,
    /// Committed snapshot count, so each snapshot refreshes counters even when list values match.
    pub snapshot: usize,
}

pub enum Message {
    Identity(ProcessId, Result<Identity, String>),
    Counters(ProcessId, Result<Counters, String>),
}

pub struct Details {
    id: Option<ProcessId>,
    identity: Option<Result<Identity, String>>,
    counters: Option<Result<Counters, String>>,
    previous: Option<Counters>,
    identity_task: Option<ComponentTask>,
    counters_task: Option<ComponentTask>,
}

impl Details {
    fn load(&mut self, input: &DetailInput, context: &ComponentContext<Self>) {
        for task in [self.identity_task.take(), self.counters_task.take()]
            .into_iter()
            .flatten()
        {
            task.cancel();
        }
        self.id = input.process.as_ref().map(|p| p.id);
        self.identity = None;
        self.counters = None;
        self.previous = None;
        if let Some(id) = self.id.filter(|_| input.live) {
            self.identity_task = Some(
                context.spawn_background(move |_| Message::Identity(id, inspect::identity(id))),
            );
            self.refresh(context);
        }
    }

    fn refresh(&mut self, context: &ComponentContext<Self>) {
        if let Some(id) = self.id
            && self.counters_task.is_none()
        {
            self.counters_task = Some(
                context.spawn_background(move |_| Message::Counters(id, inspect::counters(id))),
            );
        }
    }

    fn identity_section(&self) -> View {
        let rows = match &self.identity {
            None | Some(Err(_)) => vec![("Status", "Loading...".to_string())],
            Some(Ok(identity)) => {
                let version = identity.version.as_ref();
                vec![
                    ("Path", text(identity.path.as_ref())),
                    (
                        "Description",
                        version
                            .and_then(|v| v.description.clone())
                            .unwrap_or_else(|| "--".into()),
                    ),
                    (
                        "Company",
                        version
                            .and_then(|v| v.company.clone())
                            .unwrap_or_else(|| "--".into()),
                    ),
                    (
                        "File version",
                        version
                            .and_then(|v| v.file_version.clone())
                            .unwrap_or_else(|| "--".into()),
                    ),
                    ("User", text(identity.user.as_ref())),
                    (
                        "Elevated",
                        text(
                            identity
                                .elevated
                                .as_ref()
                                .map(|&e| if e { "Yes" } else { "No" }),
                        ),
                    ),
                    ("Architecture", text(identity.architecture.as_ref())),
                    ("Started", identity.started.clone()),
                ]
            }
        };
        section("Image", rows)
    }

    fn counter_sections(&self, process: &Process) -> Vec<View> {
        let counters = match &self.counters {
            None => return vec![],
            Some(Err(error)) => {
                return vec![section(
                    "Counters",
                    vec![("Status", format!("Unavailable: {error}"))],
                )];
            }
            Some(Ok(counters)) => counters,
        };
        let rate = |current: u64, previous: fn(&Counters) -> u64| {
            self.previous.as_ref().map_or_else(
                || "--".to_string(),
                |previous_counters| {
                    let elapsed = counters
                        .sampled
                        .duration_since(previous_counters.sampled)
                        .as_nanos() as f64
                        / 100.0;
                    let delta = current.saturating_sub(previous(previous_counters)) as f64;
                    format!(
                        "{:.1}%",
                        delta * 100.0 / (elapsed * f64::from(counters.processors.max(1)))
                    )
                },
            )
        };
        let mut sections = vec![section(
            "CPU",
            vec![
                ("Usage", cpu(process)),
                ("Kernel usage", rate(counters.kernel, |c| c.kernel)),
                ("User usage", rate(counters.user, |c| c.user)),
                ("Kernel time", duration(counters.kernel)),
                ("User time", duration(counters.user)),
                ("Priority", text(counters.priority.as_ref())),
                ("Threads", count(u64::from(process.threads))),
            ],
        )];
        sections.push(section(
            "Memory",
            match &counters.memory {
                Ok(memory) => vec![
                    ("Working set", bytes(memory.WorkingSetSize as u64)),
                    ("Peak working set", bytes(memory.PeakWorkingSetSize as u64)),
                    ("Private bytes", bytes(memory.PrivateUsage as u64)),
                    ("Peak private bytes", bytes(memory.PeakPagefileUsage as u64)),
                    ("Page faults", count(u64::from(memory.PageFaultCount))),
                    ("Paged pool", bytes(memory.QuotaPagedPoolUsage as u64)),
                    ("Nonpaged pool", bytes(memory.QuotaNonPagedPoolUsage as u64)),
                ],
                Err(error) => vec![("Status", format!("Unavailable: {error}"))],
            },
        ));
        sections.push(section(
            "I/O",
            match &counters.io {
                Ok(io) => vec![
                    ("Reads", count(io.ReadOperationCount)),
                    ("Read bytes", bytes(io.ReadTransferCount)),
                    ("Writes", count(io.WriteOperationCount)),
                    ("Write bytes", bytes(io.WriteTransferCount)),
                    ("Other", count(io.OtherOperationCount)),
                    ("Other bytes", bytes(io.OtherTransferCount)),
                ],
                Err(error) => vec![("Status", format!("Unavailable: {error}"))],
            },
        ));
        sections.push(section(
            "Handles",
            vec![
                (
                    "Handles",
                    text(counters.handles.as_ref().map(|&h| count(u64::from(h)))),
                ),
                (
                    "GDI objects",
                    text(counters.gdi_objects.as_ref().map(|&n| count(u64::from(n)))),
                ),
                (
                    "USER objects",
                    text(counters.user_objects.as_ref().map(|&n| count(u64::from(n)))),
                ),
            ],
        ));
        sections
    }
}

impl Component for Details {
    type Input = DetailInput;
    type Message = Message;

    fn create(input: &DetailInput, context: &ComponentContext<Self>) -> Self {
        let mut details = Self {
            id: None,
            identity: None,
            counters: None,
            previous: None,
            identity_task: None,
            counters_task: None,
        };
        details.load(input, context);
        details
    }

    fn input_changed(&mut self, input: &DetailInput, context: &ComponentContext<Self>) {
        if self.id != input.process.as_ref().map(|p| p.id) {
            self.load(input, context);
        } else if input.live {
            // Each new snapshot refreshes the counters, so pausing updates also pauses them.
            self.refresh(context);
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Identity(id, result) if self.id == Some(id) => {
                self.identity_task = None;
                self.identity = Some(result);
            }
            Message::Counters(id, result) if self.id == Some(id) => {
                self.counters_task = None;
                if let Some(Ok(previous)) = self.counters.take() {
                    self.previous = Some(previous);
                }
                self.counters = Some(result);
            }
            _ => {}
        }
    }

    fn view(&self, input: &DetailInput, _: &mut ViewContext<Self>) -> View {
        let Some(process) = &input.process else {
            return TextBlock::new()
                .text("Double-click a process on the Processes page to see its details.")
                .into();
        };
        let mut header = vec![
            TextBlock::new()
                .automation_id("details-name")
                .text(&process.name)
                .font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .is_text_selection_enabled(true)
                .into(),
            TextBlock::new()
                .opacity(0.7)
                .text(format!(
                    "PID {}  Parent PID {}",
                    process.id.pid, process.parent_pid
                ))
                .is_text_selection_enabled(true)
                .into(),
        ];
        if let Some(reason) = &process.unavailable {
            header.push(
                TextBlock::new()
                    .text(reason)
                    .text_wrapping(TextWrapping::Wrap)
                    .into(),
            );
        }
        let unavailable = if !input.live {
            Some("Live details are not queried during replay.".to_string())
        } else if let Some(Err(error)) = &self.identity {
            Some(format!("Unavailable: {error}"))
        } else {
            None
        };
        let (left, right): (Vec<View>, Vec<View>) = if let Some(status) = unavailable {
            (
                vec![section(
                    "Snapshot",
                    vec![
                        ("Status", status),
                        ("CPU usage", cpu(process)),
                        (
                            "Working set",
                            process.working_set.map_or_else(|| "--".into(), bytes),
                        ),
                        (
                            "Private bytes",
                            process.private_bytes.map_or_else(|| "--".into(), bytes),
                        ),
                        ("Threads", count(u64::from(process.threads))),
                    ],
                )],
                vec![],
            )
        } else {
            let mut counters = self.counter_sections(process).into_iter();
            let mut left = vec![self.identity_section()];
            left.extend(counters.next());
            (left, counters.collect())
        };
        ScrollViewer::new()
            .content(
                StackPanel::new()
                    .spacing(24.0)
                    .margin(Thickness::new(0.0, 0.0, 16.0, 16.0))
                    .children((
                        StackPanel::new().spacing(4.0).children(header),
                        Grid::new()
                            .columns([GridLength::STAR, GridLength::STAR])
                            .column_spacing(32.0)
                            .children((
                                StackPanel::new().spacing(24.0).children(left),
                                StackPanel::new()
                                    .grid_column(1)
                                    .spacing(24.0)
                                    .children(right),
                            )),
                    )),
            )
            .into()
    }
}

fn section(title: &str, rows: Vec<(&str, String)>) -> View {
    let mut children: Vec<View> = vec![
        TextBlock::new()
            .text(title)
            .font_weight(FontWeight::SEMI_BOLD)
            .grid_column_span(2)
            .into(),
    ];
    let count = rows.len();
    for (index, (label, value)) in rows.into_iter().enumerate() {
        let row = index as i32 + 1;
        children.push(
            TextBlock::new()
                .grid_row(row)
                .opacity(0.7)
                .text(label)
                .into(),
        );
        children.push(
            TextBlock::new()
                .grid_row(row)
                .grid_column(1)
                .automation_id(format!("detail-{}", label.to_lowercase().replace(' ', "-")))
                .text(value)
                .text_wrapping(TextWrapping::Wrap)
                .is_text_selection_enabled(true)
                .into(),
        );
    }
    Grid::new()
        .columns([GridLength::Pixel(140.0), GridLength::STAR])
        .rows(std::iter::repeat_n(GridLength::Auto, count + 1))
        .row_spacing(6.0)
        .children(children)
        .into()
}

fn cpu(process: &Process) -> String {
    process
        .cpu_tenths
        .map_or_else(|| "--".into(), |v| format!("{:.1}%", f64::from(v) / 10.0))
}

fn text<T: std::fmt::Display>(value: Result<T, &String>) -> String {
    match value {
        Ok(value) => value.to_string(),
        Err(error) => format!("Unavailable: {error}"),
    }
}

fn count(value: u64) -> String {
    let digits = value.to_string();
    let mut result = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            result.push(',');
        }
        result.push(digit);
    }
    result
}

fn bytes(value: u64) -> String {
    const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
    let mut scaled = value as f64;
    let mut unit = "bytes";
    for next in UNITS {
        if scaled < 1024.0 {
            break;
        }
        scaled /= 1024.0;
        unit = next;
    }
    if unit == "bytes" {
        format!("{value} bytes")
    } else {
        format!("{scaled:.1} {unit}")
    }
}

/// Formats 100-nanosecond ticks as `h:mm:ss.ss`.
fn duration(ticks: u64) -> String {
    let hundredths = ticks / 100_000;
    let seconds = hundredths / 100;
    format!(
        "{}:{:02}:{:02}.{:02}",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60,
        hundredths % 100
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting() {
        assert_eq!(count(0), "0");
        assert_eq!(count(999), "999");
        assert_eq!(count(1_000), "1,000");
        assert_eq!(count(1_234_567), "1,234,567");
        assert_eq!(bytes(512), "512 bytes");
        assert_eq!(bytes(1536), "1.5 KB");
        assert_eq!(bytes(3 * 1024 * 1024 * 1024), "3.0 GB");
        assert_eq!(duration(0), "0:00:00.00");
        assert_eq!(duration(36_610_000_000 + 100_000), "1:01:01.01");
    }
}
