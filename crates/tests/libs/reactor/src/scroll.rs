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
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::View(metrics) => self.metrics = metrics,
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
            Message::Completed(name, result) => self.completions.push(match result {
                Ok(metrics) => format!("{name} {:.0}", metrics.vertical_offset),
                Err(error) => format!("{name} {error:?}"),
            }),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let rows = (0..ROWS).map(|index| {
            Border::new()
                .height(ROW_HEIGHT)
                .content(TextBlock::new().text(format!("Row {index}")))
        });
        let view = self.attached.then(|| {
            ScrollView::new()
                .automation_id("view")
                .height(200.0)
                .element_ref(&self.view)
                .content(StackPanel::new().children(rows.map(View::from).collect::<Vec<_>>()))
        });
        let metrics = self.metrics;
        context.window_title("Reactor UI fixture: scroll");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 500.0));
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
