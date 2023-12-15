#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPerformLocalActionRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPerformLocalActionRequestedEventArgs {
    type Vtable = IPerformLocalActionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPerformLocalActionRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59359f4f_0862_53a3_a3b3_c932fb718cdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerformLocalActionRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteDesktopLocalAction) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopConnectionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteDesktopConnectionInfo {
    type Vtable = IRemoteDesktopConnectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteDesktopConnectionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68bd69d6_6dea_543b_b737_f347919f5093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopConnectionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RemoteDesktopConnectionStatus) -> ::windows_core::HRESULT,
    pub SwitchToLocalSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopConnectionInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteDesktopConnectionInfoStatics {
    type Vtable = IRemoteDesktopConnectionInfoStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteDesktopConnectionInfoStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a7dc5a1_3368_5a75_bb78_807df7ebc439);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopConnectionInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub GetForLaunchUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, launchuri: *mut ::core::ffi::c_void, windowid: super::super::super::UI::WindowId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForLaunchUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopConnectionRemoteInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteDesktopConnectionRemoteInfo {
    type Vtable = IRemoteDesktopConnectionRemoteInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteDesktopConnectionRemoteInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a3dfa7e_a7ab_547e_9a6a_4c565bbb8d71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopConnectionRemoteInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportSwitched: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SwitchToLocalSessionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSwitchToLocalSessionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PerformLocalActionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePerformLocalActionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopConnectionRemoteInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteDesktopConnectionRemoteInfoStatics {
    type Vtable = IRemoteDesktopConnectionRemoteInfoStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteDesktopConnectionRemoteInfoStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb590e64a_e4c9_53e8_b83d_a0db3676246a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopConnectionRemoteInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSwitchSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetForLaunchUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, launchuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteDesktopInfo {
    type Vtable = IRemoteDesktopInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteDesktopInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd185bb25_2f1e_5098_b9e0_f46d6358c5c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteDesktopInfoFactory {
    type Vtable = IRemoteDesktopInfoFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteDesktopInfoFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad0e8d58_b56f_5a8b_b419_8002ee0c5ee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteDesktopRegistrarStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteDesktopRegistrarStatics {
    type Vtable = IRemoteDesktopRegistrarStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteDesktopRegistrarStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x687c2750_46d9_5de3_8dc3_84a9202cecfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopRegistrarStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DesktopInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesktopInfos: usize,
    pub IsSwitchToLocalSessionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PerformLocalActionRequestedEventArgs(::windows_core::IUnknown);
impl PerformLocalActionRequestedEventArgs {
    pub fn Action(&self) -> ::windows_core::Result<RemoteDesktopLocalAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PerformLocalActionRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PerformLocalActionRequestedEventArgs {
    type Vtable = IPerformLocalActionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PerformLocalActionRequestedEventArgs {
    const IID: ::windows_core::GUID = <IPerformLocalActionRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PerformLocalActionRequestedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Provider.PerformLocalActionRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PerformLocalActionRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PerformLocalActionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PerformLocalActionRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RemoteDesktopConnectionInfo(::windows_core::IUnknown);
impl RemoteDesktopConnectionInfo {
    pub fn SetConnectionStatus(&self, value: RemoteDesktopConnectionStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetConnectionStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SwitchToLocalSession(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SwitchToLocalSession)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"UI\"`"]
    #[cfg(feature = "UI")]
    pub fn GetForLaunchUri<P0>(launchuri: P0, windowid: super::super::super::UI::WindowId) -> ::windows_core::Result<RemoteDesktopConnectionInfo>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IRemoteDesktopConnectionInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForLaunchUri)(::windows_core::Interface::as_raw(this), launchuri.into_param().abi(), windowid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteDesktopConnectionInfoStatics<R, F: FnOnce(&IRemoteDesktopConnectionInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RemoteDesktopConnectionInfo, IRemoteDesktopConnectionInfoStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RemoteDesktopConnectionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RemoteDesktopConnectionInfo {
    type Vtable = IRemoteDesktopConnectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RemoteDesktopConnectionInfo {
    const IID: ::windows_core::GUID = <IRemoteDesktopConnectionInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RemoteDesktopConnectionInfo {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo";
}
::windows_core::imp::interface_hierarchy!(RemoteDesktopConnectionInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RemoteDesktopConnectionInfo {}
unsafe impl ::core::marker::Sync for RemoteDesktopConnectionInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RemoteDesktopConnectionRemoteInfo(::windows_core::IUnknown);
impl RemoteDesktopConnectionRemoteInfo {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportSwitched(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportSwitched)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SwitchToLocalSessionRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<RemoteDesktopConnectionRemoteInfo, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SwitchToLocalSessionRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveSwitchToLocalSessionRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSwitchToLocalSessionRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PerformLocalActionRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<RemoteDesktopConnectionRemoteInfo, PerformLocalActionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PerformLocalActionRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RemovePerformLocalActionRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePerformLocalActionRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsSwitchSupported() -> ::windows_core::Result<bool> {
        Self::IRemoteDesktopConnectionRemoteInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSwitchSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForLaunchUri<P0>(launchuri: P0) -> ::windows_core::Result<RemoteDesktopConnectionRemoteInfo>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IRemoteDesktopConnectionRemoteInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForLaunchUri)(::windows_core::Interface::as_raw(this), launchuri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteDesktopConnectionRemoteInfoStatics<R, F: FnOnce(&IRemoteDesktopConnectionRemoteInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RemoteDesktopConnectionRemoteInfo, IRemoteDesktopConnectionRemoteInfoStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RemoteDesktopConnectionRemoteInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RemoteDesktopConnectionRemoteInfo {
    type Vtable = IRemoteDesktopConnectionRemoteInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RemoteDesktopConnectionRemoteInfo {
    const IID: ::windows_core::GUID = <IRemoteDesktopConnectionRemoteInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RemoteDesktopConnectionRemoteInfo {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo";
}
::windows_core::imp::interface_hierarchy!(RemoteDesktopConnectionRemoteInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for RemoteDesktopConnectionRemoteInfo {}
unsafe impl ::core::marker::Send for RemoteDesktopConnectionRemoteInfo {}
unsafe impl ::core::marker::Sync for RemoteDesktopConnectionRemoteInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RemoteDesktopInfo(::windows_core::IUnknown);
impl RemoteDesktopInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstance(id: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<RemoteDesktopInfo> {
        Self::IRemoteDesktopInfoFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteDesktopInfoFactory<R, F: FnOnce(&IRemoteDesktopInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RemoteDesktopInfo, IRemoteDesktopInfoFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RemoteDesktopInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RemoteDesktopInfo {
    type Vtable = IRemoteDesktopInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RemoteDesktopInfo {
    const IID: ::windows_core::GUID = <IRemoteDesktopInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RemoteDesktopInfo {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo";
}
::windows_core::imp::interface_hierarchy!(RemoteDesktopInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RemoteDesktopInfo {}
unsafe impl ::core::marker::Sync for RemoteDesktopInfo {}
pub struct RemoteDesktopRegistrar;
impl RemoteDesktopRegistrar {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesktopInfos() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<RemoteDesktopInfo>> {
        Self::IRemoteDesktopRegistrarStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesktopInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSwitchToLocalSessionEnabled() -> ::windows_core::Result<bool> {
        Self::IRemoteDesktopRegistrarStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSwitchToLocalSessionEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteDesktopRegistrarStatics<R, F: FnOnce(&IRemoteDesktopRegistrarStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RemoteDesktopRegistrar, IRemoteDesktopRegistrarStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for RemoteDesktopRegistrar {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Provider.RemoteDesktopRegistrar";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteDesktopConnectionStatus(pub i32);
impl RemoteDesktopConnectionStatus {
    pub const Connecting: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const UserInputNeeded: Self = Self(2i32);
    pub const Disconnected: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteDesktopConnectionStatus {}
impl ::core::clone::Clone for RemoteDesktopConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteDesktopConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RemoteDesktopConnectionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RemoteDesktopConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteDesktopConnectionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RemoteDesktopConnectionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteDesktopLocalAction(pub i32);
impl RemoteDesktopLocalAction {
    pub const ShowBluetoothSettings: Self = Self(0i32);
}
impl ::core::marker::Copy for RemoteDesktopLocalAction {}
impl ::core::clone::Clone for RemoteDesktopLocalAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteDesktopLocalAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RemoteDesktopLocalAction {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RemoteDesktopLocalAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteDesktopLocalAction").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RemoteDesktopLocalAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteDesktop.Provider.RemoteDesktopLocalAction;i4)");
}
