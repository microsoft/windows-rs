use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::{Cell, RefCell};
use std::sync::{
    Arc, Barrier,
    atomic::{AtomicUsize, Ordering},
};
use windows_collections::*;
use windows_core::*;

struct CountingAllocator;

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

std::thread_local! {
    static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
    static COPIES: RefCell<Option<Arc<Copies>>> = const { RefCell::new(None) };
    static FIRST_COPY: Cell<bool> = const { Cell::new(false) };
}

fn allocated() {
    let _ = ALLOCATIONS.try_with(|count| {
        if let Some(value) = count.get() {
            count.set(Some(value + 1));
        }
    });
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocated();
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        allocated();
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        allocated();
        unsafe { System.realloc(ptr, layout, size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn allocations<R>(run: impl FnOnce() -> R) -> (R, usize) {
    ALLOCATIONS.set(Some(0));
    let result = run();
    let count = ALLOCATIONS.replace(None).unwrap();
    (result, count)
}

struct Copies {
    first_copies: Barrier,
    calls: AtomicUsize,
}

#[repr(transparent)]
#[derive(Default, PartialEq)]
struct Scalar(u32);

impl Clone for Scalar {
    fn clone(&self) -> Self {
        COPIES.with_borrow(|copies| {
            if let Some(copies) = copies {
                copies.calls.fetch_add(1, Ordering::Relaxed);
                if FIRST_COPY.replace(false) {
                    copies.first_copies.wait();
                }
            }
        });
        Self(self.0)
    }
}

impl imp::TypeKind for Scalar {
    type TypeKind = imp::CloneType;
}

impl RuntimeType for Scalar {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"struct(Example.Scalar;u4)");
}

fn scalars() -> Vec<Scalar> {
    (0..4).map(Scalar).collect()
}

fn scalar_map() -> std::collections::BTreeMap<u32, Scalar> {
    (0..4).map(|key| (key, Scalar(key))).collect()
}

struct SharedIterator<T: RuntimeType + 'static>(IIterator<T>);

// Only stock iterators backed by the Send + Sync scalars below cross this test boundary.
// Their storage is immutable or protected by the vector's structural lock.
unsafe impl<T: RuntimeType> Send for SharedIterator<T> {}

impl<T: RuntimeType> SharedIterator<T> {
    fn get_many(&self, items: &mut [T::Default]) -> Result<u32> {
        self.0.GetMany(items)
    }
}

fn check_batches<T: RuntimeType>(iterator: IIterator<T>, extract: fn(T::Default) -> u32)
where
    T::Default: Default,
{
    let copies = Arc::new(Copies {
        first_copies: Barrier::new(2),
        calls: AtomicUsize::new(0),
    });
    let mut values = std::thread::scope(|scope| {
        let mut threads = Vec::new();
        for _ in 0..2 {
            let iterator = SharedIterator(iterator.clone());
            let copies = copies.clone();
            threads.push(scope.spawn(move || {
                let mut items: [T::Default; 2] = Default::default();
                COPIES.set(Some(copies));
                FIRST_COPY.set(true);
                let count = iterator.get_many(&mut items).unwrap();
                COPIES.set(None);
                assert_eq!(count, 2);
                items.map(extract)
            }));
        }
        threads
            .into_iter()
            .flat_map(|thread| thread.join().unwrap())
            .collect::<Vec<_>>()
    });
    values.sort_unstable();
    assert_eq!(values, [0, 1, 2, 3]);
    assert_eq!(copies.calls.load(Ordering::Relaxed), 4);
    assert!(!iterator.HasCurrent().unwrap());
    assert!(!iterator.MoveNext().unwrap());
    let mut end: [T::Default; 2] = Default::default();
    assert_eq!(iterator.GetMany(&mut end).unwrap(), 0);
}

#[test]
fn direct_iterable_batches_are_disjoint() {
    check_batches(
        IIterable::<Scalar>::from(scalars()).First().unwrap(),
        |value| value.0,
    );
}

#[test]
fn direct_vector_view_batches_are_disjoint() {
    check_batches(
        IVectorView::<Scalar>::from(scalars()).First().unwrap(),
        |value| value.0,
    );
}

#[test]
fn direct_vector_batches_are_disjoint() {
    check_batches(
        IVector::<Scalar>::from(scalars()).First().unwrap(),
        |value| value.0,
    );
}

#[test]
fn direct_observable_vector_batches_are_disjoint() {
    check_batches(
        IObservableVector::<Scalar>::from(scalars())
            .First()
            .unwrap(),
        |value| value.0,
    );
}

#[test]
fn direct_map_view_batches_are_disjoint() {
    check_batches(
        IMapView::<u32, Scalar>::from(scalar_map()).First().unwrap(),
        |pair| pair.unwrap().Value().unwrap().0,
    );
}

#[test]
fn direct_map_batches_are_disjoint() {
    check_batches(
        IMap::<u32, Scalar>::from(scalar_map()).First().unwrap(),
        |pair| pair.unwrap().Value().unwrap().0,
    );
}

#[test]
fn direct_observable_map_batches_are_disjoint() {
    check_batches(
        IObservableMap::<u32, Scalar>::from(scalar_map())
            .First()
            .unwrap(),
        |pair| pair.unwrap().Value().unwrap().0,
    );
}

#[test]
fn immutable_owners_do_not_add_allocations() {
    for kind in 0..3 {
        let values = vec![None, None];
        let (object, count) = allocations(|| -> IInspectable {
            // Null-only storage has no apartment or thread affinity.
            unsafe {
                match kind {
                    0 => IIterable::<IInspectable>::from_unchecked(values).into(),
                    1 => IVectorView::<IInspectable>::from_unchecked(values).into(),
                    2 => IVector::from_read_only_unchecked(values).unwrap().into(),
                    _ => unreachable!(),
                }
            }
        });
        assert_eq!(count, 1);
        let iterable = object.cast::<IIterable<IInspectable>>().unwrap();
        let (iterator, count) = allocations(|| iterable.First().unwrap());
        assert_eq!(count, 1);
        drop(iterable);
        drop(object);
        let mut values = [None, None];
        assert_eq!(iterator.GetMany(&mut values).unwrap(), 2);
    }
}

#[test]
fn direct_indexed_batches_do_not_allocate() {
    let vector = IVector::<u32>::from(vec![0, 1, 2, 3]);
    let observable = IObservableVector::<u32>::from(vec![0, 1, 2, 3]);
    let view = IVectorView::<u32>::from(vec![0, 1, 2, 3]);
    let mut items = [99; 3];
    for get in [
        &|items: &mut [u32]| vector.GetMany(2, items),
        &|items: &mut [u32]| observable.GetMany(2, items),
        &|items: &mut [u32]| view.GetMany(2, items),
    ] as [&dyn Fn(&mut [u32]) -> Result<u32>; 3]
    {
        let (count, allocations) = allocations(|| get(&mut items).unwrap());
        assert_eq!(count, 2);
        // The generated ABI thunk zeroes the output slice before calling the implementation.
        assert_eq!(items, [2, 3, 0]);
        assert_eq!(allocations, 0);
    }

    let read_only = unsafe { IVector::from_read_only_unchecked([None, None]).unwrap() };
    let mut items = [const { None }; 3];
    assert_eq!(
        allocations(|| read_only.GetMany(0, &mut items).unwrap()),
        (2, 0)
    );
    assert_eq!(
        allocations(|| read_only.GetView().unwrap().GetMany(0, &mut items).unwrap()),
        (2, 0)
    );
}

#[test]
fn direct_iterator_batches_do_not_allocate() {
    let iterators = [
        IIterable::<u32>::from(vec![0, 1, 2]).First().unwrap(),
        IVectorView::<u32>::from(vec![0, 1, 2]).First().unwrap(),
        IVector::<u32>::from(vec![0, 1, 2]).First().unwrap(),
        IObservableVector::<u32>::from(vec![0, 1, 2])
            .First()
            .unwrap(),
    ];
    for iterator in iterators {
        let mut items = [99; 2];
        assert_eq!(
            allocations(|| iterator.GetMany(&mut items).unwrap()),
            (2, 0)
        );
        assert_eq!(items, [0, 1]);
        assert_eq!(
            allocations(|| iterator.GetMany(&mut items).unwrap()),
            (1, 0)
        );
        assert_eq!(items, [2, 0]);
        assert_eq!(
            allocations(|| iterator.GetMany(&mut items).unwrap()),
            (0, 0)
        );
    }

    let read_only = unsafe { IVector::from_read_only_unchecked([None, None]).unwrap() };
    let iterator = read_only.First().unwrap();
    let mut items = [const { None }; 2];
    assert_eq!(
        allocations(|| iterator.GetMany(&mut items).unwrap()),
        (2, 0)
    );
}

#[test]
fn direct_map_batches_allocate_only_pairs() {
    let values = || {
        [(0_u32, 0_u32), (1, 1), (2, 2)]
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    let iterators = [
        IMapView::<u32, u32>::from(values()).First().unwrap(),
        IMap::<u32, u32>::from(values()).First().unwrap(),
        IObservableMap::<u32, u32>::from(values()).First().unwrap(),
    ];
    for iterator in iterators {
        let (pair, pair_allocations) = allocations(|| iterator.Current().unwrap());
        drop(pair);
        let mut items = [None, None];
        let (count, batch_allocations) = allocations(|| iterator.GetMany(&mut items).unwrap());
        assert_eq!(count, 2);
        assert_eq!(batch_allocations, 2 * pair_allocations);
        assert_eq!(items[0].as_ref().unwrap().Key().unwrap(), 0);
        assert_eq!(items[1].as_ref().unwrap().Key().unwrap(), 1);
    }
}

fn check_replace_all_capacity(vector: IVector<u32>, notification_allocations: usize) {
    let items: [u32; 512] = std::array::from_fn(|index| index as u32 + 10);
    for count in [256, 0, 512, 128, 511, 256] {
        let (_, actual) = allocations(|| vector.ReplaceAll(&items[..count]).unwrap());
        assert_eq!(actual, notification_allocations);
        assert_eq!(vector.Size().unwrap(), count as u32);
        let mut copied = [0; 512];
        assert_eq!(vector.GetMany(0, &mut copied).unwrap(), count as u32);
        assert_eq!(&copied[..count], &items[..count]);
    }
}

#[test]
fn direct_vector_replace_all_reuses_capacity() {
    let mut values = Vec::with_capacity(512);
    values.extend_from_slice(&[0; 256]);
    check_replace_all_capacity(IVector::<u32>::from(values), 0);
}

#[test]
fn direct_observable_vector_replace_all_reuses_capacity() {
    let mut values = Vec::with_capacity(512);
    values.extend_from_slice(&[0; 256]);
    let vector = IObservableVector::<u32>::from(values);
    // Clear retains this trivial storage's capacity and emits the same Reset notification.
    let (_, notification_allocations) = allocations(|| vector.Clear().unwrap());
    check_replace_all_capacity(vector.cast().unwrap(), notification_allocations);
}

#[cfg(not(windows))]
#[test]
fn marshaled_factories_require_windows_for_non_null_values() {
    let object: IInspectable = IVectorView::<u32>::from(vec![1]).cast().unwrap();
    let check = |factory: &dyn Fn(Option<IInspectable>) -> Result<()>| {
        assert_eq!(
            factory(Some(object.clone())).unwrap_err().code(),
            HRESULT(0x80004001_u32 as _),
        );
        factory(None).unwrap();
    };
    check(&|value| IIterable::<IInspectable>::from_marshaled(vec![value]).map(|_| ()));
    check(&|value| IVectorView::<IInspectable>::from_marshaled(vec![value]).map(|_| ()));
    check(&|value| IVector::<IInspectable>::from_marshaled(vec![value]).map(|_| ()));
    check(&|value| IObservableVector::<IInspectable>::from_marshaled(vec![value]).map(|_| ()));
    check(&|value| {
        IMapView::<u32, IInspectable>::from_marshaled([(0, value)].into_iter().collect())
            .map(|_| ())
    });
    check(&|value| {
        IMap::<u32, IInspectable>::from_marshaled([(0, value)].into_iter().collect()).map(|_| ())
    });
    check(&|value| {
        IObservableMap::<u32, IInspectable>::from_marshaled([(0, value)].into_iter().collect())
            .map(|_| ())
    });
    check(&|value| IVector::from_read_only([value]).map(|_| ()));
}
