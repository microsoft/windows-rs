#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IAtomPubClientImpl: Sized + ISyndicationClientImpl {
    fn RetrieveServiceDocumentAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ServiceDocument, super::Syndication::RetrievalProgress>>;
    fn RetrieveMediaResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IInputStream, super::Syndication::RetrievalProgress>>;
    fn RetrieveResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::RetrievalProgress>>;
    fn CreateResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, description: &::windows::core::HSTRING, item: &::core::option::Option<super::Syndication::SyndicationItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>;
    fn CreateMediaResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, mediatype: &::windows::core::HSTRING, description: &::windows::core::HSTRING, mediastream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Syndication::SyndicationItem, super::Syndication::TransferProgress>>;
    fn UpdateMediaResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, mediatype: &::windows::core::HSTRING, mediastream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn UpdateResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, item: &::core::option::Option<super::Syndication::SyndicationItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn UpdateResourceItemAsync(&self, item: &::core::option::Option<super::Syndication::SyndicationItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn DeleteResourceAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn DeleteResourceItemAsync(&self, item: &::core::option::Option<super::Syndication::SyndicationItem>) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<super::Syndication::TransferProgress>>;
    fn CancelAsyncOperations(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAtomPubClientFactoryImpl: Sized {
    fn CreateAtomPubClientWithCredentials(&self, servercredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<AtomPubClient>;
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IResourceCollectionImpl: Sized + ISyndicationNodeImpl {
    fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Syndication::SyndicationCategory>>;
    fn Accepts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IServiceDocumentImpl: Sized + ISyndicationNodeImpl {
    fn Workspaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Workspace>>;
}
#[cfg(all(feature = "Web_Syndication", feature = "implement_exclusive"))]
pub trait IWorkspaceImpl: Sized + ISyndicationNodeImpl {
    fn Title(&self) -> ::windows::core::Result<super::Syndication::ISyndicationText>;
    fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ResourceCollection>>;
}
