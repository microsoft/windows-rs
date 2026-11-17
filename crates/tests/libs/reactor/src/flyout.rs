use super::*;

pub(super) struct FlyoutFixture {
    rich: bool,
    attached: bool,
    generation: u32,
    selected: u32,
    calls: u32,
}

#[derive(Clone)]
pub(super) enum Message {
    Rich,
    Replace,
    Remove,
    Selected(u32),
}

impl Component for FlyoutFixture {
    type Input = bool;
    type Message = Message;

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self {
            rich: false,
            attached: true,
            generation: 0,
            selected: 0,
            calls: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Rich => self.rich = true,
            Message::Replace => self.generation += 1,
            Message::Remove => self.attached = false,
            Message::Selected(generation) => {
                self.selected = generation;
                self.calls += 1;
            }
        }
    }

    fn view(&self, drop_down: &bool, context: &mut ViewContext<Self>) -> View {
        let generation = self.generation;
        let owner = self.attached.then(|| {
            let content = StackPanel::new().spacing(8.0).children((
                Button::new()
                    .automation_id("action")
                    .content(format!("Action {generation}"))
                    .on_click(context.message(Message::Selected(generation))),
                Button::new()
                    .automation_id("replace")
                    .content("Replace content")
                    .on_click(context.message(Message::Replace)),
                Button::new()
                    .automation_id("remove")
                    .content("Remove owner")
                    .on_click(context.message(Message::Remove)),
            ));
            let flyout =
                Flyout::rich(StackPanel::new().keyed_children([keyed(generation, content)]))
                    .placement(FlyoutPlacement::Bottom);
            // Starts with a menu so the switch to a rich flyout reuses the owner's
            // `Button.Flyout` property.
            let menu = Menu::new(
                [MenuItem::item("rich", "Use flyout")],
                context.callback(|_: Key| Message::Rich),
            );
            let owner: View = match (*drop_down, self.rich) {
                (true, true) => DropDownButton::new()
                    .automation_id("open")
                    .content("Open flyout")
                    .flyout_with(flyout),
                (true, false) => DropDownButton::new()
                    .automation_id("open")
                    .content("Open flyout")
                    .menu(menu),
                (false, true) => Button::new()
                    .automation_id("open")
                    .content("Open flyout")
                    .flyout_with(flyout),
                (false, false) => Button::new()
                    .automation_id("open")
                    .content("Open flyout")
                    .menu(menu),
            };
            keyed("owner", owner)
        });
        context.window_title(if *drop_down {
            "Reactor UI fixture: flyout-dropdown"
        } else {
            "Reactor UI fixture: flyout"
        });
        context.window_visuals(WindowVisuals::new().client_size(500.0, 400.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                StackPanel::new().keyed_children(owner),
                TextBlock::new()
                    .automation_id("selection")
                    .text(format!("Selected {}; calls {}", self.selected, self.calls)),
            ))
            .into()
    }
}
