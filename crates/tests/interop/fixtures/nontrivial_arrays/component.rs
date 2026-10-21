use super::*;
use NontrivialArrays::*;
use windows_core::imp::Type;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "NontrivialArrays.Widget").then(activatable::<Widget, NontrivialArrays::Widget>)
}

#[implement(NontrivialArrays::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn CreateValue(&self, number: i32) -> Result<IValue> {
        Ok(Value(number).into())
    }

    fn Strings(
        &self,
        input: &[HSTRING],
        fill: &mut [HSTRING],
        output: &mut Array<HSTRING>,
    ) -> Result<Array<HSTRING>> {
        exchange(input, fill, output)
    }

    fn Entries(
        &self,
        input: &[Entry],
        fill: &mut [Entry],
        output: &mut Array<Entry>,
    ) -> Result<Array<Entry>> {
        exchange(input, fill, output)
    }

    fn Values(
        &self,
        input: &[Option<IValue>],
        fill: &mut [Option<IValue>],
        output: &mut Array<IValue>,
    ) -> Result<Array<IValue>> {
        exchange(input, fill, output)
    }
}

fn exchange<T: Type<T>>(
    input: &[T::Default],
    fill: &mut [T::Default],
    output: &mut Array<T>,
) -> Result<Array<T>>
where
    T::Default: Clone,
{
    check(input.len() == fill.len())?;
    for (target, source) in fill.iter_mut().zip(input.iter().rev()) {
        target.clone_from(source);
    }
    *output = Array::from_slice(input);
    Ok(Array::from_slice(fill))
}

#[implement(IValue)]
struct Value(i32);

impl IValue_Impl for Value_Impl {
    fn Number(&self) -> Result<i32> {
        Ok(self.0)
    }
}
