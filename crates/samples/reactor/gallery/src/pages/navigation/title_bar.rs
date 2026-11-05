use crate::controls::*;
use windows_reactor::*;

pub fn title_bar_page() -> View {
    page_content(
        "TitleBar",
        "A customizable application title bar.",
        [
            KeyedView::new(
                "preview",
                sample_card(
                    "Live TitleBar",
                    "The gallery title bar above is the live control. Its buttons, search box, \
                         title, and subtitle are all declarative.",
                ),
            ),
            KeyedView::new(
                "window",
                sample_card(
                    "Preferred Height",
                    "TitleBar automatically replaces the system title bar. Standard height is \
                         the default; the gallery requests tall system chrome.",
                ),
            ),
        ],
    )
}
