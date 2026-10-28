use super::*;

struct NativeContentDialogState {
    value: native::ContentDialog,
    owner: Option<ObjectId>,
    desired_open: bool,
    root: Option<native::XamlRoot>,
    loaded: Option<windows_core::EventRevoker>,
    theme_changed: Option<windows_core::EventRevoker>,
    pending_generation: Option<u64>,
    suppress_generation: Option<u64>,
    queued: bool,
    retired: bool,
    retained_handle: Option<GeneratedContentDialog>,
}

struct NativeContentDialogRoot {
    root: native::XamlRoot,
    schedule: ContentDialogSchedule,
}

#[derive(Default)]
pub(super) struct ContentDialogSchedule {
    pub(super) active: Option<(ObjectId, u64)>,
    queue: VecDeque<ObjectId>,
}

impl ContentDialogSchedule {
    pub(super) fn enqueue(&mut self, dialog: ObjectId) {
        if !self.queue.contains(&dialog) {
            self.queue.push_back(dialog);
        }
    }

    pub(super) fn cancel(&mut self, dialog: ObjectId) {
        self.queue.retain(|current| *current != dialog);
    }

    pub(super) fn next(&mut self) -> Option<ObjectId> {
        self.queue.pop_front()
    }

    pub(super) fn begin(&mut self, dialog: ObjectId, generation: u64) {
        self.active = Some((dialog, generation));
    }

    pub(super) fn complete(&mut self, dialog: ObjectId, generation: u64) -> bool {
        if self.active != Some((dialog, generation)) {
            return false;
        }
        self.active = None;
        true
    }
}

#[derive(Default)]
pub(super) struct NativeContentDialogScheduler {
    dialogs: HashMap<ObjectId, NativeContentDialogState>,
    roots: Vec<NativeContentDialogRoot>,
    next_generation: u64,
}

impl NativeContentDialogScheduler {
    pub(super) fn create(&mut self, object: ObjectId, value: native::ContentDialog) {
        self.dialogs.insert(
            object,
            NativeContentDialogState {
                value,
                owner: None,
                desired_open: false,
                root: None,
                loaded: None,
                theme_changed: None,
                pending_generation: None,
                suppress_generation: None,
                queued: false,
                retired: false,
                retained_handle: None,
            },
        );
    }

    fn root_index(&self, root: &native::XamlRoot) -> Option<usize> {
        self.roots.iter().position(|state| state.root == *root)
    }

    fn ensure_root(&mut self, root: native::XamlRoot) -> usize {
        if let Some(index) = self.root_index(&root) {
            index
        } else {
            self.roots.push(NativeContentDialogRoot {
                root,
                schedule: ContentDialogSchedule::default(),
            });
            self.roots.len() - 1
        }
    }

    fn remove_from_queues(&mut self, dialog: ObjectId) {
        for root in &mut self.roots {
            root.schedule.cancel(dialog);
        }
        if let Some(state) = self.dialogs.get_mut(&dialog) {
            state.queued = false;
        }
    }

    fn set_root(&mut self, dialog: ObjectId, root: native::XamlRoot) -> Result<(), WinUiError> {
        self.remove_from_queues(dialog);
        let state = self
            .dialogs
            .get_mut(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?;
        state
            .value
            .cast::<native::IUIElement>()?
            .SetXamlRoot(&root)?;
        state.root = Some(root);
        state.loaded = None;
        if state.desired_open && state.pending_generation.is_none() {
            self.enqueue(dialog)?;
        }
        Ok(())
    }

    fn enqueue(&mut self, dialog: ObjectId) -> Result<(), WinUiError> {
        let root = self
            .dialogs
            .get(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?
            .root
            .clone();
        let Some(root) = root else {
            return Ok(());
        };
        let index = self.ensure_root(root);
        self.roots[index].schedule.enqueue(dialog);
        self.dialogs.get_mut(&dialog).unwrap().queued = true;
        Ok(())
    }

    fn set_open(
        &mut self,
        dialog: ObjectId,
        open: bool,
    ) -> Result<Option<native::ContentDialog>, WinUiError> {
        let state = self
            .dialogs
            .get_mut(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?;
        if state.desired_open == open {
            return Ok(None);
        }
        state.desired_open = open;
        if !open {
            let pending = state.pending_generation;
            let value = state.value.clone();
            self.remove_from_queues(dialog);
            if let Some(generation) = pending {
                self.dialogs.get_mut(&dialog).unwrap().suppress_generation = Some(generation);
                return Ok(Some(value));
            }
            return Ok(None);
        }
        if state.pending_generation.is_some() {
            self.enqueue(dialog)?;
            return Ok(None);
        }
        self.enqueue(dialog)?;
        self.start_next_for_dialog(dialog)?;
        Ok(None)
    }

    fn start_next_for_dialog(&mut self, dialog: ObjectId) -> Result<(), WinUiError> {
        let Some(root) = self
            .dialogs
            .get(&dialog)
            .and_then(|state| state.root.clone())
        else {
            return Ok(());
        };
        self.start_next(&root)
    }

    fn start_next(&mut self, root: &native::XamlRoot) -> Result<(), WinUiError> {
        let Some(index) = self.root_index(root) else {
            return Ok(());
        };
        if self.roots[index].schedule.active.is_some() {
            return Ok(());
        }
        while let Some(dialog) = self.roots[index].schedule.next() {
            let eligible = self.dialogs.get(&dialog).is_some_and(|state| {
                state.desired_open
                    && !state.retired
                    && state.pending_generation.is_none()
                    && state.root.as_ref().is_some_and(|current| current == root)
            });
            if !eligible {
                if let Some(state) = self.dialogs.get_mut(&dialog) {
                    state.queued = false;
                }
                continue;
            }
            self.next_generation = self.next_generation.wrapping_add(1);
            let generation = self.next_generation;
            let value = {
                let state = self.dialogs.get_mut(&dialog).unwrap();
                state.queued = false;
                state.pending_generation = Some(generation);
                state.suppress_generation = None;
                state
                    .value
                    .cast::<native::IUIElement>()?
                    .SetXamlRoot(root)?;
                state.value.clone()
            };
            self.roots[index].schedule.begin(dialog, generation);
            if let Err(error) = show_content_dialog(&value) {
                self.roots[index].schedule.active = None;
                self.dialogs.get_mut(&dialog).unwrap().pending_generation = None;
                return Err(error);
            }
            break;
        }
        Ok(())
    }

    fn closed(
        &mut self,
        dialog: ObjectId,
        generation: u64,
    ) -> Result<(bool, Option<native::XamlRoot>), WinUiError> {
        let state = self
            .dialogs
            .get_mut(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?;
        if state.pending_generation != Some(generation) {
            return Ok((false, None));
        }
        let root = state.root.clone();
        let dispatch = state.suppress_generation != Some(generation);
        state.pending_generation = None;
        state.suppress_generation = None;
        if state.desired_open && !state.retired {
            self.enqueue(dialog)?;
        }
        if let Some(root) = &root
            && let Some(index) = self.root_index(root)
        {
            self.roots[index].schedule.complete(dialog, generation);
        }
        Ok((dispatch, root))
    }

    fn cleanup_retired(&mut self) {
        let retired = self
            .dialogs
            .iter()
            .filter_map(|(object, state)| {
                (state.retired && state.pending_generation.is_none()).then_some(*object)
            })
            .collect::<Vec<_>>();
        for object in retired {
            self.remove_from_queues(object);
            self.dialogs.remove(&object);
        }
        self.roots
            .retain(|root| root.schedule.active.is_some() || !root.schedule.queue.is_empty());
    }

    pub(super) fn retire(
        &mut self,
        dialog: ObjectId,
        handle: GeneratedContentDialog,
    ) -> Result<bool, WinUiError> {
        let state = self
            .dialogs
            .get_mut(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?;
        state.retired = true;
        state.owner = None;
        state.loaded = None;
        state.theme_changed = None;
        state.desired_open = false;
        let pending = state.pending_generation.is_some();
        let _ = state;
        self.remove_from_queues(dialog);
        if pending {
            self.dialogs.get_mut(&dialog).unwrap().retained_handle = Some(handle);
            Ok(true)
        } else {
            self.dialogs.remove(&dialog);
            Ok(false)
        }
    }

    pub(super) fn reset(&mut self) -> Vec<native::ContentDialog> {
        let mut hide = Vec::new();
        for state in self.dialogs.values_mut() {
            state.desired_open = false;
            state.loaded = None;
            state.theme_changed = None;
            if let Some(generation) = state.pending_generation {
                state.suppress_generation = Some(generation);
                hide.push(state.value.clone());
            }
        }
        for root in &mut self.roots {
            root.schedule.queue.clear();
        }
        hide
    }
}

pub(super) fn content_dialog_result(value: native::ContentDialogResult) -> ContentDialogResult {
    match value {
        native::ContentDialogResult::Primary => ContentDialogResult::Primary,
        native::ContentDialogResult::Secondary => ContentDialogResult::Secondary,
        _ => ContentDialogResult::None,
    }
}

fn show_content_dialog(value: &native::ContentDialog) -> Result<(), WinUiError> {
    match value.ShowAsync() {
        Ok(operation) => {
            drop(operation);
            Ok(())
        }
        Err(error) if error.code().is_ok() => Ok(()),
        Err(error) => Err(error.into()),
    }
}

impl WinUiAdapter {
    pub(super) fn set_content_dialog(
        &mut self,
        owner: ObjectId,
        dialog: Option<ObjectId>,
    ) -> Result<(), WinUiError> {
        if self.content_dialog_owners.contains_key(&owner) {
            return Err(WinUiError::StillOwned(owner));
        }
        if let Some(dialog) = dialog {
            if owner == dialog || self.kind(dialog)? != ObjectType::ContentDialog {
                return Err(WinUiError::InvalidObject(dialog));
            }
            if self.owners.contains_key(&dialog)
                || self
                    .content_dialog_owners
                    .get(&dialog)
                    .is_some_and(|current| *current != owner)
            {
                return Err(WinUiError::StillOwned(dialog));
            }
            let owner_element = self.ui_element(owner)?;
            let framework = owner_element.cast::<native::IFrameworkElement>()?;
            let value = self
                .event_queue
                .content_dialogs
                .borrow()
                .dialogs
                .get(&dialog)
                .ok_or(WinUiError::MissingObject(dialog))?
                .value
                .clone();
            let dialog_element = value.cast::<native::IFrameworkElement>()?;
            let themed_dialog = dialog_element.clone();
            let theme_changed = framework.ActualThemeChanged(move |sender, _| {
                let Some(sender) = sender.as_ref() else {
                    return;
                };
                themed_dialog
                    .SetRequestedTheme(sender.ActualTheme().unwrap())
                    .unwrap();
            })?;
            dialog_element.SetRequestedTheme(framework.ActualTheme()?)?;
            {
                let mut scheduler = self.event_queue.content_dialogs.borrow_mut();
                let state = scheduler
                    .dialogs
                    .get_mut(&dialog)
                    .ok_or(WinUiError::MissingObject(dialog))?;
                state.owner = Some(owner);
                state.theme_changed = Some(theme_changed);
            }
            match owner_element.XamlRoot() {
                Ok(root) => {
                    let mut scheduler = self.event_queue.content_dialogs.borrow_mut();
                    scheduler.set_root(dialog, root)?;
                    scheduler.start_next_for_dialog(dialog)?;
                }
                Err(error) if error.code().is_ok() => {
                    let event_queue = Rc::clone(&self.event_queue);
                    let loaded_element = owner_element;
                    let loaded = framework.Loaded(move |_, _| {
                        let root = loaded_element.XamlRoot().unwrap();
                        let mut scheduler = event_queue.content_dialogs.borrow_mut();
                        scheduler.set_root(dialog, root).unwrap();
                        scheduler.start_next_for_dialog(dialog).unwrap();
                    })?;
                    self.event_queue
                        .content_dialogs
                        .borrow_mut()
                        .dialogs
                        .get_mut(&dialog)
                        .unwrap()
                        .loaded = Some(loaded);
                }
                Err(error) => return Err(error.into()),
            }
        }
        if dialog.is_none()
            && let Some(previous) = self.content_dialogs.get(&owner)
            && self
                .event_queue
                .content_dialogs
                .borrow()
                .dialogs
                .get(previous)
                .is_some_and(|state| state.desired_open)
        {
            return Err(WinUiError::StillOwned(*previous));
        }
        if let Some(previous) = self.content_dialogs.remove(&owner) {
            self.content_dialog_owners.remove(&previous);
            let mut scheduler = self.event_queue.content_dialogs.borrow_mut();
            if let Some(state) = scheduler.dialogs.get_mut(&previous) {
                state.owner = None;
                state.loaded = None;
                state.theme_changed = None;
                if state.pending_generation.is_none() {
                    state.root = None;
                }
            }
        }
        if let Some(dialog) = dialog {
            self.content_dialogs.insert(owner, dialog);
            self.content_dialog_owners.insert(dialog, owner);
        }
        Ok(())
    }

    pub(super) fn set_content_dialog_open(
        &mut self,
        dialog: ObjectId,
        open: bool,
    ) -> Result<(), WinUiError> {
        if !self.content_dialog_owners.contains_key(&dialog) {
            return Err(WinUiError::InvalidObject(dialog));
        }
        let hide = self
            .event_queue
            .content_dialogs
            .borrow_mut()
            .set_open(dialog, open)?;
        if let Some(value) = hide {
            value.Hide()?;
        }
        Ok(())
    }

    pub(super) fn content_dialog_closed(
        event_queue: &Rc<NativeEventQueue>,
        dialog: ObjectId,
    ) -> Result<bool, WinUiError> {
        let generation = event_queue
            .content_dialogs
            .borrow()
            .dialogs
            .get(&dialog)
            .and_then(|state| state.pending_generation);
        let Some(generation) = generation else {
            return Ok(false);
        };
        let (dispatch, root) = event_queue
            .content_dialogs
            .borrow_mut()
            .closed(dialog, generation)?;
        let queue = native::DispatcherQueue::GetForCurrentThread()?;
        let deferred_queue = Rc::clone(event_queue);
        let handler = native::DispatcherQueueHandler::new(move || {
            let mut scheduler = deferred_queue.content_dialogs.borrow_mut();
            scheduler.cleanup_retired();
            if let Some(root) = &root {
                scheduler.start_next(root).unwrap();
            }
        });
        // A dispatcher that is shutting down rejects the continuation; no dialog will show again.
        queue.TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler)?;
        Ok(dispatch)
    }
}
