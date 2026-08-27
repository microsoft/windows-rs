use crate::controls::*;
use windows_reactor::*;

pub struct TitleBarPage;

impl Component for TitleBarPage {
    type Message = ();
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, _: &(), _: &mut ViewContext<Self>) -> View {
        page_content(
            "TitleBar",
            "A customizable application title bar.",
            [
                KeyedView::new(
                    "preview",
                    sample_card(
                        "Live TitleBar",
                        TextBlock::new().text(
                            "The gallery title bar above is the live control. Its buttons, search \
                             box, title, and subtitle are all declarative.",
                        ),
                        "TitleBar::new().title(title).subtitle(subtitle).slots(content)",
                    ),
                ),
                KeyedView::new(
                    "window",
                    sample_card(
                        "Preferred Height",
                        TextBlock::new().text(
                            "TitleBar automatically replaces the system title bar. Standard height \
                             is the default; the gallery requests tall system chrome.",
                        ),
                        "TitleBar::new().preferred_height(WindowTitleBarHeight::Tall)",
                    ),
                ),
            ],
        )
    }
}
