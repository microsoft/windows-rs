use super::*;

impl WinUiRuntime {
    pub(super) fn start_timer(&mut self, spec: TimerSpec) -> WindowsResult<()> {
        let timer = self.dispatcher.CreateTimer()?;
        timer.SetInterval(duration_to_timespan(spec.interval)?)?;
        timer.SetIsRepeating(spec.repeating)?;
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        #[cfg(test)]
        let timer_ticks = Rc::clone(&self.timer_ticks);
        let revoker = timer.Tick(move |_sender, _args| {
            #[cfg(test)]
            timer_ticks.set(timer_ticks.get() + 1);
            events.borrow_mut().push_back(NativeEvent::TimerFired {
                owner: spec.owner,
                slot: spec.slot,
                revision: spec.revision,
            });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        timer.Start()?;
        self.timers.insert(
            (spec.owner, spec.slot),
            NativeTimer {
                revision: spec.revision,
                timer,
                _revoker: revoker,
            },
        );
        Ok(())
    }

    pub(super) fn stop_timer(&mut self, owner: NodeId, slot: u32, revision: u64) {
        let key = (owner, slot);
        if self
            .timers
            .get(&key)
            .is_some_and(|timer| timer.revision == revision)
        {
            self.timers.remove(&key);
        }
    }

    pub(super) fn update_application(&mut self, update: &ApplicationUpdate) -> WindowsResult<()> {
        match update {
            ApplicationUpdate::Resources(resources) => {
                let map = bindings::Application::Current()?
                    .Resources()?
                    .cast::<windows_collections::IMap<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >>()?;
                for (key, _) in self.application_resources.entries() {
                    if resources.get(key).is_none() {
                        let key = windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(key),
                        )
                        .cast::<windows_core::IInspectable>()?;
                        if map.HasKey(&key)? {
                            map.Remove(&key)?;
                        }
                    }
                }
                for (key, value) in resources.entries() {
                    if self.application_resources.get(key) != Some(value) {
                        let key = windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(key),
                        )
                        .cast::<windows_core::IInspectable>()?;
                        map.Insert(&key, &application_resource_value(value)?)?;
                    }
                }
                self.application_resources = (**resources).clone();
                Ok(())
            }
        }
    }

    pub(super) fn run_deferred(
        &self,
        target: NodeId,
        window: Option<NodeId>,
        revision: u64,
        action: DeferredAction,
    ) -> WindowsResult<()> {
        match action {
            DeferredAction::ContentDialogOpen | DeferredAction::TeachingTipOpen => {
                self.run_overlay_deferred(target, window, revision, action)
            }
            DeferredAction::RadioButtonsSelection => {
                self.run_radio_buttons_deferred(target, revision)
            }
        }
    }

    fn shutdown(&mut self) {
        if std::mem::replace(&mut self.shutdown_complete, true) {
            return;
        }
        self.shutting_down.set(true);
        *self.waker.borrow_mut() = None;
        self.active_content_dialogs.borrow_mut().clear();
        self.timers.clear();
        self.events.borrow_mut().clear();
        for node in self.nodes.values_mut() {
            if let Handle::ContentDialog { value, state } = &node.handle {
                state.shutdown(value);
            }
            #[cfg(feature = "canvas")]
            if let Handle::SwapChainCanvas(state) = &node.handle {
                _ = state.detach();
            }
            #[cfg(feature = "canvas")]
            if let Handle::SwapChainHost(state) = &mut node.handle {
                _ = state.detach();
            }
            if let Handle::CompositionHost(state) = &mut node.handle {
                _ = state.detach();
            }
            #[cfg(feature = "webview")]
            if let Handle::WebViewHost(state) = &mut node.handle {
                state.detach();
            }
            if node
                .input
                .as_deref()
                .is_some_and(input::NativeInputState::captures_pointer)
                && let Ok(element) = node.handle.ui_element()
            {
                _ = element.ReleasePointerCaptures();
            }
        }
        self.nodes.clear();
        self.shutdown_windows();
    }
}

impl Drop for WinUiRuntime {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn duration_to_timespan(value: Duration) -> WindowsResult<TimeSpan> {
    let ticks = value.as_nanos() / 100;
    let duration = i64::try_from(ticks).map_err(|_| {
        windows_core::Error::new(
            windows_core::HRESULT(0x80070057_u32 as i32),
            "timer duration exceeds the WinRT TimeSpan range",
        )
    })?;
    Ok(TimeSpan { duration })
}

#[cfg(test)]
#[path = "../../testing/private/winui/lifecycle_access.rs"]
pub(super) mod tests;
