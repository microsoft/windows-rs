use super::*;
use std::cell::RefCell;
use windows_core::implement_decl;

/// Filter context matching every resource type (`COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL`).
pub(crate) const WEB_RESOURCE_CONTEXT_ALL: COREWEBVIEW2_WEB_RESOURCE_CONTEXT = 0;

/// Request sources matching every kind (`COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS_ALL`),
/// so a filter also intercepts requests from iframes and workers.
const WEB_RESOURCE_REQUEST_SOURCE_KINDS_ALL: COREWEBVIEW2_WEB_RESOURCE_REQUEST_SOURCE_KINDS =
    u32::MAX;

/// Registers a web-resource-requested filter for `uri`. On runtimes that support
/// it (`ICoreWebView2_22`) the filter covers every request source - including
/// iframes and workers - falling back to the document-only filter otherwise.
pub(crate) unsafe fn add_requested_filter(
    webview: &ICoreWebView2,
    uri: impl Param<PCWSTR>,
) -> Result<()> {
    unsafe {
        match webview.cast::<ICoreWebView2_22>() {
            Ok(webview) => webview
                .AddWebResourceRequestedFilterWithRequestSourceKinds(
                    uri,
                    WEB_RESOURCE_CONTEXT_ALL,
                    WEB_RESOURCE_REQUEST_SOURCE_KINDS_ALL,
                )
                .ok(),
            Err(_) => webview
                .AddWebResourceRequestedFilter(uri, WEB_RESOURCE_CONTEXT_ALL)
                .ok(),
        }
    }
}

/// Removes a filter registered with [`add_requested_filter`], matching whichever
/// API registered it.
pub(crate) unsafe fn remove_requested_filter(webview: &ICoreWebView2, uri: impl Param<PCWSTR>) {
    let _ = unsafe {
        match webview.cast::<ICoreWebView2_22>() {
            Ok(webview) => webview.RemoveWebResourceRequestedFilterWithRequestSourceKinds(
                uri,
                WEB_RESOURCE_CONTEXT_ALL,
                WEB_RESOURCE_REQUEST_SOURCE_KINDS_ALL,
            ),
            Err(_) => webview.RemoveWebResourceRequestedFilter(uri, WEB_RESOURCE_CONTEXT_ALL),
        }
    };
}

/// A resource request intercepted by [`WebView::on_web_resource_requested`],
/// exposing the requested URI, HTTP method, and headers.
pub struct WebResourceRequest(pub(crate) ICoreWebView2WebResourceRequest);

impl WebResourceRequest {
    /// Returns the absolute URI of the requested resource.
    pub fn uri(&self) -> String {
        unsafe { string::take_result(self.0.Uri()) }
    }

    /// Returns the HTTP method of the request, such as `GET` or `POST`.
    pub fn method(&self) -> String {
        unsafe { string::take_result(self.0.Method()) }
    }

    /// Returns the request headers as `(name, value)` pairs.
    pub fn headers(&self) -> Vec<(String, String)> {
        unsafe { self.collect_headers() }.unwrap_or_default()
    }

    unsafe fn collect_headers(&self) -> Result<Vec<(String, String)>> {
        let iterator = unsafe { self.0.Headers()?.GetIterator()? };
        let mut headers = Vec::new();

        while unsafe { iterator.HasCurrentHeader()? }.as_bool() {
            let mut name: LPWSTR = std::ptr::null_mut();
            let mut value: LPWSTR = std::ptr::null_mut();
            unsafe { iterator.GetCurrentHeader(&mut name, &mut value).ok()? };
            headers.push((unsafe { string::take(name) }, unsafe {
                string::take(value)
            }));
            let _ = unsafe { iterator.MoveNext()? };
        }

        Ok(headers)
    }
}

/// The response returned from a [`WebView::on_web_resource_requested`] handler
/// to fulfil a request from memory. Defaults to `200 OK` with no headers; add a
/// [`content_type`](Self::content_type) so the page interprets the body
/// correctly.
pub struct WebResourceResponse {
    body: Vec<u8>,
    status_code: i32,
    reason_phrase: String,
    headers: Vec<(String, String)>,
}

impl WebResourceResponse {
    /// Creates a `200 OK` response with the given body bytes.
    pub fn new(body: impl Into<Vec<u8>>) -> Self {
        Self {
            body: body.into(),
            status_code: 200,
            reason_phrase: "OK".to_string(),
            headers: Vec::new(),
        }
    }

    /// Sets the HTTP status code and reason phrase, for example `404` and
    /// `Not Found`.
    pub fn status(mut self, code: u16, reason: &str) -> Self {
        self.status_code = i32::from(code);
        self.reason_phrase = reason.to_string();
        self
    }

    /// Adds a response header.
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    /// Sets the `Content-Type` header, for example `text/html`.
    pub fn content_type(self, value: &str) -> Self {
        self.header("Content-Type", value)
    }

    unsafe fn into_response(
        self,
        environment: &ICoreWebView2Environment,
    ) -> Result<ICoreWebView2WebResourceResponse> {
        let stream = if self.body.is_empty() {
            None
        } else {
            unsafe { SHCreateMemStream(self.body.as_ptr(), self.body.len() as u32) }
        };

        let mut headers = String::new();
        for (name, value) in &self.headers {
            headers.push_str(name);
            headers.push_str(": ");
            headers.push_str(value);
            headers.push_str("\r\n");
        }

        let reason = HSTRING::from(&self.reason_phrase);
        let headers = HSTRING::from(&headers);

        unsafe {
            environment.CreateWebResourceResponse(
                stream.as_ref(),
                self.status_code,
                &reason,
                &headers,
            )
        }
    }
}

/// Adapts a Rust closure to the `ICoreWebView2WebResourceRequestedEventHandler`
/// COM interface. The captured environment turns the [`WebResourceResponse`] the
/// closure returns into the COM response handed back to WebView2.
pub(crate) struct WebResourceRequested {
    handler: RefCell<Box<dyn FnMut(WebResourceRequest) -> Option<WebResourceResponse>>>,
    environment: ICoreWebView2Environment,
}

implement_decl! {
    impl WebResourceRequested as pub(crate) WebResourceRequested_Impl:
        [ICoreWebView2WebResourceRequestedEventHandler]
}

impl WebResourceRequested {
    pub(crate) fn create<F>(
        environment: ICoreWebView2Environment,
        handler: F,
    ) -> ICoreWebView2WebResourceRequestedEventHandler
    where
        F: FnMut(WebResourceRequest) -> Option<WebResourceResponse> + 'static,
    {
        Self {
            handler: RefCell::new(Box::new(handler)),
            environment,
        }
        .into()
    }
}

impl ICoreWebView2WebResourceRequestedEventHandler_Impl for WebResourceRequested_Impl {
    fn Invoke(
        &self,
        _sender: Ref<ICoreWebView2>,
        args: Ref<ICoreWebView2WebResourceRequestedEventArgs>,
    ) -> Result<()> {
        let args = args.ok()?;
        let request = WebResourceRequest(unsafe { args.Request()? });

        if let Some(response) = (*self.handler.borrow_mut())(request) {
            let response = unsafe { response.into_response(&self.environment)? };
            unsafe { args.SetResponse(&response).ok()? };
        }

        Ok(())
    }
}
