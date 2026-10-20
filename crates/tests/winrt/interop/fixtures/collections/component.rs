use super::*;
use windows_collections::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Collections.Widget").then(activatable::<Widget, Collections::Widget>)
}

#[implement(Collections::Widget)]
#[derive(Default)]
struct Widget;

impl Collections::IWidget_Impl for Widget_Impl {
    fn Numbers(&self) -> Result<IVector<i32>> {
        Ok(IVector::from(vec![1, 2, 3]))
    }

    fn Lookup(&self, map: Ref<IMapView<HSTRING, i32>>, key: &HSTRING) -> Result<i32> {
        map.ok()?.Lookup(key)
    }

    fn Values(&self) -> Result<IIterable<HSTRING>> {
        Ok(IIterable::from(vec![
            HSTRING::from("one"),
            HSTRING::from("two"),
        ]))
    }

    fn Sum(&self, values: Ref<IIterable<i32>>) -> Result<i32> {
        Ok(values.ok()?.into_iter().sum())
    }

    fn Index(&self, values: Ref<IVectorView<HSTRING>>) -> Result<IMap<HSTRING, i32>> {
        let map: std::collections::BTreeMap<_, _> = values.ok()?.into_iter().zip(0..).collect();
        Ok(IMap::from(map))
    }
}
