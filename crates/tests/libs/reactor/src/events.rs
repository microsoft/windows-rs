use super::*;

/// Logs events whose native handlers convert event arguments or are not raised by any other
/// fixture.
pub(super) struct EventsFixture {
    minimal: bool,
    mode: Option<NavigationViewDisplayMode>,
    events: Vec<String>,
}

#[derive(Clone)]
pub(super) enum Message {
    Event(&'static str),
    Chosen(Rc<str>),
    Time(Option<TimeSpan>),
    Mode(NavigationViewDisplayMode),
    Minimal,
}

impl Component for EventsFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            minimal: false,
            mode: None,
            events: Vec::new(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Event(name) => self.events.push(name.to_string()),
            Message::Chosen(item) => self.events.push(format!("chosen {item}")),
            Message::Time(time) => self.events.push(format!(
                "time {}",
                if time.is_some() { "set" } else { "none" }
            )),
            Message::Mode(mode) => self.mode = Some(mode),
            Message::Minimal => self.minimal = true,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().client_size(900.0, 800.0));
        let event = |name| context.message(Message::Event(name));
        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((
                TitleBar::new()
                    .title("Reactor UI fixture: events")
                    .is_pane_toggle_button_visible(true)
                    .on_pane_toggle_requested(event("pane-toggle")),
                StackPanel::new()
                    .grid_row(1)
                    .margin(20.0)
                    .spacing(8.0)
                    .children((
                        TextBlock::new()
                            .automation_id("events")
                            .text(format!("Events [{}]", self.events.join(", "))),
                        TextBlock::new()
                            .automation_id("mode")
                            .text(format!("Mode {:?}", self.mode)),
                        Border::new()
                            .horizontal_alignment(HorizontalAlignment::Left)
                            .on_double_tapped(event("double"))
                            .content(
                                Button::new()
                                    .automation_id("double-target")
                                    .content("Double-click"),
                            ),
                        Button::new()
                            .automation_id("minimal")
                            .content("Minimal")
                            .on_click(context.message(Message::Minimal)),
                        AutoSuggestBox::new()
                            .automation_id("suggest")
                            .width(200.0)
                            .items_source(["Alpha", "Beta"])
                            .on_suggestion_chosen(context.callback(Message::Chosen)),
                        TimePicker::new()
                            .automation_id("time")
                            .on_time_changed(context.callback(Message::Time)),
                        TabView::new()
                            .height(40.0)
                            .is_add_tab_button_visible(true)
                            .on_add_tab_button_click(event("add-tab"))
                            .tab_items((TabViewItem::new().header("Tab"),)),
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(8.0)
                            .children((
                                CalendarView::new()
                                    .automation_id("calendar")
                                    .on_selected_dates_changed(event("dates")),
                                NavigationView::new()
                                    .width(300.0)
                                    .height(300.0)
                                    .is_settings_visible(false)
                                    .pane_display_mode(if self.minimal {
                                        NavigationViewPaneDisplayMode::LeftMinimal
                                    } else {
                                        NavigationViewPaneDisplayMode::Left
                                    })
                                    .menu_items((NavigationViewItem::new().content("Item"),))
                                    .on_display_mode_changed(context.callback(Message::Mode)),
                            )),
                    )),
            ))
            .into()
    }
}
