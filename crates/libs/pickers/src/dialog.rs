use super::*;

const ERROR_CANCELLED: u32 = 1223;
const ERROR_CANCELLED_HRESULT: HRESULT = HRESULT((0x8007_0000_u32 | ERROR_CANCELLED) as i32);
const E_INVALIDARG: HRESULT = HRESULT(0x8007_0057_u32 as i32);
const ERROR_FILE_NOT_FOUND_HRESULT: HRESULT = HRESULT(0x8007_0002_u32 as i32);
const ERROR_PATH_NOT_FOUND_HRESULT: HRESULT = HRESULT(0x8007_0003_u32 as i32);

pub struct PreparedDialog {
    _title: Option<Vec<u16>>,
    _commit_label: Option<Vec<u16>>,
}

pub struct DialogConfig<'a> {
    pub title: Option<&'a str>,
    pub commit_label: Option<&'a str>,
    pub set_options: FILEOPENDIALOGOPTIONS,
    pub clear_options: FILEOPENDIALOGOPTIONS,
    pub settings: &'a DialogSettings,
}

#[derive(Clone, Debug, Default)]
pub struct DialogSettings {
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
    pub fn set_client_id(&mut self, value: GUID) {
        self.client_id = Some(value);
    }

    pub fn set_default(&mut self, value: impl Into<PathBuf>) {
        self.default = Some(DialogFolder::Path(value.into()));
    }

    pub fn set_folder(&mut self, value: impl Into<PathBuf>) {
        self.folder = Some(DialogFolder::Path(value.into()));
    }

    pub fn set_default_location(&mut self, value: PickerLocation) {
        self.default = Some(DialogFolder::Known(value));
    }

    pub fn set_location(&mut self, value: PickerLocation) {
        self.folder = Some(DialogFolder::Known(value));
    }

    fn apply_client_id(&self, dialog: &IFileDialog) -> Result<()> {
        if let Some(client_id) = &self.client_id {
            unsafe { dialog.SetClientGuid(client_id).ok()? };
        }
        Ok(())
    }

    fn apply_folders(&self, dialog: &IFileDialog) -> Result<()> {
        if let Some(item) = self.default.as_ref().map(shell_item).transpose()?.flatten() {
            unsafe { dialog.SetDefaultFolder(&item).ok()? };
        }
        if let Some(item) = self.folder.as_ref().map(shell_item).transpose()?.flatten() {
            unsafe { dialog.SetFolder(&item).ok()? };
        }
        Ok(())
    }
}

impl PreparedDialog {
    pub fn new(dialog: &IFileDialog, config: DialogConfig<'_>) -> Result<Self> {
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

        let mut options = Default::default();
        unsafe { dialog.GetOptions(&mut options).ok()? };
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

pub fn show_dialog(dialog: &IFileDialog, owner: *mut core::ffi::c_void) -> Result<Option<PathBuf>> {
    if !show_modal(dialog, owner)? {
        return Ok(None);
    }

    let item = unsafe { dialog.GetResult()? };
    Ok(Some(item_path(&item)?))
}

pub fn show_dialog_multiple(
    dialog: &IFileOpenDialog,
    owner: *mut core::ffi::c_void,
) -> Result<Vec<PathBuf>> {
    if !show_modal(dialog, owner)? {
        return Ok(Vec::new());
    }

    let items = unsafe { dialog.GetResults()? };
    let mut count = 0;
    unsafe { items.GetCount(&mut count).ok()? };
    (0..count)
        .map(|index| {
            let item = unsafe { items.GetItemAt(index)? };
            item_path(&item)
        })
        .collect()
}

pub fn wide(value: &str) -> Result<Vec<u16>> {
    if value.contains('\0') {
        return Err(Error::new(
            E_INVALIDARG,
            "picker strings must not contain embedded nulls",
        ));
    }
    Ok(value.encode_utf16().chain([0]).collect())
}

// Returns `None` if the folder does not exist, so the dialog falls back to its usual location.
fn shell_item(folder: &DialogFolder) -> Result<Option<IShellItem>> {
    let mut result = core::ptr::null_mut();
    let hr = match folder {
        DialogFolder::Path(path) => {
            let path = path_wide(path)?;
            unsafe {
                SHCreateItemFromParsingName(
                    PCWSTR(path.as_ptr()),
                    core::ptr::null_mut(),
                    &IShellItem::IID,
                    &mut result,
                )
            }
        }
        DialogFolder::Known(location) => unsafe {
            SHCreateItemInKnownFolder(
                &location.id(),
                0,
                PCWSTR::null(),
                &IShellItem::IID,
                &mut result,
            )
        },
    };
    if hr == ERROR_FILE_NOT_FOUND_HRESULT || hr == ERROR_PATH_NOT_FOUND_HRESULT {
        return Ok(None);
    }
    hr.ok()?;
    unsafe { imp::Type::from_abi(result).map(Some) }
}

// The Shell parses only absolute, non-verbatim paths, so resolve relative paths and remove the
// `\\?\` prefix that `Path::canonicalize` adds.
fn path_wide(value: &Path) -> Result<Vec<u16>> {
    if value.as_os_str().encode_wide().any(|c| c == 0) {
        return Err(Error::new(
            E_INVALIDARG,
            "picker paths must not contain embedded nulls",
        ));
    }
    let value = std::path::absolute(value).map_err(|error| match error.raw_os_error() {
        Some(code) => Error::from(WIN32_ERROR(code as u32)),
        None => Error::new(E_INVALIDARG, "picker paths must not be empty"),
    })?;
    let wide = value.as_os_str().encode_wide();
    let wide: Vec<u16> = match value.components().next() {
        Some(std::path::Component::Prefix(prefix)) => match prefix.kind() {
            Prefix::VerbatimDisk(_) => wide.skip(4).collect(),
            Prefix::VerbatimUNC(..) => [b'\\' as u16; 2].into_iter().chain(wide.skip(8)).collect(),
            _ => wide.collect(),
        },
        _ => wide.collect(),
    };
    Ok(wide.into_iter().chain([0]).collect())
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
    let result = unsafe { dialog.Show(Some(owner.cast())) };
    if is_cancelled(result) {
        Ok(false)
    } else {
        result.ok()?;
        Ok(true)
    }
}

fn item_path(item: &IShellItem) -> Result<PathBuf> {
    let mut path = PWSTR::null();
    unsafe { item.GetDisplayName(SIGDN_FILESYSPATH, &mut path).ok()? };
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
    fn empty_paths_and_embedded_nuls_are_rejected() {
        assert_eq!(
            path_wide(Path::new("bad\0path")).unwrap_err().code(),
            E_INVALIDARG
        );
        assert_eq!(path_wide(Path::new("")).unwrap_err().code(), E_INVALIDARG);
    }

    #[test]
    fn paths_preserve_non_unicode_utf16() {
        let value = [b'C' as u16, b':' as u16, b'\\' as u16, 0xd800, b'x' as u16];
        let path = PathBuf::from(OsString::from_wide(&value));
        let expected = value.into_iter().chain([0]).collect::<Vec<_>>();

        assert_eq!(path_wide(&path).unwrap(), expected);
    }

    fn shell_path(value: &str) -> String {
        let wide = path_wide(Path::new(value)).unwrap();
        String::from_utf16(&wide[..wide.len() - 1]).unwrap()
    }

    #[test]
    fn paths_are_absolute_and_not_verbatim() {
        let current = std::env::current_dir().unwrap();
        assert_eq!(shell_path("src"), current.join("src").to_str().unwrap());
        assert_eq!(shell_path(r"\\?\C:\Windows"), r"C:\Windows");
        assert_eq!(shell_path(r"\\?\UNC\server\share\a"), r"\\server\share\a");
        assert_eq!(shell_path("C:/Windows/System32"), r"C:\Windows\System32");
    }

    #[test]
    fn missing_folders_are_ignored() {
        let _sta = init_sta();
        let missing = std::env::temp_dir().join("windows-pickers-missing-folder");
        assert!(shell_item(&DialogFolder::Path(missing)).unwrap().is_none());
        let canonical = std::env::temp_dir().canonicalize().unwrap();
        assert!(
            shell_item(&DialogFolder::Path(canonical))
                .unwrap()
                .is_some()
        );
    }
}
