use super::*;
use std::collections::HashSet;
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
    source_revision: Rc<Cell<u64>>,
    shells: Rc<RealizedShells>,
    wake: EventSink,
}

pub struct VirtualHandle {
    _factory: IElementFactory,
    pub repeater: bindings::ItemsRepeater,
    _shells: Rc<RealizedShells>,
    source: windows_collections::IObservableVector<IInspectable>,
    source_revision: Rc<Cell<u64>>,
}

impl VirtualHandle {
    pub fn create(
        identity: WindowToken,
        collection: NodeId,
        item_count: usize,
        source_revision: u64,
        requests: Rc<RefCell<Vec<NativeWork<RealizationRequest>>>>,
        wake: EventSink,
    ) -> Result<Self> {
        let shells = Rc::new(RealizedShells::default());
        let source_revision = Rc::new(Cell::new(source_revision));
        let factory = ReactorElementFactory::create(
            identity,
            collection,
            requests,
            Rc::clone(&source_revision),
            Rc::clone(&shells),
            wake,
        );
        let values = item_values(item_count)?;
        let source: windows_collections::IObservableVector<IInspectable> = values.into();
        let repeater = bindings::ItemsRepeater::new()?;
        repeater.SetItemTemplate(&factory)?;
        repeater.SetItemsSource(&source)?;
        Ok(Self {
            _factory: factory,
            repeater,
            _shells: shells,
            source,
            source_revision,
        })
    }

    pub fn ui_element(&self) -> Result<UIElement> {
        self.repeater.cast()
    }

    pub fn reset(&self, item_count: usize, source_revision: u64) -> Result<()> {
        let values = item_values(item_count)?;
        self.source_revision.set(source_revision);
        self.source.ReplaceAll(&values)
    }

    pub fn set_content(
        &self,
        container: RealizedContainer,
        content: Option<&UIElement>,
    ) -> Result<()> {
        let shell = self
            ._shells
            .borrow()
            .shell(container)
            .ok_or_else(|| Error::new(E_FAIL, "missing realized container"))?;
        if let Some(content) = content {
            shell.SetContent(content)
        } else {
            shell.SetContent(None::<&IInspectable>)
        }
    }

    pub fn clear_content(&self, container: RealizedContainer) -> Result<()> {
        self._shells.clear_content(container)
    }

    pub fn acknowledge_recycle(&self, container: RealizedContainer) -> Result<()> {
        self._shells.acknowledge_recycle(container)
    }

    #[cfg(feature = "test")]
    pub fn shell_counts(&self) -> (usize, usize) {
        (self._shells.len(), self._shells.retired_len())
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
    pool: RefCell<ShellPool<bindings::ContentControl>>,
}

struct ShellPool<T> {
    available: Vec<T>,
    next: u64,
    retired: HashSet<RealizedContainer>,
    shells: HashMap<RealizedContainer, T>,
}

impl<T> Default for ShellPool<T> {
    fn default() -> Self {
        Self {
            available: Vec::new(),
            next: 0,
            retired: HashSet::new(),
            shells: HashMap::new(),
        }
    }
}

impl<T: Clone> ShellPool<T> {
    fn next_container(&mut self) -> RealizedContainer {
        let value = self.next;
        self.next = value.checked_add(1).unwrap();
        RealizedContainer(value)
    }

    fn take(&mut self, create: impl FnOnce() -> Result<T>) -> Result<(RealizedContainer, T)> {
        let container = self.next_container();
        let shell = if let Some(shell) = self.available.pop() {
            shell
        } else {
            create()?
        };
        self.shells.insert(container, shell.clone());
        Ok((container, shell))
    }

    fn shell(&self, container: RealizedContainer) -> Option<T> {
        self.shells.get(&container).cloned()
    }

    fn retire(&mut self, container: RealizedContainer) -> bool {
        let Some(shell) = self.shells.remove(&container) else {
            return false;
        };
        self.available.push(shell);
        self.retired.insert(container);
        true
    }

    fn take_retired(&mut self, container: RealizedContainer) -> bool {
        self.retired.remove(&container)
    }

    fn acknowledge_recycle(&mut self, container: RealizedContainer) -> Result<()> {
        if self.shells.contains_key(&container) {
            return Err(Error::new(E_FAIL, "cannot acknowledge a live container"));
        }
        self.retired.remove(&container);
        Ok(())
    }
}

impl RealizedShells {
    fn take(&self) -> Result<(RealizedContainer, UIElement)> {
        let (container, shell) = self.pool.borrow_mut().take(|| {
            let shell = bindings::ContentControl::new()?;
            shell
                .cast::<IFrameworkElement>()?
                .SetMinHeight(ESTIMATED_ROW_HEIGHT)?;
            Ok(shell)
        })?;
        let element = shell.cast()?;
        Ok((container, element))
    }

    fn recycle(&self, element: &UIElement) -> Result<Option<RealizedContainer>> {
        let Some((container, shell)) =
            self.pool
                .borrow()
                .shells
                .iter()
                .find_map(|(container, shell)| {
                    (shell.cast::<UIElement>().as_ref() == Ok(element))
                        .then(|| (*container, shell.clone()))
                })
        else {
            return Ok(None);
        };
        shell.SetContent(None::<&IInspectable>)?;
        if !self.pool.borrow_mut().retire(container) {
            return Err(Error::new(E_FAIL, "realized container retired twice"));
        }
        Ok(Some(container))
    }

    pub fn len(&self) -> usize {
        self.pool.borrow().shells.len()
    }

    #[cfg(feature = "test")]
    pub fn retired_len(&self) -> usize {
        self.pool.borrow().retired.len()
    }

    fn clear_content(&self, container: RealizedContainer) -> Result<()> {
        let mut pool = self.pool.borrow_mut();
        if let Some(shell) = pool.shell(container) {
            return shell.SetContent(None::<&IInspectable>);
        }
        // RecycleElement clears and retires the shell before the queued Pump work runs.
        if pool.take_retired(container) {
            Ok(())
        } else {
            Err(Error::new(E_FAIL, "missing realized container"))
        }
    }

    fn acknowledge_recycle(&self, container: RealizedContainer) -> Result<()> {
        self.pool.borrow_mut().acknowledge_recycle(container)
    }

    fn borrow(&self) -> std::cell::Ref<'_, ShellPool<bindings::ContentControl>> {
        self.pool.borrow()
    }
}

impl ReactorElementFactory {
    pub fn create(
        identity: WindowToken,
        collection: NodeId,
        requests: Rc<RefCell<Vec<NativeWork<RealizationRequest>>>>,
        source_revision: Rc<Cell<u64>>,
        shells: Rc<RealizedShells>,
        wake: EventSink,
    ) -> IElementFactory {
        ComObject::new(Self {
            collection,
            identity,
            requests,
            source_revision,
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
            source_revision: self.source_revision.get(),
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
            source_revision: self.source_revision.get(),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn every_shell_lifetime_gets_a_fresh_token() {
        let mut shells = ShellPool::<usize>::default();

        assert_eq!(shells.next_container(), RealizedContainer(0));
        assert_eq!(shells.next_container(), RealizedContainer(1));
    }

    #[test]
    fn shell_lifetime_token_exhaustion_panics_before_mutation() {
        let mut shells = ShellPool::<usize> {
            next: u64::MAX,
            ..Default::default()
        };

        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| shells.next_container()))
                .is_err()
        );
        assert_eq!(shells.next, u64::MAX);
    }

    #[test]
    fn reused_physical_shell_gets_a_new_live_token() {
        let mut shells = ShellPool::<usize>::default();
        let (old, physical) = shells.take(|| Ok(42)).unwrap();

        assert!(shells.retire(old));
        assert_eq!(shells.shell(old), None);
        assert!(shells.take_retired(old));
        assert!(!shells.take_retired(old));

        let (new, reused) = shells
            .take(|| panic!("available shell was not reused"))
            .unwrap();
        assert_ne!(new, old);
        assert_eq!(reused, physical);
        assert_eq!(shells.shell(new), Some(physical));
    }

    #[test]
    fn recycle_acknowledgement_releases_only_retired_tokens() {
        let mut shells = ShellPool::<usize>::default();
        let (retired, _) = shells.take(|| Ok(42)).unwrap();
        assert!(shells.retire(retired));

        shells.acknowledge_recycle(retired).unwrap();
        assert!(shells.retired.is_empty());
        shells.acknowledge_recycle(retired).unwrap();

        let (live, _) = shells.take(|| Ok(43)).unwrap();
        assert!(shells.acknowledge_recycle(live).is_err());
    }

    #[test]
    fn replacing_virtual_items_raises_one_source_change() {
        let source: windows_collections::IObservableVector<IInspectable> =
            item_values(2).unwrap().into();
        let changes = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&changes);
        let _changed = source
            .VectorChanged(move |_, _| {
                observed.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();

        source.ReplaceAll(&item_values(100).unwrap()).unwrap();

        assert_eq!(changes.load(Ordering::Relaxed), 1);
    }
}
