use super::*;

static NEXT_CONTEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) struct ContextId(u64);

#[derive(Clone)]
pub struct Context<T> {
    pub(super) default: T,
    pub(super) id: ContextId,
}

impl<T> Context<T> {
    pub fn new(default: T) -> Self {
        Self {
            default,
            id: ContextId(NEXT_CONTEXT_ID.fetch_add(1, Ordering::Relaxed)),
        }
    }
}

#[derive(Clone)]
pub(super) struct ContextValue {
    pub(super) equals: fn(&dyn Any, &dyn Any) -> bool,
    pub(super) value: Rc<dyn Any>,
}

#[derive(Clone)]
pub(crate) struct ContextProvision {
    pub(super) id: ContextId,
    pub(super) value: ContextValue,
}

impl ContextProvision {
    fn new<T: Clone + PartialEq + 'static>(context: &Context<T>, value: T) -> Self {
        Self {
            id: context.id,
            value: ContextValue {
                equals: |left, right| {
                    left.downcast_ref::<T>()
                        .zip(right.downcast_ref::<T>())
                        .is_some_and(|(left, right)| left == right)
                },
                value: Rc::new(value),
            },
        }
    }
}

impl fmt::Debug for ContextProvision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ContextProvision")
            .field(&self.id)
            .finish()
    }
}

impl PartialEq for ContextProvision {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && (self.value.equals)(self.value.value.as_ref(), other.value.value.as_ref())
    }
}

pub fn provide<T>(context: &Context<T>, value: T, child: impl Into<View>) -> View
where
    T: Clone + PartialEq + 'static,
{
    View(DeclaredNode::Provider(Box::new(DeclaredProvider {
        provision: ContextProvision::new(context, value),
        child: child.into().0,
    })))
}

pub(super) struct ContextEnvironment<'a> {
    pub(super) contexts: &'a HashMap<ContextId, ContextValue>,
    pub(super) provided: &'a HashSet<ContextId>,
}

pub(super) struct OwnedContextEnvironment {
    pub(super) contexts: HashMap<ContextId, ContextValue>,
    pub(super) provided: HashSet<ContextId>,
}

impl<A: Adapter> ComponentHost<A> {
    pub(super) fn replace_dependencies(
        &mut self,
        id: ComponentId,
        dependencies: HashSet<ContextId>,
    ) {
        let previous = std::mem::replace(
            &mut self.scope_mut(id).unwrap().dependencies,
            dependencies.clone(),
        );
        for context in previous.difference(&dependencies) {
            let remove = if let Some(consumers) = self.context_consumers.get_mut(context) {
                consumers.remove(&id);
                consumers.is_empty()
            } else {
                false
            };
            if remove {
                self.context_consumers.remove(context);
            }
        }
        for context in dependencies.difference(&previous) {
            self.context_consumers
                .entry(*context)
                .or_default()
                .insert(id);
        }
    }
}
