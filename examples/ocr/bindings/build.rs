fn main() {
    windows::build!(
        windows::graphics::imaging::{SoftwareBitmap, BitmapDecoder},
        windows::media::ocr::{OcrEngine, OcrResult},
        windows::storage::{FileAccessMode, StorageFile},
        windows::foundation::IAsyncOperation,
        windows::storage::streams::IRandomAccessStream,
    );
}
