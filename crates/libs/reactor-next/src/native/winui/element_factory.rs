use super::*;
use windows_core::*;
use windows_reference::IReference;

const ESTIMATED_ROW_HEIGHT: f64 = 24.0;

implement_decl! {
    impl ReactorElementFactory as pub ReactorElementFactory_Impl: [IElementFactory]
}

pub struct ReactorElementFactory {
    collection: NodeId,
    identity: WindowToken,
    requests: Rc<RefCell<Vec<NativeWork<RealizationRequest>>>>,
    shells: Rc<RealizedShells>,
    wake: EventSink,
}

pub struct VirtualHandle {
    _factory: IElementFactory,
    pub repeater: bindings::ItemsRepeater,
    _shells: Rc<RealizedShells>,
    source: windows_collections::IObservableVector<IInspectable>,
}

impl VirtualHandle {
    pub fn create(
        identity: WindowToken,
        collection: NodeId,
        item_count: usize,
        requests: Rc<RefCell<Vec<NativeWork<RealizationRequest>>>>,
        wake: EventSink,
    ) -> Result<Self> {
        let shells = Rc::new(RealizedShells::default());
        let factory =
            ReactorElementFactory::create(identity, collection, requests, Rc::clone(&shells), wake);
        let values = item_values(item_count)?;
        let source: windows_collections::IObservableVector<IInspectable> = values.into();
        let repeater = bindings::ItemsRepeater::new()?;
        let interface = repeater.cast::<IItemsRepeater>()?;
        interface.SetItemTemplate(&factory)?;
        interface.SetItemsSource(&source)?;
        Ok(Self {
            _factory: factory,
            repeater,
            _shells: shells,
            source,
        })
    }

    pub fn ui_element(&self) -> Result<UIElement> {
        self.repeater.cast()
    }

    pub fn reset(&self, item_count: usize) -> Result<()> {
        let values = item_values(item_count)?;
        self.source.Clear()?;
        for value in values {
            self.source.Append(value.as_ref())?;
        }
        Ok(())
    }

    pub fn set_content(
        &self,
        container: RealizedContainer,
        content: Option<&UIElement>,
    ) -> Result<()> {
        let shell = self
            ._shells
            .shells
            .borrow()
            .get(&container)
            .cloned()
            .ok_or_else(|| Error::new(E_FAIL, "missing realized container"))?;
        if let Some(content) = content {
            let content = content.cast::<IInspectable>()?;
            shell.SetContent(&content)
        } else {
            shell.SetContent(None::<&IInspectable>)
        }
    }
}

fn item_values(item_count: usize) -> Result<Vec<Option<IInspectable>>> {
    (0..item_count)
        .map(|index| {
            i32::try_from(index)
                .map(|index| Some(IReference::<i32>::from(index).into()))
                .map_err(|_| Error::new(E_FAIL, "item count exceeds i32"))
        })
        .collect()
}

#[derive(Default)]
pub struct RealizedShells {
    available: RefCell<Vec<RealizedContainer>>,
    next: Cell<u64>,
    shells: RefCell<HashMap<RealizedContainer, bindings::ContentControl>>,
}

impl RealizedShells {
    fn take(&self) -> Result<(RealizedContainer, UIElement)> {
        if let Some(container) = self.available.borrow_mut().pop() {
            let shell = self.shells.borrow()[&container].clone();
            return Ok((container, shell.cast()?));
        }

        let value = self.next.get();
        self.next.set(
            value
                .checked_add(1)
                .ok_or_else(|| Error::new(E_FAIL, "realized container id exhausted"))?,
        );
        let container = RealizedContainer(value);
        let shell = bindings::ContentControl::new()?;
        shell
            .cast::<IFrameworkElement>()?
            .SetMinHeight(ESTIMATED_ROW_HEIGHT)?;
        let element = shell.cast()?;
        self.shells.borrow_mut().insert(container, shell);
        Ok((container, element))
    }

    fn recycle(&self, element: &UIElement) -> Result<Option<RealizedContainer>> {
        let Some((container, shell)) =
            self.shells.borrow().iter().find_map(|(container, shell)| {
                (shell.cast::<UIElement>().as_ref() == Ok(element))
                    .then(|| (*container, shell.clone()))
            })
        else {
            return Ok(None);
        };
        shell.SetContent(None::<&IInspectable>)?;
        let mut available = self.available.borrow_mut();
        if !available.contains(&container) {
            available.push(container);
        }
        Ok(Some(container))
    }

    pub fn len(&self) -> usize {
        self.shells.borrow().len()
    }
}

impl ReactorElementFactory {
    pub fn create(
        identity: WindowToken,
        collection: NodeId,
        requests: Rc<RefCell<Vec<NativeWork<RealizationRequest>>>>,
        shells: Rc<RealizedShells>,
        wake: EventSink,
    ) -> IElementFactory {
        ComObject::new(Self {
            collection,
            identity,
            requests,
            shells,
            wake,
        })
        .into_interface()
    }

    fn queue(&self, request: RealizationRequest) {
        self.requests.borrow_mut().push(NativeWork {
            identity: self.identity,
            work: request,
        });
        self.wake.wake();
    }
}

impl IElementFactory_Impl for ReactorElementFactory_Impl {
    fn GetElement(&self, args: Ref<ElementFactoryGetArgs>) -> Result<UIElement> {
        let data = args.ok()?.Data()?;
        let index = usize::try_from(data.cast::<IReference<i32>>()?.Value()?)
            .map_err(|_| Error::new(E_FAIL, "negative item index"))?;
        let (container, element) = self.shells.take()?;
        self.queue(RealizationRequest::Realize {
            collection: self.collection,
            container,
            index,
        });
        Ok(element)
    }

    fn RecycleElement(&self, args: Ref<ElementFactoryRecycleArgs>) -> Result<()> {
        let element = args.ok()?.Element()?;
        let container = self
            .shells
            .recycle(&element)?
            .ok_or_else(|| Error::new(E_FAIL, "element factory received an unknown shell"))?;
        self.queue(RealizationRequest::Recycle {
            collection: self.collection,
            container,
        });
        Ok(())
    }
}
