// import!(
//     dependencies
//         os
//     modules
//         "windows.application_model.data_transfer"
// );

// use winrt::*;

// #[test]
// fn uri() -> Result<()> {
//     unsafe { CoInitializeEx(0, 2) };

//     use windows::application_model::data_transfer::*;

//     let content = DataPackage::new()?;
//     content.set_text("Rust/WinRT")?;

//     Clipboard::set_content(content)?;
//     Clipboard::flush()?;

//     Ok(())
// }

// #[link(name = "onecore")]
// extern "system" {
//     pub fn CoInitializeEx(reserved: usize, apartment: u32) -> ErrorCode;
// }
