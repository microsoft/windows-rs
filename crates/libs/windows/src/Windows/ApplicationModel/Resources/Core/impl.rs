#[cfg(feature = "implement_exclusive")]
pub trait INamedResourceImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Candidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
    fn Resolve(&self) -> ::windows::core::Result<ResourceCandidate>;
    fn ResolveForContext(&self, resourcecontext: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<ResourceCandidate>;
    fn ResolveAll(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
    fn ResolveAllForContext(&self, resourcecontext: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceCandidateImpl: Sized {
    fn Qualifiers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>>;
    fn IsMatch(&self) -> ::windows::core::Result<bool>;
    fn IsMatchAsDefault(&self) -> ::windows::core::Result<bool>;
    fn IsDefault(&self) -> ::windows::core::Result<bool>;
    fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetValueAsFileAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>;
    fn GetQualifierValue(&self, qualifiername: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceCandidate2Impl: Sized {
    fn GetValueAsStreamAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceCandidate3Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<ResourceCandidateKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextImpl: Sized {
    fn QualifierValues(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn ResetQualifierValues(&self, qualifiernames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn OverrideToMatch(&self, result: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ResourceContext>;
    fn Languages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetLanguages(&self, languages: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStaticsImpl: Sized {
    fn CreateMatchingContext(&self, result: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStatics2Impl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ResourceContext>;
    fn SetGlobalQualifierValue(&self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResetGlobalQualifierValues(&self) -> ::windows::core::Result<()>;
    fn ResetGlobalQualifierValuesForSpecifiedQualifiers(&self, qualifiernames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn GetForViewIndependentUse(&self) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStatics3Impl: Sized {
    fn SetGlobalQualifierValueWithPersistence(&self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING, persistence: ResourceQualifierPersistence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStatics4Impl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::super::super::UI::UIContext>) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceManagerImpl: Sized {
    fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap>;
    fn AllResourceMaps(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>>;
    fn DefaultContext(&self) -> ::windows::core::Result<ResourceContext>;
    fn LoadPriFiles(&self, files: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>) -> ::windows::core::Result<()>;
    fn UnloadPriFiles(&self, files: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceManager2Impl: Sized {
    fn GetAllNamedResourcesForPackage(&self, packagename: &::windows::core::HSTRING, resourcelayoutinfo: &ResourceLayoutInfo) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<NamedResource>>;
    fn GetAllSubtreesForPackage(&self, packagename: &::windows::core::HSTRING, resourcelayoutinfo: &ResourceLayoutInfo) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceMap>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceManagerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<ResourceManager>;
    fn IsResourceReference(&self, resourcereference: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResourceMapImpl: Sized + IIterableImpl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> + IMapViewImpl<::windows::core::HSTRING, NamedResource> {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn GetValue(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceCandidate>;
    fn GetValueForContext(&self, resource: &::windows::core::HSTRING, context: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<ResourceCandidate>;
    fn GetSubtree(&self, reference: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceMap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceQualifierImpl: Sized {
    fn QualifierName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QualifierValue(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsDefault(&self) -> ::windows::core::Result<bool>;
    fn IsMatch(&self) -> ::windows::core::Result<bool>;
    fn Score(&self) -> ::windows::core::Result<f64>;
}
