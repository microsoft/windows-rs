use crate::controls::*;
use windows_reactor::*;

pub struct BreadcrumbBarPage {
    clicked: String,
}

impl Component for BreadcrumbBarPage {
    type Message = String;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            clicked: "No item clicked".to_string(),
        }
    }

    fn update(&mut self, item: String, _: &ComponentContext<Self>) {
        self.clicked = format!("Clicked: {item}");
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "BreadcrumbBar",
            "A trail showing the current navigation path.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic BreadcrumbBar",
                        StackPanel::new().spacing(8.0).children((
                            BreadcrumbBar::new()
                                .items_source(["Home", "Documents", "Report"])
                                .on_item_clicked(context.forward()),
                            TextBlock::new().text(self.clicked.clone()).opacity(0.6),
                        )),
                        r#"BreadcrumbBar::new()
    .items_source(["Home", "Documents", "Report"])
    .on_item_clicked(handler)"#,
                    ),
                ),
                KeyedView::new(
                    "deep",
                    sample_card(
                        "Deeper Path",
                        BreadcrumbBar::new()
                            .items_source(["Root", "Users", "Settings", "Profile"])
                            .on_item_clicked(context.forward()),
                        "BreadcrumbBar::new().items_source(path)",
                    ),
                ),
            ],
        )
    }
}
