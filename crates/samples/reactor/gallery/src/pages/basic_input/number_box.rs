use crate::controls::*;
use windows_reactor::*;

pub struct NumberBoxPage {
    value: f64,
    clamped: f64,
}

#[derive(Clone)]
pub enum Message {
    Value(f64),
    Clamped(f64),
}

impl Component for NumberBoxPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            value: 42.0,
            clamped: 5.0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Value(value) => self.value = value,
            Message::Clamped(value) => self.clamped = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "NumberBox",
            "A text control for entering numeric values with validation.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic NumberBox",
                        StackPanel::new().spacing(8.0).children((
                            NumberBox::new()
                                .value(self.value)
                                .on_value_changed(context.callback(Message::Value))
                                .slots([SlotView::new(
                                    NumberBoxSlot::Header,
                                    TextBlock::new().text("Quantity"),
                                )]),
                            TextBlock::new().text(format!("Value: {}", self.value)),
                        )),
                        "NumberBox::new().value(value).on_value_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "range",
                    sample_card(
                        "NumberBox with Range",
                        StackPanel::new().spacing(8.0).children((
                            NumberBox::new()
                                .minimum(1.0)
                                .maximum(10.0)
                                .value(self.clamped)
                                .on_value_changed(context.callback(Message::Clamped)),
                            TextBlock::new()
                                .text(format!("Clamped value: {}", self.clamped))
                                .opacity(0.6),
                        )),
                        "NumberBox::new().minimum(1.0).maximum(10.0).value(value)",
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled NumberBox",
                        NumberBox::new().value(99.0).is_enabled(false),
                        "NumberBox::new().value(99.0).is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}
