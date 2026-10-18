use super::*;
use std::cell::Cell;
use std::time::Duration;

pub(super) struct RetirementFixture {
    target: ElementRef<Button>,
    callback: Option<Callback<()>>,
    released: Rc<Cell<bool>>,
    publication: Option<(bool, bool)>,
    selected: u32,
    calls: u32,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Remove,
    Selected(u32),
    Published(bool, bool),
}

struct ClickLifetime {
    sender: LocalSender<Message>,
    released: Rc<Cell<bool>>,
}

impl ClickLifetime {
    fn invoke(&self) {
        assert!(self.sender.send(Message::Selected(0)));
    }
}

impl Drop for ClickLifetime {
    fn drop(&mut self) {
        self.released.set(true);
    }
}

impl Component for RetirementFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        let released = Rc::new(Cell::new(false));
        let click = ClickLifetime {
            sender: context.sender(),
            released: Rc::clone(&released),
        };
        Self {
            target: ElementRef::new(),
            callback: Some(Callback::new(move |()| click.invoke())),
            released,
            publication: None,
            selected: 0,
            calls: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Remove => self.callback = None,
            Message::Selected(generation) => {
                self.selected = generation;
                self.calls += 1;
            }
            Message::Published(released, queued) => self.publication = Some((released, queued)),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let target = self.target.clone();
        let released = Rc::clone(&self.released);
        let sender = context.sender();
        context.use_effect("retirement", self.callback.is_none(), move || {
            // Snapshot logical retirement before returning to the native event loop.
            assert!(sender.send(Message::Published(released.get(), target.request_focus())));
            None
        });
        let child = if let Some(callback) = &self.callback {
            keyed(
                0u32,
                Button::new()
                    .automation_id("old")
                    .element_ref(&self.target)
                    .exit_fade(Duration::from_millis(500))
                    .content("Old action")
                    .on_click(callback.clone()),
            )
        } else {
            keyed(
                1u32,
                Button::new()
                    .automation_id("replacement")
                    .content("Replacement action")
                    .on_click(context.message(Message::Selected(1))),
            )
        };
        let publication = match self.publication {
            Some((released, queued)) => {
                format!("Callback released {released}; reference queued {queued}")
            }
            None => "Awaiting publication".to_owned(),
        };
        context.window_title("Reactor UI fixture: retirement");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 350.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                StackPanel::new().keyed_children([child]),
                Button::new()
                    .automation_id("remove")
                    .content("Remove fading child")
                    .on_click(context.message(Message::Remove)),
                TextBlock::new()
                    .automation_id("publication")
                    .text(publication),
                TextBlock::new()
                    .automation_id("selection")
                    .text(format!("Selected {}; calls {}", self.selected, self.calls)),
            ))
            .into()
    }
}
