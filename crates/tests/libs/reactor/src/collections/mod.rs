use super::*;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_collections::*;
use windows_core::*;
use windows_reference::IReference;

mod live;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Source {
    Mutable,
    ReadOnly,
    Shared,
    Trusted,
}

pub(super) fn run_live(source: Source) -> Result<()> {
    live::run(source)
}

#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::all
)]
mod native {
    include!(concat!(env!("OUT_DIR"), "\\collection_bindings.rs"));
}

pub(super) fn run() -> Result<()> {
    let result = Rc::new(RefCell::new(None));
    App::run_application::<CollectionFixture>(result.clone())?;
    result.borrow_mut().take().unwrap()
}

struct CollectionFixture;

impl Application for CollectionFixture {
    type Input = Rc<RefCell<Option<Result<()>>>>;
    type Message = ();

    fn create(result: &Self::Input, context: &ApplicationContext<Self>) -> Self {
        *result.borrow_mut() = Some(check_sources());
        context.exit();
        Self
    }

    fn update(&mut self, _: (), _: &ApplicationContext<Self>) {
        unreachable!();
    }

    fn view(&self, _: &Self::Input, _: &ApplicationViewContext<Self>) -> ApplicationView {
        ApplicationView::new()
    }
}

fn boxed(value: &str) -> IInspectable {
    IReference::from(value).into()
}

fn check_sources() -> Result<()> {
    eprintln!("agility: boxed string");
    boxed("text").cast::<imp::IAgileObject>()?;
    eprintln!("agility: Border");
    let border: IInspectable = native::Border::new()?.into();
    // WinUI uses the FTM without exposing IAgileObject.
    assert_eq!(
        border.cast::<imp::IAgileObject>().unwrap_err().code(),
        HRESULT(0x80004002_u32 as _)
    );
    let mut raw_ftm = std::ptr::null_mut();
    let ftm: IUnknown = unsafe {
        native::CoCreateFreeThreadedMarshaler(std::ptr::null_mut(), &mut raw_ftm).ok()?;
        imp::from_abi(raw_ftm)?
    };
    assert_eq!(unmarshal_class(&border)?, unmarshal_class(&ftm)?);
    let vector = IVector::from_read_only([Some(border.clone())])?;
    let agile = AgileReference::new(&vector)?;
    let identity = border.cast::<IUnknown>()?.as_raw() as usize;
    let resolved_identity = std::thread::spawn(move || -> Result<usize> {
        unsafe {
            native::CoInitializeEx(std::ptr::null_mut(), native::COINIT_MULTITHREADED as u32)
                .ok()?;
        }
        let identity = agile
            .resolve()
            .and_then(|value| Ok(value.GetAt(0)?.cast::<IUnknown>()?.as_raw() as usize));
        drop(agile);
        unsafe { native::CoUninitialize() };
        identity
    })
    .join()
    .unwrap()?;
    assert_eq!(resolved_identity, identity);

    eprintln!("constructing ItemsControl consumers");
    for control in [
        native::ComboBox::new()?.cast::<native::IItemsControl>()?,
        native::AutoSuggestBox::new()?.cast::<native::IItemsControl>()?,
    ] {
        eprintln!("ItemsControl: mutable vector");
        let first = boxed("first");
        let replacement = boxed("replacement");
        let vector = IVector::<IInspectable>::from_marshaled(vec![Some(first.clone())])?;
        control.SetItemsSource(&vector)?;
        assert_eq!(control.Items()?.GetAt(0)?, first);
        vector.SetAt(0, &replacement)?;
        assert_eq!(control.Items()?.GetAt(0)?, replacement);

        eprintln!("ItemsControl: vector view");
        let source = new_source()?;
        let view: IVectorView<IInspectable> = source.to_interface();
        control.SetItemsSource(&view)?;
        assert_eq!(source.counts.enumerated.load(Ordering::Relaxed), 3);
        assert_eq!(control.Items()?.Size()?, 3);
        assert_eq!(control.Items()?.GetAt(1)?, source.values[1].resolve()?);
        control.SetItemsSource(None)?;

        eprintln!("ItemsControl: read-only vector");
        let source = new_source()?;
        let vector: IVector<IInspectable> =
            ComObject::new(ReadOnlyVector(source.clone())).into_interface();
        control.SetItemsSource(&vector)?;
        assert_eq!(source.counts.enumerated.load(Ordering::Relaxed), 0);
        assert_eq!(control.Items()?.Size()?, 3);
        assert_eq!(control.Items()?.GetAt(1)?, source.values[1].resolve()?);

        let replacement = new_source()?;
        let replacement_vector: IVector<IInspectable> =
            ComObject::new(ReadOnlyVector(replacement.clone())).into_interface();
        control.SetItemsSource(&replacement_vector)?;
        assert_eq!(control.Items()?.GetAt(0)?, replacement.values[0].resolve()?);
        control.SetItemsSource(None)?;
        assert_eq!(source.counts.mutations.load(Ordering::Relaxed), 0);
        assert_eq!(replacement.counts.mutations.load(Ordering::Relaxed), 0);
        assert_eq!(replacement.counts.enumerated.load(Ordering::Relaxed), 0);

        let item = source.values[0].resolve()?;
        source.counts.expected_mutations.store(7, Ordering::Relaxed);
        for result in [
            vector.SetAt(0, &item),
            vector.InsertAt(0, &item),
            vector.RemoveAt(0),
            vector.Append(&item),
            vector.RemoveAtEnd(),
            vector.Clear(),
            vector.ReplaceAll(&[Some(item.clone())]),
        ] {
            assert_eq!(result.unwrap_err().code(), E_NOTIMPL);
        }
        assert_eq!(source.counts.mutations.load(Ordering::Relaxed), 7);
        assert_eq!(vector.Size()?, 3);
        assert_eq!(vector.GetAt(0)?, item);
        let view = vector.GetView()?;
        assert_eq!(view.Size()?, 3);
        assert_eq!(view.GetAt(0)?, item);
    }

    eprintln!("ComboBox: read-only vector selection");
    let combo = native::ComboBox::new()?;
    let control = combo.cast::<native::IItemsControl>()?;
    let selector = combo.cast::<native::ISelector>()?;
    let source = new_source()?;
    let vector: IVector<IInspectable> =
        ComObject::new(ReadOnlyVector(source.clone())).into_interface();
    control.SetItemsSource(&vector)?;
    selector.SetSelectedIndex(2)?;
    assert_eq!(selector.SelectedItem()?, source.values[2].resolve()?);
    selector.SetSelectedItem(&source.values[0].resolve()?)?;
    assert_eq!(selector.SelectedIndex()?, 0);
    control.SetItemsSource(None)?;
    assert_eq!(source.counts.mutations.load(Ordering::Relaxed), 0);
    assert_eq!(source.counts.enumerated.load(Ordering::Relaxed), 0);

    eprintln!("ItemsSourceView: read-only vector");
    let source = new_source()?;
    let vector: IVector<IInspectable> =
        ComObject::new(ReadOnlyVector(source.clone())).into_interface();
    let view = native::ItemsSourceView::CreateInstance(&vector)?;
    assert_eq!(view.Count()?, 3);
    assert_eq!(source.counts.enumerated.load(Ordering::Relaxed), 0);
    assert_eq!(source.counts.indexed.load(Ordering::Relaxed), 0);
    assert_eq!(view.GetAt(1)?, source.values[1].resolve()?);
    assert_eq!(source.counts.indexed.load(Ordering::Relaxed), 1);
    drop(view);
    assert_eq!(source.counts.mutations.load(Ordering::Relaxed), 0);

    eprintln!("ItemsSourceView: vector view");
    let source = new_source()?;
    let vector_view: IVectorView<IInspectable> = source.to_interface();
    let view = native::ItemsSourceView::CreateInstance(&vector_view)?;
    assert_eq!(view.Count()?, 3);
    assert_eq!(source.counts.enumerated.load(Ordering::Relaxed), 0);
    assert_eq!(source.counts.indexed.load(Ordering::Relaxed), 0);
    assert_eq!(view.GetAt(1)?, source.values[1].resolve()?);
    assert_eq!(source.counts.indexed.load(Ordering::Relaxed), 1);

    eprintln!("ItemsSourceView: iterable");
    let source = new_source()?;
    let iterable: IIterable<IInspectable> =
        ComObject::new(IterableSource(source.clone())).into_interface();
    let view = native::ItemsSourceView::CreateInstance(&iterable)?;
    assert_eq!(source.counts.enumerated.load(Ordering::Relaxed), 3);
    assert_eq!(view.Count()?, 3);
    assert_eq!(view.GetAt(1)?, source.values[1].resolve()?);
    Ok(())
}

fn unmarshal_class<T: Interface>(value: &T) -> Result<GUID> {
    let marshaler = value.cast::<native::IMarshal>()?;
    let mut class = GUID::zeroed();
    unsafe {
        marshaler
            .GetUnmarshalClass(
                &T::IID,
                Some(value.as_raw()),
                native::MSHCTX_INPROC as u32,
                None,
                native::MSHLFLAGS_NORMAL as u32,
                &mut class,
            )
            .ok()?;
    }
    Ok(class)
}

#[derive(Default)]
struct Counts {
    indexed: AtomicUsize,
    enumerated: AtomicUsize,
    mutations: AtomicUsize,
    expected_mutations: AtomicUsize,
}

const E_NOTIMPL: HRESULT = HRESULT(0x80004001_u32 as _);

#[implement(IVector<IInspectable>, IIterable<IInspectable>)]
struct ReadOnlyVector(ComObject<ViewSource>);

impl Drop for ReadOnlyVector {
    fn drop(&mut self) {
        assert_eq!(
            self.0.counts.mutations.load(Ordering::Relaxed),
            self.0.counts.expected_mutations.load(Ordering::Relaxed),
        );
    }
}

impl ReadOnlyVector_Impl {
    fn reject_mutation(&self) -> Result<()> {
        self.0.counts.mutations.fetch_add(1, Ordering::Relaxed);
        Err(E_NOTIMPL.into())
    }
}

impl IVector_Impl<IInspectable> for ReadOnlyVector_Impl {
    fn GetAt(&self, index: u32) -> Result<IInspectable> {
        IVectorView_Impl::GetAt(&*self.0, index)
    }

    fn Size(&self) -> Result<u32> {
        IVectorView_Impl::Size(&*self.0)
    }

    fn GetView(&self) -> Result<IVectorView<IInspectable>> {
        Ok(self.0.to_interface())
    }

    fn IndexOf(&self, value: Ref<IInspectable>, index: &mut u32) -> Result<bool> {
        IVectorView_Impl::IndexOf(&*self.0, value, index)
    }

    fn GetMany(&self, start: u32, items: &mut [Option<IInspectable>]) -> Result<u32> {
        IVectorView_Impl::GetMany(&*self.0, start, items)
    }

    fn SetAt(&self, _: u32, _: Ref<IInspectable>) -> Result<()> {
        self.reject_mutation()
    }

    fn InsertAt(&self, _: u32, _: Ref<IInspectable>) -> Result<()> {
        self.reject_mutation()
    }

    fn RemoveAt(&self, _: u32) -> Result<()> {
        self.reject_mutation()
    }

    fn Append(&self, _: Ref<IInspectable>) -> Result<()> {
        self.reject_mutation()
    }

    fn RemoveAtEnd(&self) -> Result<()> {
        self.reject_mutation()
    }

    fn Clear(&self) -> Result<()> {
        self.reject_mutation()
    }

    fn ReplaceAll(&self, _: &[Option<IInspectable>]) -> Result<()> {
        self.reject_mutation()
    }
}

impl IIterable_Impl<IInspectable> for ReadOnlyVector_Impl {
    fn First(&self) -> Result<IIterator<IInspectable>> {
        IIterable_Impl::First(&*self.0)
    }
}

#[implement(IVectorView<IInspectable>, IIterable<IInspectable>)]
struct ViewSource {
    values: Vec<AgileReference<IInspectable>>,
    counts: Counts,
}

fn new_source() -> Result<ComObject<ViewSource>> {
    source_values(["one", "two", "three"])
}

fn source_values(
    values: impl IntoIterator<Item = impl AsRef<str>>,
) -> Result<ComObject<ViewSource>> {
    Ok(ComObject::new(ViewSource {
        values: values
            .into_iter()
            .map(|value| AgileReference::new(&boxed(value.as_ref())))
            .collect::<Result<_>>()?,
        counts: Counts::default(),
    }))
}

impl ViewSource_Impl {
    fn item(&self, index: usize) -> Result<IInspectable> {
        self.values
            .get(index)
            .ok_or_else(|| Error::from_hresult(HRESULT(0x8000000B_u32 as _)))?
            .resolve()
    }
}

impl IVectorView_Impl<IInspectable> for ViewSource_Impl {
    fn GetAt(&self, index: u32) -> Result<IInspectable> {
        self.counts.indexed.fetch_add(1, Ordering::Relaxed);
        self.item(index as usize)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.values.len() as u32)
    }

    fn IndexOf(&self, value: Ref<IInspectable>, index: &mut u32) -> Result<bool> {
        for (position, item) in self.values.iter().enumerate() {
            if Some(&item.resolve()?) == value.as_ref() {
                *index = position as u32;
                return Ok(true);
            }
        }
        *index = 0;
        Ok(false)
    }

    fn GetMany(&self, start: u32, items: &mut [Option<IInspectable>]) -> Result<u32> {
        let mut count = 0;
        for (item, value) in items
            .iter_mut()
            .zip(self.values.iter().skip(start as usize))
        {
            *item = Some(value.resolve()?);
            count += 1;
        }
        self.counts.indexed.fetch_add(count, Ordering::Relaxed);
        Ok(count as u32)
    }
}

impl IIterable_Impl<IInspectable> for ViewSource_Impl {
    fn First(&self) -> Result<IIterator<IInspectable>> {
        Ok(ComObject::new(SourceIterator {
            source: self.to_object(),
            current: AtomicUsize::new(0),
        })
        .into_interface())
    }
}

#[implement(IIterable<IInspectable>)]
struct IterableSource(ComObject<ViewSource>);

impl IIterable_Impl<IInspectable> for IterableSource_Impl {
    fn First(&self) -> Result<IIterator<IInspectable>> {
        IIterable_Impl::First(&*self.0)
    }
}

#[implement(IIterator<IInspectable>)]
struct SourceIterator {
    source: ComObject<ViewSource>,
    current: AtomicUsize,
}

impl IIterator_Impl<IInspectable> for SourceIterator_Impl {
    fn Current(&self) -> Result<IInspectable> {
        self.source
            .counts
            .enumerated
            .fetch_add(1, Ordering::Relaxed);
        self.source.item(self.current.load(Ordering::Relaxed))
    }

    fn HasCurrent(&self) -> Result<bool> {
        Ok(self.current.load(Ordering::Relaxed) < self.source.values.len())
    }

    fn MoveNext(&self) -> Result<bool> {
        let current = self.current.load(Ordering::Relaxed);
        if current < self.source.values.len() {
            self.current.fetch_add(1, Ordering::Relaxed);
        }
        Ok(current + 1 < self.source.values.len())
    }

    fn GetMany(&self, items: &mut [Option<IInspectable>]) -> Result<u32> {
        let start = self.current.load(Ordering::Relaxed);
        let mut count = 0;
        for (item, value) in items.iter_mut().zip(self.source.values.iter().skip(start)) {
            *item = Some(value.resolve()?);
            count += 1;
        }
        self.current.fetch_add(count, Ordering::Relaxed);
        self.source
            .counts
            .enumerated
            .fetch_add(count, Ordering::Relaxed);
        Ok(count as u32)
    }
}
