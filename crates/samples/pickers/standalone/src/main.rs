use windows::Win32::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use windows_pickers::{FolderPicker, GUID, OpenFilePicker, PickerLocation, Result, SaveFilePicker};
use windows_window::Window;

struct ComApartment;

impl ComApartment {
    fn sta() -> Result<Self> {
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED as u32).ok()? };
        Ok(Self)
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

fn main() -> Result<()> {
    let _apartment = ComApartment::sta()?;
    let window = Window::new("Windows Pickers").create()?;

    match std::env::args().nth(1).as_deref() {
        None | Some("open") => open(&window),
        Some("open-multiple") => open_multiple(&window),
        Some("folder") => folder(&window),
        Some("folder-multiple") => folder_multiple(&window),
        Some("save") => save(&window),
        Some(_) => {
            println!("usage: pickers-standalone [open|open-multiple|folder|folder-multiple|save]");
            Ok(())
        }
    }
}

fn open(window: &Window) -> Result<()> {
    let path = OpenFilePicker::new()
        .title("Open a Rust source file")
        .filter_extensions("Rust source", ["rs"])
        .filter_all()
        .initial_filter(0)
        .default_location(PickerLocation::Documents)
        .client_id(GUID::from_u128(0x6d81d46c_77da_4874_8ca7_754546ba6803))
        .show(window)?;

    if let Some(path) = path {
        println!("{}", path.display());
    }
    Ok(())
}

fn open_multiple(window: &Window) -> Result<()> {
    for path in OpenFilePicker::new()
        .title("Open files")
        .filter_all()
        .show_multiple(window)?
    {
        println!("{}", path.display());
    }
    Ok(())
}

fn folder(window: &Window) -> Result<()> {
    if let Some(path) = FolderPicker::new()
        .title("Choose a folder")
        .commit_label("Choose")
        .default_location(PickerLocation::Documents)
        .show(window)?
    {
        println!("{}", path.display());
    }
    Ok(())
}

fn folder_multiple(window: &Window) -> Result<()> {
    for path in FolderPicker::new()
        .title("Choose folders")
        .show_multiple(window)?
    {
        println!("{}", path.display());
    }
    Ok(())
}

fn save(window: &Window) -> Result<()> {
    if let Some(path) = SaveFilePicker::new()
        .title("Save report")
        .filter_extensions("Text", ["txt"])
        .suggested_name("report")
        .default_extension("txt")
        .overwrite_prompt(true)
        .show(window)?
    {
        println!("{}", path.display());
    }
    Ok(())
}
