#[cfg(feature = "implement_exclusive")]
pub trait IDataProtectionProviderImpl: Sized {
    fn ProtectAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
    fn UnprotectAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
    fn ProtectStreamAsync(&self, src: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, dest: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn UnprotectStreamAsync(&self, src: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, dest: &::core::option::Option<super::super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProtectionProviderFactoryImpl: Sized {
    fn CreateOverloadExplicit(&self, protectiondescriptor: &::windows::core::HSTRING) -> ::windows::core::Result<DataProtectionProvider>;
}
