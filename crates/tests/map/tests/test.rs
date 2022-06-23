use windows::core::*;
use windows::Foundation::Collections::*;

#[implement(
    IKeyValuePair<i32, f32>,
)]
struct KeyValuePair();

#[allow(non_snake_case)]
impl IKeyValuePair_Impl<i32, f32> for KeyValuePair {
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

#[allow(non_snake_case)]
impl IIterator_Impl<IKeyValuePair<i32, f32>> for Iterator {
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

#[allow(non_snake_case)]
impl IMapView_Impl<i32, f32> for MapView {
    // TODO: shouldn't require & for primtiive
    fn HasKey(&self, _key: &i32) -> Result<bool> {
        Ok(true)
    }
    fn Lookup(&self, _key: &i32) -> Result<f32> {
        Ok(0.0)
    }
    fn Split(&self, _first: &mut Option<IMapView<i32, f32>>, _second: &mut Option<IMapView<i32, f32>>) -> Result<()> {
        Ok(())
    }
    fn Size(&self) -> Result<u32> {
        Ok(0)
    }
}

#[allow(non_snake_case)]
impl IIterable_Impl<IKeyValuePair<i32, f32>> for MapView {
    fn First(&self) -> Result<IIterator<IKeyValuePair<i32, f32>>> {
        Ok(Iterator().into())
    }
}

#[implement(
    IMap<i32, f32>,
    IIterable<IKeyValuePair<i32, f32>>,
)]
struct Map();

#[allow(non_snake_case)]
impl IMap_Impl<i32, f32> for Map {
    fn Clear(&self) -> Result<()> {
        Ok(())
    }
    fn GetView(&self) -> Result<IMapView<i32, f32>> {
        Ok(MapView().into())
    }
    fn HasKey(&self, _key: &i32) -> Result<bool> {
        Ok(true)
    }
    fn Insert(&self, _key: &i32, _value: &f32) -> Result<bool> {
        Ok(true)
    }
    fn Lookup(&self, _key: &i32) -> Result<f32> {
        Ok(0.0)
    }
    fn Remove(&self, _key: &i32) -> Result<()> {
        Ok(())
    }
    fn Size(&self) -> Result<u32> {
        Ok(0)
    }
}

impl IIterable_Impl<IKeyValuePair<i32, f32>> for Map {
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
