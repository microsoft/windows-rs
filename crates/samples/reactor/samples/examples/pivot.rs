#![windows_subsystem = "windows"]

use windows_reactor::*;

struct PivotSample {
    selected: Option<usize>,
}

impl Component for PivotSample {
    type Message = Option<usize>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { selected: Some(0) }
    }

    fn update(&mut self, message: Option<usize>, _context: &ComponentContext<Self>) {
        self.selected = message;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let item = |header: &'static str, text: &'static str| {
            KeyedView::new(
                header,
                PivotItem::new().header(header).content(
                    Border::new()
                        .padding(Thickness::uniform(12.0))
                        .content(text),
                ),
            )
        };

        context.window_title("Pivot");
        StackPanel::new().spacing(8.0).children((
            Pivot::new()
                .title("Demo")
                .selected_index(self.selected)
                .on_selection_changed(context.callback(|index| index))
                .collection_slot(
                    PivotSlot::Items,
                    [
                        item("First", "Pivot - first tab"),
                        item("Second", "Pivot - second tab"),
                        item("Third", "Pivot - third tab"),
                    ],
                ),
            format!("selected_index = {:?}", self.selected),
        ))
    }
}

fn main() {
    App::run_component::<PivotSample>(()).unwrap();
}
