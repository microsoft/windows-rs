use windows::{
    core::*,
    Graphics::Imaging::BitmapDecoder,
    Media::Ocr::OcrEngine,
    Storage::{FileAccessMode, StorageFile},
};

fn main() -> Result<()> {
    let mut message = std::env::current_dir().unwrap();
    message.push("message.png");

    let file =
        StorageFile::GetFileFromPathAsync(&HSTRING::from(message.to_str().unwrap()))?.get()?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.get()?;

    let decode = BitmapDecoder::CreateAsync(&stream)?.get()?;
    let bitmap = decode.GetSoftwareBitmapAsync()?.get()?;

    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = engine.RecognizeAsync(&bitmap)?.get()?;

    println!("{}", result.Text()?);
    Ok(())
}
