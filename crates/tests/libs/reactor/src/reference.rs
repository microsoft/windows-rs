use super::*;

pub(super) struct ReferenceFixture {
    target: ElementRef<TextBox>,
    generation: u32,
    attached: bool,
    queued: bool,
    completions: u32,
    result: Option<Result<bool, FocusError>>,
}

#[derive(Clone)]
pub(super) enum Message {
    Focus,
    Replace,
    Remove,
    Attach,
    Completed(Result<bool, FocusError>),
}

impl ReferenceFixture {
    fn request_focus(&mut self, context: &ComponentContext<Self>) {
        self.result = None;
        let completed = context.callback(Message::Completed);
        self.queued = self
            .target
            .request_focus_result(move |result| completed.call(result));
    }
}

impl Component for ReferenceFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            target: ElementRef::new(),
            generation: 0,
            attached: true,
            queued: false,
            completions: 0,
            result: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Focus => self.request_focus(context),
            Message::Replace => {
                // Retire the queued request's binding in the same component update.
                self.request_focus(context);
                self.generation += 1;
            }
            Message::Remove => {
                self.request_focus(context);
                self.attached = false;
            }
            Message::Attach => {
                self.generation += 1;
                self.attached = true;
            }
            Message::Completed(result) => {
                self.completions += 1;
                self.result = Some(result);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let generation = self.generation;
        let target = self.attached.then(|| {
            keyed(
                generation,
                TextBox::new()
                    .automation_id("target")
                    .element_ref(&self.target),
            )
        });
        context.window_title("Reactor UI fixture: reference");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 480.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                Button::new()
                    .automation_id("focus")
                    .content("Request focus")
                    .on_click(context.message(Message::Focus)),
                Button::new()
                    .automation_id("replace")
                    .content("Queue focus and replace")
                    .on_click(context.message(Message::Replace)),
                Button::new()
                    .automation_id("remove")
                    .content("Queue focus and remove")
                    .on_click(context.message(Message::Remove)),
                Button::new()
                    .automation_id("attach")
                    .content("Attach target")
                    .on_click(context.message(Message::Attach)),
                StackPanel::new().keyed_children(target),
                TextBlock::new().automation_id("state").text(format!(
                    "Generation {}; attached {}",
                    self.generation, self.attached
                )),
                TextBlock::new().automation_id("result").text(format!(
                    "Queued {}; completions {}; result {:?}",
                    self.queued, self.completions, self.result
                )),
            ))
            .into()
    }
}
