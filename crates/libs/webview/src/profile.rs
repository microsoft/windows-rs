use super::*;

/// The colour scheme a [`Profile`] reports to pages through the
/// `prefers-color-scheme` media query.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreferredColorScheme {
    /// Follow the operating system's setting.
    Auto,
    Light,
    Dark,
}

impl PreferredColorScheme {
    fn from_raw(value: COREWEBVIEW2_PREFERRED_COLOR_SCHEME) -> Self {
        match value {
            1 => Self::Light,
            2 => Self::Dark,
            _ => Self::Auto,
        }
    }

    fn to_raw(self) -> COREWEBVIEW2_PREFERRED_COLOR_SCHEME {
        match self {
            Self::Auto => 0,
            Self::Light => 1,
            Self::Dark => 2,
        }
    }
}

/// A WebView2 browser profile — the on-disk identity (name, path, private mode)
/// shared by every `WebView` created with it, plus its colour scheme, download
/// folder, and browsing-data controls. Obtain it with [`WebView::profile`].
pub struct Profile(pub(crate) ICoreWebView2Profile);

impl Profile {
    /// Returns the profile's name.
    pub fn name(&self) -> String {
        unsafe { string::take_result(self.0.ProfileName()) }
    }

    /// Returns `true` if the profile runs in private (incognito) mode.
    pub fn is_in_private_mode(&self) -> bool {
        unsafe { self.0.IsInPrivateModeEnabled() }.is_ok_and(|value| value.as_bool())
    }

    /// Returns the absolute path of the profile's on-disk directory.
    pub fn path(&self) -> String {
        unsafe { string::take_result(self.0.ProfilePath()) }
    }

    /// Returns the folder downloads are saved to by default.
    pub fn default_download_folder_path(&self) -> String {
        unsafe { string::take_result(self.0.DefaultDownloadFolderPath()) }
    }

    /// Sets the folder downloads are saved to by default.
    pub fn set_default_download_folder_path(&self, path: &str) -> Result<()> {
        let path = string::encode(path);
        unsafe { self.0.SetDefaultDownloadFolderPath(path.as_ptr()) }
    }

    /// Returns the colour scheme reported to pages.
    pub fn preferred_color_scheme(&self) -> PreferredColorScheme {
        unsafe { self.0.PreferredColorScheme() }
            .map_or(PreferredColorScheme::Auto, PreferredColorScheme::from_raw)
    }

    /// Sets the colour scheme reported to pages through `prefers-color-scheme`,
    /// driving a site's light/dark theme.
    pub fn set_preferred_color_scheme(&self, scheme: PreferredColorScheme) -> Result<()> {
        unsafe { self.0.SetPreferredColorScheme(scheme.to_raw()) }
    }

    /// Asynchronously clears all of the profile's browsing data (cookies, cache,
    /// history, storage, and so on). The `handler` closure runs on the UI thread
    /// once the data has been cleared.
    pub fn clear_browsing_data_all<F: FnOnce(Result<()>) + 'static>(
        &self,
        handler: F,
    ) -> Result<()> {
        let source: ICoreWebView2Profile2 = self.0.cast()?;
        let handler = handler::ClearBrowsingDataCompleted::create(handler);
        unsafe { source.ClearBrowsingDataAll(&handler) }
    }
}
