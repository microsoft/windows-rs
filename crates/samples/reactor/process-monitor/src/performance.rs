use crate::model::{HISTORY_SECONDS, History, HistoryPoint, ProcessorId, SECOND};
use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::{
    Brush as CanvasBrush, ColorF, DrawingSession, Ellipse, GpuDevice, Invalidator, PathBuilder,
    Rect, Vector2,
};
use windows_reactor::*;
use windows_text::{TextFormat, Trimming, WordWrapping};

#[derive(Clone, Copy, Default, PartialEq)]
pub enum CpuView {
    #[default]
    Overall,
    Logical,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Metric {
    #[default]
    Cpu,
    Memory,
}

impl Metric {
    fn label(self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Memory => "Memory",
        }
    }

    fn value(self, point: &HistoryPoint) -> Option<f64> {
        match self {
            Self::Cpu => point.cpu,
            Self::Memory => point.memory,
        }
    }
}

pub fn page(
    history: &Rc<History>,
    metric: Metric,
    cpu_view: CpuView,
    color_scheme: ColorScheme,
    on_select: Callback<Metric>,
    on_cpu_view: Callback<CpuView>,
) -> View {
    let summary = |metric: Metric| {
        history
            .points
            .back()
            .and_then(|p| metric.value(p))
            .map_or_else(|| "--".into(), |value| format!("{value:.1}%"))
    };
    let mut description = match &history.latest {
        None => "System metrics are not available in this recording or have not arrived yet.".into(),
        Some(latest) => match metric {
            Metric::Cpu => latest.cpu_error.as_ref().map_or_else(
                || if latest.cpu_percent.is_some() {
                    "Total processor time across logical processors. Not frequency-adjusted utilization.".into()
                } else {
                    "Waiting for two CPU samples at least a second apart.".into()
                },
                |error| format!("CPU unavailable: {error}"),
            ),
            Metric::Memory => match &latest.memory {
                Some(memory) => format!(
                    "{:.1} GB in use  |  {:.1} GB available  |  {:.1} GB total physical",
                    (memory.total - memory.available) as f64 / 1_073_741_824.0,
                    memory.available as f64 / 1_073_741_824.0,
                    memory.total as f64 / 1_073_741_824.0,
                ),
                None => format!("Memory unavailable: {}", latest.memory_error.as_deref().unwrap()),
            },
        },
    };
    if metric == Metric::Cpu && cpu_view == CpuView::Logical {
        description = logical_description(history);
    }
    let choices: Vec<View> = [Metric::Cpu, Metric::Memory]
        .into_iter()
        .map(|choice| {
            let on_select = on_select.clone();
            Button::new()
                .automation_id(if choice == Metric::Cpu {
                    "metric-cpu"
                } else {
                    "metric-memory"
                })
                .automation_name(choice.label())
                .automation_help_text(format!(
                    "{}. {}",
                    summary(choice),
                    if choice == metric {
                        "Selected performance metric"
                    } else {
                        "Show history"
                    },
                ))
                .style(ButtonStyle::Subtle)
                .background(if choice == metric {
                    ThemeBrush::CardBackground
                } else {
                    ThemeBrush::SolidBackground
                })
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .horizontal_content_alignment(HorizontalAlignment::Left)
                .content(
                    StackPanel::new().margin(8.0).spacing(4.0).children((
                        TextBlock::new().text(choice.label()).font_size(18.0),
                        TextBlock::new()
                            .text(summary(choice))
                            .foreground(ThemeBrush::AccentText),
                    )),
                )
                .on_click(move || on_select.call(choice))
                .into()
        })
        .collect();
    let mut chart = vec![keyed(
        "surface",
        View::component::<Chart>(ChartInput {
            history: Rc::clone(history),
            metric,
            cpu_view,
            color_scheme,
        }),
    )];
    let no_history = if metric == Metric::Cpu && cpu_view == CpuView::Logical {
        history.logical_ids().all(|id| {
            history
                .points
                .iter()
                .all(|point| point.logical_cpu(id).is_none())
        })
    } else {
        history
            .points
            .iter()
            .all(|point| metric.value(point).is_none())
    };
    if no_history {
        chart.push(keyed(
            "empty",
            TextBlock::new()
                .automation_id("chart-empty")
                .text("No samples to display")
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        ));
    }
    let mut help = chart_help(history, metric, cpu_view);
    if metric == Metric::Cpu {
        help.push_str(" Right-click or press Shift+F10 to change the CPU graph.");
    }
    let chart = Border::new()
        .grid_row(2)
        .automation_id("history-chart")
        .automation_name(format!("{} history chart", metric.label()))
        .automation_help_text(help)
        .is_tab_stop(metric == Metric::Cpu)
        .content(Grid::new().keyed_children(chart));
    let chart: View = if metric == Metric::Cpu {
        chart.menu(Menu::new(
            [MenuItem::submenu(
                "view",
                "Change graph to",
                [
                    ("overall", "Overall utilization", CpuView::Overall),
                    ("logical", "Logical processors", CpuView::Logical),
                ]
                .map(|(key, label, view)| {
                    if cpu_view == view {
                        MenuItem::disabled(key, format!("{label} (current)"))
                    } else {
                        MenuItem::item(key, label)
                    }
                }),
            )],
            move |key: Key| match key.as_str() {
                Some("overall") => on_cpu_view.call(CpuView::Overall),
                Some("logical") => on_cpu_view.call(CpuView::Logical),
                _ => unreachable!(),
            },
        ))
    } else {
        chart.into()
    };
    Grid::new()
        .columns([GridLength::Pixel(150.0), GridLength::STAR])
        .children((
            StackPanel::new().spacing(8.0).children(choices),
            Grid::new()
                .grid_column(1)
                .margin(Thickness::new(24.0, 0.0, 0.0, 0.0))
                .rows([
                    GridLength::Auto,
                    GridLength::Auto,
                    GridLength::STAR,
                    GridLength::Auto,
                    GridLength::Auto,
                ])
                .children((
                    Grid::new()
                        .columns([GridLength::STAR, GridLength::Auto])
                        .children((
                            TextBlock::new().text(metric.label()).font_size(28.0),
                            TextBlock::new()
                                .automation_id("metric-value")
                                .grid_column(1)
                                .text(summary(metric))
                                .font_size(28.0),
                        )),
                    Grid::new()
                        .grid_row(1)
                        .margin(Thickness::xy(0.0, 12.0))
                        .columns([GridLength::STAR, GridLength::Auto])
                        .children((
                            TextBlock::new().text("Usage over 60 seconds"),
                            TextBlock::new().grid_column(1).text("100%"),
                        )),
                    chart,
                    Grid::new()
                        .grid_row(3)
                        .margin(Thickness::xy(0.0, 8.0))
                        .columns([GridLength::STAR, GridLength::Auto])
                        .children((
                            TextBlock::new().text("60 seconds ago").font_size(12.0),
                            TextBlock::new()
                                .grid_column(1)
                                .text("Latest sample")
                                .font_size(12.0),
                        )),
                    TextBlock::new()
                        .automation_id("metric-description")
                        .grid_row(4)
                        .margin(Thickness::new(0.0, 8.0, 0.0, 0.0))
                        .text(description)
                        .text_wrapping(TextWrapping::Wrap),
                )),
        ))
        .into()
}

fn logical_description(history: &History) -> String {
    let Some(latest) = &history.latest else {
        return "Logical processor data is not available in this recording.".into();
    };
    if let Some(error) = &latest.logical_cpu_error {
        return format!("Logical CPU unavailable: {error}");
    }
    let Some(cpus) = &latest.logical_cpus else {
        return "Waiting for logical processor data. Version-1 and version-2 recordings do not contain it.".into();
    };
    let errors: Vec<_> = cpus.iter().filter(|cpu| cpu.error.is_some()).collect();
    if let Some(first) = errors.first() {
        return format!(
            "{} counters unavailable. N{}:{}: {}",
            errors.len(),
            first.id.node,
            first.id.number,
            first.error.as_deref().unwrap()
        );
    }
    format!(
        "{} logical processors. Labels are NUMA node:number. Processor time, not frequency-adjusted utilization.",
        cpus.len()
    )
}

fn chart_help(history: &History, metric: Metric, cpu_view: CpuView) -> String {
    if metric != Metric::Cpu || cpu_view != CpuView::Logical {
        return "Missing observations leave gaps. Current values are available in the surrounding controls.".into();
    }
    let Some(latest) = &history.latest else {
        return logical_description(history);
    };
    let Some(cpus) = &latest.logical_cpus else {
        return logical_description(history);
    };
    cpus.iter()
        .map(|cpu| {
            let value = cpu.percent.map_or_else(
                || {
                    cpu.error
                        .clone()
                        .unwrap_or_else(|| "Waiting for samples".into())
                },
                |value| format!("{value:.1}%"),
            );
            format!("N{}:{}: {value}", cpu.id.node, cpu.id.number)
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[derive(Clone)]
struct ChartInput {
    history: Rc<History>,
    metric: Metric,
    cpu_view: CpuView,
    color_scheme: ColorScheme,
}

impl PartialEq for ChartInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.history, &other.history)
            && self.metric == other.metric
            && self.cpu_view == other.cpu_view
            && self.color_scheme == other.color_scheme
    }
}

struct Chart {
    input: Rc<RefCell<ChartInput>>,
    invalidator: Invalidator,
    surface: View,
}

impl Component for Chart {
    type Input = ChartInput;
    type Message = ();

    fn create(input: &ChartInput, _: &ComponentContext<Self>) -> Self {
        let input = Rc::new(RefCell::new(input.clone()));
        let draw_input = Rc::clone(&input);
        let invalidator = Invalidator::new();
        let label_format = TextFormat::new("Segoe UI", 12.0)
            .with_word_wrapping(WordWrapping::NoWrap)
            .with_trimming(Trimming::Character);
        // One host owns the device and swap chain; every plot uses its current drawing context.
        let surface = windows_canvas::Canvas::invalidated(&invalidator, move |ctx| {
            ctx.clear(ColorF::TRANSPARENT);
            let input = draw_input.borrow();
            if ctx.width > 4.0 && ctx.height > 4.0 {
                draw_chart(
                    ctx,
                    ctx.device(),
                    Rect::from_xywh(0.0, 0.0, ctx.width, ctx.height),
                    &input,
                    &label_format,
                )?;
            }
            Ok(())
        })
        .into();
        Self {
            input,
            invalidator,
            surface,
        }
    }

    fn input_changed(&mut self, input: &ChartInput, _: &ComponentContext<Self>) {
        *self.input.borrow_mut() = input.clone();
        self.invalidator.invalidate();
    }

    fn view(&self, _: &ChartInput, _: &mut ViewContext<Self>) -> View {
        self.surface.clone()
    }
}

fn draw_chart(
    session: &DrawingSession<'_>,
    device: &GpuDevice,
    rect: Rect,
    input: &ChartInput,
    label_format: &TextFormat,
) -> windows_canvas::Result<()> {
    let brushes = PlotBrushes::new(session, input.metric, input.color_scheme)?;
    if input.metric != Metric::Cpu || input.cpu_view == CpuView::Overall {
        return draw_plot(
            session,
            device,
            Rect::from_xywh(
                rect.left + 2.0,
                rect.top + 2.0,
                rect.width() - 4.0,
                rect.height() - 4.0,
            ),
            &input.history,
            input.metric,
            None,
            &brushes,
        );
    }
    let ids = input.history.logical_ids();
    let count = ids.len();
    if count == 0 {
        return Ok(());
    }
    let columns =
        ((count as f32 * rect.width() / rect.height()).sqrt().round() as usize).clamp(1, count);
    let rows = count.div_ceil(columns);
    let width = rect.width() / columns as f32;
    let height = rect.height() / rows as f32;
    for (index, id) in ids.enumerate() {
        let left = rect.left + (index % columns) as f32 * width;
        let top = rect.top + (index / columns) as f32 * height;
        let header = if width >= 48.0 && height >= 40.0 {
            20.0
        } else {
            0.0
        };
        if header > 0.0 {
            let mut text = format!("N{}:{}", id.node, id.number);
            if width >= 100.0 {
                let value = input
                    .history
                    .points
                    .back()
                    .and_then(|point| point.logical_cpu(id));
                text.push_str(
                    &value.map_or_else(|| "  --".into(), |value| format!("  {value:.0}%")),
                );
            }
            session.draw_text(
                &text,
                label_format,
                &Rect::from_xywh(left + 4.0, top, width - 8.0, header),
                &brushes.line,
            );
        }
        if width > 8.0 && height > header + 8.0 {
            draw_plot(
                session,
                device,
                Rect::from_xywh(
                    left + 4.0,
                    top + header + 2.0,
                    width - 8.0,
                    height - header - 8.0,
                ),
                &input.history,
                Metric::Cpu,
                Some(id),
                &brushes,
            )?;
        }
    }
    Ok(())
}

struct PlotBrushes {
    grid: CanvasBrush,
    line: CanvasBrush,
    fill: CanvasBrush,
}

impl PlotBrushes {
    fn new(
        session: &DrawingSession<'_>,
        metric: Metric,
        scheme: ColorScheme,
    ) -> windows_canvas::Result<Self> {
        let color = match (metric, scheme) {
            (Metric::Cpu, ColorScheme::Light) => ColorF::new(0.0, 0.38, 0.48, 1.0),
            (Metric::Cpu, ColorScheme::Dark) => ColorF::new(0.3, 0.78, 0.9, 1.0),
            (Metric::Memory, ColorScheme::Light) => ColorF::new(0.18, 0.32, 0.65, 1.0),
            (Metric::Memory, ColorScheme::Dark) => ColorF::new(0.48, 0.64, 1.0, 1.0),
        };
        let grid = match scheme {
            ColorScheme::Light => ColorF::new(0.0, 0.0, 0.0, 0.14),
            ColorScheme::Dark => ColorF::new(1.0, 1.0, 1.0, 0.16),
        };
        Ok(Self {
            grid: session.create_solid_brush(grid)?,
            line: session.create_solid_brush(color)?,
            fill: session.create_solid_brush(ColorF::new(color.r, color.g, color.b, 0.12))?,
        })
    }
}

fn draw_plot(
    session: &DrawingSession<'_>,
    device: &GpuDevice,
    rect: Rect,
    history: &History,
    metric: Metric,
    processor: Option<ProcessorId>,
    brushes: &PlotBrushes,
) -> windows_canvas::Result<()> {
    let columns = if processor.is_some() {
        (rect.width() / 40.0) as usize
    } else {
        12
    }
    .clamp(1, 12);
    let rows = if processor.is_some() {
        (rect.height() / 20.0) as usize
    } else {
        10
    }
    .clamp(1, 10);
    for column in 1..columns {
        let x = rect.left + rect.width() * column as f32 / columns as f32;
        session.draw_line(
            Vector2::new(x, rect.top),
            Vector2::new(x, rect.bottom),
            &brushes.grid,
            1.0,
        );
    }
    for row in 1..rows {
        let y = rect.top + rect.height() * row as f32 / rows as f32;
        session.draw_line(
            Vector2::new(rect.left, y),
            Vector2::new(rect.right, y),
            &brushes.grid,
            1.0,
        );
    }
    session.draw_rect(&rect, &brushes.grid, 1.0);
    let Some(latest) = history.points.back() else {
        return Ok(());
    };
    let points: Vec<_> = history
        .points
        .iter()
        .map(|point| {
            processor
                .map_or_else(|| metric.value(point), |id| point.logical_cpu(id))
                .map(|value| {
                    Vector2::new(
                        rect.right
                            - rect.width() * (latest.sampled_at - point.sampled_at) as f32
                                / (HISTORY_SECONDS * SECOND) as f32,
                        rect.bottom - rect.height() * value as f32 / 100.0,
                    )
                })
        })
        .collect();
    for run in points.split(Option::is_none).filter(|run| !run.is_empty()) {
        let first = run.first().unwrap().unwrap();
        if run.len() == 1 {
            session.fill_ellipse(&Ellipse::circle(first, 1.5), &brushes.line);
            continue;
        }
        let last = run.last().unwrap().unwrap();
        let area = PathBuilder::new(device)?.polygon(
            std::iter::once(Vector2::new(first.x, rect.bottom))
                .chain(run.iter().map(|p| p.unwrap()))
                .chain(std::iter::once(Vector2::new(last.x, rect.bottom))),
        )?;
        session.fill_path(&area, &brushes.fill);
        for pair in run.windows(2) {
            session.draw_line(pair[0].unwrap(), pair[1].unwrap(), &brushes.line, 1.5);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolated_samples_remain_visible_in_both_color_schemes_without_filling_gaps() {
        let device = GpuDevice::new_warp().unwrap();
        let target = device.create_render_target(200, 100).unwrap();
        let format = TextFormat::new("Segoe UI", 12.0);
        let mut history = History::default();
        for time in [0, 30, 60] {
            history.points.push_back(HistoryPoint {
                sampled_at: time * SECOND,
                cpu: (time == 30).then_some(50.0),
                memory: (time == 30).then_some(50.0),
                logical_cpus: Vec::new(),
            });
        }
        let history = Rc::new(history);
        for color_scheme in [ColorScheme::Light, ColorScheme::Dark] {
            let background = match color_scheme {
                ColorScheme::Light => ColorF::new(1.0, 1.0, 1.0, 1.0),
                ColorScheme::Dark => ColorF::new(0.125, 0.125, 0.125, 1.0),
            };
            for metric in [Metric::Cpu, Metric::Memory] {
                let input = ChartInput {
                    history: Rc::clone(&history),
                    metric,
                    cpu_view: CpuView::Overall,
                    color_scheme,
                };
                target
                    .draw(|session| {
                        session.clear(background);
                        draw_chart(
                            session,
                            &device,
                            Rect::from_xywh(0.0, 0.0, 200.0, 100.0),
                            &input,
                            &format,
                        )
                    })
                    .unwrap();
                let pixels = target.read_pixels().unwrap();
                let pixel = |x: usize, y: usize| &pixels[(y * 200 + x) * 4..(y * 200 + x) * 4 + 4];
                let background = pixel(6, 6);
                assert_eq!(pixel(87, 67), background);
                let luminance = |pixel: &[u8]| {
                    let linear = |channel: u8| {
                        let value = f64::from(channel) / 255.0;
                        if value <= 0.04045 {
                            value / 12.92
                        } else {
                            ((value + 0.055) / 1.055).powf(2.4)
                        }
                    };
                    0.2126 * linear(pixel[2])
                        + 0.7152 * linear(pixel[1])
                        + 0.0722 * linear(pixel[0])
                };
                let foreground = luminance(pixel(100, 50));
                let background = luminance(background);
                let contrast =
                    (foreground.max(background) + 0.05) / (foreground.min(background) + 0.05);
                assert!(contrast >= 4.5, "Sample contrast: {contrast}");
            }
        }
    }

    #[test]
    fn logical_grid_uses_node_identity_and_does_not_fill_missing_values() {
        let device = GpuDevice::new_warp().unwrap();
        let target = device.create_render_target(240, 120).unwrap();
        let format = TextFormat::new("Segoe UI", 11.0);
        let first = ProcessorId { node: 0, number: 0 };
        let second = ProcessorId { node: 1, number: 0 };
        let mut input = ChartInput {
            history: Rc::new(History::default()),
            metric: Metric::Cpu,
            cpu_view: CpuView::Logical,
            color_scheme: ColorScheme::Light,
        };
        for time in [0, 30, 60] {
            Rc::get_mut(&mut input.history)
                .unwrap()
                .points
                .push_back(HistoryPoint {
                    sampled_at: time * SECOND,
                    cpu: Some(50.0),
                    memory: None,
                    logical_cpus: vec![(first, Some(0.0)), (second, Some(100.0))],
                });
        }
        let render = |input: &ChartInput| {
            target
                .draw(|session| {
                    session.clear(ColorF::TRANSPARENT);
                    draw_chart(
                        session,
                        &device,
                        Rect::from_xywh(0.0, 0.0, 240.0, 120.0),
                        input,
                        &format,
                    )
                })
                .unwrap();
            target.read_pixels().unwrap()
        };
        let alpha = |pixels: &[u8], x: usize| pixels[(75 * 240 + x) * 4 + 3];
        let full = render(&input);
        assert_eq!(alpha(&full, 43), 0);
        assert!(alpha(&full, 163) > 0);
        Rc::get_mut(&mut input.history).unwrap().points[1].logical_cpus[1].1 = None;
        let gap = render(&input);
        assert_eq!(alpha(&gap, 43), 0);
        assert_eq!(alpha(&gap, 163), 0);
    }

    #[test]
    fn plots_share_one_device_and_surface_and_preserve_missing_samples() {
        let device = GpuDevice::new_warp().unwrap();
        let target = device.create_render_target(200, 100).unwrap();
        let mut history = History::default();
        for second in [0, 30, 60] {
            history.points.push_back(HistoryPoint {
                sampled_at: second * SECOND,
                cpu: Some(75.0),
                memory: Some(25.0),
                logical_cpus: Vec::new(),
            });
        }
        let render = |history: &History| {
            target
                .draw(|session| {
                    session.clear(ColorF::TRANSPARENT);
                    draw_plot(
                        session,
                        &device,
                        Rect::from_xywh(0.0, 0.0, 100.0, 100.0),
                        history,
                        Metric::Cpu,
                        None,
                        &PlotBrushes::new(session, Metric::Cpu, ColorScheme::Light)?,
                    )?;
                    draw_plot(
                        session,
                        &device,
                        Rect::from_xywh(100.0, 0.0, 100.0, 100.0),
                        history,
                        Metric::Memory,
                        None,
                        &PlotBrushes::new(session, Metric::Memory, ColorScheme::Light)?,
                    )
                })
                .unwrap();
            target.read_pixels().unwrap()
        };
        let alpha = |pixels: &[u8], x: usize| pixels[(85 * 200 + x) * 4 + 3];
        let filled = render(&history);
        assert!(alpha(&filled, 47) > 0);
        assert!(alpha(&filled, 147) > 0);
        history.points[1].cpu = None;
        let gap = render(&history);
        assert_eq!(alpha(&gap, 47), 0);
        assert!(alpha(&gap, 147) > 0);
    }
}
