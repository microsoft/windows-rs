use crate::controls::*;
use windows_reactor::*;

pub struct RadioButtonPage {
    selected: i32,
}

impl Component for RadioButtonPage {
    type Message = i32;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { selected: 0 }
    }

    fn update(&mut self, selected: i32, _: &ComponentContext<Self>) {
        self.selected = selected;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let options = ["Option A", "Option B", "Option C"];
        let label = usize::try_from(self.selected)
            .ok()
            .and_then(|index| options.get(index))
            .unwrap_or(&"?");
        page_content(
            "RadioButton",
            "Select one option from a group.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic RadioButtons",
                        StackPanel::new().spacing(8.0).children((
                            RadioButtons::new()
                                .items_source(options)
                                .selected_index(self.selected)
                                .on_selection_changed(context.forward())
                                .slot(RadioButtonsSlot::Header, "Pick one"),
                            TextBlock::new()
                                .text(format!("Selected: {label}"))
                                .opacity(0.6),
                        )),
                        "RadioButtons::new().items_source(options).selected_index(index)",
                    ),
                ),
                KeyedView::new(
                    "sizes",
                    sample_card(
                        "RadioButtons with Header",
                        RadioButtons::new()
                            .items_source(["Small", "Medium", "Large", "Extra Large"])
                            .selected_index(1)
                            .slot(RadioButtonsSlot::Header, "T-shirt size"),
                        "RadioButtons::new().items_source(sizes).selected_index(1)",
                    ),
                ),
            ],
        )
    }
}
