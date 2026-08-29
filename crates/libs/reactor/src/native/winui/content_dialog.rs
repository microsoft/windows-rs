use super::*;

#[derive(Default)]
pub(super) struct ContentDialogScheduler {
    dialogs: HashMap<NodeId, ContentDialogLifecycle>,
    next_generation: u64,
    request_order: u64,
    subscriptions: HashMap<NodeId, NativeSubscription>,
}

pub(super) enum ContentDialogAction {
    None,
    WaitForRoot(u64),
    Hide(bindings::ContentDialog),
}

struct ContentDialogLifecycle {
    dialog: bindings::ContentDialog,
    desired_open: bool,
    generation: u64,
    pending: bool,
    queued: bool,
    request_order: u64,
    retired: bool,
    revision: u32,
    root_loaded: Option<windows_core::EventRevoker>,
    suppress_callback: bool,
    xaml_root: Option<XamlRoot>,
}

#[cfg(feature = "test")]
#[derive(Debug)]
pub(crate) struct LiveContentDialogState {
    pub(crate) desired_open: bool,
    pub(crate) generation: u64,
    pub(crate) node: NodeId,
    pub(crate) pending: bool,
    pub(crate) queued: bool,
}

impl ContentDialogScheduler {
    pub(super) fn contains(&self, node: NodeId) -> bool {
        self.dialogs.contains_key(&node)
    }

    pub(super) fn create(&mut self, node: NodeId, dialog: bindings::ContentDialog) {
        self.next_generation += 1;
        self.dialogs.insert(
            node,
            ContentDialogLifecycle {
                dialog,
                desired_open: false,
                generation: self.next_generation,
                pending: false,
                queued: false,
                request_order: 0,
                retired: false,
                revision: 0,
                root_loaded: None,
                suppress_callback: false,
                xaml_root: None,
            },
        );
    }

    pub(super) fn set_open(
        &mut self,
        node: NodeId,
        open: bool,
        xaml_root: Option<XamlRoot>,
    ) -> Result<ContentDialogAction, RuntimeError> {
        let occupied = xaml_root
            .as_ref()
            .is_some_and(|root| self.root_occupied(root, Some(node)));
        let state = self
            .dialogs
            .get_mut(&node)
            .ok_or(RuntimeError::MissingNode(node))?;
        if state.desired_open == open {
            return Ok(ContentDialogAction::None);
        }
        state.desired_open = open;
        if open {
            if let Some(xaml_root) = xaml_root {
                state.xaml_root = Some(xaml_root.clone());
                state
                    .dialog
                    .cast::<IUIElement>()
                    .and_then(|dialog| dialog.SetXamlRoot(&xaml_root))
                    .map_err(native_error)?;
            }
            if state.xaml_root.is_none() {
                self.request_order += 1;
                state.request_order = self.request_order;
                return Ok(ContentDialogAction::WaitForRoot(state.generation));
            }
            if state.pending || occupied {
                state.queued = true;
                self.request_order += 1;
                state.request_order = self.request_order;
            } else {
                show(&state.dialog)?;
                state.pending = true;
            }
        } else {
            state.queued = false;
            state.root_loaded = None;
            if state.pending {
                state.suppress_callback = true;
                return Ok(ContentDialogAction::Hide(state.dialog.clone()));
            }
        }
        Ok(ContentDialogAction::None)
    }

    pub(super) fn set_root_loaded(
        &mut self,
        node: NodeId,
        loaded: windows_core::EventRevoker,
    ) -> Result<(), RuntimeError> {
        self.dialogs
            .get_mut(&node)
            .ok_or(RuntimeError::MissingNode(node))?
            .root_loaded = Some(loaded);
        Ok(())
    }

    pub(super) fn root_ready(
        &mut self,
        node: NodeId,
        generation: u64,
        xaml_root: XamlRoot,
    ) -> Result<(), RuntimeError> {
        let occupied = self.root_occupied(&xaml_root, Some(node));
        let Some(state) = self.dialogs.get_mut(&node) else {
            return Ok(());
        };
        if state.generation != generation || !state.desired_open || state.retired {
            return Ok(());
        }
        state.xaml_root = Some(xaml_root.clone());
        state
            .dialog
            .cast::<IUIElement>()
            .and_then(|dialog| dialog.SetXamlRoot(&xaml_root))
            .map_err(native_error)?;
        if occupied {
            state.queued = true;
        } else {
            show(&state.dialog)?;
            state.pending = true;
        }
        Ok(())
    }

    pub(super) fn has_subscription(&self, node: NodeId) -> bool {
        self.subscriptions.contains_key(&node)
    }

    pub(super) fn subscribe(
        &mut self,
        node: NodeId,
        revision: u32,
        subscription: NativeSubscription,
    ) -> Result<(), RuntimeError> {
        self.dialogs
            .get_mut(&node)
            .ok_or(RuntimeError::MissingNode(node))?
            .revision = revision;
        self.subscriptions.insert(node, subscription);
        Ok(())
    }

    pub(super) fn unsubscribe(
        &mut self,
        node: NodeId,
        event: EventId,
    ) -> Result<bool, RuntimeError> {
        let Some(state) = self.dialogs.get(&node) else {
            return Ok(false);
        };
        if !state.pending {
            self.subscriptions
                .remove(&node)
                .ok_or(RuntimeError::MissingSubscription(node, event))?;
        }
        Ok(true)
    }

    #[cfg(feature = "test")]
    pub(super) fn subscription_count(&self) -> usize {
        self.subscriptions.len()
    }

    #[cfg(feature = "test")]
    pub(super) fn states(&self) -> Vec<LiveContentDialogState> {
        let mut states = self
            .dialogs
            .iter()
            .map(|(node, state)| LiveContentDialogState {
                desired_open: state.desired_open,
                generation: state.generation,
                node: *node,
                pending: state.pending,
                queued: state.queued,
            })
            .collect::<Vec<_>>();
        states.sort_unstable_by_key(|state| state.generation);
        states
    }

    #[cfg(feature = "test")]
    pub(super) fn dialog(&self, node: NodeId) -> Result<bindings::ContentDialog, RuntimeError> {
        Ok(self
            .dialogs
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?
            .dialog
            .clone())
    }

    fn root_occupied(&self, root: &XamlRoot, except: Option<NodeId>) -> bool {
        self.dialogs.iter().any(|(node, state)| {
            Some(*node) != except
                && state.pending
                && state
                    .xaml_root
                    .as_ref()
                    .is_some_and(|current| current == root)
        })
    }
}

pub(super) fn reset(scheduler: &Rc<RefCell<ContentDialogScheduler>>) {
    let dialogs = {
        let mut scheduler = scheduler.borrow_mut();
        let dialogs = scheduler
            .dialogs
            .drain()
            .map(|(_, state)| state)
            .collect::<Vec<_>>();
        scheduler.subscriptions.clear();
        scheduler.request_order = 0;
        dialogs
    };
    for state in dialogs {
        if state.pending {
            _ = state.dialog.Hide();
        }
    }
}

pub(super) fn retire(scheduler: &Rc<RefCell<ContentDialogScheduler>>, node: NodeId) {
    let retain = {
        let mut scheduler = scheduler.borrow_mut();
        if let Some(dialog) = scheduler.dialogs.get_mut(&node) {
            if dialog.pending {
                dialog.retired = true;
                true
            } else {
                scheduler.dialogs.remove(&node);
                false
            }
        } else {
            false
        }
    };
    if !retain {
        scheduler.borrow_mut().subscriptions.remove(&node);
    }
}

pub(super) fn closed(
    scheduler: &Rc<RefCell<ContentDialogScheduler>>,
    sink: &EventSink,
    node: NodeId,
) -> Result<bool, RuntimeError> {
    let (xaml_root, invoke_callback, retired) = {
        let mut scheduler = scheduler.borrow_mut();
        let Some(state) = scheduler.dialogs.get_mut(&node) else {
            return Ok(false);
        };
        if !state.pending {
            return Ok(false);
        }
        state.pending = false;
        let invoke_callback = !state.suppress_callback;
        state.suppress_callback = false;
        (state.xaml_root.clone(), invoke_callback, state.retired)
    };

    let candidate = if let Some(root) = xaml_root.as_ref() {
        scheduler
            .borrow()
            .dialogs
            .iter()
            .filter(|(_, state)| {
                state.queued
                    && state.desired_open
                    && !state.retired
                    && state
                        .xaml_root
                        .as_ref()
                        .is_some_and(|current| current == root)
            })
            .min_by_key(|(_, state)| state.request_order)
            .map(|(node, state)| (*node, state.generation))
    } else {
        None
    };
    if let Some((candidate, generation)) = candidate {
        let callback_sink = sink.clone();
        let candidate_root = xaml_root.unwrap();
        let handler = DispatcherQueueHandler::new(move || {
            if callback_sink.current_identity.get() != Some(callback_sink.identity) {
                return;
            }
            let (candidate, result) = {
                let mut scheduler = callback_sink.content_dialogs.borrow_mut();
                if scheduler.root_occupied(&candidate_root, None) {
                    return;
                }
                let candidate = scheduler
                    .dialogs
                    .get(&candidate)
                    .filter(|state| {
                        state.generation == generation
                            && state.desired_open
                            && !state.retired
                            && !state.pending
                            && state.queued
                            && state
                                .xaml_root
                                .as_ref()
                                .is_some_and(|root| root == &candidate_root)
                    })
                    .map(|_| candidate)
                    .or_else(|| {
                        scheduler
                            .dialogs
                            .iter()
                            .filter(|(_, state)| {
                                state.desired_open
                                    && !state.retired
                                    && !state.pending
                                    && state.queued
                                    && state
                                        .xaml_root
                                        .as_ref()
                                        .is_some_and(|root| root == &candidate_root)
                            })
                            .min_by_key(|(_, state)| state.request_order)
                            .map(|(node, _)| *node)
                    });
                let Some(candidate) = candidate else {
                    return;
                };
                let state = scheduler.dialogs.get_mut(&candidate).unwrap();
                state.queued = false;
                (
                    candidate,
                    state.dialog.ShowAsync().map(|_| {
                        state.pending = true;
                    }),
                )
            };
            if let Err(error) = result {
                let revision = callback_sink
                    .content_dialogs
                    .borrow()
                    .dialogs
                    .get(&candidate)
                    .map_or(0, |state| state.revision);
                callback_sink.error(
                    candidate,
                    EventId::ContentDialogClosed,
                    revision,
                    native_error(error),
                );
            }
        });
        match sink
            .dispatcher
            .TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &handler)
        {
            Ok(true) => {}
            Ok(false) => return Err(RuntimeError::DispatcherRejected),
            Err(error) => return Err(native_error(error)),
        }
    }
    if retired {
        scheduler.borrow_mut().dialogs.remove(&node);
        scheduler.borrow_mut().subscriptions.remove(&node);
    }
    Ok(invoke_callback)
}

fn show(dialog: &bindings::ContentDialog) -> Result<(), RuntimeError> {
    match dialog.ShowAsync() {
        Ok(operation) => {
            drop(operation);
            Ok(())
        }
        // WinUI may return S_OK without an async handle. The Closed event owns completion, so
        // the runtime does not depend on that handle.
        Err(error) if error.code().is_ok() => Ok(()),
        Err(error) => Err(native_error(error)),
    }
}
