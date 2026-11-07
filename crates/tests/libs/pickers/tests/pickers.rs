use std::path::{Path, PathBuf};
use test_pickers::{Action, show};
use windows_pickers::{FolderPicker, OpenFilePicker, SaveFilePicker};

fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("test_pickers-{name}"));
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn same_folder(left: &Path, right: &Path) -> bool {
    left.canonicalize().unwrap() == right.canonicalize().unwrap()
}

#[test]
fn save_from_canonical_folder() {
    let folder = scratch("save");
    let canonical = folder.canonicalize().unwrap();
    let (result, dialog) = show("test_pickers save", true, Action::Accept, move |owner| {
        SaveFilePicker::new()
            .title("test_pickers save")
            .commit_label("Export")
            .filter_extensions("Text", ["txt"])
            .default_extension("txt")
            .suggested_name("notes")
            .folder(canonical)
            .show_for_hwnd(owner)
    });

    assert!(dialog.owner_matches);
    assert_eq!(dialog.commit_label, "Export");
    let path = result.unwrap().unwrap();
    assert_eq!(path.file_name().unwrap(), "notes.txt");
    assert!(same_folder(path.parent().unwrap(), &folder));
}

#[test]
fn folder_from_relative_path() {
    let (result, dialog) = show("test_pickers folder", true, Action::Accept, |owner| {
        FolderPicker::new()
            .title("test_pickers folder")
            .folder("tests")
            .show_multiple_for_hwnd(owner)
    });

    assert!(dialog.owner_matches);
    let paths = result.unwrap();
    assert_eq!(paths.len(), 1);
    assert!(same_folder(
        &paths[0],
        &std::env::current_dir().unwrap().join("tests")
    ));
}

#[test]
fn unowned_with_missing_folder() {
    let missing = std::env::temp_dir().join("test_pickers-missing");
    let (result, dialog) = show(
        "test_pickers unowned",
        false,
        Action::Cancel,
        move |owner| {
            OpenFilePicker::new()
                .title("test_pickers unowned")
                .folder(missing)
                .show_for_hwnd(owner)
        },
    );

    assert!(dialog.owner_matches);
    assert_eq!(result.unwrap(), None);
}

#[test]
fn cancel_multiple() {
    let (result, dialog) = show("test_pickers cancel", true, Action::Cancel, |owner| {
        OpenFilePicker::new()
            .title("test_pickers cancel")
            .filter_all()
            .show_multiple_for_hwnd(owner)
    });

    assert!(dialog.owner_matches);
    assert!(result.unwrap().is_empty());
}
