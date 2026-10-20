use super::*;
use Arrays::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Arrays.Widget").then(activatable::<Widget, Arrays::Widget>)
}

#[implement(Arrays::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn Sum(&self, values: &[i32]) -> Result<i32> {
        Ok(values.iter().sum())
    }

    fn Join(&self, values: &[HSTRING]) -> Result<HSTRING> {
        let values: Vec<_> = values.iter().map(HSTRING::to_string_lossy).collect();
        Ok(values.join(",").into())
    }

    fn Fill(&self, values: &mut [i32]) -> Result<()> {
        for (index, value) in values.iter_mut().enumerate() {
            *value = index as i32 + 1;
        }

        Ok(())
    }

    fn Range(&self, count: i32) -> Result<Array<i32>> {
        Ok(Array::from_slice(&(1..=count).collect::<Vec<_>>()))
    }

    fn Split(&self, value: &HSTRING) -> Result<Array<HSTRING>> {
        let value = value.to_string_lossy();
        let values: Vec<HSTRING> = value.split(',').map(HSTRING::from).collect();
        Ok(Array::from_slice(&values))
    }

    fn RangeOut(&self, count: i32, values: &mut Array<i32>) -> Result<()> {
        *values = self.Range(count)?;
        Ok(())
    }
}
