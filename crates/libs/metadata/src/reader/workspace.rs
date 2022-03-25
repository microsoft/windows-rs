use super::*;

// TODO: should just pass files directly to the reader. https://github.com/microsoft/windows-rs/issues/1406
pub fn workspace_winmds() -> &'static [File] {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<Vec<File>> = MaybeUninit::uninit();

    ONCE.call_once(|| unsafe { VALUE = MaybeUninit::new(get_workspace_winmds()) });

    unsafe { &*VALUE.as_ptr() }
}

fn get_workspace_winmds() -> Vec<File> {
    let mut result = vec![];

    if let Ok(files) = std::fs::read_dir(".windows/winmd") {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_file() {
                    let path = file.path();
                    if path.extension().and_then(|extension| extension.to_str()) == Some("winmd") {
                        result.push(File::new(path));
                    }
                }
            }
        }
    }

    if !result.iter().any(|file| file.name.starts_with("Windows.")) {
        result.push(File::from_bytes("Windows.winmd".to_string(), include_bytes!("../../default/Windows.winmd").to_vec()));
        result.push(File::from_bytes("Windows.Win32.winmd".to_string(), include_bytes!("../../default/Windows.Win32.winmd").to_vec()));
        result.push(File::from_bytes("Windows.Win32.Interop.winmd".to_string(), include_bytes!("../../default/Windows.Win32.Interop.winmd").to_vec()));
    }

    result
}
