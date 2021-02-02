fn main() {
    windows::build!(
        windows::graphics::imaging::BitmapDecoder,
        windows::media::ocr::OcrEngine,
        windows::storage::StorageFile,
    );
}
