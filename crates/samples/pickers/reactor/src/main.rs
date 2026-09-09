#![windows_subsystem = "windows"]

use std::path::PathBuf;
use windows_pickers::{FileFilter, FolderPicker, OpenFilePicker, Result, SaveFilePicker};
use windows_reactor::*;

enum Message {
    OpenFile,
    OpenFiles,
    PickFolder,
    PickFolders,
    SaveFile,
    Picked(Result<Option<PathBuf>>),
    PickedMultiple(Result<Vec<PathBuf>>),
}

struct PickerSample {
    status: String,
}

impl Component for PickerSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            status: "No file selected".to_string(),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::OpenFile => {
                self.status = "Choose a file...".to_string();
                let accepted = OpenFilePicker::new()
                    .title("Open a Rust source file")
                    .filter(FileFilter::extensions("Rust source", ["rs"]))
                    .filter(FileFilter::all())
                    .request(context, Message::Picked);
                if !accepted {
                    self.status = "Picker request unavailable".to_string();
                }
            }
            Message::OpenFiles => {
                self.status = "Choose files...".to_string();
                let accepted = OpenFilePicker::new()
                    .title("Open files")
                    .filter(FileFilter::all())
                    .request_multiple(context, Message::PickedMultiple);
                if !accepted {
                    self.status = "Picker request unavailable".to_string();
                }
            }
            Message::PickFolder => {
                self.status = "Choose a folder...".to_string();
                let accepted = FolderPicker::new()
                    .title("Choose a folder")
                    .request(context, Message::Picked);
                if !accepted {
                    self.status = "Picker request unavailable".to_string();
                }
            }
            Message::PickFolders => {
                self.status = "Choose folders...".to_string();
                let accepted = FolderPicker::new()
                    .title("Choose folders")
                    .request_multiple(context, Message::PickedMultiple);
                if !accepted {
                    self.status = "Picker request unavailable".to_string();
                }
            }
            Message::SaveFile => {
                self.status = "Choose where to save...".to_string();
                let accepted = SaveFilePicker::new()
                    .title("Save report")
                    .filter(FileFilter::extensions("Text", ["txt"]))
                    .suggested_name("report")
                    .default_extension("txt")
                    .request(context, Message::Picked);
                if !accepted {
                    self.status = "Picker request unavailable".to_string();
                }
            }
            Message::Picked(Ok(Some(path))) => {
                self.status = path.display().to_string();
            }
            Message::Picked(Ok(None)) => {
                self.status = "Selection cancelled".to_string();
            }
            Message::Picked(Err(error)) => {
                self.status = format!("Picker failed: {error}");
            }
            Message::PickedMultiple(Ok(paths)) if paths.is_empty() => {
                self.status = "Selection cancelled".to_string();
            }
            Message::PickedMultiple(Ok(paths)) => {
                self.status = paths
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
            }
            Message::PickedMultiple(Err(error)) => {
                self.status = format!("Picker failed: {error}");
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Windows Pickers");

        StackPanel::new().spacing(8.0).children((
            Button::new()
                .on_click(context.callback(|()| Message::OpenFile))
                .content("Open file"),
            Button::new()
                .on_click(context.callback(|()| Message::OpenFiles))
                .content("Open files"),
            Button::new()
                .on_click(context.callback(|()| Message::PickFolder))
                .content("Choose folder"),
            Button::new()
                .on_click(context.callback(|()| Message::PickFolders))
                .content("Choose folders"),
            Button::new()
                .on_click(context.callback(|()| Message::SaveFile))
                .content("Save file"),
            self.status.clone(),
        ))
    }
}

fn main() {
    App::run_component::<PickerSample>(()).unwrap();
}
