use crate::controls::*;
use windows_reactor::*;

pub struct DropDownButtonPage {
    selected: String,
}

impl Component for DropDownButtonPage {
    type Message = String;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            selected: "(none)".to_string(),
        }
    }

    fn update(&mut self, selected: String, _: &ComponentContext<Self>) {
        self.selected = selected;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "DropDownButton",
            "A button that displays a flyout of choices.",
            [
                KeyedView::new(
                    "menu",
                    sample_card(
                        "DropDownButton with Menu",
                        StackPanel::new().spacing(8.0).children((
                            DropDownButton::new()
                                .content("File Actions")
                                .menu(Menu::new(
                                    [
                                        MenuItem::item("open", "Open"),
                                        MenuItem::item("save", "Save"),
                                        MenuItem::separator("separator"),
                                        MenuItem::item("exit", "Exit"),
                                    ],
                                    context.forward(),
                                )),
                            TextBlock::new()
                                .text(format!("Last action: {}", self.selected))
                                .opacity(0.6),
                        )),
                        "DropDownButton::new().content(label).menu(Menu::new(items, handler))",
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled DropDownButton",
                        DropDownButton::new().is_enabled(false).content("Disabled"),
                        "DropDownButton::new().is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}
