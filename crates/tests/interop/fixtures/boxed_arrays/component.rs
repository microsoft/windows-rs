use super::*;
use BoxedArrays::*;
use Windows::Foundation::{IReferenceArray, PropertyValue};

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "BoxedArrays.Widget").then(activatable::<Widget, BoxedArrays::Widget>)
}

#[implement(BoxedArrays::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn BoxNumbers(&self, values: &[i32]) -> Result<IInspectable> {
        PropertyValue::CreateInt32Array(values)
    }

    fn BoxBooleans(&self, values: &[bool]) -> Result<IInspectable> {
        PropertyValue::CreateBooleanArray(values)
    }

    fn BoxStrings(&self, values: &[HSTRING]) -> Result<IInspectable> {
        PropertyValue::CreateStringArray(values)
    }

    fn Numbers(&self, value: Ref<IInspectable>) -> Result<Array<i32>> {
        value.ok()?.cast::<IReferenceArray<i32>>()?.Value()
    }

    fn Booleans(&self, value: Ref<IInspectable>) -> Result<Array<bool>> {
        value.ok()?.cast::<IReferenceArray<bool>>()?.Value()
    }

    fn Strings(&self, value: Ref<IInspectable>) -> Result<Array<HSTRING>> {
        value.ok()?.cast::<IReferenceArray<HSTRING>>()?.Value()
    }
}
