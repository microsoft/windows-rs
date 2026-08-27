use crate::controls::*;
use windows_reactor::*;

pub struct CalendarViewPage {
    today_highlighted: bool,
    changes: u32,
}

#[derive(Clone)]
pub enum Message {
    HighlightTodayToggled(bool),
    SelectionChanged,
}

impl Component for CalendarViewPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            today_highlighted: true,
            changes: 0,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::HighlightTodayToggled(value) => self.today_highlighted = value,
            Message::SelectionChanged => self.changes += 1,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "CalendarView",
            "A calendar display for selecting a date.",
            [
                KeyedView::new(
                    "basic-calendar-view",
                    sample_card(
                        "Basic CalendarView",
                        StackPanel::new().spacing(8.0).children((
                            ToggleSwitch::new()
                                .is_on(self.today_highlighted)
                                .on_toggled(context.callback(Message::HighlightTodayToggled))
                                .slots([SlotView::new(
                                    ToggleSwitchSlot::Header,
                                    "Highlight today",
                                )]),
                            CalendarView::new()
                                .is_today_highlighted(self.today_highlighted)
                                .on_selected_dates_changed(
                                    context.message(Message::SelectionChanged),
                                ),
                            TextBlock::new()
                                .text(format!("Selection changed {} time(s)", self.changes))
                                .opacity(0.6),
                        )),
                        "ToggleSwitch::new().on_toggled(...)\nCalendarView::new()\n    .is_today_highlighted(today_highlighted)\n    .on_selected_dates_changed(|| ...)",
                    ),
                ),
                KeyedView::new(
                    "calendar-view-no-labels",
                    sample_card(
                        "CalendarView without Labels",
                        CalendarView::new().is_group_label_visible(false),
                        "CalendarView::new().is_group_label_visible(false)",
                    ),
                ),
            ],
        )
    }
}
