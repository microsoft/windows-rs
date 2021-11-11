#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CloseCompressor();
    fn CloseDecompressor();
    fn Compress();
    fn CreateCompressor();
    fn CreateDecompressor();
    fn Decompress();
    fn QueryCompressorInformation();
    fn QueryDecompressorInformation();
    fn ResetCompressor();
    fn ResetDecompressor();
    fn SetCompressorInformation();
    fn SetDecompressorInformation();
}
