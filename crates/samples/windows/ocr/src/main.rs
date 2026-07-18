fn main() -> windows::core::Result<()> {
    use windows::{
        Graphics::Imaging::BitmapDecoder,
        Media::Ocr::OcrEngine,
        Storage::{FileAccessMode, StorageFile},
        core::*};

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
