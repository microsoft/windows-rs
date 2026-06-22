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
}
