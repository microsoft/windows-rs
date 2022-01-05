#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICompressorImpl: Sized + IClosableImpl + IOutputStreamImpl {
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IOutputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompressorFactoryImpl: Sized {
    fn CreateCompressor(&self, underlyingstream: &::core::option::Option<super::Streams::IOutputStream>) -> ::windows::core::Result<Compressor>;
    fn CreateCompressorEx(&self, underlyingstream: &::core::option::Option<super::Streams::IOutputStream>, algorithm: CompressAlgorithm, blocksize: u32) -> ::windows::core::Result<Compressor>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDecompressorImpl: Sized + IClosableImpl + IInputStreamImpl {
    fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IInputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDecompressorFactoryImpl: Sized {
    fn CreateDecompressor(&self, underlyingstream: &::core::option::Option<super::Streams::IInputStream>) -> ::windows::core::Result<Decompressor>;
}
