use super::*;

/// Details about a completed navigation, delivered to a
/// [`WebView::on_navigation_completed`] handler.
pub struct NavigationCompletedArgs(pub(crate) ICoreWebView2NavigationCompletedEventArgs);

impl NavigationCompletedArgs {
    /// Returns `true` if the navigation succeeded.
    pub fn is_success(&self) -> bool {
        unsafe { self.0.IsSuccess() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns the unique identifier for the navigation.
    pub fn navigation_id(&self) -> u64 {
        unsafe { self.0.NavigationId() }.unwrap_or(0)
    }
}

/// Details about a navigation that is about to start, delivered to a
/// [`WebView::on_navigation_starting`] handler. The navigation can be cancelled
/// with [`set_cancel`](Self::set_cancel).
pub struct NavigationStartingArgs(pub(crate) ICoreWebView2NavigationStartingEventArgs);

impl NavigationStartingArgs {
    /// Returns the URI the navigation is targeting.
    pub fn uri(&self) -> String {
        unsafe { string::take_result(self.0.Uri()) }
    }

    /// Returns `true` if the navigation was initiated by the user (for example a
    /// link click) rather than by script.
    pub fn is_user_initiated(&self) -> bool {
        unsafe { self.0.IsUserInitiated() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns `true` if the navigation is a redirect.
    pub fn is_redirected(&self) -> bool {
        unsafe { self.0.IsRedirected() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns the unique identifier for the navigation.
    pub fn navigation_id(&self) -> u64 {
        unsafe { self.0.NavigationId() }.unwrap_or(0)
    }

    /// Returns `true` if the navigation is currently marked to be cancelled.
    pub fn is_cancelled(&self) -> bool {
        unsafe { self.0.Cancel() }.is_ok_and(|value| value.as_bool())
    }

    /// Cancels (or un-cancels) the navigation.
    pub fn set_cancel(&self, cancel: bool) -> Result<()> {
        unsafe { self.0.SetCancel(cancel) }.ok()
    }
}

/// A message posted from the hosted page's JavaScript, delivered to a
/// [`WebView::on_web_message_received`] handler.
pub struct WebMessageReceivedArgs(pub(crate) ICoreWebView2WebMessageReceivedEventArgs);

impl WebMessageReceivedArgs {
    /// Returns the URI of the document that sent the message.
    pub fn source(&self) -> String {
        unsafe { string::take_result(self.0.Source()) }
    }

    /// Returns the message serialized as a JSON string. Messages sent with
    /// `window.chrome.webview.postMessage` arrive here regardless of type.
    pub fn web_message_as_json(&self) -> String {
        unsafe { string::take_result(self.0.WebMessageAsJson()) }
    }

    /// Returns the message as a string. Fails if the page posted a value that is
    /// not a JavaScript string.
    pub fn try_web_message_as_string(&self) -> Result<String> {
        let value = unsafe { self.0.TryGetWebMessageAsString()? };
        Ok(unsafe { string::take(value) })
    }
}

/// Details about content that is starting to load, delivered to a
/// [`WebView::on_content_loading`] handler.
pub struct ContentLoadingArgs(pub(crate) ICoreWebView2ContentLoadingEventArgs);

impl ContentLoadingArgs {
    /// Returns `true` if the loading content is the error page.
    pub fn is_error_page(&self) -> bool {
        unsafe { self.0.IsErrorPage() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns the unique identifier for the navigation.
    pub fn navigation_id(&self) -> u64 {
        unsafe { self.0.NavigationId() }.unwrap_or(0)
    }
}

/// Details about a page requesting to open a new window (for example via
/// `window.open` or a target link), delivered to a
/// [`WebView::on_new_window_requested`] handler. Mark it
/// [handled](Self::set_handled) to suppress the default new-window behaviour.
pub struct NewWindowRequestedArgs(pub(crate) ICoreWebView2NewWindowRequestedEventArgs);

impl NewWindowRequestedArgs {
    /// Returns the URI the new window would navigate to.
    pub fn uri(&self) -> String {
        unsafe { string::take_result(self.0.Uri()) }
    }

    /// Returns `true` if the request was initiated by the user rather than by
    /// script.
    pub fn is_user_initiated(&self) -> bool {
        unsafe { self.0.IsUserInitiated() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns `true` if the request has been marked as handled.
    pub fn is_handled(&self) -> bool {
        unsafe { self.0.Handled() }.is_ok_and(|value| value.as_bool())
    }

    /// Marks the request as handled, suppressing the creation of a default new
    /// window. Set this (without providing a [new window](Self::set_new_window))
    /// to block the popup entirely.
    pub fn set_handled(&self, handled: bool) -> Result<()> {
        unsafe { self.0.SetHandled(handled) }.ok()
    }

    /// Provides an existing [`WebView`] to host the requested content instead of
    /// creating a new window.
    pub fn set_new_window(&self, webview: &WebView) -> Result<()> {
        unsafe { self.0.SetNewWindow(&webview.0) }.ok()
    }

    /// Takes a [`Deferral`] so the request can be resolved after the handler
    /// returns, for example once an asynchronously created window is ready.
    pub fn defer(&self) -> Result<Deferral> {
        Ok(Deferral::new(unsafe { self.0.GetDeferral()? }))
    }
}

/// The kind of capability a page is requesting in a
/// [`PermissionRequestedArgs`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PermissionKind {
    Unknown,
    Microphone,
    Camera,
    Geolocation,
    Notifications,
    OtherSensors,
    ClipboardRead,
    MultipleAutomaticDownloads,
    FileReadWrite,
    Autoplay,
    LocalFonts,
    MidiSystemExclusiveMessages,
    WindowManagement,
}

impl PermissionKind {
    fn from_raw(value: COREWEBVIEW2_PERMISSION_KIND) -> Self {
        match value {
            1 => Self::Microphone,
            2 => Self::Camera,
            3 => Self::Geolocation,
            4 => Self::Notifications,
            5 => Self::OtherSensors,
            6 => Self::ClipboardRead,
            7 => Self::MultipleAutomaticDownloads,
            8 => Self::FileReadWrite,
            9 => Self::Autoplay,
            10 => Self::LocalFonts,
            11 => Self::MidiSystemExclusiveMessages,
            12 => Self::WindowManagement,
            _ => Self::Unknown,
        }
    }
}

/// How a [`PermissionRequestedArgs`] should be resolved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermissionState {
    /// Defer to the browser default (typically prompting the user).
    Default,
    /// Grant the permission.
    Allow,
    /// Deny the permission.
    Deny,
}

impl PermissionState {
    fn from_raw(value: COREWEBVIEW2_PERMISSION_STATE) -> Self {
        match value {
            1 => Self::Allow,
            2 => Self::Deny,
            _ => Self::Default,
        }
    }

    fn to_raw(self) -> COREWEBVIEW2_PERMISSION_STATE {
        match self {
            Self::Default => 0,
            Self::Allow => 1,
            Self::Deny => 2,
        }
    }
}

/// Details about a permission a page is requesting (for example camera or
/// geolocation access), delivered to a [`WebView::on_permission_requested`]
/// handler. Decide the outcome with [`set_state`](Self::set_state).
pub struct PermissionRequestedArgs(pub(crate) ICoreWebView2PermissionRequestedEventArgs);

impl PermissionRequestedArgs {
    /// Returns the URI of the page requesting the permission.
    pub fn uri(&self) -> String {
        unsafe { string::take_result(self.0.Uri()) }
    }

    /// Returns the kind of permission being requested.
    pub fn kind(&self) -> PermissionKind {
        unsafe { self.0.PermissionKind() }.map_or(PermissionKind::Unknown, PermissionKind::from_raw)
    }

    /// Returns `true` if the request was initiated by the user rather than by
    /// script.
    pub fn is_user_initiated(&self) -> bool {
        unsafe { self.0.IsUserInitiated() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns the current resolution state of the request.
    pub fn state(&self) -> PermissionState {
        unsafe { self.0.State() }.map_or(PermissionState::Default, PermissionState::from_raw)
    }

    /// Sets how the request should be resolved.
    pub fn set_state(&self, state: PermissionState) -> Result<()> {
        unsafe { self.0.SetState(state.to_raw()) }.ok()
    }

    /// Takes a [`Deferral`] so the request can be resolved after the handler
    /// returns, for example once the user has answered a prompt.
    pub fn defer(&self) -> Result<Deferral> {
        Ok(Deferral::new(unsafe { self.0.GetDeferral()? }))
    }
}

/// The kind of process that failed, reported by [`ProcessFailedArgs::kind`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProcessFailedKind {
    Unknown,
    /// The browser process ended unexpectedly; the `WebView` is no longer usable.
    BrowserProcessExited,
    /// A render process ended unexpectedly (a renderer crash); the page is gone
    /// but the `WebView` can be reloaded.
    RenderProcessExited,
    /// A render process is unresponsive (hung).
    RenderProcessUnresponsive,
    FrameRenderProcessExited,
    UtilityProcessExited,
    SandboxHelperProcessExited,
    GpuProcessExited,
    PpapiPluginProcessExited,
    PpapiBrokerProcessExited,
    UnknownProcessExited,
}

impl ProcessFailedKind {
    fn from_raw(value: COREWEBVIEW2_PROCESS_FAILED_KIND) -> Self {
        match value {
            0 => Self::BrowserProcessExited,
            1 => Self::RenderProcessExited,
            2 => Self::RenderProcessUnresponsive,
            3 => Self::FrameRenderProcessExited,
            4 => Self::UtilityProcessExited,
            5 => Self::SandboxHelperProcessExited,
            6 => Self::GpuProcessExited,
            7 => Self::PpapiPluginProcessExited,
            8 => Self::PpapiBrokerProcessExited,
            9 => Self::UnknownProcessExited,
            _ => Self::Unknown,
        }
    }
}

/// Details about a failed or unresponsive browser process, delivered to a
/// [`WebView::on_process_failed`] handler.
pub struct ProcessFailedArgs(pub(crate) ICoreWebView2ProcessFailedEventArgs);

impl ProcessFailedArgs {
    /// Returns which kind of process failed.
    pub fn kind(&self) -> ProcessFailedKind {
        unsafe { self.0.ProcessFailedKind() }
            .map_or(ProcessFailedKind::Unknown, ProcessFailedKind::from_raw)
    }
}

/// The reason focus is moving, reported by [`MoveFocusRequestedArgs::reason`]
/// and passed to [`Controller::move_focus`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveFocusReason {
    /// Focus is being set programmatically.
    Programmatic,
    /// Focus is moving forward (for example Tab).
    Next,
    /// Focus is moving backward (for example Shift+Tab).
    Previous,
}

impl MoveFocusReason {
    fn from_raw(value: COREWEBVIEW2_MOVE_FOCUS_REASON) -> Self {
        match value {
            1 => Self::Next,
            2 => Self::Previous,
            _ => Self::Programmatic,
        }
    }

    pub(crate) fn to_raw(self) -> COREWEBVIEW2_MOVE_FOCUS_REASON {
        match self {
            Self::Programmatic => 0,
            Self::Next => 1,
            Self::Previous => 2,
        }
    }
}

/// The request to move focus out of the browser (for example the user tabbed
/// past the last element), delivered to a [`Controller::on_move_focus_requested`]
/// handler. Move focus to the appropriate host control and call
/// [`set_handled(true)`](Self::set_handled) to suppress the default behaviour.
pub struct MoveFocusRequestedArgs(pub(crate) ICoreWebView2MoveFocusRequestedEventArgs);

impl MoveFocusRequestedArgs {
    /// Returns the direction focus is moving.
    pub fn reason(&self) -> MoveFocusReason {
        unsafe { self.0.Reason() }.map_or(MoveFocusReason::Programmatic, MoveFocusReason::from_raw)
    }

    /// Returns `true` if the request has been marked as handled.
    pub fn is_handled(&self) -> bool {
        unsafe { self.0.Handled() }.is_ok_and(|value| value.as_bool())
    }

    /// Marks the request as handled, indicating the host moved focus itself and
    /// WebView2 should not apply its default focus behaviour.
    pub fn set_handled(&self, handled: bool) -> Result<()> {
        unsafe { self.0.SetHandled(handled) }.ok()
    }
}

/// The kind of key event reported by [`AcceleratorKeyPressedArgs::key_event_kind`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyEventKind {
    KeyDown,
    KeyUp,
    /// A system key was pressed (for example a key combined with Alt).
    SystemKeyDown,
    /// A system key was released.
    SystemKeyUp,
}

impl KeyEventKind {
    fn from_raw(value: COREWEBVIEW2_KEY_EVENT_KIND) -> Self {
        match value {
            1 => Self::KeyUp,
            2 => Self::SystemKeyDown,
            3 => Self::SystemKeyUp,
            _ => Self::KeyDown,
        }
    }
}

/// An accelerator (browser-level) key press, delivered to a
/// [`Controller::on_accelerator_key_pressed`] handler before the page sees it —
/// the place to implement application keyboard shortcuts. Call
/// [`set_handled(true)`](Self::set_handled) to stop WebView2 from acting on the
/// key.
pub struct AcceleratorKeyPressedArgs(pub(crate) ICoreWebView2AcceleratorKeyPressedEventArgs);

impl AcceleratorKeyPressedArgs {
    /// Returns whether the key was pressed or released, and whether it is a
    /// system key.
    pub fn key_event_kind(&self) -> KeyEventKind {
        unsafe { self.0.KeyEventKind() }.map_or(KeyEventKind::KeyDown, KeyEventKind::from_raw)
    }

    /// Returns the Win32 virtual-key code of the key.
    pub fn virtual_key(&self) -> u32 {
        unsafe { self.0.VirtualKey() }.unwrap_or(0)
    }

    /// Returns `true` if the key has been marked as handled.
    pub fn is_handled(&self) -> bool {
        unsafe { self.0.Handled() }.is_ok_and(|value| value.as_bool())
    }

    /// Marks the key as handled, preventing WebView2's default processing so the
    /// host can act on the shortcut itself.
    pub fn set_handled(&self, handled: bool) -> Result<()> {
        unsafe { self.0.SetHandled(handled) }.ok()
    }
}

/// A Chrome DevTools Protocol event, delivered to a
/// [`WebView::on_dev_tools_protocol_event`] handler. The event's parameters are
/// available as a JSON object string.
pub struct DevToolsProtocolEventReceivedArgs(
    pub(crate) ICoreWebView2DevToolsProtocolEventReceivedEventArgs,
);

impl DevToolsProtocolEventReceivedArgs {
    /// Returns the event's parameter object as a JSON string, matching the
    /// `params` of the corresponding CDP event.
    pub fn parameter_object_as_json(&self) -> String {
        unsafe { string::take_result(self.0.ParameterObjectAsJson()) }
    }
}

/// An RAII guard for an event subscription. The handler stays registered until
/// this value is dropped or [`EventRegistration::remove`] is called.
#[must_use]
pub struct EventRegistration(Option<Box<dyn FnOnce()>>);

impl EventRegistration {
    pub(crate) fn new<F: FnOnce() + 'static>(remove: F) -> Self {
        Self(Some(Box::new(remove)))
    }

    /// Unsubscribes the handler.
    pub fn remove(mut self) {
        if let Some(remove) = self.0.take() {
            remove();
        }
    }
}

impl Drop for EventRegistration {
    fn drop(&mut self) {
        if let Some(remove) = self.0.take() {
            remove();
        }
    }
}
