#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceCatalogStaticsImpl: Sized {
    fn FindAppServiceProvidersAsync(&self, appservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceClosedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AppServiceClosedStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceConnectionImpl: Sized {
    fn AppServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>;
    fn SendMessageAsync(&self, message: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponse>>;
    fn RequestReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ServiceClosed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceConnection2Impl: Sized {
    fn OpenRemoteAsync(&self, remotesystemconnectionrequest: &::core::option::Option<super::super::System::RemoteSystems::RemoteSystemConnectionRequest>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn SetUser(&self, value: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceConnectionStaticsImpl: Sized {
    fn SendStatelessMessageAsync(&self, connection: &::core::option::Option<AppServiceConnection>, connectionrequest: &::core::option::Option<super::super::System::RemoteSystems::RemoteSystemConnectionRequest>, message: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceRequestImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn SendResponseAsync(&self, message: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceRequestReceivedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<AppServiceRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<AppServiceDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceResponseImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn Status(&self) -> ::windows::core::Result<AppServiceResponseStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceTriggerDetailsImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppServiceConnection(&self) -> ::windows::core::Result<AppServiceConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceTriggerDetails2Impl: Sized {
    fn IsRemoteSystemConnection(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceTriggerDetails3Impl: Sized {
    fn CheckCallerForCapabilityAsync(&self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceTriggerDetails4Impl: Sized {
    fn CallerRemoteConnectionToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStatelessAppServiceResponseImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn Status(&self) -> ::windows::core::Result<StatelessAppServiceResponseStatus>;
}
