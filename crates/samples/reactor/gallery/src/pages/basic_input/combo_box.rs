use crate::controls::*;
use windows_reactor::*;

pub struct ComboBoxPage {
    selected: Option<usize>,
}

impl Component for ComboBoxPage {
    type Message = Option<usize>;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { selected: None }
    }

    fn update(&mut self, selected: Option<usize>, _: &ComponentContext<Self>) {
        self.selected = selected;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let colors = ["Red", "Green", "Blue", "Yellow"];
        let label = self
            .selected
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
                                .on_selection_changed(context.forward())
                                .slot(ComboBoxSlot::Header, "Color"),
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
                            .slot(ComboBoxSlot::Header, "Animal"),
                        "ComboBox::new().items_source(items).is_editable(true)",
                    ),
                ),
            ],
        )
    }
}
