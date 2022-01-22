use windows::{
    core::*,
    Graphics::Imaging::BitmapDecoder,
    Media::Ocr::OcrEngine,
    Storage::{FileAccessMode, StorageFile},
};

fn main() -> Result<()> {
    futures::executor::block_on(main_async())
}

async fn main_async() -> Result<()> {
    let mut message = std::env::current_dir().unwrap();
    message.push("message.png");

    let file = StorageFile::GetFileFromPathAsync(message.to_str().unwrap())?.await?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.await?;

    let decode = BitmapDecoder::CreateAsync(stream)?.await?;
    let bitmap = decode.GetSoftwareBitmapAsync()?.await?;

    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = engine.RecognizeAsync(bitmap)?.await?;

    println!("{}", result.Text()?);
    Ok(())
}
