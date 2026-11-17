use super::*;

const ROWS: usize = 50;
const ROW_HEIGHT: f64 = 40.0;

pub(super) struct ScrollFixture {
    view: ElementRef<ScrollView>,
    empty: ElementRef<ScrollView>,
    show_empty: bool,
    _observation: ElementObservation,
    metrics: ScrollViewMetrics,
    attached: bool,
    completions: Vec<String>,
    remove_active: bool,
    active_started: bool,
    resized: bool,
    grown: bool,
    #[cfg(windows_cast_diagnostics)]
    detached_delegates: Option<usize>,
}

#[derive(Clone)]
pub(super) enum Message {
    View(ScrollViewMetrics),
    Jump,
    End,
    Interrupt,
    Remove,
    ShowEmpty,
    Empty,
    RemoveActive,
    Attach,
    Resize,
    Grow,
    Completed(&'static str, Result<ScrollViewMetrics, IntegrationError>),
}

fn scroll(
    view: &ElementRef<ScrollView>,
    name: &'static str,
    offset: f64,
    animate: bool,
    context: &ComponentContext<ScrollFixture>,
) {
    let completed = context.callback(move |result| Message::Completed(name, result));
    assert!(view.request_scroll_to(0.0, offset, animate, move |result| completed.call(result)));
}

pub(super) struct ScrollLayoutFixture {
    view: ElementRef<ScrollView>,
    _observation: ElementObservation,
    metrics: ScrollViewMetrics,
    orientation: Option<ScrollingContentOrientation>,
    anchor: Option<f64>,
    cells: usize,
    completions: usize,
    completed: String,
}

#[derive(Clone)]
pub(super) enum LayoutMessage {
    View(ScrollViewMetrics),
    Orientation(Option<ScrollingContentOrientation>),
    Anchor(Option<f64>),
    Grow,
    Shrink,
    Scroll(f64),
    Completed(Result<ScrollViewMetrics, IntegrationError>),
}

impl Component for ScrollLayoutFixture {
    type Input = bool;
    type Message = LayoutMessage;

    fn create(_: &bool, context: &ComponentContext<Self>) -> Self {
        let view = ElementRef::new();
        let observed = context.callback(LayoutMessage::View);
        let observation = view.observe_view(move |metrics| observed.call(metrics));
        Self {
            view,
            _observation: observation,
            metrics: ScrollViewMetrics::default(),
            orientation: Some(ScrollingContentOrientation::Both),
            anchor: Some(1.0),
            cells: 10,
            completions: 0,
            completed: String::new(),
        }
    }

    fn update(&mut self, message: LayoutMessage, context: &ComponentContext<Self>) {
        match message {
            LayoutMessage::View(metrics) => self.metrics = metrics,
            LayoutMessage::Orientation(value) => self.orientation = value,
            LayoutMessage::Anchor(value) => self.anchor = value,
            LayoutMessage::Grow => self.cells += 2,
            LayoutMessage::Shrink => self.cells -= 2,
            LayoutMessage::Scroll(offset) => {
                let completed = context.callback(LayoutMessage::Completed);
                assert!(
                    self.view
                        .request_scroll_to(offset, offset, false, move |result| {
                            completed.call(result);
                        })
                );
            }
            LayoutMessage::Completed(result) => {
                let metrics = result.unwrap();
                self.completions += 1;
                self.completed = format!(
                    "Completed {}: {:.0},{:.0}",
                    self.completions, metrics.horizontal_offset, metrics.vertical_offset
                );
            }
        }
    }

    fn view(&self, image: &bool, context: &mut ViewContext<Self>) -> View {
        let rows: Vec<View> = (0..self.cells)
            .map(|_| {
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children(
                        (0..self.cells)
                            .map(|_| Border::new().width(40.0).height(40.0).into())
                            .collect::<Vec<View>>(),
                    )
                    .into()
            })
            .collect();
        let content: View = if *image {
            Image::new()
                .source_data(EncodedImage::from_static(image::PNG))
                .stretch(Stretch::Uniform)
                .into()
        } else {
            StackPanel::new().children(rows).into()
        };
        let mut view = ScrollView::new()
            .width(if *image { 300.0 } else { 200.0 })
            .height(200.0)
            .element_ref(&self.view)
            .content(content);
        if let Some(orientation) = self.orientation {
            view = view.content_orientation(orientation);
        }
        if let Some(anchor) = self.anchor {
            view = view
                .horizontal_anchor_ratio(anchor)
                .vertical_anchor_ratio(anchor);
        }
        let orientation_buttons: [View; 5] = [
            ("both", Some(ScrollingContentOrientation::Both)),
            ("horizontal", Some(ScrollingContentOrientation::Horizontal)),
            ("vertical", Some(ScrollingContentOrientation::Vertical)),
            ("neither", Some(ScrollingContentOrientation::None)),
            ("default-orientation", None),
        ]
        .map(|(id, value)| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(LayoutMessage::Orientation(value)))
                .into()
        });
        let anchor_buttons: [View; 3] = [
            ("far", Some(1.0)),
            ("disabled", Some(f64::NAN)),
            ("default-anchor", None),
        ]
        .map(|(id, value)| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(LayoutMessage::Anchor(value)))
                .into()
        });
        let actions: [View; 4] = [
            ("end", LayoutMessage::Scroll(1e6)),
            ("middle", LayoutMessage::Scroll(100.0)),
            ("grow", LayoutMessage::Grow),
            ("shrink", LayoutMessage::Shrink),
        ]
        .map(|(id, message)| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(message))
                .into()
        });
        let metrics = self.metrics;
        context.window_title(if *image {
            "Reactor UI fixture: scroll-orientation"
        } else {
            "Reactor UI fixture: scroll-layout"
        });
        context.window_visuals(WindowVisuals::new().client_size(700.0, 500.0));
        StackPanel::new()
            .spacing(8.0)
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children(Vec::from(orientation_buttons)),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children(Vec::from(anchor_buttons)),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children(Vec::from(actions)),
                TextBlock::new()
                    .automation_id("anchor")
                    .text(format!("{:?}", self.anchor)),
                TextBlock::new().automation_id("metrics").text(format!(
                    "Offset {:.0},{:.0}; viewport {:.0},{:.0}; extent {:.0},{:.0}",
                    metrics.horizontal_offset,
                    metrics.vertical_offset,
                    metrics.viewport_width,
                    metrics.viewport_height,
                    metrics.extent_width,
                    metrics.extent_height,
                )),
                TextBlock::new()
                    .automation_id("completed")
                    .text(&self.completed),
                view,
            ))
            .into()
    }
}

impl Component for ScrollFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        let view = ElementRef::new();
        let observed = context.callback(Message::View);
        let observation = view.observe_view(move |metrics| observed.call(metrics));
        Self {
            view,
            empty: ElementRef::new(),
            show_empty: false,
            _observation: observation,
            metrics: ScrollViewMetrics::default(),
            attached: true,
            completions: Vec::new(),
            remove_active: false,
            active_started: false,
            resized: false,
            grown: false,
            #[cfg(windows_cast_diagnostics)]
            detached_delegates: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::View(metrics) => {
                self.metrics = metrics;
                if self.remove_active
                    && metrics.vertical_offset > 0.0
                    && metrics.vertical_offset < metrics.extent_height - metrics.viewport_height
                {
                    self.active_started = true;
                    self.remove_active = false;
                    self.attached = false;
                }
            }
            Message::Jump => scroll(&self.view, "jump", 500.0, false, context),
            Message::End => scroll(&self.view, "end", 1e6, false, context),
            Message::Interrupt => {
                scroll(&self.view, "animated", 1e6, true, context);
                scroll(&self.view, "interrupt", 300.0, false, context);
            }
            Message::Remove => {
                scroll(&self.view, "removed", 0.0, false, context);
                self.attached = false;
            }
            Message::ShowEmpty => self.show_empty = true,
            Message::Empty => {
                scroll(&self.empty, "empty 1", 100.0, false, context);
                scroll(&self.empty, "empty 2", 100.0, false, context);
            }
            Message::RemoveActive => {
                self.remove_active = true;
                let count = std::cell::Cell::new(0);
                let completed = context.callback(|result| Message::Completed("active", result));
                assert!(
                    self.view
                        .request_scroll_to(0.0, 1800.0, true, move |result| {
                            assert_eq!(count.replace(count.get() + 1), 0);
                            completed.call(result);
                        })
                );
            }
            Message::Attach => {
                self.active_started = false;
                self.attached = true;
            }
            Message::Resize => self.resized = true,
            Message::Grow => self.grown = true,
            Message::Completed(name, result) => {
                #[cfg(windows_cast_diagnostics)]
                if name == "active" {
                    let delegates = windows_core::imp::live_objects().delegates;
                    if let Some(previous) = self.detached_delegates.replace(delegates) {
                        assert_eq!(delegates, previous, "scroll subscriptions survived removal");
                    }
                }
                self.completions.push(match result {
                    Ok(metrics) => format!("{name} {:.0}", metrics.vertical_offset),
                    Err(error) => format!("{name} {error:?}"),
                });
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let rows = (0..if self.grown { ROWS + 10 } else { ROWS }).map(|index| {
            Border::new()
                .height(ROW_HEIGHT)
                .content(TextBlock::new().text(format!("Row {index}")))
        });
        let view = self.attached.then(|| {
            ScrollView::new()
                .automation_id("view")
                .height(if self.resized { 240.0 } else { 200.0 })
                .element_ref(&self.view)
                .content(StackPanel::new().children(rows.map(View::from).collect::<Vec<_>>()))
        });
        let metrics = self.metrics;
        context.window_title("Reactor UI fixture: scroll");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 700.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                Button::new()
                    .automation_id("jump")
                    .content("Jump")
                    .on_click(context.message(Message::Jump)),
                Button::new()
                    .automation_id("end")
                    .content("End")
                    .on_click(context.message(Message::End)),
                Button::new()
                    .automation_id("interrupt")
                    .content("Interrupt")
                    .on_click(context.message(Message::Interrupt)),
                Button::new()
                    .automation_id("remove")
                    .content("Remove")
                    .on_click(context.message(Message::Remove)),
                Button::new()
                    .automation_id("empty")
                    .content("Empty")
                    .on_click(context.message(Message::Empty)),
                Button::new()
                    .automation_id("show-empty")
                    .content("Show empty")
                    .on_click(context.message(Message::ShowEmpty)),
                Button::new()
                    .automation_id("remove-active")
                    .content("Remove active")
                    .on_click(context.message(Message::RemoveActive)),
                Button::new()
                    .automation_id("attach")
                    .content("Attach")
                    .on_click(context.message(Message::Attach)),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children((
                        Button::new()
                            .automation_id("resize")
                            .content("Resize")
                            .on_click(context.message(Message::Resize)),
                        Button::new()
                            .automation_id("grow")
                            .content("Grow")
                            .on_click(context.message(Message::Grow)),
                    )),
                TextBlock::new()
                    .automation_id("active-started")
                    .text(self.active_started.to_string()),
                TextBlock::new().automation_id("metrics").text(format!(
                    "Offset {:.0}; viewport {:.0}; extent {:.0}",
                    metrics.vertical_offset, metrics.viewport_height, metrics.extent_height
                )),
                TextBlock::new()
                    .automation_id("completions")
                    .text(format!("Completed {}", self.completions.join(", "))),
                StackPanel::new().children(
                    view.into_iter()
                        .chain(
                            self.show_empty
                                .then(|| ScrollView::new().height(50.0).element_ref(&self.empty)),
                        )
                        .map(View::from)
                        .collect::<Vec<_>>(),
                ),
            ))
            .into()
    }
}

pub(super) struct ScrollPrependFixture {
    view: ElementRef<ScrollView>,
    _observation: ElementObservation,
    metrics: ScrollViewMetrics,
    rows: Vec<u32>,
}

#[derive(Clone)]
pub(super) enum PrependMessage {
    View(ScrollViewMetrics),
    Scroll(f64),
    Prepend,
}

impl Component for ScrollPrependFixture {
    type Input = ();
    type Message = PrependMessage;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        let view = ElementRef::new();
        let observed = context.callback(PrependMessage::View);
        let observation = view.observe_view(move |metrics| observed.call(metrics));
        Self {
            view,
            _observation: observation,
            metrics: ScrollViewMetrics::default(),
            rows: (1000..1050).collect(),
        }
    }

    fn update(&mut self, message: PrependMessage, _: &ComponentContext<Self>) {
        match message {
            PrependMessage::View(metrics) => self.metrics = metrics,
            PrependMessage::Scroll(offset) => {
                assert!(self.view.request_scroll_to(0.0, offset, false, |_| {}));
            }
            PrependMessage::Prepend => {
                let first = self.rows[0];
                self.rows.splice(0..0, first - 5..first);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let rows = self.rows.iter().fold(ItemsRepeater::new(), |list, &id| {
            list.item(
                id,
                Border::new().height(ROW_HEIGHT).content(
                    TextBlock::new()
                        .automation_id(format!("row-{id}"))
                        .text(format!("Row {id}")),
                ),
            )
        });
        let metrics = self.metrics;
        let first = self.rows[(metrics.vertical_offset / ROW_HEIGHT) as usize];
        context.window_title("Reactor UI fixture: scroll-prepend");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 500.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children((
                        Button::new()
                            .automation_id("top")
                            .content("Top")
                            .on_click(context.message(PrependMessage::Scroll(0.0))),
                        Button::new()
                            .automation_id("middle")
                            .content("Middle")
                            .on_click(context.message(PrependMessage::Scroll(400.0))),
                        Button::new()
                            .automation_id("prepend")
                            .content("Prepend")
                            .on_click(context.message(PrependMessage::Prepend)),
                    )),
                TextBlock::new().automation_id("metrics").text(format!(
                    "Offset {:.0}; extent {:.0}; first Row {first}",
                    metrics.vertical_offset, metrics.extent_height
                )),
                ScrollView::new()
                    .height(200.0)
                    .element_ref(&self.view)
                    .content(rows),
            ))
            .into()
    }
}
