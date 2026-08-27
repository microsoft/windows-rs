use crate::controls::*;
use windows_reactor::*;

pub struct InfoBarPage {
    visible: bool,
}

impl Component for InfoBarPage {
    type Message = bool;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { visible: true }
    }

    fn update(&mut self, visible: bool, _: &ComponentContext<Self>) {
        self.visible = visible;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let bar = |title, message, severity| {
            InfoBar::new()
                .title(title)
                .message(message)
                .severity(severity)
                .is_open(true)
        };
        page_content(
            "InfoBar",
            "A dismissible bar for app-level messages.",
            [
                KeyedView::new(
                    "toggle",
                    sample_card(
                        "Toggle InfoBar",
                        StackPanel::new().spacing(8.0).children((
                            ToggleSwitch::new()
                                .is_on(self.visible)
                                .on_toggled(context.forward())
                                .slot(ToggleSwitchSlot::Header, "Show InfoBar"),
                            InfoBar::new()
                                .title("Update available")
                                .message("A new version is ready to install.")
                                .severity(InfoBarSeverity::Informational)
                                .is_open(self.visible)
                                .on_closed(context.message(false)),
                        )),
                        "InfoBar::new().title(title).message(message).is_open(visible)",
                    ),
                ),
                KeyedView::new(
                    "severity",
                    sample_card(
                        "Severity Variants",
                        StackPanel::new().spacing(8.0).children((
                            bar("Success", "Operation completed.", InfoBarSeverity::Success),
                            bar("Warning", "Check your input.", InfoBarSeverity::Warning),
                            bar("Error", "Something went wrong.", InfoBarSeverity::Error),
                        )),
                        "InfoBar::new().severity(InfoBarSeverity::Warning)",
                    ),
                ),
            ],
        )
    }
}
