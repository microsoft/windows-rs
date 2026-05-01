#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
mod imp {
    use windows::{
        core::*,
        Graphics::Imaging::BitmapDecoder,
        Media::Ocr::OcrEngine,
        Storage::{FileAccessMode, StorageFile},
    };

    pub fn main() -> Result<()> {
        let mut message = std::env::current_dir().unwrap();
        message.push("message.png");

        let file =
            StorageFile::GetFileFromPathAsync(&HSTRING::from(message.to_str().unwrap()))?.join()?;
        let stream = file.OpenAsync(FileAccessMode::Read)?.join()?;

        let decode = BitmapDecoder::CreateAsync(&stream)?.join()?;
        let bitmap = decode.GetSoftwareBitmapAsync()?.join()?;

        let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
        let result = engine.RecognizeAsync(&bitmap)?.join()?;

        println!("{:?}", result.Text()?);
        Ok(())
    }
}

#[cfg(windows)]
fn main() -> impl std::process::Termination {
    imp::main()
}
