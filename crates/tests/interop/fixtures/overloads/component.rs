use super::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Overloads.Widget").then(activatable::<Widget, Overloads::Widget>)
}

#[implement(Overloads::Widget, Overloads::IWidget2)]
#[derive(Default)]
struct Widget;

impl Overloads::IWidget_Impl for Widget_Impl {
    fn Sum(&self) -> Result<i32> {
        Ok(0)
    }

    fn Sum2(&self, a: i32) -> Result<i32> {
        Ok(a)
    }

    fn Format(&self, value: i32) -> Result<HSTRING> {
        Ok(format!("int {value}").into())
    }

    fn FormatText(&self, value: &HSTRING) -> Result<HSTRING> {
        Ok(format!("text {}", value.to_string_lossy()).into())
    }
}

impl Overloads::IWidget2_Impl for Widget_Impl {
    fn Sum(&self, a: i32, b: i32) -> Result<i32> {
        Ok(a + b)
    }

    fn Sum2(&self, a: i32, b: i32, c: i32) -> Result<i32> {
        Ok(a + b + c)
    }
}
