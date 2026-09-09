use super::*;

const ERROR_CANCELLED: u32 = 1223;
const ERROR_CANCELLED_HRESULT: HRESULT = HRESULT((0x8007_0000_u32 | ERROR_CANCELLED) as i32);
const E_INVALIDARG: HRESULT = HRESULT(0x8007_0057_u32 as i32);

pub(crate) struct PreparedDialog {
    _title: Option<Vec<u16>>,
    _commit_label: Option<Vec<u16>>,
}

pub(crate) struct DialogConfig<'a> {
    pub(crate) title: Option<&'a str>,
    pub(crate) commit_label: Option<&'a str>,
    pub(crate) set_options: FILEOPENDIALOGOPTIONS,
    pub(crate) clear_options: FILEOPENDIALOGOPTIONS,
    pub(crate) settings: &'a DialogSettings,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct DialogSettings {
    client_id: Option<GUID>,
    default: Option<DialogFolder>,
    folder: Option<DialogFolder>,
}

#[derive(Clone, Debug)]
enum DialogFolder {
    Path(PathBuf),
    Known(PickerLocation),
}

impl DialogSettings {
    pub(crate) fn set_client_id(&mut self, value: GUID) {
        self.client_id = Some(value);
    }

    pub(crate) fn set_default(&mut self, value: impl Into<PathBuf>) {
        self.default = Some(DialogFolder::Path(value.into()));
    }

    pub(crate) fn set_folder(&mut self, value: impl Into<PathBuf>) {
        self.folder = Some(DialogFolder::Path(value.into()));
    }

    pub(crate) fn set_default_location(&mut self, value: PickerLocation) {
        self.default = Some(DialogFolder::Known(value));
    }

    pub(crate) fn set_location(&mut self, value: PickerLocation) {
        self.folder = Some(DialogFolder::Known(value));
    }

    fn apply_client_id(&self, dialog: &IFileDialog) -> Result<()> {
        if let Some(client_id) = &self.client_id {
            unsafe { dialog.SetClientGuid(client_id).ok()? };
        }
        Ok(())
    }

    fn apply_folders(&self, dialog: &IFileDialog) -> Result<()> {
        if let Some(folder) = &self.default {
            let item = shell_item(folder)?;
            unsafe { dialog.SetDefaultFolder(&item).ok()? };
        }
        if let Some(folder) = &self.folder {
            let item = shell_item(folder)?;
            unsafe { dialog.SetFolder(&item).ok()? };
        }
        Ok(())
    }
}

impl PreparedDialog {
    pub(crate) fn new(dialog: &IFileDialog, config: DialogConfig<'_>) -> Result<Self> {
        config.settings.apply_client_id(dialog)?;

        let title = config.title.map(wide).transpose()?;
        if let Some(title) = &title {
            unsafe { dialog.SetTitle(PCWSTR(title.as_ptr())).ok()? };
        }

        let commit_label = config.commit_label.map(wide).transpose()?;
        if let Some(commit_label) = &commit_label {
            unsafe {
                dialog
                    .SetOkButtonLabel(PCWSTR(commit_label.as_ptr()))
                    .ok()?;
            };
        }

        let options = unsafe { dialog.GetOptions()? };
        unsafe {
            dialog
                .SetOptions(merge_options(
                    options,
                    config.set_options,
                    config.clear_options,
                ))
                .ok()?;
        };
        config.settings.apply_folders(dialog)?;

        Ok(Self {
            _title: title,
            _commit_label: commit_label,
        })
    }
}

pub(crate) fn show_dialog(
    dialog: &IFileDialog,
    owner: *mut core::ffi::c_void,
) -> Result<Option<PathBuf>> {
    if !show_modal(dialog, owner)? {
        return Ok(None);
    }

    let item = unsafe { dialog.GetResult()? };
    Ok(Some(item_path(&item)?))
}

pub(crate) fn show_dialog_multiple(
    dialog: &IFileOpenDialog,
    owner: *mut core::ffi::c_void,
) -> Result<Vec<PathBuf>> {
    if !show_modal(dialog, owner)? {
        return Ok(Vec::new());
    }

    let items = unsafe { dialog.GetResults()? };
    let count = unsafe { items.GetCount()? };
    (0..count)
        .map(|index| {
            let item = unsafe { items.GetItemAt(index)? };
            item_path(&item)
        })
        .collect()
}

pub(crate) fn validate_owner(owner: *mut core::ffi::c_void) -> Result<()> {
    if owner.is_null() {
        return Err(Error::new(
            E_INVALIDARG,
            "picker owner HWND must not be null",
        ));
    }
    Ok(())
}

pub(crate) fn wide(value: &str) -> Result<Vec<u16>> {
    if value.contains('\0') {
        return Err(Error::new(
            E_INVALIDARG,
            "picker strings must not contain embedded nulls",
        ));
    }
    Ok(value.encode_utf16().chain([0]).collect())
}

fn shell_item(folder: &DialogFolder) -> Result<IShellItem> {
    unsafe {
        let mut result = core::ptr::null_mut();
        match folder {
            DialogFolder::Path(path) => {
                let path = path_wide(path)?;
                SHCreateItemFromParsingName(
                    PCWSTR(path.as_ptr()),
                    core::ptr::null_mut(),
                    &IShellItem::IID,
                    &mut result,
                )
                .ok()?;
            }
            DialogFolder::Known(location) => {
                SHCreateItemInKnownFolder(
                    &location.id(),
                    0,
                    PCWSTR::null(),
                    &IShellItem::IID,
                    &mut result,
                )
                .ok()?;
            }
        }
        imp::Type::from_abi(result)
    }
}

fn path_wide(value: &Path) -> Result<Vec<u16>> {
    let value = value.as_os_str().encode_wide().collect::<Vec<_>>();
    if value.contains(&0) {
        return Err(Error::new(
            E_INVALIDARG,
            "picker paths must not contain embedded nulls",
        ));
    }
    Ok(value.into_iter().chain([0]).collect())
}

fn is_cancelled(result: HRESULT) -> bool {
    result == ERROR_CANCELLED_HRESULT
}

fn merge_options(
    current: FILEOPENDIALOGOPTIONS,
    set: FILEOPENDIALOGOPTIONS,
    clear: FILEOPENDIALOGOPTIONS,
) -> FILEOPENDIALOGOPTIONS {
    (current | set) & !clear
}

fn show_modal(dialog: &IFileDialog, owner: *mut core::ffi::c_void) -> Result<bool> {
    let result = unsafe { dialog.Show(Some(owner)) };
    if is_cancelled(result) {
        Ok(false)
    } else {
        result.ok()?;
        Ok(true)
    }
}

fn item_path(item: &IShellItem) -> Result<PathBuf> {
    let path = unsafe { item.GetDisplayName(SIGDN_FILESYSPATH)? };
    Ok(copy_path(path))
}

struct CoTaskMemString(PWSTR);

impl Drop for CoTaskMemString {
    fn drop(&mut self) {
        unsafe { CoTaskMemFree(self.0.as_ptr().cast()) };
    }
}

fn copy_path(value: PWSTR) -> PathBuf {
    let value = CoTaskMemString(value);
    let path = unsafe { OsString::from_wide(value.0.as_wide()) };
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_error_cancelled_is_cancellation() {
        assert!(is_cancelled(ERROR_CANCELLED_HRESULT));
        assert!(!is_cancelled(HRESULT(0x8000_4005_u32 as i32)));
    }

    #[test]
    fn embedded_nuls_in_titles_are_rejected() {
        assert_eq!(wide("bad\0title").unwrap_err().code(), E_INVALIDARG);
    }

    #[test]
    fn option_changes_preserve_unrelated_flags() {
        assert_eq!(merge_options(0b1010, 0b0100, 0b0010), 0b1100);
    }

    #[test]
    fn embedded_nuls_in_paths_are_rejected() {
        assert_eq!(
            path_wide(Path::new("bad\0path")).unwrap_err().code(),
            E_INVALIDARG
        );
    }

    #[test]
    fn paths_preserve_non_unicode_utf16() {
        let value = [b'C' as u16, b':' as u16, b'\\' as u16, 0xd800, b'x' as u16];
        let path = PathBuf::from(OsString::from_wide(&value));
        let expected = value.into_iter().chain([0]).collect::<Vec<_>>();

        assert_eq!(path_wide(&path).unwrap(), expected);
    }
}
