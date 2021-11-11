#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn COMPRESSOR_HANDLE();
    fn COMPRESS_ALGORITHM();
    fn COMPRESS_ALGORITHM_INVALID();
    fn COMPRESS_ALGORITHM_MAX();
    fn COMPRESS_ALGORITHM_NULL();
    fn COMPRESS_ALLOCATION_ROUTINES();
    fn COMPRESS_INFORMATION_CLASS();
    fn COMPRESS_RAW();
    fn CloseCompressor();
    fn CloseDecompressor();
    fn Compress();
    fn CreateCompressor();
    fn CreateDecompressor();
    fn Decompress();
    fn PFN_COMPRESS_ALLOCATE();
    fn PFN_COMPRESS_FREE();
    fn QueryCompressorInformation();
    fn QueryDecompressorInformation();
    fn ResetCompressor();
    fn ResetDecompressor();
    fn SetCompressorInformation();
    fn SetDecompressorInformation();
}
