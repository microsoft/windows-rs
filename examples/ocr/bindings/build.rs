fn main() {
    windows::build!(
        Windows::Graphics::Imaging::{SoftwareBitmap, BitmapDecoder},
        Windows::Media::Ocr::{OcrEngine, OcrResult},
        Windows::Storage::{FileAccessMode, StorageFile},
        Windows::Foundation::IAsyncOperation,
        Windows::Storage::Streams::IRandomAccessStream,
    );
}
