#![windows_subsystem = "windows"]

use windows_reactor::*;

struct NumberBoxSample {
    quantity: Option<f64>,
}

impl Component for NumberBoxSample {
    type Message = Option<f64>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            quantity: Some(3.0),
        }
    }

    fn update(&mut self, quantity: Self::Message, _context: &ComponentContext<Self>) {
        self.quantity = quantity;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("NumberBox");
        let quantity = self
            .quantity
            .map_or_else(|| "(empty)".to_string(), |value| format!("{value:.0}"));
        StackPanel::new().max_width(320.0).spacing(8.0).children((
            NumberBox::new()
                .minimum(0.0)
                .maximum(10.0)
                .value(self.quantity)
                .on_value_changed(context.callback(|value| value))
                .slot(NumberBoxSlot::Header, "Quantity"),
            format!("Quantity = {quantity}"),
            NumberBox::new()
                .value(42.0)
                .is_enabled(false)
                .slot(NumberBoxSlot::Header, "Disabled"),
        ))
    }
}

fn main() {
    App::run_component::<NumberBoxSample>(()).unwrap();
}
