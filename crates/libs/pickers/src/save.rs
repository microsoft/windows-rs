use crate::bindings::*;
use crate::dialog::{
    DialogConfig, DialogSettings, PreparedDialog, show_dialog, validate_owner, wide,
};
use crate::filter::PreparedFilters;
use crate::{FileFilter, GUID, PickerLocation};
use std::path::PathBuf;
use windows_core::{PCWSTR, Result, create_instance};

/// Configures a Windows dialog that selects a filesystem path for saving a file.
#[derive(Clone, Debug, Default)]
pub struct SaveFilePicker {
    title: Option<String>,
    commit_label: Option<String>,
    filters: Vec<FileFilter>,
    initial_filter: Option<usize>,
    suggested_name: Option<String>,
    default_extension: Option<String>,
    overwrite_prompt: Option<bool>,
    settings: DialogSettings,
}

impl SaveFilePicker {
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

    /// Adds a file-type filter.
    pub fn filter(mut self, value: FileFilter) -> Self {
        self.filters.push(value);
        self
    }

    /// Selects the initially active file filter by zero-based index.
    pub fn initial_filter(mut self, index: usize) -> Self {
        self.initial_filter = Some(index);
        self
    }

    /// Sets the file name initially shown by the dialog.
    pub fn suggested_name(mut self, value: impl Into<String>) -> Self {
        self.suggested_name = Some(value.into());
        self
    }

    /// Sets the extension appended when the entered file name has no extension.
    ///
    /// A leading period is optional.
    pub fn default_extension(mut self, value: impl Into<String>) -> Self {
        self.default_extension = Some(value.into().trim_start_matches('.').to_string());
        self
    }

    /// Controls whether the dialog confirms before replacing an existing file.
    ///
    /// If not called, the native dialog default is preserved.
    pub fn overwrite_prompt(mut self, value: bool) -> Self {
        self.overwrite_prompt = Some(value);
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
    /// The calling thread must be initialized for COM and suitable for a modal UI operation.
    pub fn show_for_hwnd(self, owner: *mut core::ffi::c_void) -> Result<Option<PathBuf>> {
        validate_owner(owner)?;
        let filters = PreparedFilters::new(&self.filters, self.initial_filter)?;
        let dialog: IFileSaveDialog = create_instance(&FileSaveDialog)?;
        let (set_options, clear_options) = options(self.overwrite_prompt, !self.filters.is_empty());
        let _prepared = PreparedDialog::new(
            &dialog,
            DialogConfig {
                title: self.title.as_deref(),
                commit_label: self.commit_label.as_deref(),
                set_options,
                clear_options,
                settings: &self.settings,
            },
        )?;

        filters.apply(&dialog)?;

        let suggested_name = self.suggested_name.as_deref().map(wide).transpose()?;
        if let Some(value) = &suggested_name {
            unsafe { dialog.SetFileName(PCWSTR(value.as_ptr())).ok()? };
        }

        let default_extension = self.default_extension.as_deref().map(wide).transpose()?;
        if let Some(value) = &default_extension {
            unsafe { dialog.SetDefaultExtension(PCWSTR(value.as_ptr())).ok()? };
        }

        show_dialog(&dialog, owner)
    }
}

fn options(
    overwrite_prompt: Option<bool>,
    has_filters: bool,
) -> (FILEOPENDIALOGOPTIONS, FILEOPENDIALOGOPTIONS) {
    let mut set = FOS_FORCEFILESYSTEM;
    let mut clear = 0;
    if has_filters {
        set |= FOS_STRICTFILETYPES;
    }
    match overwrite_prompt {
        Some(true) => set |= FOS_OVERWRITEPROMPT,
        Some(false) => clear |= FOS_OVERWRITEPROMPT,
        None => {}
    }
    (set, clear)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_period_is_removed_from_default_extension() {
        let picker = SaveFilePicker::new().default_extension(".txt");
        assert_eq!(picker.default_extension.as_deref(), Some("txt"));
    }

    #[test]
    fn overwrite_prompt_changes_only_the_requested_flag() {
        assert_eq!(options(None, false), (FOS_FORCEFILESYSTEM, 0));
        assert_eq!(
            options(Some(true), false),
            (FOS_FORCEFILESYSTEM | FOS_OVERWRITEPROMPT, 0)
        );
        assert_eq!(
            options(Some(false), false),
            (FOS_FORCEFILESYSTEM, FOS_OVERWRITEPROMPT)
        );
    }

    #[test]
    fn configured_filters_enforce_save_file_types() {
        assert_eq!(
            options(None, true),
            (FOS_FORCEFILESYSTEM | FOS_STRICTFILETYPES, 0)
        );
    }
}
