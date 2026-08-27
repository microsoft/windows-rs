use crate::controls::*;
use windows_reactor::*;

pub struct ComboBoxPage {
    selected: i32,
}

impl Component for ComboBoxPage {
    type Message = i32;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { selected: -1 }
    }

    fn update(&mut self, selected: i32, _: &ComponentContext<Self>) {
        self.selected = selected;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let colors = ["Red", "Green", "Blue", "Yellow"];
        let label = usize::try_from(self.selected)
            .ok()
            .and_then(|index| colors.get(index))
            .unwrap_or(&"(none)");
        page_content(
            "ComboBox",
            "A drop-down list of items a user can select from.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic ComboBox",
                        StackPanel::new().spacing(8.0).children((
                            ComboBox::new()
                                .items_source(colors)
                                .selected_index(self.selected)
                                .placeholder_text("Pick a color")
                                .on_selection_changed(context.callback(std::convert::identity))
                                .slots([SlotView::new(ComboBoxSlot::Header, "Color")]),
                            TextBlock::new()
                                .text(format!("Selected: {label}"))
                                .opacity(0.6),
                        )),
                        "ComboBox::new().items_source(colors).on_selection_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "editable",
                    sample_card(
                        "Editable ComboBox",
                        ComboBox::new()
                            .items_source(["Cat", "Dog", "Fox"])
                            .placeholder_text("Type or pick")
                            .is_editable(true)
                            .slots([SlotView::new(ComboBoxSlot::Header, "Animal")]),
                        "ComboBox::new().items_source(items).is_editable(true)",
                    ),
                ),
            ],
        )
    }
}
