#[cfg(feature = "implement_exclusive")]
pub trait IIndexedResourceCandidateImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<IndexedResourceType>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Metadata(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Qualifiers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>>;
    fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetQualifierValue(&self, qualifiername: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIndexedResourceQualifierImpl: Sized {
    fn QualifierName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QualifierValue(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexerImpl: Sized {
    fn IndexFilePath(&self, filepath: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<IndexedResourceCandidate>;
    fn IndexFileContentsAsync(&self, file: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexerFactoryImpl: Sized {
    fn CreateResourceIndexer(&self, projectroot: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<ResourceIndexer>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexerFactory2Impl: Sized {
    fn CreateResourceIndexerWithExtension(&self, projectroot: &::core::option::Option<super::super::super::Foundation::Uri>, extensiondllpath: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<ResourceIndexer>;
}
