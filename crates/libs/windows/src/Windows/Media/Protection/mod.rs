#[cfg(feature = "Media_Protection_PlayReady")]
#[doc = "Required features: `\"Media_Protection_PlayReady\"`"]
pub mod PlayReady;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IComponentLoadFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IComponentLoadFailedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95972e93_7746_417e_8495_f031bbc5862c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentLoadFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Completion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IComponentRenewalStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComponentRenewalStatics {
    type Vtable = IComponentRenewalStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IComponentRenewalStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ffbcd67_b795_48c5_8b7b_a7c4efe202e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentRenewalStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RenewSystemComponentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, information: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenewSystemComponentsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHdcpSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHdcpSession {
    type Vtable = IHdcpSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHdcpSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x718845e9_64d7_426d_809b_1be461941a2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdcpSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEffectiveProtectionAtLeast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetEffectiveProtection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetEffectiveProtection: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredMinProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredMinProtectionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaProtectionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProtectionManager {
    type Vtable = IMediaProtectionManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaProtectionManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45694947_c741_434b_a79e_474c12d93d2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ServiceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RebootNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RebootNeeded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRebootNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRebootNeeded: usize,
    #[cfg(feature = "Foundation")]
    pub ComponentLoadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComponentLoadFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveComponentLoadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveComponentLoadFailed: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaProtectionPMPServer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaProtectionPMPServer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c111226_7b26_4d31_95bb_9c1b08ef7fc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaProtectionPMPServerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProtectionPMPServerFactory {
    type Vtable = IMediaProtectionPMPServerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaProtectionPMPServerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x602c8e5e_f7d2_487e_af91_dbc4252b2182);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePMPServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePMPServer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaProtectionServiceCompletion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaProtectionServiceCompletion {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b5cca18_cfd5_44ee_a2ed_df76010c14b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionServiceCompletion_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, success: bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaProtectionServiceRequest(::windows_core::IUnknown);
impl IMediaProtectionServiceRequest {
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaProtectionServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IMediaProtectionServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{b1de0ea6-2094-478d-87a4-8b95200f85c6}");
}
unsafe impl ::windows_core::Interface for IMediaProtectionServiceRequest {
    type Vtable = IMediaProtectionServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaProtectionServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1de0ea6_2094_478d_87a4_8b95200f85c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProtectionSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionCapabilities {
    type Vtable = IProtectionCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7ac5d7e_7480_4d29_a464_7bcd913dd8e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsTypeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::std::mem::MaybeUninit<::windows_core::HSTRING>, keysystem: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ProtectionCapabilityResult) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRevocationAndRenewalInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRevocationAndRenewalInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3a1937b_2501_439e_a6e7_6fc95e175fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevocationAndRenewalInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRevocationAndRenewalItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRevocationAndRenewalItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3099c20c_3cf0_49ea_902d_caf32d2dde2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevocationAndRenewalItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reasons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RevocationAndRenewalReasons) -> ::windows_core::HRESULT,
    pub HeaderHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PublicKeyHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RenewalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IServiceRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IServiceRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34283baf_abb4_4fc1_bd89_93f106573a49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Completion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IServiceRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IServiceRequestedEventArgs2 {
    type Vtable = IServiceRequestedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IServiceRequestedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x553c69d6_fafe_4128_8dfa_130e398a13a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Playback")]
    pub MediaPlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    MediaPlaybackItem: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ComponentLoadFailedEventArgs(::windows_core::IUnknown);
impl ComponentLoadFailedEventArgs {
    pub fn Information(&self) -> ::windows_core::Result<RevocationAndRenewalInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Information)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Completion(&self) -> ::windows_core::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ComponentLoadFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ComponentLoadFailedEventArgs;{95972e93-7746-417e-8495-f031bbc5862c})");
}
unsafe impl ::windows_core::Interface for ComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ComponentLoadFailedEventArgs {
    const IID: ::windows_core::GUID = <IComponentLoadFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ComponentLoadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ComponentLoadFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ComponentLoadFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ComponentLoadFailedEventArgs {}
unsafe impl ::core::marker::Sync for ComponentLoadFailedEventArgs {}
pub struct ComponentRenewal;
impl ComponentRenewal {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RenewSystemComponentsAsync<P0>(information: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<RenewalStatus, u32>>
    where
        P0: ::windows_core::IntoParam<RevocationAndRenewalInformation>,
    {
        Self::IComponentRenewalStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenewSystemComponentsAsync)(::windows_core::Interface::as_raw(this), information.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IComponentRenewalStatics<R, F: FnOnce(&IComponentRenewalStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ComponentRenewal, IComponentRenewalStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ComponentRenewal {
    const NAME: &'static str = "Windows.Media.Protection.ComponentRenewal";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HdcpSession(::windows_core::IUnknown);
impl HdcpSession {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<HdcpSession, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsEffectiveProtectionAtLeast(&self, protection: HdcpProtection) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEffectiveProtectionAtLeast)(::windows_core::Interface::as_raw(this), protection, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetEffectiveProtection(&self) -> ::windows_core::Result<super::super::Foundation::IReference<HdcpProtection>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEffectiveProtection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredMinProtectionAsync(&self, protection: HdcpProtection) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HdcpSetProtectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetDesiredMinProtectionAsync)(::windows_core::Interface::as_raw(this), protection, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ProtectionChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<HdcpSession, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProtectionChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for HdcpSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.HdcpSession;{718845e9-64d7-426d-809b-1be461941a2a})");
}
unsafe impl ::windows_core::Interface for HdcpSession {
    type Vtable = IHdcpSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HdcpSession {
    const IID: ::windows_core::GUID = <IHdcpSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HdcpSession {
    const NAME: &'static str = "Windows.Media.Protection.HdcpSession";
}
::windows_core::imp::interface_hierarchy!(HdcpSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for HdcpSession {}
unsafe impl ::core::marker::Send for HdcpSession {}
unsafe impl ::core::marker::Sync for HdcpSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaProtectionManager(::windows_core::IUnknown);
impl MediaProtectionManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaProtectionManager, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<ServiceRequestedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveServiceRequested)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RebootNeeded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<RebootNeededEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RebootNeeded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRebootNeeded(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRebootNeeded)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ComponentLoadFailed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<ComponentLoadFailedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ComponentLoadFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveComponentLoadFailed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveComponentLoadFailed)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MediaProtectionManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionManager;{45694947-c741-434b-a79e-474c12d93d2f})");
}
unsafe impl ::windows_core::Interface for MediaProtectionManager {
    type Vtable = IMediaProtectionManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaProtectionManager {
    const IID: ::windows_core::GUID = <IMediaProtectionManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaProtectionManager {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionManager";
}
::windows_core::imp::interface_hierarchy!(MediaProtectionManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaProtectionManager {}
unsafe impl ::core::marker::Sync for MediaProtectionManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaProtectionPMPServer(::windows_core::IUnknown);
impl MediaProtectionPMPServer {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePMPServer<P0>(pproperties: P0) -> ::windows_core::Result<MediaProtectionPMPServer>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IMediaProtectionPMPServerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePMPServer)(::windows_core::Interface::as_raw(this), pproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaProtectionPMPServerFactory<R, F: FnOnce(&IMediaProtectionPMPServerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaProtectionPMPServer, IMediaProtectionPMPServerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MediaProtectionPMPServer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionPMPServer;{0c111226-7b26-4d31-95bb-9c1b08ef7fc0})");
}
unsafe impl ::windows_core::Interface for MediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaProtectionPMPServer {
    const IID: ::windows_core::GUID = <IMediaProtectionPMPServer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaProtectionPMPServer {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionPMPServer";
}
::windows_core::imp::interface_hierarchy!(MediaProtectionPMPServer, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaProtectionPMPServer {}
unsafe impl ::core::marker::Sync for MediaProtectionPMPServer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaProtectionServiceCompletion(::windows_core::IUnknown);
impl MediaProtectionServiceCompletion {
    pub fn Complete(&self, success: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this), success).ok() }
    }
}
impl ::windows_core::RuntimeType for MediaProtectionServiceCompletion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionServiceCompletion;{8b5cca18-cfd5-44ee-a2ed-df76010c14b5})");
}
unsafe impl ::windows_core::Interface for MediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaProtectionServiceCompletion {
    const IID: ::windows_core::GUID = <IMediaProtectionServiceCompletion as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaProtectionServiceCompletion {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionServiceCompletion";
}
::windows_core::imp::interface_hierarchy!(MediaProtectionServiceCompletion, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaProtectionServiceCompletion {}
unsafe impl ::core::marker::Sync for MediaProtectionServiceCompletion {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectionCapabilities(::windows_core::IUnknown);
impl ProtectionCapabilities {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProtectionCapabilities, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsTypeSupported(&self, r#type: &::windows_core::HSTRING, keysystem: &::windows_core::HSTRING) -> ::windows_core::Result<ProtectionCapabilityResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTypeSupported)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(r#type), ::core::mem::transmute_copy(keysystem), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtectionCapabilities {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ProtectionCapabilities;{c7ac5d7e-7480-4d29-a464-7bcd913dd8e4})");
}
unsafe impl ::windows_core::Interface for ProtectionCapabilities {
    type Vtable = IProtectionCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectionCapabilities {
    const IID: ::windows_core::GUID = <IProtectionCapabilities as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectionCapabilities {
    const NAME: &'static str = "Windows.Media.Protection.ProtectionCapabilities";
}
::windows_core::imp::interface_hierarchy!(ProtectionCapabilities, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectionCapabilities {}
unsafe impl ::core::marker::Sync for ProtectionCapabilities {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RevocationAndRenewalInformation(::windows_core::IUnknown);
impl RevocationAndRenewalInformation {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<RevocationAndRenewalItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for RevocationAndRenewalInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.RevocationAndRenewalInformation;{f3a1937b-2501-439e-a6e7-6fc95e175fcf})");
}
unsafe impl ::windows_core::Interface for RevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RevocationAndRenewalInformation {
    const IID: ::windows_core::GUID = <IRevocationAndRenewalInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RevocationAndRenewalInformation {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalInformation";
}
::windows_core::imp::interface_hierarchy!(RevocationAndRenewalInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RevocationAndRenewalInformation {}
unsafe impl ::core::marker::Sync for RevocationAndRenewalInformation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RevocationAndRenewalItem(::windows_core::IUnknown);
impl RevocationAndRenewalItem {
    pub fn Reasons(&self) -> ::windows_core::Result<RevocationAndRenewalReasons> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reasons)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HeaderHash(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeaderHash)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PublicKeyHash(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PublicKeyHash)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RenewalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenewalId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for RevocationAndRenewalItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.RevocationAndRenewalItem;{3099c20c-3cf0-49ea-902d-caf32d2dde2c})");
}
unsafe impl ::windows_core::Interface for RevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RevocationAndRenewalItem {
    const IID: ::windows_core::GUID = <IRevocationAndRenewalItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RevocationAndRenewalItem {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalItem";
}
::windows_core::imp::interface_hierarchy!(RevocationAndRenewalItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RevocationAndRenewalItem {}
unsafe impl ::core::marker::Sync for RevocationAndRenewalItem {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ServiceRequestedEventArgs(::windows_core::IUnknown);
impl ServiceRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<IMediaProtectionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Completion(&self) -> ::windows_core::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Media_Playback\"`"]
    #[cfg(feature = "Media_Playback")]
    pub fn MediaPlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::ComInterface::cast::<IServiceRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaPlaybackItem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ServiceRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ServiceRequestedEventArgs;{34283baf-abb4-4fc1-bd89-93f106573a49})");
}
unsafe impl ::windows_core::Interface for ServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ServiceRequestedEventArgs {
    const IID: ::windows_core::GUID = <IServiceRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ServiceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ServiceRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ServiceRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ServiceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ServiceRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GraphicsTrustStatus(pub i32);
impl GraphicsTrustStatus {
    pub const TrustNotRequired: Self = Self(0i32);
    pub const TrustEstablished: Self = Self(1i32);
    pub const EnvironmentNotSupported: Self = Self(2i32);
    pub const DriverNotSupported: Self = Self(3i32);
    pub const DriverSigningFailure: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for GraphicsTrustStatus {}
impl ::core::clone::Clone for GraphicsTrustStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GraphicsTrustStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GraphicsTrustStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GraphicsTrustStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GraphicsTrustStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GraphicsTrustStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.GraphicsTrustStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdcpProtection(pub i32);
impl HdcpProtection {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const OnWithTypeEnforcement: Self = Self(2i32);
}
impl ::core::marker::Copy for HdcpProtection {}
impl ::core::clone::Clone for HdcpProtection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdcpProtection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HdcpProtection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HdcpProtection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcpProtection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HdcpProtection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpProtection;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdcpSetProtectionResult(pub i32);
impl HdcpSetProtectionResult {
    pub const Success: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for HdcpSetProtectionResult {}
impl ::core::clone::Clone for HdcpSetProtectionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdcpSetProtectionResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HdcpSetProtectionResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HdcpSetProtectionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcpSetProtectionResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HdcpSetProtectionResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpSetProtectionResult;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProtectionCapabilityResult(pub i32);
impl ProtectionCapabilityResult {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionCapabilityResult {}
impl ::core::clone::Clone for ProtectionCapabilityResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionCapabilityResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProtectionCapabilityResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProtectionCapabilityResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionCapabilityResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProtectionCapabilityResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.ProtectionCapabilityResult;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RenewalStatus(pub i32);
impl RenewalStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const UpdatesInProgress: Self = Self(1i32);
    pub const UserCancelled: Self = Self(2i32);
    pub const AppComponentsMayNeedUpdating: Self = Self(3i32);
    pub const NoComponentsFound: Self = Self(4i32);
}
impl ::core::marker::Copy for RenewalStatus {}
impl ::core::clone::Clone for RenewalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RenewalStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RenewalStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RenewalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenewalStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RenewalStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RenewalStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RevocationAndRenewalReasons(pub u32);
impl RevocationAndRenewalReasons {
    pub const UserModeComponentLoad: Self = Self(1u32);
    pub const KernelModeComponentLoad: Self = Self(2u32);
    pub const AppComponent: Self = Self(4u32);
    pub const GlobalRevocationListLoadFailed: Self = Self(16u32);
    pub const InvalidGlobalRevocationListSignature: Self = Self(32u32);
    pub const GlobalRevocationListAbsent: Self = Self(4096u32);
    pub const ComponentRevoked: Self = Self(8192u32);
    pub const InvalidComponentCertificateExtendedKeyUse: Self = Self(16384u32);
    pub const ComponentCertificateRevoked: Self = Self(32768u32);
    pub const InvalidComponentCertificateRoot: Self = Self(65536u32);
    pub const ComponentHighSecurityCertificateRevoked: Self = Self(131072u32);
    pub const ComponentLowSecurityCertificateRevoked: Self = Self(262144u32);
    pub const BootDriverVerificationFailed: Self = Self(1048576u32);
    pub const ComponentSignedWithTestCertificate: Self = Self(16777216u32);
    pub const EncryptionFailure: Self = Self(268435456u32);
}
impl ::core::marker::Copy for RevocationAndRenewalReasons {}
impl ::core::clone::Clone for RevocationAndRenewalReasons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RevocationAndRenewalReasons {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RevocationAndRenewalReasons {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RevocationAndRenewalReasons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevocationAndRenewalReasons").field(&self.0).finish()
    }
}
impl RevocationAndRenewalReasons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RevocationAndRenewalReasons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RevocationAndRenewalReasons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RevocationAndRenewalReasons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for RevocationAndRenewalReasons {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RevocationAndRenewalReasons;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ComponentLoadFailedEventHandler(pub ::windows_core::IUnknown);
impl ComponentLoadFailedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ComponentLoadFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ComponentLoadFailedEventHandlerBox::<F> { vtable: &ComponentLoadFailedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaProtectionManager>,
        P1: ::windows_core::IntoParam<ComponentLoadFailedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ComponentLoadFailedEventHandlerBox<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ComponentLoadFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ComponentLoadFailedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ComponentLoadFailedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ComponentLoadFailedEventHandlerBox<F> {
    const VTABLE: ComponentLoadFailedEventHandler_Vtbl = ComponentLoadFailedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <ComponentLoadFailedEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
unsafe impl ::windows_core::Interface for ComponentLoadFailedEventHandler {
    type Vtable = ComponentLoadFailedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ComponentLoadFailedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95da643c_6db9_424b_86ca_091af432081c);
}
impl ::windows_core::RuntimeType for ComponentLoadFailedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{95da643c-6db9-424b-86ca-091af432081c}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ComponentLoadFailedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RebootNeededEventHandler(pub ::windows_core::IUnknown);
impl RebootNeededEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&MediaProtectionManager>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RebootNeededEventHandlerBox::<F> { vtable: &RebootNeededEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaProtectionManager>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RebootNeededEventHandlerBox<F: FnMut(::core::option::Option<&MediaProtectionManager>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RebootNeededEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&MediaProtectionManager>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> RebootNeededEventHandlerBox<F> {
    const VTABLE: RebootNeededEventHandler_Vtbl = RebootNeededEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <RebootNeededEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender)).into()
    }
}
unsafe impl ::windows_core::Interface for RebootNeededEventHandler {
    type Vtable = RebootNeededEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RebootNeededEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64e12a45_973b_4a3a_b260_91898a49a82c);
}
impl ::windows_core::RuntimeType for RebootNeededEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{64e12a45-973b-4a3a-b260-91898a49a82c}");
}
#[repr(C)]
#[doc(hidden)]
pub struct RebootNeededEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ServiceRequestedEventHandler(pub ::windows_core::IUnknown);
impl ServiceRequestedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ServiceRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ServiceRequestedEventHandlerBox::<F> { vtable: &ServiceRequestedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaProtectionManager>,
        P1: ::windows_core::IntoParam<ServiceRequestedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ServiceRequestedEventHandlerBox<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ServiceRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ServiceRequestedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ServiceRequestedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ServiceRequestedEventHandlerBox<F> {
    const VTABLE: ServiceRequestedEventHandler_Vtbl = ServiceRequestedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <ServiceRequestedEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
unsafe impl ::windows_core::Interface for ServiceRequestedEventHandler {
    type Vtable = ServiceRequestedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ServiceRequestedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2d690ba_cac9_48e1_95c0_d38495a84055);
}
impl ::windows_core::RuntimeType for ServiceRequestedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d2d690ba-cac9-48e1-95c0-d38495a84055}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ServiceRequestedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
