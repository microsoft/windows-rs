#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CompressAlgorithm();
    fn Compressor();
    fn Decompressor();
    fn ICompressor();
    fn ICompressorFactory();
    fn IDecompressor();
    fn IDecompressorFactory();
}
