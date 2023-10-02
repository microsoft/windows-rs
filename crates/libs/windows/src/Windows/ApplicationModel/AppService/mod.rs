#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceCatalogStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceCatalogStatics {
    type Vtable = IAppServiceCatalogStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceCatalogStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef0d2507_d132_4c85_8395_3c31d5a1e941);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceCatalogStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppServiceProvidersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appservicename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppServiceProvidersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceClosedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde6016f6_cb03_4d35_ac8d_cc6303239731);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppServiceClosedStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceConnection {
    type Vtable = IAppServiceConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9dd474a2_871f_4d52_89a9_9e090531bd27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAppServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRequestReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRequestReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ServiceClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceConnection2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceConnection2 {
    type Vtable = IAppServiceConnection2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceConnection2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bdfcd5f_2302_4fbd_8061_52511c2f8bf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnection2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub OpenRemoteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    OpenRemoteAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "System")]
    pub SetUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceConnectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceConnectionStatics {
    type Vtable = IAppServiceConnectionStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceConnectionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadc56ce9_d408_5673_8637_827a4b274168);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub SendStatelessMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connection: *mut ::core::ffi::c_void, connectionrequest: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    SendStatelessMessageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceDeferral {
    type Vtable = IAppServiceDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e1b5322_eab0_4248_ae04_fdf93838e472);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceRequest {
    type Vtable = IAppServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20e58d9d_18de_4b01_80ba_90a76204e3c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendResponseAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceRequestReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceRequestReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e122360_ff65_44ae_9e45_857fe4180681);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceRequestReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceResponse {
    type Vtable = IAppServiceResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceResponse {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d503cec_9aa3_4e68_9559_9de63e372ce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceResponse_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppServiceResponseStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88a2dcac_ad28_41b8_80bb_bdf1b2169e19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppServiceConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceTriggerDetails2 {
    type Vtable = IAppServiceTriggerDetails2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceTriggerDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe83d54b2_28cc_43f2_b465_c0482e59e2dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsRemoteSystemConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceTriggerDetails3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceTriggerDetails3 {
    type Vtable = IAppServiceTriggerDetails3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceTriggerDetails3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbd71e21_7939_4e68_9e3c_7780147aabb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CheckCallerForCapabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckCallerForCapabilityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppServiceTriggerDetails4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppServiceTriggerDetails4 {
    type Vtable = IAppServiceTriggerDetails4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppServiceTriggerDetails4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1185b180_8861_5e30_ab55_1cf4d08bbf6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceTriggerDetails4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CallerRemoteConnectionToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStatelessAppServiceResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStatelessAppServiceResponse {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43754af7_a9ec_52fe_82e7_939b68dc9388);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatelessAppServiceResponse_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StatelessAppServiceResponseStatus) -> ::windows_core::HRESULT,
}
pub struct AppServiceCatalog;
impl AppServiceCatalog {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppServiceProvidersAsync(appservicename: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>> {
        Self::IAppServiceCatalogStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAppServiceProvidersAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(appservicename), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppServiceCatalogStatics<R, F: FnOnce(&IAppServiceCatalogStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppServiceCatalog, IAppServiceCatalogStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AppServiceCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceCatalog";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppServiceClosedEventArgs(::windows_core::IUnknown);
impl AppServiceClosedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<AppServiceClosedStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppServiceClosedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceClosedEventArgs;{de6016f6-cb03-4d35-ac8d-cc6303239731})");
}
unsafe impl ::windows_core::Interface for AppServiceClosedEventArgs {
    type Vtable = IAppServiceClosedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppServiceClosedEventArgs {
    const IID: ::windows_core::GUID = <IAppServiceClosedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceClosedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceClosedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppServiceClosedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppServiceClosedEventArgs {}
unsafe impl ::core::marker::Sync for AppServiceClosedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppServiceConnection(::windows_core::IUnknown);
impl AppServiceConnection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppServiceConnection, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AppServiceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppServiceName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppServiceName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppServiceName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPackageFamilyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPackageFamilyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendMessageAsync<P0>(&self, message: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponse>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendMessageAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestReceived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRequestReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRequestReceived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceClosed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceClosed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveServiceClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"System_RemoteSystems\"`"]
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn OpenRemoteAsync<P0>(&self, remotesystemconnectionrequest: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>
    where
        P0: ::windows_core::IntoParam<super::super::System::RemoteSystems::RemoteSystemConnectionRequest>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenRemoteAsync)(::windows_core::Interface::as_raw(this), remotesystemconnectionrequest.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IAppServiceConnection2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn SetUser<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppServiceConnection2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUser)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"System_RemoteSystems\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn SendStatelessMessageAsync<P0, P1, P2>(connection: P0, connectionrequest: P1, message: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>>
    where
        P0: ::windows_core::IntoParam<AppServiceConnection>,
        P1: ::windows_core::IntoParam<super::super::System::RemoteSystems::RemoteSystemConnectionRequest>,
        P2: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        Self::IAppServiceConnectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendStatelessMessageAsync)(::windows_core::Interface::as_raw(this), connection.into_param().abi(), connectionrequest.into_param().abi(), message.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAppServiceConnectionStatics<R, F: FnOnce(&IAppServiceConnectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppServiceConnection, IAppServiceConnectionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppServiceConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceConnection;{9dd474a2-871f-4d52-89a9-9e090531bd27})");
}
unsafe impl ::windows_core::Interface for AppServiceConnection {
    type Vtable = IAppServiceConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppServiceConnection {
    const IID: ::windows_core::GUID = <IAppServiceConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceConnection";
}
::windows_core::imp::interface_hierarchy!(AppServiceConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for AppServiceConnection {}
unsafe impl ::core::marker::Send for AppServiceConnection {}
unsafe impl ::core::marker::Sync for AppServiceConnection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppServiceDeferral(::windows_core::IUnknown);
impl AppServiceDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for AppServiceDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceDeferral;{7e1b5322-eab0-4248-ae04-fdf93838e472})");
}
unsafe impl ::windows_core::Interface for AppServiceDeferral {
    type Vtable = IAppServiceDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppServiceDeferral {
    const IID: ::windows_core::GUID = <IAppServiceDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceDeferral";
}
::windows_core::imp::interface_hierarchy!(AppServiceDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppServiceDeferral {}
unsafe impl ::core::marker::Sync for AppServiceDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppServiceRequest(::windows_core::IUnknown);
impl AppServiceRequest {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendResponseAsync<P0>(&self, message: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendResponseAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceRequest;{20e58d9d-18de-4b01-80ba-90a76204e3c8})");
}
unsafe impl ::windows_core::Interface for AppServiceRequest {
    type Vtable = IAppServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppServiceRequest {
    const IID: ::windows_core::GUID = <IAppServiceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequest";
}
::windows_core::imp::interface_hierarchy!(AppServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppServiceRequest {}
unsafe impl ::core::marker::Sync for AppServiceRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppServiceRequestReceivedEventArgs(::windows_core::IUnknown);
impl AppServiceRequestReceivedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<AppServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<AppServiceDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppServiceRequestReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs;{6e122360-ff65-44ae-9e45-857fe4180681})");
}
unsafe impl ::windows_core::Interface for AppServiceRequestReceivedEventArgs {
    type Vtable = IAppServiceRequestReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppServiceRequestReceivedEventArgs {
    const IID: ::windows_core::GUID = <IAppServiceRequestReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceRequestReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppServiceRequestReceivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppServiceRequestReceivedEventArgs {}
unsafe impl ::core::marker::Sync for AppServiceRequestReceivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppServiceResponse(::windows_core::IUnknown);
impl AppServiceResponse {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<AppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppServiceResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceResponse;{8d503cec-9aa3-4e68-9559-9de63e372ce4})");
}
unsafe impl ::windows_core::Interface for AppServiceResponse {
    type Vtable = IAppServiceResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppServiceResponse {
    const IID: ::windows_core::GUID = <IAppServiceResponse as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceResponse";
}
::windows_core::imp::interface_hierarchy!(AppServiceResponse, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppServiceResponse {}
unsafe impl ::core::marker::Sync for AppServiceResponse {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppServiceTriggerDetails(::windows_core::IUnknown);
impl AppServiceTriggerDetails {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppServiceConnection(&self) -> ::windows_core::Result<AppServiceConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppServiceConnection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRemoteSystemConnection(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppServiceTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRemoteSystemConnection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CheckCallerForCapabilityAsync(&self, capabilityname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IAppServiceTriggerDetails3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckCallerForCapabilityAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(capabilityname), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerRemoteConnectionToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppServiceTriggerDetails4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerRemoteConnectionToken)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppServiceTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.AppServiceTriggerDetails;{88a2dcac-ad28-41b8-80bb-bdf1b2169e19})");
}
unsafe impl ::windows_core::Interface for AppServiceTriggerDetails {
    type Vtable = IAppServiceTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppServiceTriggerDetails {
    const IID: ::windows_core::GUID = <IAppServiceTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppServiceTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.AppServiceTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(AppServiceTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppServiceTriggerDetails {}
unsafe impl ::core::marker::Sync for AppServiceTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StatelessAppServiceResponse(::windows_core::IUnknown);
impl StatelessAppServiceResponse {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<StatelessAppServiceResponseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for StatelessAppServiceResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppService.StatelessAppServiceResponse;{43754af7-a9ec-52fe-82e7-939b68dc9388})");
}
unsafe impl ::windows_core::Interface for StatelessAppServiceResponse {
    type Vtable = IStatelessAppServiceResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StatelessAppServiceResponse {
    const IID: ::windows_core::GUID = <IStatelessAppServiceResponse as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StatelessAppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.StatelessAppServiceResponse";
}
::windows_core::imp::interface_hierarchy!(StatelessAppServiceResponse, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for StatelessAppServiceResponse {}
unsafe impl ::core::marker::Sync for StatelessAppServiceResponse {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppServiceClosedStatus(pub i32);
impl AppServiceClosedStatus {
    pub const Completed: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for AppServiceClosedStatus {}
impl ::core::clone::Clone for AppServiceClosedStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppServiceClosedStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppServiceClosedStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppServiceClosedStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceClosedStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppServiceClosedStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceClosedStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppServiceConnectionStatus(pub i32);
impl AppServiceConnectionStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const RemoteSystemUnavailable: Self = Self(5i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(6i32);
    pub const NotAuthorized: Self = Self(7i32);
    pub const AuthenticationError: Self = Self(8i32);
    pub const NetworkNotAvailable: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const WebServiceUnavailable: Self = Self(11i32);
}
impl ::core::marker::Copy for AppServiceConnectionStatus {}
impl ::core::clone::Clone for AppServiceConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppServiceConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppServiceConnectionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppServiceConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceConnectionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppServiceConnectionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceConnectionStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppServiceResponseStatus(pub i32);
impl AppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const Failure: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const MessageSizeTooLarge: Self = Self(5i32);
    pub const AppUnavailable: Self = Self(6i32);
    pub const AuthenticationError: Self = Self(7i32);
    pub const NetworkNotAvailable: Self = Self(8i32);
    pub const DisabledByPolicy: Self = Self(9i32);
    pub const WebServiceUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for AppServiceResponseStatus {}
impl ::core::clone::Clone for AppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppServiceResponseStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppServiceResponseStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppServiceResponseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppServiceResponseStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppServiceResponseStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.AppServiceResponseStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StatelessAppServiceResponseStatus(pub i32);
impl StatelessAppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(5i32);
    pub const NotAuthorized: Self = Self(6i32);
    pub const ResourceLimitsExceeded: Self = Self(7i32);
    pub const MessageSizeTooLarge: Self = Self(8i32);
    pub const Failure: Self = Self(9i32);
    pub const Unknown: Self = Self(10i32);
    pub const AuthenticationError: Self = Self(11i32);
    pub const NetworkNotAvailable: Self = Self(12i32);
    pub const DisabledByPolicy: Self = Self(13i32);
    pub const WebServiceUnavailable: Self = Self(14i32);
}
impl ::core::marker::Copy for StatelessAppServiceResponseStatus {}
impl ::core::clone::Clone for StatelessAppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StatelessAppServiceResponseStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StatelessAppServiceResponseStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StatelessAppServiceResponseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatelessAppServiceResponseStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StatelessAppServiceResponseStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppService.StatelessAppServiceResponseStatus;i4)");
}
