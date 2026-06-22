use super::*;
use windows_core::implement_decl;

/// Configures the WebView2 [`Environment`], including the user data folder, the
/// browser executable folder, additional browser command-line arguments, and the
/// UI language. Build one with the fluent setters and pass it to
/// [`Environment::with_options`].
///
/// ```
/// use windows_webview::EnvironmentOptions;
///
/// let options = EnvironmentOptions::new()
///     .user_data_folder(r"C:\MyApp\WebView2")
///     .additional_browser_arguments("--disable-features=msSmartScreenProtection")
///     .language("en-US");
/// ```
#[derive(Clone, Debug, Default)]
pub struct EnvironmentOptions {
    browser_executable_folder: Option<String>,
    user_data_folder: Option<String>,
    additional_browser_arguments: Option<String>,
    language: Option<String>,
    target_compatible_browser_version: Option<String>,
    allow_single_sign_on_using_os_primary_account: bool,
}

impl EnvironmentOptions {
    /// Creates options with all WebView2 defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the folder containing the WebView2 browser (Edge) binaries to use
    /// instead of the installed runtime.
    pub fn browser_executable_folder(mut self, value: impl Into<String>) -> Self {
        self.browser_executable_folder = Some(value.into());
        self
    }

    /// Sets the folder where WebView2 stores its user data (cache, cookies, …).
    pub fn user_data_folder(mut self, value: impl Into<String>) -> Self {
        self.user_data_folder = Some(value.into());
        self
    }

    /// Sets additional command-line arguments passed to the browser process.
    pub fn additional_browser_arguments(mut self, value: impl Into<String>) -> Self {
        self.additional_browser_arguments = Some(value.into());
        self
    }

    /// Sets the default display language (for example `"en-US"`).
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    /// Sets the minimum compatible browser version the environment requires.
    pub fn target_compatible_browser_version(mut self, value: impl Into<String>) -> Self {
        self.target_compatible_browser_version = Some(value.into());
        self
    }

    /// Enables single sign-on using the operating system's primary account.
    pub fn allow_single_sign_on_using_os_primary_account(mut self, value: bool) -> Self {
        self.allow_single_sign_on_using_os_primary_account = value;
        self
    }

    pub(crate) fn create_environment<F: FnOnce(Result<Environment>) + 'static>(
        &self,
        handler: F,
    ) -> Result<()> {
        let handler = handler::EnvironmentCompleted::create(handler);
        let options: ICoreWebView2EnvironmentOptions = OptionsObject::new(self).into();

        let browser = self
            .browser_executable_folder
            .as_deref()
            .map(string::encode);
        let user_data = self.user_data_folder.as_deref().map(string::encode);
        let browser = browser
            .as_ref()
            .map_or(std::ptr::null(), |value| value.as_ptr());
        let user_data = user_data
            .as_ref()
            .map_or(std::ptr::null(), |value| value.as_ptr());

        unsafe {
            CreateCoreWebView2EnvironmentWithOptions(
                browser,
                user_data,
                Interface::as_raw(&options),
                Interface::as_raw(&handler),
            )
            .ok()
        }
    }
}

/// A read-only `ICoreWebView2EnvironmentOptions` implementation backed by an
/// [`EnvironmentOptions`]. WebView2 reads the configured values through the
/// getters; the setters exist only to satisfy the COM vtable and are never
/// invoked, so they are inert.
pub(crate) struct OptionsObject {
    additional_browser_arguments: String,
    language: String,
    target_compatible_browser_version: String,
    allow_single_sign_on_using_os_primary_account: BOOL,
}

implement_decl! {
    impl OptionsObject as pub(crate) OptionsObject_Impl:
        [ICoreWebView2EnvironmentOptions]
}

impl OptionsObject {
    fn new(options: &EnvironmentOptions) -> Self {
        Self {
            additional_browser_arguments: options
                .additional_browser_arguments
                .clone()
                .unwrap_or_default(),
            language: options.language.clone().unwrap_or_default(),
            target_compatible_browser_version: options
                .target_compatible_browser_version
                .clone()
                .unwrap_or_default(),
            allow_single_sign_on_using_os_primary_account: options
                .allow_single_sign_on_using_os_primary_account
                .into(),
        }
    }
}

impl ICoreWebView2EnvironmentOptions_Impl for OptionsObject_Impl {
    fn AdditionalBrowserArguments(&self) -> Result<LPWSTR> {
        unsafe { string::allocate(&self.additional_browser_arguments) }
    }

    fn SetAdditionalBrowserArguments(&self, _value: LPCWSTR) -> Result<()> {
        Ok(())
    }

    fn Language(&self) -> Result<LPWSTR> {
        unsafe { string::allocate(&self.language) }
    }

    fn SetLanguage(&self, _value: LPCWSTR) -> Result<()> {
        Ok(())
    }

    fn TargetCompatibleBrowserVersion(&self) -> Result<LPWSTR> {
        unsafe { string::allocate(&self.target_compatible_browser_version) }
    }

    fn SetTargetCompatibleBrowserVersion(&self, _value: LPCWSTR) -> Result<()> {
        Ok(())
    }

    fn AllowSingleSignOnUsingOSPrimaryAccount(&self) -> Result<BOOL> {
        Ok(self.allow_single_sign_on_using_os_primary_account)
    }

    fn SetAllowSingleSignOnUsingOSPrimaryAccount(&self, _allow: BOOL) -> Result<()> {
        Ok(())
    }
}
