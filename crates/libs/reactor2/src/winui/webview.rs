use super::*;

pub(super) struct WebViewHostState {
    _value: bindings::WebView2,
    ui: bindings::UIElement,
    framework: bindings::FrameworkElement,
    owner: Option<windows_webview::XamlWebViewHost>,
    completion: Rc<RefCell<Option<windows_core::Result<windows_webview::WebView>>>>,
    revision: u64,
    webview: Option<windows_webview::WebView>,
    navigation: Option<windows_webview::EventRegistration>,
    source: Option<String>,
    pending: Vec<crate::webview::WebViewAction>,
}

impl WebViewHostState {
    pub(super) fn ui_element(&self) -> bindings::UIElement {
        self.ui.clone()
    }

    pub(super) fn framework_element(&self) -> bindings::FrameworkElement {
        self.framework.clone()
    }

    pub(super) fn detach(&mut self) {
        self.pending.clear();
        self.navigation = None;
        self.webview = None;
        self.owner = None;
        self.completion.borrow_mut().take();
    }
}

fn run_webview_action(
    webview: &windows_webview::WebView,
    action: &crate::webview::WebViewAction,
) -> WindowsResult<()> {
    match action {
        crate::webview::WebViewAction::Navigate(uri) => webview.navigate(uri),
        crate::webview::WebViewAction::NavigateToString(html) => webview.navigate_to_string(html),
        crate::webview::WebViewAction::Reload => webview.reload(),
        crate::webview::WebViewAction::Stop => webview.stop(),
        crate::webview::WebViewAction::GoBack => webview.go_back(),
        crate::webview::WebViewAction::GoForward => webview.go_forward(),
    }
}

impl WinUiRuntime {
    pub(super) fn create_webview_host(&mut self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::WebView2::new()?;
        let ui = value.cast()?;
        let framework = value.cast()?;
        let inspectable = value.cast::<windows_core::IInspectable>()?;
        let completion = Rc::new(RefCell::new(None));
        let ready_completion = Rc::clone(&completion);
        let events = Rc::clone(&self.events);
        let ready_events = Rc::clone(&events);
        let waker = Rc::clone(&self.waker);
        let ready_waker = Rc::clone(&waker);
        let revision = 1;
        let owner = windows_webview::XamlWebViewHost::new(&inspectable, move |result| {
            *ready_completion.borrow_mut() = Some(result);
            ready_events
                .borrow_mut()
                .push_back(NativeEvent::WebViewInitializationReady {
                    target: id,
                    revision,
                });
            if let Some(wake) = ready_waker.borrow().as_ref() {
                wake();
            }
        });
        let owner = match owner {
            Ok(owner) => Some(owner),
            Err(error) => {
                events.borrow_mut().push_back(NativeEvent::WebViewCreated {
                    target: id,
                    result: Err(error),
                });
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
                None
            }
        };
        Ok(Handle::WebViewHost(Box::new(WebViewHostState {
            _value: value,
            ui,
            framework,
            owner,
            completion,
            revision,
            webview: None,
            navigation: None,
            source: None,
            pending: Vec::new(),
        })))
    }

    pub(super) fn apply_webview_host_update(
        &mut self,
        id: NodeId,
        update: &WebViewHostUpdate,
    ) -> WindowsResult<()> {
        let Handle::WebViewHost(state) = &mut self.node_mut(id)?.handle else {
            panic!("WebView update target is not a WebViewHost");
        };
        match update {
            WebViewHostUpdate::Source(source) => {
                if state.source == *source {
                    return Ok(());
                }
                state.source.clone_from(source);
                if let (Some(webview), Some(source)) = (&state.webview, source) {
                    webview.navigate(source)?;
                }
                Ok(())
            }
            WebViewHostUpdate::Action(action) => {
                if let Some(webview) = &state.webview {
                    run_webview_action(webview, action)
                } else {
                    state.pending.push(action.clone());
                    Ok(())
                }
            }
        }
    }

    pub(super) fn finish_webview_initialization(
        &mut self,
        id: NodeId,
        revision: u64,
    ) -> WindowsResult<()> {
        let result = {
            let Handle::WebViewHost(state) = &mut self.node_mut(id)?.handle else {
                panic!("WebView initialization target is not a WebViewHost");
            };
            if state.revision != revision {
                return Ok(());
            }
            let Some(result) = state.completion.borrow_mut().take() else {
                return Ok(());
            };
            result
        };

        let created = result.and_then(|webview| {
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            let navigation_webview = webview.clone();
            let navigation = webview.on_navigation_completed(move |args| {
                events
                    .borrow_mut()
                    .push_back(NativeEvent::WebViewNavigationCompleted {
                        target: id,
                        navigation_id: args.navigation_id(),
                        is_success: args.is_success(),
                        source: navigation_webview.source().into_boxed_str(),
                    });
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
            })?;
            let Handle::WebViewHost(state) = &mut self.node_mut(id)?.handle else {
                unreachable!()
            };
            if let Some(source) = &state.source {
                webview.navigate(source)?;
            }
            for action in state.pending.drain(..) {
                run_webview_action(&webview, &action)?;
            }
            state.navigation = Some(navigation);
            state.webview = Some(webview);
            Ok(())
        });
        self.events
            .borrow_mut()
            .push_back(NativeEvent::WebViewCreated {
                target: id,
                result: created,
            });
        if let Some(wake) = self.waker.borrow().as_ref() {
            wake();
        }
        Ok(())
    }
}
