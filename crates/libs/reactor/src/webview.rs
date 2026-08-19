use std::fmt;

use crate::element::tree::ElementKind;
use crate::element::{Element, Framework, RenderCx};
use crate::framework_properties::FrameworkProps;
use crate::interaction::Callback;
use crate::references::{ElementRef, NativeElementRef};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebViewNavigationCompleted {
    pub navigation_id: u64,
    pub is_success: bool,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WebViewAction {
    Navigate(String),
    NavigateToString(String),
    Reload,
    Stop,
    GoBack,
    GoForward,
}

pub(crate) struct WebViewHostProps {
    pub source: Option<String>,
    pub on_created: Option<Callback<windows_core::Result<()>>>,
    pub on_navigation_completed: Option<Callback<WebViewNavigationCompleted>>,
    pub framework: FrameworkProps,
}

pub struct WebViewHost {
    props: WebViewHostProps,
    reference: NativeElementRef,
}

impl WebViewHost {
    pub fn new(reference: &WebViewRef) -> Framework<Self> {
        Framework::new(Self {
            props: WebViewHostProps {
                source: None,
                on_created: None,
                on_navigation_completed: None,
                framework: FrameworkProps::default(),
            },
            reference: reference.reference.binding(),
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        let element = Element::new(ElementKind::WebViewHost(Box::new(self.props)));
        Element::new(ElementKind::Reference {
            reference: self.reference,
            child: Box::new(element),
        })
    }
}

impl Framework<WebViewHost> {
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.control.props.source = Some(source.into());
        self
    }

    pub fn on_created(mut self, callback: impl Fn(windows_core::Result<()>) + 'static) -> Self {
        self.control.props.on_created = Some(Callback::new(callback));
        self
    }

    pub fn on_navigation_completed(
        mut self,
        callback: impl Fn(WebViewNavigationCompleted) + 'static,
    ) -> Self {
        self.control.props.on_navigation_completed = Some(Callback::new(callback));
        self
    }
}

pub struct WebViewRef {
    reference: ElementRef<WebViewHost>,
}

impl WebViewRef {
    pub fn new() -> Self {
        Self {
            reference: ElementRef::new(),
        }
    }

    pub fn is_mounted(&self) -> bool {
        self.reference.is_mounted()
    }

    fn action(&self, action: WebViewAction) -> bool {
        self.reference.schedule(move |scheduler, node| {
            scheduler.run_webview_action(node, action);
        })
    }

    pub fn navigate(&self, uri: impl Into<String>) -> bool {
        self.action(WebViewAction::Navigate(uri.into()))
    }

    pub fn navigate_to_string(&self, html: impl Into<String>) -> bool {
        self.action(WebViewAction::NavigateToString(html.into()))
    }

    pub fn reload(&self) -> bool {
        self.action(WebViewAction::Reload)
    }

    pub fn stop(&self) -> bool {
        self.action(WebViewAction::Stop)
    }

    pub fn go_back(&self) -> bool {
        self.action(WebViewAction::GoBack)
    }

    pub fn go_forward(&self) -> bool {
        self.action(WebViewAction::GoForward)
    }
}

impl Clone for WebViewRef {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
        }
    }
}

impl Default for WebViewRef {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for WebViewRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebViewRef")
            .field("mounted", &self.is_mounted())
            .finish()
    }
}

impl RenderCx<'_> {
    pub fn use_webview_ref(&mut self) -> WebViewRef {
        WebViewRef {
            reference: self.use_element_ref(),
        }
    }
}
