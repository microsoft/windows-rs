#![cfg(windows)]

use std::collections::BTreeMap;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::sync::{Arc, atomic::*};
use std::thread::{self, ThreadId};
use test_collections::*;
use windows_collections::*;
use windows_core::*;
use windows_reference::IReference;

const E_NOTIMPL: HRESULT = HRESULT(0x80004001_u32 as _);
const E_BOUNDS: HRESULT = HRESULT(0x8000000B_u32 as _);

struct Com;

impl Com {
    fn new(mode: u32) -> Result<Self> {
        unsafe { CoInitializeEx(None, mode).ok()? };
        Ok(Self)
    }
}

impl Drop for Com {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

type MakeVector = fn(Vec<Option<IInspectable>>) -> Result<IVector<IInspectable>>;
// The shared cases use stock value boxes or implementations with thread-safe state.
const VECTORS: [MakeVector; 4] = [
    |values| Ok(unsafe { IVector::from_unchecked(values) }),
    IVector::from_marshaled,
    IVector::from_read_only,
    |values| unsafe { IVector::from_read_only_unchecked(values) },
];

type MakeIterable = fn(Vec<Option<IInspectable>>) -> Result<IIterable<IInspectable>>;
const ITERABLES: [MakeIterable; 4] = [
    |values| Ok(unsafe { IIterable::from_unchecked(values) }),
    IIterable::from_marshaled,
    |values| unsafe { IVectorView::<IInspectable>::from_unchecked(values) }.cast(),
    |values| IVectorView::from_marshaled(values)?.cast(),
];

type MakeCollection = fn(Vec<Option<IInspectable>>) -> Result<IInspectable>;
const MARSHAL_FACTORIES: [MakeCollection; 8] = [
    |values| IIterable::<IInspectable>::from_marshaled(values).map(Into::into),
    |values| IVectorView::<IInspectable>::from_marshaled(values).map(Into::into),
    |values| IVector::<IInspectable>::from_marshaled(values).map(Into::into),
    |values| IObservableVector::<IInspectable>::from_marshaled(values).map(Into::into),
    |values| IMapView::<u32, IInspectable>::from_marshaled(keyed(values)).map(Into::into),
    |values| IMap::<u32, IInspectable>::from_marshaled(keyed(values)).map(Into::into),
    |values| IObservableMap::<u32, IInspectable>::from_marshaled(keyed(values)).map(Into::into),
    |values| IVector::from_read_only(values).map(Into::into),
];

// Used only with thread-safe implementations or within one apartment for the complete lifetime.
const DIRECT_FACTORIES: [MakeCollection; 8] = [
    |values| Ok(unsafe { IIterable::<IInspectable>::from_unchecked(values) }.into()),
    |values| Ok(unsafe { IVectorView::<IInspectable>::from_unchecked(values) }.into()),
    |values| Ok(unsafe { IVector::<IInspectable>::from_unchecked(values) }.into()),
    |values| Ok(unsafe { IObservableVector::<IInspectable>::from_unchecked(values) }.into()),
    |values| Ok(unsafe { IMapView::<u32, IInspectable>::from_unchecked(keyed(values)) }.into()),
    |values| Ok(unsafe { IMap::<u32, IInspectable>::from_unchecked(keyed(values)) }.into()),
    |values| {
        Ok(unsafe { IObservableMap::<u32, IInspectable>::from_unchecked(keyed(values)) }.into())
    },
    |values| unsafe { IVector::from_read_only_unchecked(values) }.map(Into::into),
];

fn keyed(values: Vec<Option<IInspectable>>) -> BTreeMap<u32, Option<IInspectable>> {
    values
        .into_iter()
        .enumerate()
        .map(|(index, value)| (index as u32, value))
        .collect()
}

fn read_collection(kind: usize, collection: &IInspectable, index: u32) -> Result<IInspectable> {
    match kind {
        0 => {
            let iterator = collection.cast::<IIterable<IInspectable>>()?.First()?;
            for _ in 0..index {
                iterator.MoveNext()?;
            }
            iterator.Current()
        }
        1 => collection.cast::<IVectorView<IInspectable>>()?.GetAt(index),
        2 | 3 | 7 => collection.cast::<IVector<IInspectable>>()?.GetAt(index),
        4 => collection
            .cast::<IMapView<u32, IInspectable>>()?
            .Lookup(index),
        5 | 6 => collection.cast::<IMap<u32, IInspectable>>()?.Lookup(index),
        _ => unreachable!(),
    }
}

type MakeIterator = fn(IVector<IInspectable>) -> Result<BufferedIterator<IInspectable>>;
const ITERATORS: [MakeIterator; 6] = [
    |vector| Ok(vector.into_iter()),
    |vector| Ok((&vector).into_iter()),
    |vector| Ok(vector.GetView()?.into_iter()),
    |vector| Ok((&vector.GetView()?).into_iter()),
    |vector| Ok(vector.cast::<IIterable<IInspectable>>()?.into_iter()),
    |vector| Ok((&vector.cast::<IIterable<IInspectable>>()?).into_iter()),
];

fn read_cases() -> [Vec<Option<IInspectable>>; 6] {
    let a: IInspectable = IReference::from("a").into();
    let b: IInspectable = IReference::from("b").into();
    let c: IInspectable = IReference::from("c").into();
    [
        vec![],
        vec![None],
        vec![Some(a.clone())],
        vec![Some(a.clone()), None],
        vec![Some(a.clone()), Some(b.clone()), Some(c)],
        vec![None, Some(a.clone()), None, Some(a), Some(b)],
    ]
}

fn assert_item(actual: Result<IInspectable>, expected: Option<&Option<IInspectable>>) {
    match expected {
        Some(Some(value)) => assert_eq!(&actual.unwrap(), value),
        Some(None) => assert_eq!(actual.unwrap_err().code(), HRESULT(0)),
        None => assert_eq!(actual.unwrap_err().code(), E_BOUNDS),
    }
}

macro_rules! assert_indexed {
    ($source:expr, $values:expr, $missing:expr) => {{
        let source = $source;
        let values = $values;
        let missing = $missing;
        assert_eq!(source.Size()? as usize, values.len());
        for index in (0..=values.len() as u32).chain([u32::MAX]) {
            assert_item(source.GetAt(index), values.get(index as usize));
        }
        for target in values.iter().cloned().chain([Some(missing.clone()), None]) {
            let expected = values.iter().position(|value| value == &target);
            let mut index = 99;
            assert_eq!(
                source.IndexOf(target.as_ref(), &mut index)?,
                expected.is_some()
            );
            assert_eq!(index as usize, expected.unwrap_or(0));
        }
        for start in (0..=values.len() as u32 + 1).chain([u32::MAX]) {
            for capacity in [0, 1, 2, 5] {
                let mut expected: Vec<_> = values
                    .iter()
                    .skip(start as usize)
                    .take(capacity)
                    .cloned()
                    .collect();
                let count = expected.len();
                expected.resize(capacity, None);
                let mut actual = vec![Some(missing.clone()); capacity];
                assert_eq!(source.GetMany(start, &mut actual)? as usize, count);
                assert_eq!(actual, expected);
            }
        }
    }};
}

#[test]
fn indexed_read_parity() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    let missing: IInspectable = IReference::from("missing").into();
    let stock: IInspectable = unsafe { IVector::<IInspectable>::from_unchecked(vec![]) }.into();
    let name = stock.GetRuntimeClassName()?;
    for values in read_cases() {
        for make in VECTORS {
            let vector = make(values.clone())?;
            let object: IInspectable = vector.clone().into();
            assert_eq!(object.GetRuntimeClassName()?, name);
            assert_indexed!(&vector, &values, &missing);
            let view = vector.GetView()?;
            assert_indexed!(&view, &values, &missing);
            let iterable = vector.cast::<IIterable<IInspectable>>()?;
            assert_eq!(iterable.cast::<IVector<IInspectable>>()?, vector);
            let iterable = view.cast::<IIterable<IInspectable>>()?;
            assert_eq!(iterable.cast::<IVectorView<IInspectable>>()?, view);
        }
    }
    Ok(())
}

fn assert_position(
    iterator: &IIterator<IInspectable>,
    values: &[Option<IInspectable>],
    position: usize,
) -> Result<()> {
    for _ in 0..2 {
        assert_eq!(iterator.HasCurrent()?, position < values.len());
        assert_item(iterator.Current(), values.get(position));
    }
    Ok(())
}

fn assert_batch(
    iterator: &IIterator<IInspectable>,
    values: &[Option<IInspectable>],
    position: &mut usize,
    capacity: usize,
) -> Result<()> {
    let mut expected: Vec<_> = values[*position..].iter().take(capacity).cloned().collect();
    let count = expected.len();
    expected.resize(capacity, None);
    let mut actual = vec![None; capacity];
    assert_eq!(iterator.GetMany(&mut actual)? as usize, count);
    assert_eq!(actual, expected);
    *position += count;
    assert_position(iterator, values, *position)
}

fn assert_iterator(
    source: &IIterable<IInspectable>,
    values: &[Option<IInspectable>],
) -> Result<()> {
    let iterator = source.First()?;
    let independent = source.First()?;
    for position in 0..values.len() {
        assert_position(&iterator, values, position)?;
        assert_eq!(iterator.MoveNext()?, position + 1 < values.len());
    }
    assert_position(&independent, values, 0)?;
    for _ in 0..2 {
        assert_position(&iterator, values, values.len())?;
        assert!(!iterator.MoveNext()?);
        assert_eq!(iterator.GetMany(&mut [None, None, None])?, 0);
    }

    for capacities in [&[5][..], &[1, 2], &[0, 1, 0, 2, 5]] {
        let iterator = source.First()?;
        let mut position = 0;
        for &capacity in capacities {
            assert_batch(&iterator, values, &mut position, capacity)?;
        }
        assert_batch(&iterator, values, &mut position, values.len() + 1)?;
        assert_eq!(position, values.len());
    }

    let iterator = source.First()?;
    let mut position = 0;
    for capacity in [1, 0, 2, 5] {
        assert_position(&iterator, values, position)?;
        if position < values.len() {
            position += 1;
        }
        assert_eq!(iterator.MoveNext()?, position < values.len());
        assert_batch(&iterator, values, &mut position, capacity)?;
    }

    let mut iterator = source.First()?;
    let prefix: Vec<_> = values.iter().map_while(Clone::clone).collect();
    assert_eq!(iterator.by_ref().collect::<Vec<_>>(), prefix);
    assert_eq!(Iterator::next(&mut iterator), None);
    assert_eq!(iterator.HasCurrent()?, prefix.len() < values.len());
    Ok(())
}

#[test]
fn iterator_read_parity() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    for values in read_cases() {
        for make in ITERABLES {
            assert_iterator(&make(values.clone())?, &values)?;
        }
        for make in VECTORS {
            let vector = make(values.clone())?;
            assert_iterator(&vector.cast::<IIterable<IInspectable>>()?, &values)?;
            assert_iterator(
                &vector.GetView()?.cast::<IIterable<IInspectable>>()?,
                &values,
            )?;
        }
    }
    Ok(())
}

#[test]
fn buffered_iteration_parity() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    for size in [
        0, 1, 2, 63, 64, 65, 127, 128, 129, 200, 255, 256, 257, 300, 384, 400, 1000,
    ] {
        let values: Vec<IInspectable> = (0..size)
            .map(|value| IReference::from(value).into())
            .collect();
        for make in VECTORS {
            let vector = make(values.iter().cloned().map(Some).collect())?;
            for iterate in ITERATORS {
                let mut iterator = iterate(vector.clone())?;
                assert_eq!(iterator.by_ref().collect::<Vec<_>>(), values);
                assert!(iterator.next().is_none());
                assert!(iterator.next().is_none());
                assert_eq!(iterate(vector.clone())?.count(), size as usize);
                let sum = iterate(vector.clone())?.try_fold(0, |sum, value| -> Result<i64> {
                    Ok(sum + i64::from(value.cast::<IReference<i32>>()?.Value()?))
                })?;
                assert_eq!(sum, i64::from(size) * i64::from(size - 1) / 2);
            }
        }
    }
    for values in read_cases() {
        for make in VECTORS {
            let vector = make(values.clone())?;
            for iterate in ITERATORS {
                let mut iterator = iterate(vector.clone())?;
                for value in &values {
                    assert_eq!(iterator.next(), *value);
                }
                assert!(iterator.next().is_none());
                assert!(iterator.next().is_none());
            }
        }
    }
    Ok(())
}

#[implement(IStringable)]
struct TrackedValue {
    value: usize,
    dropped: Arc<AtomicUsize>,
}

impl IStringable_Impl for TrackedValue_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.value.to_string().into())
    }
}

impl Drop for TrackedValue {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn buffered_early_release_parity() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    for make in VECTORS {
        for iterate in ITERATORS {
            for stop in [0, 1, 64, 127, 128, 129, 200, 255, 256, 257, 300, 399, 400] {
                let dropped = Arc::new(AtomicUsize::new(0));
                let values = (0..400)
                    .map(|value| {
                        let value: IStringable = TrackedValue {
                            value,
                            dropped: dropped.clone(),
                        }
                        .into();
                        Some(value.into())
                    })
                    .collect();
                let mut iterator = iterate(make(values)?)?;
                for expected in 0..stop {
                    let value = iterator.next().unwrap().cast::<IStringable>()?;
                    assert_eq!(value.ToString()?, expected.to_string());
                }
                assert_eq!(dropped.load(Ordering::Relaxed), 0);
                drop(iterator);
                assert_eq!(dropped.load(Ordering::Relaxed), 400);
            }
        }
    }
    Ok(())
}

#[test]
fn immutable_interface_surfaces() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    let no_interface = HRESULT(0x80004002_u32 as _);
    for source in [
        unsafe { IIterable::<IInspectable>::from_unchecked(vec![None]) },
        IIterable::<IInspectable>::from_marshaled(vec![None])?,
    ] {
        assert_eq!(
            source
                .cast::<IVectorView<IInspectable>>()
                .unwrap_err()
                .code(),
            no_interface
        );
        assert_eq!(
            source.cast::<IVector<IInspectable>>().unwrap_err().code(),
            no_interface
        );
        let object: IInspectable = source.into();
        assert_eq!(
            object.GetRuntimeClassName()?,
            "Windows.Foundation.Collections.IIterable`1<Object>"
        );
    }
    for view in [
        unsafe { IVectorView::<IInspectable>::from_unchecked(vec![None]) },
        IVectorView::<IInspectable>::from_marshaled(vec![None])?,
        unsafe { IVector::<IInspectable>::from_unchecked(vec![None]) }.GetView()?,
        IVector::<IInspectable>::from_marshaled(vec![None])?.GetView()?,
        unsafe { IObservableVector::<IInspectable>::from_unchecked(vec![None]) }.GetView()?,
        IObservableVector::<IInspectable>::from_marshaled(vec![None])?.GetView()?,
    ] {
        assert_eq!(
            view.cast::<IVector<IInspectable>>().unwrap_err().code(),
            no_interface
        );
        assert_eq!(
            view.cast::<IObservableVector<IInspectable>>()
                .unwrap_err()
                .code(),
            no_interface
        );
        assert_eq!(
            view.cast::<IIterable<IInspectable>>()?
                .cast::<IVectorView<IInspectable>>()?,
            view
        );
        let object: IInspectable = view.into();
        assert_eq!(
            object.GetRuntimeClassName()?,
            "Windows.Foundation.Collections.IVectorView`1<Object>"
        );
    }
    for make in [VECTORS[2], VECTORS[3]] {
        let vector = make(vec![None])?;
        let view = vector.GetView()?;
        assert_eq!(view.cast::<IVector<IInspectable>>()?, vector);
        assert_eq!(vector.cast::<IUnknown>()?, view.cast::<IUnknown>()?);
        assert_eq!(
            vector
                .cast::<IObservableVector<IInspectable>>()
                .unwrap_err()
                .code(),
            no_interface
        );
        let object: IInspectable = view.into();
        assert_eq!(
            object.GetRuntimeClassName()?,
            "Windows.Foundation.Collections.IVector`1<Object>"
        );
    }
    Ok(())
}

#[test]
fn read_only_contract() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    for make in [VECTORS[2], VECTORS[3]] {
        check_read_only_contract(make)?;
    }
    Ok(())
}

fn check_read_only_contract(make: MakeVector) -> Result<()> {
    let item: IInspectable = IReference::from("value").into();
    let vector = make(vec![Some(item.clone()), None])?;
    for result in [
        vector.SetAt(0, &item),
        vector.InsertAt(0, &item),
        vector.RemoveAt(0),
        vector.Append(&item),
        vector.RemoveAtEnd(),
        vector.Clear(),
        vector.ReplaceAll(&[]),
    ] {
        assert_eq!(result.unwrap_err().code(), E_NOTIMPL);
    }
    assert_eq!(vector.Size()?, 2);
    assert_eq!(vector.GetAt(0)?, item);
    assert_eq!(vector.GetAt(2).unwrap_err().code(), E_BOUNDS);
    unsafe {
        let mut value = std::ptr::null_mut();
        assert_eq!(
            (vector.vtable().GetAt)(vector.as_raw(), 1, &mut value),
            HRESULT(0)
        );
        assert!(value.is_null());
    }
    let view = vector.GetView()?;
    let iterator = vector.First()?;
    drop(vector);
    assert_eq!(view.GetAt(0)?, item);
    let mut values = [None, None, None];
    assert_eq!(view.GetMany(0, &mut values)?, 2);
    assert_eq!(values, [Some(item.clone()), None, None]);
    assert_eq!(view.GetMany(2, &mut values)?, 0);
    assert_eq!(iterator.Current()?, item);
    assert!(iterator.MoveNext()?);
    assert_eq!(iterator.GetMany(&mut values)?, 1);
    assert_eq!(values, [None, None, None]);
    assert!(!iterator.HasCurrent()?);
    assert!(!iterator.MoveNext()?);
    assert!(!iterator.MoveNext()?);
    assert_eq!(iterator.Current().unwrap_err().code(), E_BOUNDS);
    Ok(())
}

#[implement(IStringable, IClosable)]
struct Identity;

impl IStringable_Impl for Identity_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("identity".into())
    }
}

impl IClosable_Impl for Identity_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn identity_and_null_search() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    let object = ComObject::new(Identity);
    let first: IInspectable = object.to_interface::<IStringable>().into();
    let second: IInspectable = object.to_interface::<IClosable>().into();
    assert_ne!(first.as_raw(), second.as_raw());
    let other: IInspectable = IReference::from("other").into();
    for make in [VECTORS[2], VECTORS[3]] {
        let vector = make(vec![None, Some(first.clone())])?;
        let mut index = 99;
        assert!(vector.IndexOf(&second, &mut index)?);
        assert_eq!(index, 1);
        assert!(vector.GetView()?.IndexOf(None, &mut index)?);
        assert_eq!(index, 0);
        index = 99;
        assert!(!vector.IndexOf(&other, &mut index)?);
        assert_eq!(index, 0);
    }
    Ok(())
}

#[implement(IStringable, INoMarshal, Agile = false)]
struct NonMarshaled;

impl INoMarshal_Impl for NonMarshaled_Impl {}

impl IStringable_Impl for NonMarshaled_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        unreachable!()
    }
}

#[test]
fn unmarshalable_value_rejects_construction() -> Result<()> {
    let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
    let blocked: IStringable = NonMarshaled.into();
    let blocked: IInspectable = blocked.into();
    let expected = AgileReference::new(&blocked).unwrap_err().code();
    for (kind, make) in MARSHAL_FACTORIES.into_iter().enumerate() {
        let dropped = Arc::new(AtomicUsize::new(0));
        let first: IStringable = TrackedValue {
            value: 0,
            dropped: dropped.clone(),
        }
        .into();
        assert_eq!(
            make(vec![Some(first.into()), Some(blocked.clone())])
                .unwrap_err()
                .code(),
            expected,
            "factory {kind}"
        );
        assert_eq!(dropped.load(Ordering::Relaxed), 1, "factory {kind}");
    }
    Ok(())
}

#[test]
fn direct_storage_accepts_caller_confined_objects() -> Result<()> {
    let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
    let blocked: IStringable = NonMarshaled.into();
    let blocked: IInspectable = blocked.into();
    assert!(AgileReference::new(&blocked).is_err());
    for (kind, make) in DIRECT_FACTORIES.into_iter().enumerate() {
        let collection = make(vec![Some(blocked.clone()), None])?;
        assert_eq!(read_collection(kind, &collection, 0)?, blocked);
        assert_eq!(
            read_collection(kind, &collection, 1).unwrap_err().code(),
            HRESULT(0)
        );
    }
    Ok(())
}

#[test]
fn marshaled_nulls() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    for (kind, make) in MARSHAL_FACTORIES.into_iter().enumerate() {
        let collection = make(vec![None])?;
        assert_eq!(
            read_collection(kind, &collection, 0).unwrap_err().code(),
            HRESULT(0)
        );
        assert_eq!(
            read_collection(kind, &collection, 1).unwrap_err().code(),
            E_BOUNDS
        );
    }
    Ok(())
}

#[test]
fn failed_vector_ingress_preserves_contents_and_notifications() -> Result<()> {
    let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
    let blocked: IStringable = NonMarshaled.into();
    let blocked: IInspectable = blocked.into();
    let expected = AgileReference::new(&blocked).unwrap_err().code();
    let initial: IInspectable = IReference::from("initial").into();
    let observable =
        IObservableVector::<IInspectable>::from_marshaled(vec![Some(initial.clone()), None])?;
    let events = Arc::new(AtomicUsize::new(0));
    let capture = events.clone();
    let _subscription = observable.VectorChanged(move |_, _| {
        capture.fetch_add(1, Ordering::Relaxed);
    })?;
    for vector in [
        IVector::from_marshaled(vec![Some(initial.clone()), None])?,
        observable.cast::<IVector<IInspectable>>()?,
    ] {
        let before = events.load(Ordering::Relaxed);
        for operation in 0..4 {
            let dropped = Arc::new(AtomicUsize::new(0));
            let result = match operation {
                0 => vector.SetAt(0, &blocked),
                1 => vector.InsertAt(0, &blocked),
                2 => vector.Append(&blocked),
                3 => {
                    let prefix: IStringable = TrackedValue {
                        value: 0,
                        dropped: dropped.clone(),
                    }
                    .into();
                    vector.ReplaceAll(&[Some(prefix.into()), Some(blocked.clone())])
                }
                _ => unreachable!(),
            };
            assert_eq!(result.unwrap_err().code(), expected);
            assert_eq!(vector.Size()?, 2);
            let mut values = [None, None];
            assert_eq!(vector.GetMany(0, &mut values)?, 2);
            assert_eq!(values, [Some(initial.clone()), None]);
            assert_eq!(events.load(Ordering::Relaxed), before);
            assert_eq!(dropped.load(Ordering::Relaxed), usize::from(operation == 3));
        }
        vector.SetAt(0, None)?;
        assert_eq!(vector.GetAt(0).unwrap_err().code(), HRESULT(0));
        vector.SetAt(0, &initial)?;
        assert_eq!(vector.GetAt(0)?, initial);
    }
    assert_eq!(events.load(Ordering::Relaxed), 2);
    Ok(())
}

#[test]
fn failed_map_ingress_preserves_contents_and_notifications() -> Result<()> {
    let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
    let blocked: IStringable = NonMarshaled.into();
    let blocked: IInspectable = blocked.into();
    let expected = AgileReference::new(&blocked).unwrap_err().code();
    let initial: IInspectable = IReference::from("initial").into();
    let observable = IObservableMap::<u32, IInspectable>::from_marshaled(keyed(vec![
        Some(initial.clone()),
        None,
    ]))?;
    let events = Arc::new(AtomicUsize::new(0));
    let capture = events.clone();
    let _subscription = observable.MapChanged(move |_, _| {
        capture.fetch_add(1, Ordering::Relaxed);
    })?;
    for map in [
        IMap::from_marshaled(keyed(vec![Some(initial.clone()), None]))?,
        observable.cast::<IMap<u32, IInspectable>>()?,
    ] {
        let before = events.load(Ordering::Relaxed);
        for key in [0, 2] {
            assert_eq!(map.Insert(key, &blocked).unwrap_err().code(), expected);
            assert_eq!(map.Size()?, 2);
            assert_eq!(map.Lookup(0)?, initial);
            assert_eq!(map.Lookup(1).unwrap_err().code(), HRESULT(0));
            assert!(!map.HasKey(2)?);
            assert_eq!(events.load(Ordering::Relaxed), before);
        }
        assert!(map.Insert(0, None)?);
        assert_eq!(map.Lookup(0).unwrap_err().code(), HRESULT(0));
        assert!(map.Insert(0, &initial)?);
        assert_eq!(map.Lookup(0)?, initial);
    }
    assert_eq!(events.load(Ordering::Relaxed), 2);
    Ok(())
}

#[implement(IStringable, Agile = false)]
struct ApartmentValue {
    owner: ThreadId,
    calls: Arc<AtomicUsize>,
    dropped: Arc<OwnedHandle>,
}

impl IStringable_Impl for ApartmentValue_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        assert_eq!(thread::current().id(), self.owner);
        self.calls.fetch_add(1, Ordering::Relaxed);
        Ok("apartment".into())
    }
}

impl Drop for ApartmentValue {
    fn drop(&mut self) {
        assert_eq!(thread::current().id(), self.owner);
        assert!(unsafe { SetEvent(self.dropped.as_raw_handle()) }.as_bool());
    }
}

#[test]
fn marshaled_derived_owners_preserve_apartment_and_final_release() -> Result<()> {
    let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
    for (kind, make) in MARSHAL_FACTORIES.into_iter().enumerate() {
        let handle = unsafe { CreateEventW(None, true, false, None) };
        if handle.is_null() {
            return Err(Error::from_thread());
        }
        let dropped = Arc::new(unsafe { OwnedHandle::from_raw_handle(handle) });
        let calls = Arc::new(AtomicUsize::new(0));
        let value: IStringable = ApartmentValue {
            owner: thread::current().id(),
            calls: calls.clone(),
            dropped: dropped.clone(),
        }
        .into();
        let value: IInspectable = value.into();
        let collection = make(vec![Some(value.clone())])?;
        match kind {
            2 | 3 => {
                let vector = collection.cast::<IVector<IInspectable>>()?;
                vector.SetAt(0, &value)?;
                vector.InsertAt(0, &value)?;
                vector.Append(&value)?;
                vector.ReplaceAll(&[Some(value.clone())])?;
            }
            5 | 6 => assert!(
                collection
                    .cast::<IMap<u32, IInspectable>>()?
                    .Insert(0, &value)?
            ),
            _ => {}
        }
        drop(value);
        // Create derived owners in the STA, before marshaling them to the worker.
        let child: IInspectable = match kind {
            0 => collection
                .cast::<IIterable<IInspectable>>()?
                .First()?
                .into(),
            1 => collection
                .cast::<IVectorView<IInspectable>>()?
                .First()?
                .into(),
            2 | 3 | 7 => collection
                .cast::<IVector<IInspectable>>()?
                .GetView()?
                .First()?
                .into(),
            4 => collection
                .cast::<IMapView<u32, IInspectable>>()?
                .First()?
                .Current()?
                .into(),
            5 | 6 => collection
                .cast::<IMap<u32, IInspectable>>()?
                .GetView()?
                .First()?
                .Current()?
                .into(),
            _ => unreachable!(),
        };
        let root = AgileReference::new(&collection)?;
        let child_reference = AgileReference::new(&child)?;
        drop(collection);
        drop(child);
        let worker = thread::spawn(move || -> Result<()> {
            let _com = Com::new(COINIT_MULTITHREADED as u32)?;
            let collection = root.resolve()?;
            drop(root);
            assert_eq!(
                read_collection(kind, &collection, 0)?
                    .cast::<IStringable>()?
                    .ToString()?,
                "apartment"
            );
            match kind {
                2 | 3 => collection.cast::<IVector<IInspectable>>()?.Clear()?,
                5 | 6 => collection.cast::<IMap<u32, IInspectable>>()?.Clear()?,
                _ => {}
            }
            drop(collection);
            let child = child_reference.resolve()?;
            drop(child_reference);
            let value = if (4..=6).contains(&kind) {
                let pair = child.cast::<IKeyValuePair<u32, IInspectable>>()?;
                assert_eq!(pair.Key()?, 0);
                pair.Value()?
            } else {
                let iterator = child.cast::<IIterator<IInspectable>>()?;
                let mut values = [None];
                assert_eq!(iterator.GetMany(&mut values)?, 1);
                assert!(!iterator.HasCurrent()?);
                values[0].take().unwrap()
            };
            drop(child);
            assert_eq!(value.cast::<IStringable>()?.ToString()?, "apartment");
            Ok(())
        });
        unsafe {
            CoWaitForMultipleHandles(
                COWAIT_DISPATCH_CALLS as u32,
                5_000,
                1,
                &mut dropped.as_raw_handle(),
                &mut 0,
            )
            .ok()?;
        }
        worker.join().unwrap()?;
        assert_eq!(calls.load(Ordering::Relaxed), 2, "factory {kind}");
    }
    Ok(())
}

#[test]
fn apartment_reads_and_final_release() -> Result<()> {
    let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
    let handle = unsafe { CreateEventW(None, true, false, None) };
    if handle.is_null() {
        return Err(Error::from_thread());
    }
    let dropped = Arc::new(unsafe { OwnedHandle::from_raw_handle(handle) });
    let calls = Arc::new(AtomicUsize::new(0));
    let value: IStringable = ApartmentValue {
        owner: thread::current().id(),
        calls: calls.clone(),
        dropped: dropped.clone(),
    }
    .into();
    assert!(value.cast::<IAgileObject>().is_err());
    assert!(value.cast::<IMarshal>().is_err());
    let vector = IVector::from_read_only([Some(value.into())])?;
    let reference = AgileReference::new(&vector)?;
    drop(vector);
    let worker = thread::spawn(move || -> Result<()> {
        let _com = Com::new(COINIT_MULTITHREADED as u32)?;
        let vector = reference.resolve()?;
        let view = vector.GetView()?;
        let iterator = vector.First()?;
        let first = vector.GetAt(0)?;
        assert_eq!(first.cast::<IStringable>()?.ToString()?, "apartment");
        assert!(vector.IndexOf(&first, &mut 0)?);
        drop(first);
        drop(vector);
        drop(reference);
        assert_eq!(
            view.GetAt(0)?.cast::<IStringable>()?.ToString()?,
            "apartment"
        );
        drop(view);
        let mut values = [None];
        assert_eq!(iterator.GetMany(&mut values)?, 1);
        assert_eq!(
            values[0]
                .as_ref()
                .unwrap()
                .cast::<IStringable>()?
                .ToString()?,
            "apartment"
        );
        Ok(())
    });
    unsafe {
        CoWaitForMultipleHandles(
            COWAIT_DISPATCH_CALLS as u32,
            5_000,
            1,
            &mut dropped.as_raw_handle(),
            &mut 0,
        )
        .ok()?;
    }
    worker.join().unwrap()?;
    assert_eq!(calls.load(Ordering::Relaxed), 3);
    Ok(())
}

#[implement(IStringable, Agile = false)]
struct Disconnected;

impl IStringable_Impl for Disconnected_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        unreachable!()
    }
}

#[test]
fn disconnected_values_are_not_used_directly() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    let (vector, disconnected) = thread::spawn(|| -> Result<_> {
        let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
        let value: IStringable = Disconnected.into();
        let disconnected = AgileReference::new(&value.cast::<IInspectable>()?)?;
        let first: IInspectable = IReference::from("first").into();
        let vector = IVector::from_read_only([Some(first), Some(value.clone().into())])?;
        unsafe { CoDisconnectObject(&value, 0).ok()? };
        Ok((AgileReference::new(&vector)?, disconnected))
    })
    .join()
    .unwrap()?;
    let expected = disconnected
        .resolve()
        .and_then(|value| value.cast::<IStringable>())
        .unwrap_err()
        .code();
    assert!(expected.is_err());
    let vector = vector.resolve().unwrap();
    assert_eq!(
        vector
            .GetAt(1)
            .and_then(|value| value.cast::<IStringable>())
            .unwrap_err()
            .code(),
        expected
    );
    let mut values = [None, None];
    let view = vector.GetView()?;
    match view.GetMany(0, &mut values) {
        Ok(count) => {
            assert_eq!(count, 2);
            assert_eq!(
                values[1]
                    .as_ref()
                    .unwrap()
                    .cast::<IStringable>()
                    .unwrap_err()
                    .code(),
                expected
            );
        }
        Err(error) => {
            assert_eq!(error.code(), disconnected.resolve().unwrap_err().code());
            assert!(values.iter().all(Option::is_none));
        }
    }
    let iterator = vector.First()?;
    assert_eq!(iterator.Current()?, vector.GetAt(0)?);
    assert!(iterator.MoveNext()?);
    match iterator.GetMany(&mut values) {
        Ok(count) => {
            assert_eq!(count, 1);
            assert_eq!(
                values[0]
                    .as_ref()
                    .unwrap()
                    .cast::<IStringable>()
                    .unwrap_err()
                    .code(),
                expected
            );
            assert!(!iterator.HasCurrent()?);
        }
        Err(error) => {
            assert_eq!(error.code(), disconnected.resolve().unwrap_err().code());
            assert!(values.iter().all(Option::is_none));
            assert!(iterator.HasCurrent()?);
            assert_eq!(
                iterator
                    .Current()
                    .and_then(|value| value.cast::<IStringable>())
                    .unwrap_err()
                    .code(),
                error.code()
            );
        }
    }
    Ok(())
}
