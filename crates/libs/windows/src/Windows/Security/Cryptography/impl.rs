#[cfg(feature = "implement_exclusive")]
pub trait ICryptographicBufferStaticsImpl: Sized {
    fn Compare(&self, object1: &::core::option::Option<super::super::Storage::Streams::IBuffer>, object2: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<bool>;
    fn GenerateRandom(&self, length: u32) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn GenerateRandomNumber(&self) -> ::windows::core::Result<u32>;
    fn CreateFromByteArray(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CopyToByteArray(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn DecodeFromHexString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn EncodeToHexString(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DecodeFromBase64String(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn EncodeToBase64String(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConvertStringToBinary(&self, value: &::windows::core::HSTRING, encoding: BinaryStringEncoding) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ConvertBinaryToString(&self, encoding: BinaryStringEncoding, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
