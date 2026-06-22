use super::*;
use crate::handler::subscription;

/// A WebView2 browser. Navigate to URLs and run JavaScript against the hosted
/// page.
#[derive(Clone)]
pub struct WebView(pub(crate) ICoreWebView2);

impl WebView {
    /// Navigates the browser to the given URI.
    pub fn navigate(&self, uri: &str) -> Result<()> {
        let uri = string::encode(uri);
        unsafe { self.0.Navigate(uri.as_ptr()) }
    }

    /// Navigates the browser to the given HTML content as the document.
    pub fn navigate_to_string(&self, html: &str) -> Result<()> {
        let html = string::encode(html);
        unsafe { self.0.NavigateToString(html.as_ptr()) }
    }

    /// Reloads the current page.
    pub fn reload(&self) -> Result<()> {
        unsafe { self.0.Reload() }
    }

    /// Stops any in-progress navigation or download.
    pub fn stop(&self) -> Result<()> {
        unsafe { self.0.Stop() }
    }

    /// Navigates back to the previous page in the navigation history.
    pub fn go_back(&self) -> Result<()> {
        unsafe { self.0.GoBack() }
    }

    /// Navigates forward to the next page in the navigation history.
    pub fn go_forward(&self) -> Result<()> {
        unsafe { self.0.GoForward() }
    }

    /// Returns the URI of the current top-level document.
    pub fn source(&self) -> String {
        unsafe { self.0.Source() }
            .map(|value| unsafe { string::take(value) })
            .unwrap_or_default()
    }

    /// Returns the title of the current top-level document.
    pub fn document_title(&self) -> String {
        unsafe { self.0.DocumentTitle() }
            .map(|value| unsafe { string::take(value) })
            .unwrap_or_default()
    }

    /// Returns the [`Settings`] controlling features such as JavaScript, the dev
    /// tools, and context menus.
    pub fn settings(&self) -> Result<Settings> {
        unsafe { Ok(Settings(self.0.Settings()?)) }
    }

    /// Asynchronously runs JavaScript in the context of the current page. The
    /// `handler` closure receives the JSON-encoded result on the UI thread.
    pub fn execute_script<F: FnOnce(Result<String>) + 'static>(
        &self,
        javascript: &str,
        handler: F,
    ) -> Result<()> {
        let javascript = string::encode(javascript);
        let handler = handler::ExecuteScriptCompleted::create(handler);
        unsafe { self.0.ExecuteScript(javascript.as_ptr(), &handler) }
    }

    /// Registers JavaScript to run before any other script each time a document
    /// is created, returning a [`ScriptId`] that can later be passed to
    /// [`remove_script_to_execute_on_document_created`](Self::remove_script_to_execute_on_document_created).
    ///
    /// This is the way to inject code (for example a host-communication
    /// bootstrap) before the page's own scripts run. It pumps the calling
    /// thread's message loop until registration completes, so call it during
    /// setup before handing control to your own message loop.
    pub fn add_script_to_execute_on_document_created(&self, javascript: &str) -> Result<ScriptId> {
        let javascript = string::encode(javascript);
        let slot = pump::slot();
        let handler = handler::AddScriptCompleted::create(pump::slot_handler(&slot));
        unsafe {
            self.0
                .AddScriptToExecuteOnDocumentCreated(javascript.as_ptr(), &handler)?;
        }
        Ok(ScriptId(pump::wait(&slot)?))
    }

    /// Removes a script previously registered with
    /// [`add_script_to_execute_on_document_created`](Self::add_script_to_execute_on_document_created).
    pub fn remove_script_to_execute_on_document_created(&self, id: &ScriptId) -> Result<()> {
        let id = string::encode(&id.0);
        unsafe { self.0.RemoveScriptToExecuteOnDocumentCreated(id.as_ptr()) }
    }

    subscription! {
        /// Subscribes to the navigation-starting event, raised before each
        /// navigation. The handler may inspect the target and cancel it via
        /// [`NavigationStartingArgs::set_cancel`].
        on_navigation_starting(NavigationStartingArgs) =>
            NavigationStarting, add_NavigationStarting / remove_NavigationStarting
    }

    subscription! {
        /// Subscribes to the navigation-completed event.
        on_navigation_completed(NavigationCompletedArgs) =>
            NavigationCompleted, add_NavigationCompleted / remove_NavigationCompleted
    }

    /// Posts a message to the hosted page as a JSON value. The page receives it
    /// via the `window.chrome.webview.addEventListener("message", …)` event,
    /// with `event.data` set to the parsed JSON.
    pub fn post_web_message_as_json(&self, json: &str) -> Result<()> {
        let json = string::encode(json);
        unsafe { self.0.PostWebMessageAsJson(json.as_ptr()) }
    }

    /// Posts a message to the hosted page as a string. The page receives it via
    /// the `window.chrome.webview.addEventListener("message", …)` event, with
    /// `event.data` set to the string.
    pub fn post_web_message_as_string(&self, message: &str) -> Result<()> {
        let message = string::encode(message);
        unsafe { self.0.PostWebMessageAsString(message.as_ptr()) }
    }

    subscription! {
        /// Subscribes to the web-message-received event, raised when the hosted
        /// page calls `window.chrome.webview.postMessage`.
        on_web_message_received(WebMessageReceivedArgs) =>
            WebMessageReceived, add_WebMessageReceived / remove_WebMessageReceived
    }

    subscription! {
        /// Subscribes to the content-loading event, raised when the browser
        /// starts loading content for a new document.
        on_content_loading(ContentLoadingArgs) =>
            ContentLoading, add_ContentLoading / remove_ContentLoading
    }

    subscription! {
        /// Subscribes to the document-title-changed event. The handler receives
        /// the new [`document_title`](Self::document_title).
        on_document_title_changed(String) =>
            DocumentTitleChanged, add_DocumentTitleChanged / remove_DocumentTitleChanged
    }

    /// Subscribes to the window-close-requested event, raised when the hosted
    /// page calls `window.close()`. The host typically responds by closing its
    /// window.
    pub fn on_window_close_requested<F: FnMut() + 'static>(
        &self,
        handler: F,
    ) -> Result<EventRegistration> {
        let handler = handler::WindowCloseRequested::create(handler);
        let token = unsafe { self.0.add_WindowCloseRequested(&handler)? };
        let source = self.0.clone();
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_WindowCloseRequested(token) };
        }))
    }

    subscription! {
        /// Subscribes to the new-window-requested event, raised when the page
        /// tries to open a new window (for example via `window.open`). The
        /// handler may suppress, redirect, or
        /// [defer](NewWindowRequestedArgs::defer) the request.
        on_new_window_requested(NewWindowRequestedArgs) =>
            NewWindowRequested, add_NewWindowRequested / remove_NewWindowRequested
    }

    subscription! {
        /// Subscribes to the permission-requested event, raised when the page
        /// requests access to a capability such as the camera or geolocation.
        /// The handler decides the outcome via
        /// [`PermissionRequestedArgs::set_state`] and may
        /// [defer](PermissionRequestedArgs::defer) the decision.
        on_permission_requested(PermissionRequestedArgs) =>
            PermissionRequested, add_PermissionRequested / remove_PermissionRequested
    }

    /// Subscribes to the download-starting event, raised when a download begins.
    /// The handler receives a [`DownloadStartingArgs`] to inspect or control the
    /// [`DownloadOperation`], change its destination, or cancel it.
    pub fn on_download_starting<F: FnMut(DownloadStartingArgs) + 'static>(
        &self,
        handler: F,
    ) -> Result<EventRegistration> {
        let source: ICoreWebView2_4 = self.0.cast()?;
        let handler = handler::DownloadStarting::create(handler);
        let token = unsafe { source.add_DownloadStarting(&handler)? };
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_DownloadStarting(token) };
        }))
    }

    /// Subscribes to the web-resource-requested event, raised when the page
    /// requests a resource whose URI matches `uri_filter`, a wildcard pattern
    /// such as `https://app.example/*`. The handler runs synchronously on the UI
    /// thread; return a [`WebResourceResponse`] to fulfil the request from memory
    /// (typically serving embedded assets under a custom host), or `None` to let
    /// WebView2 handle it normally.
    pub fn on_web_resource_requested<F>(
        &self,
        uri_filter: &str,
        handler: F,
    ) -> Result<EventRegistration>
    where
        F: FnMut(WebResourceRequest) -> Option<WebResourceResponse> + 'static,
    {
        let environment = unsafe { self.0.cast::<ICoreWebView2_2>()?.Environment()? };
        let filter = string::encode(uri_filter);
        unsafe {
            self.0.AddWebResourceRequestedFilter(
                filter.as_ptr(),
                protocol::WEB_RESOURCE_CONTEXT_ALL,
            )?;
        }
        let handler = protocol::WebResourceRequested::create(environment, handler);
        let token = unsafe { self.0.add_WebResourceRequested(&handler)? };
        let source = self.0.clone();
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_WebResourceRequested(token) };
            let _ = unsafe {
                source.RemoveWebResourceRequestedFilter(
                    filter.as_ptr(),
                    protocol::WEB_RESOURCE_CONTEXT_ALL,
                )
            };
        }))
    }
}
