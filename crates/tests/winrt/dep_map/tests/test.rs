use windows::core::*;
use windows::Foundation::Collections::*;
use windows as Windows;

// This test ensures that all interfaces required by IMap can be both called and implemented
// simply by importing IMap. This consists of the following interfaces:
//   IMap
//   IMapView
//   IIterable
//   IIterator
//   KeyValuePair

#[implement(
    Windows::Foundation::Collections::IKeyValuePair<i32, f32>,
)]
struct KeyValuePair();

#[allow(non_snake_case)]
impl KeyValuePair {
    fn Key(&self) -> Result<i32> {
        Ok(0)
    }
    fn Value(&self) -> Result<f32> {
        Ok(0.0)
    }
}

#[implement(
    Windows::Foundation::Collections::IIterator<Windows::Foundation::Collections::IKeyValuePair<i32, f32>>,
)]
struct Iterator();

#[allow(non_snake_case)]
impl Iterator {
    fn GetMany(&self, _items: &mut [IKeyValuePair<i32, f32>]) -> Result<u32> {
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
    Windows::Foundation::Collections::IMapView<i32, f32>,
    Windows::Foundation::Collections::IIterable<Windows::Foundation::Collections::IKeyValuePair<i32, f32>>,
)]
struct MapView();

#[allow(non_snake_case)]
impl MapView {
    fn HasKey(&self, _key: i32) -> Result<bool> {
        Ok(true)
    }
    fn Lookup(&self, _key: i32) -> Result<f32> {
        Ok(0.0)
    }
    fn Split(&self, _first: &mut IMapView<i32, f32>, _second: &mut IMapView<i32, f32>) -> Result<()> {
        Ok(())
    }
    fn Size(&self) -> Result<u32> {
        Ok(0)
    }
    fn First(&self) -> Result<IIterator<IKeyValuePair<i32, f32>>> {
        Ok(Iterator().into())
    }
}

#[implement(
    Windows::Foundation::Collections::IMap<i32, f32>,
    Windows::Foundation::Collections::IIterable<Windows::Foundation::Collections::IKeyValuePair<i32, f32>>,
)]
struct Map();

#[allow(non_snake_case)]
impl Map {
    fn Clear(&self) -> Result<()> {
        Ok(())
    }
    fn GetView(&self) -> Result<IMapView<i32, f32>> {
        Ok(MapView().into())
    }
    fn HasKey(&self, _key: i32) -> Result<bool> {
        Ok(true)
    }
    fn Insert(&self, _key: i32, _value: f32) -> Result<bool> {
        Ok(true)
    }
    fn Lookup(&self, _key: i32) -> Result<f32> {
        Ok(0.0)
    }
    fn Remove(&self, _key: i32) -> Result<()> {
        Ok(())
    }
    fn Size(&self) -> Result<u32> {
        Ok(0)
    }
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
