fn main() {
    windows::build! {
        Windows::Foundation::IAsyncOperation,
        Windows::Graphics::Imaging::{BitmapDecoder, SoftwareBitmap},
        Windows::Media::Ocr::{OcrEngine, OcrResult},
        Windows::Storage::Streams::IRandomAccessStream,
        Windows::Storage::{FileAccessMode, StorageFile},
    };
}
