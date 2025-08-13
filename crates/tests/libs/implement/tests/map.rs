use windows::core::*;
use windows_collections::*;

#[implement(
    IKeyValuePair<i32, f32>,
)]
struct KeyValuePair();

impl IKeyValuePair_Impl<i32, f32> for KeyValuePair_Impl {
    fn Key(&self) -> Result<i32> {
        Ok(0)
    }
    fn Value(&self) -> Result<f32> {
        Ok(0.0)
    }
}

#[implement(
    IIterator<IKeyValuePair<i32, f32>>,
)]
struct Iterator();

impl IIterator_Impl<IKeyValuePair<i32, f32>> for Iterator_Impl {
    fn GetMany(&self, _items: &mut [Option<IKeyValuePair<i32, f32>>]) -> Result<u32> {
        Ok(0)
    }
    fn MoveNext(&self) -> Result<bool> {
        Ok(true)
    }
    fn Current(&self) -> Result<IKeyValuePair<i32, f32>> {
        Ok(KeyValuePair().into())
    }
    fn HasCurrent(&self) -> Result<bool> {
        Ok(true)
    }
}

#[implement(
    IMapView<i32, f32>,
    IIterable<IKeyValuePair<i32, f32>>,
)]
struct MapView();

impl IMapView_Impl<i32, f32> for MapView_Impl {
    // TODO: shouldn't require `Ref` for primitive
    fn HasKey(&self, _key: Ref<i32>) -> Result<bool> {
        Ok(true)
    }
    fn Lookup(&self, _key: Ref<i32>) -> Result<f32> {
        Ok(0.0)
    }
    fn Split(
        &self,
        _first: OutRef<IMapView<i32, f32>>,
        _second: OutRef<IMapView<i32, f32>>,
    ) -> Result<()> {
        Ok(())
    }
    fn Size(&self) -> Result<u32> {
        Ok(0)
    }
}

impl IIterable_Impl<IKeyValuePair<i32, f32>> for MapView_Impl {
    fn First(&self) -> Result<IIterator<IKeyValuePair<i32, f32>>> {
        Ok(Iterator().into())
    }
}

#[implement(
    IMap<i32, f32>,
    IIterable<IKeyValuePair<i32, f32>>,
)]
struct Map();

impl IMap_Impl<i32, f32> for Map_Impl {
    fn Clear(&self) -> Result<()> {
        Ok(())
    }
    fn GetView(&self) -> Result<IMapView<i32, f32>> {
        Ok(MapView().into())
    }
    fn HasKey(&self, _key: Ref<i32>) -> Result<bool> {
        Ok(true)
    }
    fn Insert(&self, _key: Ref<i32>, _value: Ref<f32>) -> Result<bool> {
        Ok(true)
    }
    fn Lookup(&self, _key: Ref<i32>) -> Result<f32> {
        Ok(0.0)
    }
    fn Remove(&self, _key: Ref<i32>) -> Result<()> {
        Ok(())
    }
    fn Size(&self) -> Result<u32> {
        Ok(0)
    }
}

impl IIterable_Impl<IKeyValuePair<i32, f32>> for Map_Impl {
    fn First(&self) -> Result<IIterator<IKeyValuePair<i32, f32>>> {
        Ok(Iterator().into())
    }
}

fn main() -> Result<()> {
    let map: IMap<i32, f32> = Map().into();
    map.Clear()?;
    map.HasKey(0)?;
    map.Insert(0, 0.0)?;
    map.Lookup(0)?;
    map.Remove(0)?;
    map.Size()?;
    map.First()?;

    let view = map.GetView()?;
    view.HasKey(0)?;
    view.Lookup(0)?;
    view.Split(&mut None, &mut None)?;
    view.Size()?;

    let iterator = view.First()?;
    iterator.GetMany(&mut [None])?;
    iterator.MoveNext()?;
    iterator.Current()?;
    iterator.HasCurrent()?;

    Ok(())
}
