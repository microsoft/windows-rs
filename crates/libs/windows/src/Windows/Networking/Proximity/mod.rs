#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct ConnectionRequestedEventArgs(::windows::core::IUnknown);
impl ConnectionRequestedEventArgs {
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn PeerInformation(&self) -> ::windows::core::Result<PeerInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PeerInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for ConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectionRequestedEventArgs {}
impl ::core::fmt::Debug for ConnectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ConnectionRequestedEventArgs;{eb6891ae-4f1e-4c66-bd0d-46924a942e08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IConnectionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.ConnectionRequestedEventArgs";
}
impl ::core::convert::From<ConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ConnectionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ConnectionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ConnectionRequestedEventArgs {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct DeviceArrivedEventHandler(pub ::windows::core::IUnknown);
impl DeviceArrivedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DeviceArrivedEventHandlerBox::<F> { vtable: &DeviceArrivedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ProximityDevice>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DeviceArrivedEventHandlerBox<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DeviceArrivedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DeviceArrivedEventHandlerBox<F> {
    const VTABLE: DeviceArrivedEventHandler_Vtbl = DeviceArrivedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DeviceArrivedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
impl ::core::clone::Clone for DeviceArrivedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceArrivedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceArrivedEventHandler {}
impl ::core::fmt::Debug for DeviceArrivedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceArrivedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DeviceArrivedEventHandler {
    type Vtable = DeviceArrivedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa9da69_f6e1_49c9_a49e_8e0fc58fb911);
}
unsafe impl ::windows::core::RuntimeType for DeviceArrivedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{efa9da69-f6e1-49c9-a49e-8e0fc58fb911}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DeviceArrivedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct DeviceDepartedEventHandler(pub ::windows::core::IUnknown);
impl DeviceDepartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DeviceDepartedEventHandlerBox::<F> { vtable: &DeviceDepartedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ProximityDevice>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DeviceDepartedEventHandlerBox<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DeviceDepartedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DeviceDepartedEventHandlerBox<F> {
    const VTABLE: DeviceDepartedEventHandler_Vtbl = DeviceDepartedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DeviceDepartedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
impl ::core::clone::Clone for DeviceDepartedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceDepartedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceDepartedEventHandler {}
impl ::core::fmt::Debug for DeviceDepartedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDepartedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DeviceDepartedEventHandler {
    type Vtable = DeviceDepartedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa9da69_f6e2_49c9_a49e_8e0fc58fb911);
}
unsafe impl ::windows::core::RuntimeType for DeviceDepartedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{efa9da69-f6e2-49c9-a49e-8e0fc58fb911}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DeviceDepartedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb6891ae_4f1e_4c66_bd0d_46924a942e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PeerInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerFinderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeerFinderStatics {
    type Vtable = IPeerFinderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x914b3b61_f6e1_47c4_a14c_148a1903d0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFinderStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllowBluetooth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowBluetooth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowInfrastructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowInfrastructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowWiFiDirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowWiFiDirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SupportedDiscoveryTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeerDiscoveryTypes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateIdentities: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartWithMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peermessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TriggeredConnectionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggeredConnectionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggeredConnectionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggeredConnectionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllPeersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllPeersAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peerinformation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    ConnectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerFinderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeerFinderStatics2 {
    type Vtable = IPeerFinderStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e73c65_fdd0_4b0b_9312_866408935d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFinderStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeerRole) -> ::windows::core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PeerRole) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DiscoveryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscoveryData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDiscoveryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDiscoveryData: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeerInformation {
    type Vtable = IPeerInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20024f08_9fff_45f4_b6e9_408b2ebef373);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerInformation3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeerInformation3 {
    type Vtable = IPeerInformation3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb20f612a_dbd0_40f8_95bd_2d4209c7836f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformation3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DiscoveryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscoveryData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerInformationWithHostAndService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeerInformationWithHostAndService {
    type Vtable = IPeerInformationWithHostAndService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc7ccad_1b70_4e8b_92db_bbe781419308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformationWithHostAndService_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPeerWatcher {
    type Vtable = IPeerWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cee21f8_2fa6_4679_9691_03c94a420f34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeerWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximityDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximityDevice {
    type Vtable = IProximityDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8a552_f6e1_4329_a0fc_ab6b0fd28262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SubscribeForMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, messagereceivedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub PublishMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i64) -> ::windows::core::HRESULT,
    pub PublishMessageWithCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PublishBinaryMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishBinaryMessage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PublishBinaryMessageWithCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::windows::core::RawPtr, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishBinaryMessageWithCallback: usize,
    #[cfg(feature = "Foundation")]
    pub PublishUriMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishUriMessage: usize,
    #[cfg(feature = "Foundation")]
    pub PublishUriMessageWithCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishUriMessageWithCallback: usize,
    pub StopSubscribingForMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriptionid: i64) -> ::windows::core::HRESULT,
    pub StopPublishingMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeviceArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arrivedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceArrived: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceDeparted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, departedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceDeparted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceDeparted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceDeparted: usize,
    pub MaxMessageBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub BitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximityDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximityDeviceStatics {
    type Vtable = IProximityDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x914ba01d_f6e1_47c4_a14c_148a1903d0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximityMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProximityMessage {
    type Vtable = IProximityMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab0782_f6e1_4675_a045_d8e320c24808);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SubscriptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    pub DataAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITriggeredConnectionStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6a780ad_f6e1_4d54_96e2_33f620bca88a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggeredConnectionStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TriggeredConnectState) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub Socket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    Socket: usize,
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct MessageReceivedHandler(pub ::windows::core::IUnknown);
impl MessageReceivedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MessageReceivedHandlerBox::<F> { vtable: &MessageReceivedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ProximityDevice>, Param1: ::windows::core::IntoParam<'a, ProximityMessage>>(&self, sender: Param0, message: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct MessageReceivedHandlerBox<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MessageReceivedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MessageReceivedHandlerBox<F> {
    const VTABLE: MessageReceivedHandler_Vtbl = MessageReceivedHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MessageReceivedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&message)).into()
    }
}
impl ::core::clone::Clone for MessageReceivedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageReceivedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageReceivedHandler {}
impl ::core::fmt::Debug for MessageReceivedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageReceivedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for MessageReceivedHandler {
    type Vtable = MessageReceivedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab0782_f6e2_4675_a045_d8e320c24808);
}
unsafe impl ::windows::core::RuntimeType for MessageReceivedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{efab0782-f6e2-4675-a045-d8e320c24808}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct MessageTransmittedHandler(pub ::windows::core::IUnknown);
impl MessageTransmittedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MessageTransmittedHandlerBox::<F> { vtable: &MessageTransmittedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ProximityDevice>>(&self, sender: Param0, messageid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), messageid).ok() }
    }
}
#[repr(C)]
struct MessageTransmittedHandlerBox<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MessageTransmittedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MessageTransmittedHandlerBox<F> {
    const VTABLE: MessageTransmittedHandler_Vtbl = MessageTransmittedHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MessageTransmittedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, messageid: i64) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), messageid).into()
    }
}
impl ::core::clone::Clone for MessageTransmittedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageTransmittedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageTransmittedHandler {}
impl ::core::fmt::Debug for MessageTransmittedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageTransmittedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for MessageTransmittedHandler {
    type Vtable = MessageTransmittedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefaa0b4a_f6e2_4d7d_856c_78fc8efc021e);
}
unsafe impl ::windows::core::RuntimeType for MessageTransmittedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{efaa0b4a-f6e2-4d7d-856c-78fc8efc021e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct MessageTransmittedHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, messageid: i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PeerDiscoveryTypes(pub u32);
impl PeerDiscoveryTypes {
    pub const None: Self = Self(0u32);
    pub const Browse: Self = Self(1u32);
    pub const Triggered: Self = Self(2u32);
}
impl ::core::marker::Copy for PeerDiscoveryTypes {}
impl ::core::clone::Clone for PeerDiscoveryTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeerDiscoveryTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PeerDiscoveryTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeerDiscoveryTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerDiscoveryTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PeerDiscoveryTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PeerDiscoveryTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PeerDiscoveryTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PeerDiscoveryTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PeerDiscoveryTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PeerDiscoveryTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerDiscoveryTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
pub struct PeerFinder {}
impl PeerFinder {
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn AllowBluetooth() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowBluetooth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn SetAllowBluetooth(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetAllowBluetooth)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn AllowInfrastructure() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowInfrastructure)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn SetAllowInfrastructure(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetAllowInfrastructure)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn AllowWiFiDirect() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowWiFiDirect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn SetAllowWiFiDirect(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetAllowWiFiDirect)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn DisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn SupportedDiscoveryTypes() -> ::windows::core::Result<PeerDiscoveryTypes> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: PeerDiscoveryTypes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedDiscoveryTypes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerDiscoveryTypes>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateIdentities() -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlternateIdentities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Start() -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn StartWithMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(peermessage: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).StartWithMessage)(::core::mem::transmute_copy(this), peermessage.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Stop() -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TriggeredConnectionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TriggeredConnectionStateChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TriggeredConnectionStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTriggeredConnectionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveTriggeredConnectionStateChanged)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ConnectionRequestedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionRequested)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllPeersAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllPeersAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, PeerInformation>>(peerinformation: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::core::mem::transmute_copy(this), peerinformation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Role() -> ::windows::core::Result<PeerRole> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: PeerRole = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Role)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerRole>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn SetRole(value: PeerRole) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetRole)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscoveryData() -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiscoveryData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDiscoveryData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetDiscoveryData)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn CreateWatcher() -> ::windows::core::Result<PeerWatcher> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerWatcher>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPeerFinderStatics<R, F: FnOnce(&IPeerFinderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PeerFinder, IPeerFinderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPeerFinderStatics2<R, F: FnOnce(&IPeerFinderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PeerFinder, IPeerFinderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PeerFinder {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerFinder";
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct PeerInformation(::windows::core::IUnknown);
impl PeerInformation {
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscoveryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiscoveryData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn HostName(&self) -> ::windows::core::Result<super::HostName> {
        let this = &::windows::core::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HostName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PeerInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PeerInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PeerInformation {}
impl ::core::fmt::Debug for PeerInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeerInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.PeerInformation;{20024f08-9fff-45f4-b6e9-408b2ebef373})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PeerInformation {
    type Vtable = IPeerInformation_Vtbl;
    const IID: ::windows::core::GUID = <IPeerInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PeerInformation {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerInformation";
}
impl ::core::convert::From<PeerInformation> for ::windows::core::IUnknown {
    fn from(value: PeerInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeerInformation> for ::windows::core::IUnknown {
    fn from(value: &PeerInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PeerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PeerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PeerInformation> for ::windows::core::IInspectable {
    fn from(value: PeerInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeerInformation> for ::windows::core::IInspectable {
    fn from(value: &PeerInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PeerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PeerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PeerInformation {}
unsafe impl ::core::marker::Sync for PeerInformation {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PeerRole(pub i32);
impl PeerRole {
    pub const Peer: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Client: Self = Self(2i32);
}
impl ::core::marker::Copy for PeerRole {}
impl ::core::clone::Clone for PeerRole {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeerRole {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PeerRole {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeerRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerRole").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeerRole {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerRole;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct PeerWatcher(::windows::core::IUnknown);
impl PeerWatcher {
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Added)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Removed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Updated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PeerWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: PeerWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for PeerWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PeerWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PeerWatcher {}
impl ::core::fmt::Debug for PeerWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeerWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.PeerWatcher;{3cee21f8-2fa6-4679-9691-03c94a420f34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PeerWatcher {
    type Vtable = IPeerWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IPeerWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PeerWatcher {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerWatcher";
}
impl ::core::convert::From<PeerWatcher> for ::windows::core::IUnknown {
    fn from(value: PeerWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeerWatcher> for ::windows::core::IUnknown {
    fn from(value: &PeerWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PeerWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PeerWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PeerWatcher> for ::windows::core::IInspectable {
    fn from(value: PeerWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeerWatcher> for ::windows::core::IInspectable {
    fn from(value: &PeerWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PeerWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PeerWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PeerWatcher {}
unsafe impl ::core::marker::Sync for PeerWatcher {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PeerWatcherStatus(pub i32);
impl PeerWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for PeerWatcherStatus {}
impl ::core::clone::Clone for PeerWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeerWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PeerWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeerWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeerWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct ProximityDevice(::windows::core::IUnknown);
impl ProximityDevice {
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn SubscribeForMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, MessageReceivedHandler>>(&self, messagetype: Param0, messagereceivedhandler: Param1) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SubscribeForMessage)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), messagereceivedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn PublishMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, messagetype: Param0, message: Param1) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublishMessage)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn PublishMessageWithCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, MessageTransmittedHandler>>(&self, messagetype: Param0, message: Param1, messagetransmittedhandler: Param2) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublishMessageWithCallback)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishBinaryMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, messagetype: Param0, message: Param1) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublishBinaryMessage)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishBinaryMessageWithCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, MessageTransmittedHandler>>(&self, messagetype: Param0, message: Param1, messagetransmittedhandler: Param2) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublishBinaryMessageWithCallback)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PublishUriMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, message: Param0) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublishUriMessage)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PublishUriMessageWithCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, MessageTransmittedHandler>>(&self, message: Param0, messagetransmittedhandler: Param1) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublishUriMessageWithCallback)(::core::mem::transmute_copy(this), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn StopSubscribingForMessage(&self, subscriptionid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopSubscribingForMessage)(::core::mem::transmute_copy(this), subscriptionid).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn StopPublishingMessage(&self, messageid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopPublishingMessage)(::core::mem::transmute_copy(this), messageid).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeviceArrived<'a, Param0: ::windows::core::IntoParam<'a, DeviceArrivedEventHandler>>(&self, arrivedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceArrived)(::core::mem::transmute_copy(this), arrivedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDeviceArrived)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeviceDeparted<'a, Param0: ::windows::core::IntoParam<'a, DeviceDepartedEventHandler>>(&self, departedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceDeparted)(::core::mem::transmute_copy(this), departedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceDeparted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDeviceDeparted)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn MaxMessageBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxMessageBytes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn BitsPerSecond(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitsPerSecond)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProximityDevice>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromId)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<ProximityDevice>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProximityDeviceStatics<R, F: FnOnce(&IProximityDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProximityDevice, IProximityDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ProximityDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProximityDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximityDevice {}
impl ::core::fmt::Debug for ProximityDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximityDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProximityDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ProximityDevice;{efa8a552-f6e1-4329-a0fc-ab6b0fd28262})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProximityDevice {
    type Vtable = IProximityDevice_Vtbl;
    const IID: ::windows::core::GUID = <IProximityDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProximityDevice {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityDevice";
}
impl ::core::convert::From<ProximityDevice> for ::windows::core::IUnknown {
    fn from(value: ProximityDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximityDevice> for ::windows::core::IUnknown {
    fn from(value: &ProximityDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximityDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximityDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProximityDevice> for ::windows::core::IInspectable {
    fn from(value: ProximityDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximityDevice> for ::windows::core::IInspectable {
    fn from(value: &ProximityDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximityDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximityDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProximityDevice {}
unsafe impl ::core::marker::Sync for ProximityDevice {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct ProximityMessage(::windows::core::IUnknown);
impl ProximityMessage {
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn MessageType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MessageType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn SubscriptionId(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SubscriptionId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn DataAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataAsString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ProximityMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProximityMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximityMessage {}
impl ::core::fmt::Debug for ProximityMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximityMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProximityMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ProximityMessage;{efab0782-f6e1-4675-a045-d8e320c24808})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProximityMessage {
    type Vtable = IProximityMessage_Vtbl;
    const IID: ::windows::core::GUID = <IProximityMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProximityMessage {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityMessage";
}
impl ::core::convert::From<ProximityMessage> for ::windows::core::IUnknown {
    fn from(value: ProximityMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximityMessage> for ::windows::core::IUnknown {
    fn from(value: &ProximityMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximityMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximityMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProximityMessage> for ::windows::core::IInspectable {
    fn from(value: ProximityMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProximityMessage> for ::windows::core::IInspectable {
    fn from(value: &ProximityMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximityMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximityMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProximityMessage {}
unsafe impl ::core::marker::Sync for ProximityMessage {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TriggeredConnectState(pub i32);
impl TriggeredConnectState {
    pub const PeerFound: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const Failed: Self = Self(5i32);
}
impl ::core::marker::Copy for TriggeredConnectState {}
impl ::core::clone::Clone for TriggeredConnectState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TriggeredConnectState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TriggeredConnectState {
    type Abi = Self;
}
impl ::core::fmt::Debug for TriggeredConnectState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TriggeredConnectState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TriggeredConnectState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.TriggeredConnectState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct TriggeredConnectionStateChangedEventArgs(::windows::core::IUnknown);
impl TriggeredConnectionStateChangedEventArgs {
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn State(&self) -> ::windows::core::Result<TriggeredConnectState> {
        let this = self;
        unsafe {
            let mut result__: TriggeredConnectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TriggeredConnectState>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Proximity\"`, `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn Socket(&self) -> ::windows::core::Result<super::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Socket)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Sockets::StreamSocket>(result__)
        }
    }
}
impl ::core::clone::Clone for TriggeredConnectionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TriggeredConnectionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TriggeredConnectionStateChangedEventArgs {}
impl ::core::fmt::Debug for TriggeredConnectionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TriggeredConnectionStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TriggeredConnectionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs;{c6a780ad-f6e1-4d54-96e2-33f620bca88a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ITriggeredConnectionStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TriggeredConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs";
}
impl ::core::convert::From<TriggeredConnectionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TriggeredConnectionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TriggeredConnectionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TriggeredConnectionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TriggeredConnectionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TriggeredConnectionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TriggeredConnectionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TriggeredConnectionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TriggeredConnectionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for TriggeredConnectionStateChangedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
