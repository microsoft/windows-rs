use super::*;

/// The configurable settings of a [`WebView`], such as whether JavaScript, the
/// dev tools, or the default context menus are enabled. Obtain it with
/// [`WebView::settings`].
///
/// Each property is a getter / `set_*` pair; the setters take effect on the next
/// navigation.
pub struct Settings(pub(crate) ICoreWebView2Settings);

macro_rules! settings_bool {
    ($(#[$meta:meta])* $get:ident / $set:ident => $com_get:ident / $com_set:ident) => {
        $(#[$meta])*
        pub fn $get(&self) -> bool {
            unsafe { self.0.$com_get() }.is_ok_and(|value| value.as_bool())
        }

        $(#[$meta])*
        pub fn $set(&self, value: bool) -> Result<()> {
            unsafe { self.0.$com_set(value) }
        }
    };
}

macro_rules! settings_versioned_bool {
    ($iface:ty, $(#[$meta:meta])* $get:ident / $set:ident => $com_get:ident / $com_set:ident) => {
        $(#[$meta])*
        pub fn $get(&self) -> Result<bool> {
            let source: $iface = self.0.cast()?;
            Ok(unsafe { source.$com_get()? }.as_bool())
        }

        $(#[$meta])*
        pub fn $set(&self, value: bool) -> Result<()> {
            let source: $iface = self.0.cast()?;
            unsafe { source.$com_set(value) }
        }
    };
}

impl Settings {
    settings_bool! {
        /// Whether running JavaScript is enabled.
        is_script_enabled / set_script_enabled => IsScriptEnabled / SetIsScriptEnabled
    }

    settings_bool! {
        /// Whether communicating with the host via `postMessage` is enabled.
        is_web_message_enabled / set_web_message_enabled => IsWebMessageEnabled / SetIsWebMessageEnabled
    }

    settings_bool! {
        /// Whether the default JavaScript dialogs (`alert`, `confirm`, `prompt`,
        /// `beforeunload`) are shown.
        are_default_script_dialogs_enabled / set_default_script_dialogs_enabled => AreDefaultScriptDialogsEnabled / SetAreDefaultScriptDialogsEnabled
    }

    settings_bool! {
        /// Whether the status bar is displayed.
        is_status_bar_enabled / set_status_bar_enabled => IsStatusBarEnabled / SetIsStatusBarEnabled
    }

    settings_bool! {
        /// Whether the browser dev tools are available.
        are_dev_tools_enabled / set_dev_tools_enabled => AreDevToolsEnabled / SetAreDevToolsEnabled
    }

    settings_bool! {
        /// Whether the default right-click context menus are shown.
        are_default_context_menus_enabled / set_default_context_menus_enabled => AreDefaultContextMenusEnabled / SetAreDefaultContextMenusEnabled
    }

    settings_bool! {
        /// Whether host objects added via the WebView2 host-object APIs are
        /// accessible from the page.
        are_host_objects_allowed / set_host_objects_allowed => AreHostObjectsAllowed / SetAreHostObjectsAllowed
    }

    settings_bool! {
        /// Whether zoom control (for example Ctrl+`+`/`-`) is enabled.
        is_zoom_control_enabled / set_zoom_control_enabled => IsZoomControlEnabled / SetIsZoomControlEnabled
    }

    settings_bool! {
        /// Whether the built-in error page is shown for navigation and rendering
        /// failures.
        is_built_in_error_page_enabled / set_built_in_error_page_enabled => IsBuiltInErrorPageEnabled / SetIsBuiltInErrorPageEnabled
    }

    /// Returns the user-agent string sent with requests and reported to script.
    pub fn user_agent(&self) -> Result<String> {
        let source: ICoreWebView2Settings2 = self.0.cast()?;
        let value = unsafe { source.UserAgent()? };
        Ok(unsafe { string::take(value) })
    }

    /// Overrides the user-agent string sent with requests and reported to script.
    pub fn set_user_agent(&self, user_agent: &str) -> Result<()> {
        let source: ICoreWebView2Settings2 = self.0.cast()?;
        let user_agent = string::encode(user_agent);
        unsafe { source.SetUserAgent(user_agent.as_ptr()) }
    }

    settings_versioned_bool! {
        ICoreWebView2Settings3,
        /// Whether the browser's built-in accelerator keys (such as Ctrl+F or
        /// F5) are enabled. Disable to handle these keys yourself via
        /// [`Controller::on_accelerator_key_pressed`].
        are_browser_accelerator_keys_enabled / set_browser_accelerator_keys_enabled => AreBrowserAcceleratorKeysEnabled / SetAreBrowserAcceleratorKeysEnabled
    }

    settings_versioned_bool! {
        ICoreWebView2Settings4,
        /// Whether general form autofill (names, addresses, and similar) is
        /// enabled.
        is_general_autofill_enabled / set_general_autofill_enabled => IsGeneralAutofillEnabled / SetIsGeneralAutofillEnabled
    }

    settings_versioned_bool! {
        ICoreWebView2Settings4,
        /// Whether saving and autofilling passwords is enabled.
        is_password_autosave_enabled / set_password_autosave_enabled => IsPasswordAutosaveEnabled / SetIsPasswordAutosaveEnabled
    }

    settings_versioned_bool! {
        ICoreWebView2Settings5,
        /// Whether pinch-zoom (touch and trackpad) is enabled.
        is_pinch_zoom_enabled / set_pinch_zoom_enabled => IsPinchZoomEnabled / SetIsPinchZoomEnabled
    }

    settings_versioned_bool! {
        ICoreWebView2Settings6,
        /// Whether swipe gestures navigate back and forward.
        is_swipe_navigation_enabled / set_swipe_navigation_enabled => IsSwipeNavigationEnabled / SetIsSwipeNavigationEnabled
    }

    settings_versioned_bool! {
        ICoreWebView2Settings9,
        /// Whether elements styled with `app-region: drag` are treated as
        /// draggable non-client regions, letting a custom title bar move the host
        /// window.
        is_non_client_region_support_enabled / set_non_client_region_support_enabled => IsNonClientRegionSupportEnabled / SetIsNonClientRegionSupportEnabled
    }
}
