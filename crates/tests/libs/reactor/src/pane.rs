use super::*;

/// A SplitView and a NavigationView that declare `is_pane_open` from component state and adopt
/// user changes until `reject` is pressed.
pub(super) struct PaneFixture {
    split: bool,
    navigation: bool,
    adopt: bool,
    events: Vec<String>,
}

#[derive(Clone)]
pub(super) enum Message {
    Split(bool),
    Navigation(bool),
    SplitClosed(bool),
    NavigationChanged(bool),
    Reject,
}

impl Component for PaneFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            split: false,
            navigation: true,
            adopt: true,
            events: Vec::new(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Split(open) => self.split = open,
            Message::Navigation(open) => self.navigation = open,
            Message::SplitClosed(open) => {
                self.events.push(format!("Split {open}"));
                if self.adopt {
                    self.split = open;
                }
            }
            Message::NavigationChanged(open) => {
                self.events.push(format!("Navigation {open}"));
                if self.adopt {
                    self.navigation = open;
                }
            }
            Message::Reject => self.adopt = false,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: pane");
        context.window_visuals(WindowVisuals::new().client_size(800.0, 600.0));
        let button = |id: &str, message| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(message))
        };
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        button("open-split", Message::Split(true)),
                        button("close-split", Message::Split(false)),
                        button("open-navigation", Message::Navigation(true)),
                        button("close-navigation", Message::Navigation(false)),
                        button("reject", Message::Reject),
                    )),
                TextBlock::new().automation_id("state").text(format!(
                    "Split {}; navigation {}",
                    self.split, self.navigation
                )),
                TextBlock::new()
                    .automation_id("events")
                    .text(format!("Events [{}]", self.events.join(", "))),
                SplitView::new()
                    .height(150.0)
                    .display_mode(SplitViewDisplayMode::Overlay)
                    .is_pane_open(self.split)
                    .pane(Button::new().automation_id("split-pane").content("Pane"))
                    .content(TextBlock::new().text("Content"))
                    .on_pane_closed(context.callback(Message::SplitClosed)),
                NavigationView::new()
                    .height(250.0)
                    .pane_display_mode(NavigationViewPaneDisplayMode::Left)
                    .is_settings_visible(false)
                    .is_pane_open(self.navigation)
                    .keyed_menu_items([keyed(
                        "item",
                        NavigationViewItem::new()
                            .automation_id("navigation-item")
                            .content("Item"),
                    )])
                    .on_is_pane_open_changed(context.callback(Message::NavigationChanged)),
            ))
            .into()
    }
}
