use super::*;
use crate::handler::subscription;

/// The level WebView2 should target for the browser's memory usage, set with
/// [`WebView::set_memory_usage_target_level`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryUsageTargetLevel {
    /// Normal memory usage.
    Normal,
    /// Reduced memory usage, suitable for a hidden or background `WebView`.
    Low,
}

impl MemoryUsageTargetLevel {
    fn from_raw(value: COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL) -> Self {
        match value {
            1 => Self::Low,
            _ => Self::Normal,
        }
    }

    fn to_raw(self) -> COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL {
        match self {
            Self::Normal => 0,
            Self::Low => 1,
        }
    }
}

/// A request to navigate with a custom HTTP method, headers, or body, passed to
/// [`WebView::navigate_with_request`]. Defaults to a `GET` with no extra headers
/// or body.
#[derive(Clone, Debug)]
pub struct NavigationRequest {
    uri: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl NavigationRequest {
    /// Creates a `GET` request for `uri`.
    pub fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
            method: "GET".to_string(),
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    /// Sets the HTTP method, for example `POST`.
    pub fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    /// Adds a request header, such as an `Authorization` token.
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    /// Sets the request body bytes.
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }
}

/// How a folder mapped with
/// [`WebView::set_virtual_host_name_to_folder_mapping`] may be accessed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostResourceAccessKind {
    /// Resources from other origins cannot access the mapped content.
    Deny,
    /// Resources from any origin may access the mapped content.
    Allow,
    /// Like [`Deny`](Self::Deny), but cross-origin requests are allowed through
    /// CORS.
    DenyCors,
}

impl HostResourceAccessKind {
    fn to_raw(self) -> COREWEBVIEW2_HOST_RESOURCE_ACCESS_KIND {
        match self {
            Self::Deny => 0,
            Self::Allow => 1,
            Self::DenyCors => 2,
        }
    }
}

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

    /// Navigates the browser using a [`NavigationRequest`], allowing a custom
    /// HTTP method, request headers, or body — for example a `POST` or an
    /// `Authorization` header that a plain [`navigate`](Self::navigate) cannot
    /// supply.
    pub fn navigate_with_request(&self, request: &NavigationRequest) -> Result<()> {
        let source: ICoreWebView2_2 = self.0.cast()?;
        let environment: ICoreWebView2Environment2 = unsafe { source.Environment()? }.cast()?;

        let uri = string::encode(&request.uri);
        let method = string::encode(&request.method);
        let mut headers = String::new();
        for (name, value) in &request.headers {
            headers.push_str(name);
            headers.push_str(": ");
            headers.push_str(value);
            headers.push_str("\r\n");
        }
        let headers = string::encode(&headers);
        let stream = if request.body.is_empty() {
            None
        } else {
            unsafe { SHCreateMemStream(request.body.as_ptr(), request.body.len() as u32) }
        };

        unsafe {
            let request = environment.CreateWebResourceRequest(
                uri.as_ptr(),
                method.as_ptr(),
                stream.as_ref(),
                headers.as_ptr(),
            )?;
            source.NavigateWithWebResourceRequest(&request)
        }
    }

    /// Reloads the current page.
    pub fn reload(&self) -> Result<()> {
        unsafe { self.0.Reload() }
    }

    /// Opens the DevTools window for the page, the same view shown by the
    /// browser's "Inspect" command.
    pub fn open_dev_tools_window(&self) -> Result<()> {
        unsafe { self.0.OpenDevToolsWindow() }
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
        unsafe { string::take_result(self.0.Source()) }
    }

    /// Returns the title of the current top-level document.
    pub fn document_title(&self) -> String {
        unsafe { string::take_result(self.0.DocumentTitle()) }
    }

    /// Maps a virtual host name to a local folder so the page can load its files
    /// over a normal URL such as `https://app.example/index.html`. This is the
    /// simplest way to serve a bundled web app: map a host to the asset folder
    /// once, then navigate to it. `access_kind` controls cross-origin access to
    /// the mapped files.
    pub fn set_virtual_host_name_to_folder_mapping(
        &self,
        host_name: &str,
        folder_path: &str,
        access_kind: HostResourceAccessKind,
    ) -> Result<()> {
        let source: ICoreWebView2_3 = self.0.cast()?;
        let host_name = string::encode(host_name);
        let folder_path = string::encode(folder_path);
        unsafe {
            source.SetVirtualHostNameToFolderMapping(
                host_name.as_ptr(),
                folder_path.as_ptr(),
                access_kind.to_raw(),
            )
        }
    }

    /// Removes a mapping previously created with
    /// [`set_virtual_host_name_to_folder_mapping`](Self::set_virtual_host_name_to_folder_mapping).
    pub fn clear_virtual_host_name_to_folder_mapping(&self, host_name: &str) -> Result<()> {
        let source: ICoreWebView2_3 = self.0.cast()?;
        let host_name = string::encode(host_name);
        unsafe { source.ClearVirtualHostNameToFolderMapping(host_name.as_ptr()) }
    }

    /// Returns the [`CookieManager`] for reading, writing, and deleting the
    /// browser's cookies.
    pub fn cookie_manager(&self) -> Result<CookieManager> {
        let source: ICoreWebView2_2 = self.0.cast()?;
        Ok(CookieManager(unsafe { source.CookieManager()? }))
    }

    /// Returns the [`Profile`] this browser belongs to, exposing its colour
    /// scheme, download folder, and browsing-data controls.
    pub fn profile(&self) -> Result<Profile> {
        let source: ICoreWebView2_13 = self.0.cast()?;
        Ok(Profile(unsafe { source.Profile()? }))
    }

    /// Returns `true` if the page currently has an element displayed full screen
    /// (for example a video using the HTML Fullscreen API).
    pub fn contains_fullscreen_element(&self) -> bool {
        unsafe { self.0.ContainsFullScreenElement() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns the memory-usage level WebView2 is targeting.
    pub fn memory_usage_target_level(&self) -> Result<MemoryUsageTargetLevel> {
        let source: ICoreWebView2_19 = self.0.cast()?;
        Ok(MemoryUsageTargetLevel::from_raw(unsafe {
            source.MemoryUsageTargetLevel()?
        }))
    }

    /// Hints the memory-usage level WebView2 should target. Set
    /// [`MemoryUsageTargetLevel::Low`] when the `WebView` is hidden so the
    /// browser can trim memory, and back to `Normal` when it is shown again.
    pub fn set_memory_usage_target_level(&self, level: MemoryUsageTargetLevel) -> Result<()> {
        let source: ICoreWebView2_19 = self.0.cast()?;
        unsafe { source.SetMemoryUsageTargetLevel(level.to_raw()) }
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

    subscription! {
        /// Subscribes to the process-failed event, raised when a browser process
        /// crashes, exits unexpectedly, or becomes unresponsive. Inspect
        /// [`ProcessFailedArgs::kind`] to decide whether to reload the page (a
        /// render-process crash) or recreate the `WebView` (a browser-process
        /// exit). Without a handler a crash leaves a blank page with no notice.
        on_process_failed(ProcessFailedArgs) =>
            ProcessFailed, add_ProcessFailed / remove_ProcessFailed
    }

    /// Subscribes to the contains-fullscreen-element-changed event, raised when
    /// the page enters or leaves HTML fullscreen (for example a video going full
    /// screen). The handler receives the new
    /// [`contains_fullscreen_element`](Self::contains_fullscreen_element) state
    /// so the host can resize or restore its window to match.
    pub fn on_contains_fullscreen_element_changed<F: FnMut(bool) + 'static>(
        &self,
        handler: F,
    ) -> Result<EventRegistration> {
        let handler = handler::ContainsFullScreenElementChanged::create(handler);
        let token = unsafe { self.0.add_ContainsFullScreenElementChanged(&handler)? };
        let source = self.0.clone();
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_ContainsFullScreenElementChanged(token) };
        }))
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
        unsafe { protocol::add_requested_filter(&self.0, filter.as_ptr())? };
        let handler = protocol::WebResourceRequested::create(environment, handler);
        let token = unsafe { self.0.add_WebResourceRequested(&handler)? };
        let source = self.0.clone();
        Ok(EventRegistration::new(move || {
            let _ = unsafe { source.remove_WebResourceRequested(token) };
            unsafe { protocol::remove_requested_filter(&source, filter.as_ptr()) };
        }))
    }
}
