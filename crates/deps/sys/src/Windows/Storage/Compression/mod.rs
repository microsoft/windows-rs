#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CompressAlgorithm(i32);
pub struct Compressor(i32);
pub struct Decompressor(i32);
pub struct ICompressor(i32);
pub struct ICompressorFactory(i32);
pub struct IDecompressor(i32);
pub struct IDecompressorFactory(i32);
