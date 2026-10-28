use super::*;

pub struct ComponentContext<C: Component> {
    pub(super) reference: ElementRef,
    pub(super) local_sender: LocalSender<C::Message>,
    pub(super) sender: ComponentSender<C::Message>,
    pub(super) services: Arc<dyn ComponentServices>,
    pub(super) tasks: Arc<Mutex<Vec<SyncWeak<TaskControl>>>>,
    pub(super) ui_services: Rc<dyn ComponentUiServices>,
}

impl<C: Component> ComponentContext<C> {
    pub fn sender(&self) -> LocalSender<C::Message> {
        self.local_sender.clone()
    }

    pub fn callback<T>(&self, map: impl Fn(T) -> C::Message + 'static) -> Callback<T> {
        self.local_sender.callback(map)
    }

    pub fn forward(&self) -> Callback<C::Message> {
        self.local_sender.forward()
    }

    pub fn message(&self, message: C::Message) -> Callback<()>
    where
        C::Message: Clone,
    {
        self.local_sender.message(message)
    }

    pub fn root(&self) -> ElementRef {
        self.reference.clone()
    }

    #[must_use]
    pub fn open_window<W: Component>(&self, input: W::Input) -> bool {
        self.open_window_with_policy::<W>(input, WindowPolicy::new())
    }

    #[must_use]
    pub fn open_window_with_policy<W: Component>(
        &self,
        input: W::Input,
        policy: WindowPolicy,
    ) -> bool {
        self.ui_services
            .open_window(component::<W>("root", input), policy)
    }

    #[must_use]
    pub fn activate_window(&self) -> bool {
        self.ui_services.activate_window()
    }

    #[must_use]
    pub fn close_window(&self) -> bool {
        self.ui_services.close_window()
    }

    #[must_use]
    pub fn run_window(
        &self,
        work: impl for<'a> FnOnce(WindowHandle<'a>) -> C::Message + 'static,
    ) -> bool {
        let sender = self.local_sender.clone();
        self.ui_services.run_window(Box::new(move |raw| {
            _ = sender.send(work(WindowHandle::new(raw)));
        }))
    }

    #[must_use = "dropping the timer cancels message delivery"]
    pub fn set_local_timeout(
        &self,
        delay: Duration,
        message: impl FnOnce() -> C::Message + Send + 'static,
    ) -> ComponentTimer {
        let control = Arc::new(TaskControl::default());
        let mut tasks = self.tasks.lock().unwrap();
        tasks.retain(|task| task.strong_count() != 0);
        tasks.push(Arc::downgrade(&control));
        drop(tasks);
        let component = self.sender.component;
        let queue = Arc::clone(&self.sender.queue);
        let timer_control = Arc::clone(&control);
        let registration = self.services.set_timeout(
            delay,
            Box::new(move || {
                timer_control.timer_fired();
                if !timer_control.queue() {
                    return;
                }
                let mut queue = queue.lock().unwrap();
                if queue.closed || queue.messages.len() >= MESSAGE_CAPACITY {
                    timer_control.reject();
                    return;
                }
                let sequence = queue.next_sequence;
                queue.next_sequence = queue.next_sequence.wrapping_add(1);
                queue.messages.push_back(Message {
                    component,
                    control: Some(Arc::clone(&timer_control)),
                    sequence,
                    value: MessageValue::Local(Box::new(move || Box::new(message()))),
                });
                let wake = (!queue.wake_pending).then(|| queue.waker.clone()).flatten();
                queue.wake_pending |= wake.is_some();
                drop(queue);
                if let Some(wake) = wake {
                    wake();
                }
            }),
        );
        control.set_timer(registration);
        ComponentTimer {
            task: ComponentTask { control },
        }
    }
}

impl<C> ComponentContext<C>
where
    C: Component,
    C::Message: Send,
{
    pub fn completion(&self) -> ComponentCompletion<C::Message> {
        self.sender.completion()
    }

    pub fn spawn_background(
        &self,
        work: impl FnOnce(CancellationToken) -> C::Message + Send + 'static,
    ) -> ComponentTask {
        let control = Arc::new(TaskControl::default());
        let mut tasks = self.tasks.lock().unwrap();
        tasks.retain(|task| task.strong_count() != 0);
        tasks.push(Arc::downgrade(&control));
        drop(tasks);
        let sender = self.sender.clone();
        let thread_control = Arc::clone(&control);
        self.services.spawn_background(Box::new(move || {
            let message = work(CancellationToken {
                control: Arc::clone(&thread_control),
            });
            if thread_control.queue() {
                sender.send_controlled(message, Arc::clone(&thread_control));
            }
        }));
        ComponentTask { control }
    }

    #[must_use = "dropping the timer cancels message delivery"]
    pub fn set_timeout(&self, delay: Duration, message: C::Message) -> ComponentTimer {
        let control = Arc::new(TaskControl::default());
        let mut tasks = self.tasks.lock().unwrap();
        tasks.retain(|task| task.strong_count() != 0);
        tasks.push(Arc::downgrade(&control));
        drop(tasks);
        let sender = self.sender.clone();
        let timer_control = Arc::clone(&control);
        let registration = self.services.set_timeout(
            delay,
            Box::new(move || {
                timer_control.timer_fired();
                if timer_control.queue() {
                    sender.send_controlled(message, Arc::clone(&timer_control));
                }
            }),
        );
        control.set_timer(registration);
        ComponentTimer {
            task: ComponentTask { control },
        }
    }
}

pub struct ViewContext<'a, C: Component> {
    pub(super) context: ComponentContext<C>,
    pub(super) contexts: &'a HashMap<ContextId, ContextValue>,
    pub(super) dependencies: HashSet<ContextId>,
    pub(super) effects: EffectDraft,
    pub(super) window: WindowPublication,
}

impl<C: Component> ViewContext<'_, C> {
    pub fn sender(&self) -> LocalSender<C::Message> {
        self.context.sender()
    }

    pub fn callback<T>(&self, map: impl Fn(T) -> C::Message + 'static) -> Callback<T> {
        self.context.callback(map)
    }

    pub fn forward(&self) -> Callback<C::Message> {
        self.context.forward()
    }

    pub fn message(&self, message: C::Message) -> Callback<()>
    where
        C::Message: Clone,
    {
        self.context.message(message)
    }

    pub fn root(&self) -> ElementRef {
        self.context.root()
    }

    pub fn use_context<T: Clone + 'static>(&mut self, context: &Context<T>) -> T {
        self.dependencies.insert(context.id);
        self.contexts.get(&context.id).map_or_else(
            || context.default.clone(),
            |value| value.value.downcast_ref::<T>().unwrap().clone(),
        )
    }

    pub fn use_effect<D>(
        &mut self,
        key: impl Into<EffectKey>,
        dependency: D,
        setup: impl FnOnce() -> Option<EffectCleanup> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        self.effects.register(key.into(), dependency, setup);
    }

    pub fn use_effect_guard<D, G>(
        &mut self,
        key: impl Into<EffectKey>,
        dependency: D,
        setup: impl FnOnce() -> G + 'static,
    ) where
        D: PartialEq + 'static,
        G: 'static,
    {
        self.use_effect(key, dependency, move || {
            let guard = setup();
            Some(Box::new(move || drop(guard)))
        });
    }

    /// Observes native size changes without replaying them when the callback is replaced.
    pub fn on_window_size(&mut self, callback: impl IntoPayloadCallback<WindowSize>) {
        let previous = self.window.on_size.replace(WindowObserverDeclaration {
            owner: self.context.local_sender.component,
            generation: 0,
            callback: callback.into_payload_callback(),
        });
        assert!(
            previous.is_none(),
            "window size observer declared more than once"
        );
    }

    /// Observes restored outer bounds and maximized state, including the initial placement.
    ///
    /// Notifications are coalesced, exclude minimized state, and stop when the window or
    /// subscribing component closes. Persist values in this callback's message handler rather
    /// than relying on a shutdown notification. Replacing the callback does not replay the current
    /// placement; removing and re-adding the observer or changing its subscribing component starts
    /// a new subscription.
    pub fn on_window_placement(&mut self, callback: impl IntoPayloadCallback<WindowPlacement>) {
        let previous = self.window.on_placement.replace(WindowObserverDeclaration {
            owner: self.context.local_sender.component,
            generation: 0,
            callback: callback.into_payload_callback(),
        });
        assert!(
            previous.is_none(),
            "window placement observer declared more than once"
        );
    }

    /// Reports the current scheme when subscribed, then only changes to that scheme.
    ///
    /// Replacing the callback does not replay the current scheme. Removing and re-adding the
    /// observer or changing its subscribing component starts a new subscription and reports the
    /// current scheme again.
    pub fn on_color_scheme(&mut self, callback: impl IntoPayloadCallback<ColorScheme>) {
        let previous = self
            .window
            .on_color_scheme
            .replace(WindowObserverDeclaration {
                owner: self.context.local_sender.component,
                generation: 0,
                callback: callback.into_payload_callback(),
            });
        assert!(
            previous.is_none(),
            "color scheme observer declared more than once"
        );
    }

    pub fn window_title(&mut self, title: impl Into<String>) {
        let previous = self.window.title.replace(title.into());
        assert!(previous.is_none(), "window title declared more than once");
    }

    #[must_use]
    pub fn window_frame(&mut self, title: impl Into<String>, content: impl Into<View>) -> View {
        let title = title.into();
        self.window_title(title.clone());
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                TitleBar::new().title(title),
                Border::new().grid_row(1).content(content),
            ))
            .into()
    }

    pub fn window_visuals(&mut self, visuals: WindowVisuals) {
        let previous = self.window.visuals.replace(visuals);
        assert!(previous.is_none(), "window visuals declared more than once");
    }
}

impl<C> ViewContext<'_, C>
where
    C: Component,
    C::Message: Send,
{
    pub fn completion(&self) -> ComponentCompletion<C::Message> {
        self.context.completion()
    }
}
