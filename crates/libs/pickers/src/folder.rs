use super::*;

/// Configures a Windows dialog that selects one filesystem folder.
#[derive(Clone, Debug, Default)]
pub struct FolderPicker {
    title: Option<String>,
    commit_label: Option<String>,
    settings: DialogSettings,
}

impl FolderPicker {
    /// Creates a picker with the native dialog defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the dialog title.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Sets the label of the button that confirms the selection.
    pub fn commit_label(mut self, value: impl Into<String>) -> Self {
        self.commit_label = Some(value.into());
        self
    }

    /// Sets the folder used when the dialog has no persisted location.
    pub fn default_folder(mut self, value: impl Into<PathBuf>) -> Self {
        self.settings.set_default(value);
        self
    }

    /// Sets the folder initially displayed by the dialog.
    pub fn folder(mut self, value: impl Into<PathBuf>) -> Self {
        self.settings.set_folder(value);
        self
    }

    /// Sets the known folder used when the dialog has no persisted location.
    pub fn default_location(mut self, value: PickerLocation) -> Self {
        self.settings.set_default_location(value);
        self
    }

    /// Sets the known folder initially displayed by the dialog.
    pub fn location(mut self, value: PickerLocation) -> Self {
        self.settings.set_location(value);
        self
    }

    /// Sets the stable identifier used by the Shell to persist dialog state.
    pub fn client_id(mut self, value: GUID) -> Self {
        self.settings.set_client_id(value);
        self
    }

    /// Shows the picker owned by the given window.
    #[cfg(feature = "system")]
    pub fn show(self, owner: &windows_window::Window) -> Result<Option<PathBuf>> {
        self.show_for_hwnd(owner.hwnd())
    }

    /// Shows the picker owned by a raw window handle.
    ///
    /// The calling thread must be a COM single-threaded apartment (STA).
    pub fn show_for_hwnd(self, owner: *mut core::ffi::c_void) -> Result<Option<PathBuf>> {
        validate_owner(owner)?;
        let (dialog, _prepared) = self.prepare(FOS_FORCEFILESYSTEM | FOS_PICKFOLDERS)?;
        show_dialog(&dialog, owner)
    }

    /// Shows the picker with multiple selection enabled.
    ///
    /// An empty vector means the user cancelled.
    #[cfg(feature = "system")]
    pub fn show_multiple(self, owner: &windows_window::Window) -> Result<Vec<PathBuf>> {
        self.show_multiple_for_hwnd(owner.hwnd())
    }

    /// Shows the picker with multiple selection enabled and owned by a raw window handle.
    ///
    /// An empty vector means the user cancelled. The calling thread must be a COM single-threaded
    /// apartment (STA).
    pub fn show_multiple_for_hwnd(self, owner: *mut core::ffi::c_void) -> Result<Vec<PathBuf>> {
        validate_owner(owner)?;
        let (dialog, _prepared) =
            self.prepare(FOS_FORCEFILESYSTEM | FOS_PICKFOLDERS | FOS_ALLOWMULTISELECT)?;
        show_dialog_multiple(&dialog, owner)
    }

    fn prepare(
        self,
        required_options: FILEOPENDIALOGOPTIONS,
    ) -> Result<(IFileOpenDialog, PreparedDialog)> {
        let dialog: IFileOpenDialog = create_instance(&FileOpenDialog)?;
        let prepared = PreparedDialog::new(
            &dialog,
            DialogConfig {
                title: self.title.as_deref(),
                commit_label: self.commit_label.as_deref(),
                set_options: required_options,
                clear_options: 0,
                settings: &self.settings,
            },
        )?;
        Ok((dialog, prepared))
    }
}
