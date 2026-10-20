use super::*;
use Enums::*;
use std::sync::Mutex;
use windows_collections::{IIterable, IVectorView};
use windows_reference::IReference;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Enums.Widget").then(activatable::<Widget, Enums::Widget>)
}

#[implement(Enums::Widget)]
struct Widget(Mutex<Styles>);

impl Default for Widget {
    fn default() -> Self {
        Self(Mutex::new(Styles::Bold | Styles::Underline))
    }
}

impl IWidget_Impl for Widget_Impl {
    fn Next(&self, color: Color) -> Result<Color> {
        Ok(match color {
            Color::Red => Color::Green,
            Color::Green => Color::Blue,
            Color::Blue => Color::Red,
            _ => Color::Unknown,
        })
    }

    fn Styles(&self) -> Result<Styles> {
        Ok(*self.0.lock().unwrap())
    }

    fn SetStyles(&self, value: Styles) -> Result<()> {
        *self.0.lock().unwrap() = value;
        Ok(())
    }

    fn Colors(&self) -> Result<IVectorView<Color>> {
        Ok(IVectorView::from(vec![
            Color::Red,
            Color::Green,
            Color::Blue,
        ]))
    }

    fn Combine(&self, styles: Ref<IIterable<Styles>>) -> Result<Styles> {
        Ok(styles
            .ok()?
            .into_iter()
            .fold(Styles::None, |all, style| all | style))
    }

    fn BoxColor(&self, color: Color) -> Result<IInspectable> {
        Ok(IReference::from(color).into())
    }

    fn UnboxStyles(&self, value: Ref<IInspectable>) -> Result<Styles> {
        value.ok()?.cast::<IReference<Styles>>()?.Value()
    }
}
