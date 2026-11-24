#![cfg(windows)]

use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::sync::{Arc, atomic::*};
use std::thread::{self, ThreadId};
use windows::Foundation::*;
use windows::Win32::*;
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
const VECTORS: [MakeVector; 2] = [|values| Ok(values.into()), IVector::from_read_only];

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
    let stock: IInspectable = IVector::<IInspectable>::from(vec![]).into();
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
        assert_iterator(&IIterable::from(values.clone()), &values)?;
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
fn read_only_contract() -> Result<()> {
    let _com = Com::new(COINIT_MULTITHREADED as u32)?;
    let item: IInspectable = IReference::from("value").into();
    let vector = IVector::from_read_only([Some(item.clone()), None])?;
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
    let vector = IVector::from_read_only([None, Some(first)])?;
    let mut index = 99;
    assert!(vector.IndexOf(&second, &mut index)?);
    assert_eq!(index, 1);
    assert!(vector.GetView()?.IndexOf(None, &mut index)?);
    assert_eq!(index, 0);
    let other: IInspectable = IReference::from("other").into();
    index = 99;
    assert!(!vector.IndexOf(&other, &mut index)?);
    assert_eq!(index, 0);
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
    let first: IInspectable = IReference::from("first").into();
    assert_eq!(
        IVector::from_read_only([Some(first), Some(blocked)])
            .unwrap_err()
            .code(),
        expected
    );
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
