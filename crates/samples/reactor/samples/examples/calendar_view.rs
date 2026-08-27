use windows_reactor::*;

struct CalendarViewSample {
    changes: u32,
}

impl Component for CalendarViewSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { changes: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.changes += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("CalendarView");
        StackPanel::new().spacing(8.0).children((
            CalendarView::new()
                .is_today_highlighted(true)
                .is_group_label_visible(true)
                .on_selected_dates_changed(context.message(())),
            format!("Selection changed {} time(s)", self.changes),
        ))
    }
}

fn main() {
    App::run_component::<CalendarViewSample>(()).unwrap();
}
