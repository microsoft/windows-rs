#[cfg(feature = "implement_exclusive")]
pub trait IComponentLoadFailedEventArgsImpl: Sized {
    fn Information(&self) -> ::windows::core::Result<RevocationAndRenewalInformation>;
    fn Completion(&self) -> ::windows::core::Result<MediaProtectionServiceCompletion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IComponentRenewalStaticsImpl: Sized {
    fn RenewSystemComponentsAsync(&self, information: &::core::option::Option<RevocationAndRenewalInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<RenewalStatus, u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHdcpSessionImpl: Sized + IClosableImpl {
    fn IsEffectiveProtectionAtLeast(&self, protection: HdcpProtection) -> ::windows::core::Result<bool>;
    fn GetEffectiveProtection(&self) -> ::windows::core::Result<super::super::Foundation::IReference<HdcpProtection>>;
    fn SetDesiredMinProtectionAsync(&self, protection: HdcpProtection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HdcpSetProtectionResult>>;
    fn ProtectionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HdcpSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionManagerImpl: Sized {
    fn ServiceRequested(&self, handler: &::core::option::Option<ServiceRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RebootNeeded(&self, handler: &::core::option::Option<RebootNeededEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRebootNeeded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ComponentLoadFailed(&self, handler: &::core::option::Option<ComponentLoadFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveComponentLoadFailed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionPMPServerImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionPMPServerFactoryImpl: Sized {
    fn CreatePMPServer(&self, pproperties: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<MediaProtectionPMPServer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionServiceCompletionImpl: Sized {
    fn Complete(&self, success: bool) -> ::windows::core::Result<()>;
}
pub trait IMediaProtectionServiceRequestImpl: Sized {
    fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionCapabilitiesImpl: Sized {
    fn IsTypeSupported(&self, r#type: &::windows::core::HSTRING, keysystem: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionCapabilityResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevocationAndRenewalInformationImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<RevocationAndRenewalItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevocationAndRenewalItemImpl: Sized {
    fn Reasons(&self) -> ::windows::core::Result<RevocationAndRenewalReasons>;
    fn HeaderHash(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicKeyHash(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RenewalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IServiceRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<IMediaProtectionServiceRequest>;
    fn Completion(&self) -> ::windows::core::Result<MediaProtectionServiceCompletion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IServiceRequestedEventArgs2Impl: Sized {
    fn MediaPlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
}
