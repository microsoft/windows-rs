#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConnectionRequestedEventArgs(pub ::windows::core::IInspectable);
impl ConnectionRequestedEventArgs {
    pub fn PeerInformation(&self) -> ::windows::core::Result<PeerInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerInformation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ConnectionRequestedEventArgs;{eb6891ae-4f1e-4c66-bd0d-46924a942e08})");
}
unsafe impl ::windows::core::Interface for ConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb6891ae_4f1e_4c66_bd0d_46924a942e08);
}
impl ::windows::core::RuntimeName for ConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.ConnectionRequestedEventArgs";
}
impl ::core::convert::From<ConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ConnectionRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ConnectionRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ConnectionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ConnectionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ConnectionRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceArrivedEventHandler(::windows::core::IUnknown);
impl DeviceArrivedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DeviceArrivedEventHandler_box::<F> { vtable: &DeviceArrivedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ProximityDevice>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceArrivedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({efa9da69-f6e1-49c9-a49e-8e0fc58fb911})");
}
unsafe impl ::windows::core::Interface for DeviceArrivedEventHandler {
    type Vtable = DeviceArrivedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa9da69_f6e1_49c9_a49e_8e0fc58fb911);
}
#[repr(C)]
#[doc(hidden)]
pub struct DeviceArrivedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct DeviceArrivedEventHandler_box<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const DeviceArrivedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + 'static> DeviceArrivedEventHandler_box<F> {
    const VTABLE: DeviceArrivedEventHandler_abi = DeviceArrivedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <ProximityDevice as ::windows::core::Abi>::Abi as *const <ProximityDevice as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceDepartedEventHandler(::windows::core::IUnknown);
impl DeviceDepartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DeviceDepartedEventHandler_box::<F> { vtable: &DeviceDepartedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ProximityDevice>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceDepartedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({efa9da69-f6e2-49c9-a49e-8e0fc58fb911})");
}
unsafe impl ::windows::core::Interface for DeviceDepartedEventHandler {
    type Vtable = DeviceDepartedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa9da69_f6e2_49c9_a49e_8e0fc58fb911);
}
#[repr(C)]
#[doc(hidden)]
pub struct DeviceDepartedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct DeviceDepartedEventHandler_box<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const DeviceDepartedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::core::Result<()> + 'static> DeviceDepartedEventHandler_box<F> {
    const VTABLE: DeviceDepartedEventHandler_abi = DeviceDepartedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <ProximityDevice as ::windows::core::Abi>::Abi as *const <ProximityDevice as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectionRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb6891ae_4f1e_4c66_bd0d_46924a942e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerFinderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPeerFinderStatics {
    type Vtable = IPeerFinderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x914b3b61_f6e1_47c4_a14c_148a1903d0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFinderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PeerDiscoveryTypes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peermessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peerinformation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerFinderStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPeerFinderStatics2 {
    type Vtable = IPeerFinderStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e73c65_fdd0_4b0b_9312_866408935d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFinderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PeerRole) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PeerRole) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPeerInformation {
    type Vtable = IPeerInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20024f08_9fff_45f4_b6e9_408b2ebef373);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerInformation3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPeerInformation3 {
    type Vtable = IPeerInformation3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb20f612a_dbd0_40f8_95bd_2d4209c7836f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformation3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerInformationWithHostAndService(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPeerInformationWithHostAndService {
    type Vtable = IPeerInformationWithHostAndService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc7ccad_1b70_4e8b_92db_bbe781419308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformationWithHostAndService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPeerWatcher {
    type Vtable = IPeerWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cee21f8_2fa6_4679_9691_03c94a420f34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PeerWatcherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximityDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximityDevice {
    type Vtable = IProximityDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8a552_f6e1_4329_a0fc_ab6b0fd28262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, messagereceivedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::windows::core::RawPtr, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::windows::core::RawPtr, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subscriptionid: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, messageid: i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, arrivedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, departedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximityDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximityDeviceStatics {
    type Vtable = IProximityDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x914ba01d_f6e1_47c4_a14c_148a1903d0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximityMessage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProximityMessage {
    type Vtable = IProximityMessage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab0782_f6e1_4675_a045_d8e320c24808);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITriggeredConnectionStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6a780ad_f6e1_4d54_96e2_33f620bca88a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggeredConnectionStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TriggeredConnectState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MessageReceivedHandler(::windows::core::IUnknown);
impl MessageReceivedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = MessageReceivedHandler_box::<F> { vtable: &MessageReceivedHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ProximityDevice>, Param1: ::windows::core::IntoParam<'a, ProximityMessage>>(&self, sender: Param0, message: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), message.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MessageReceivedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({efab0782-f6e2-4675-a045-d8e320c24808})");
}
unsafe impl ::windows::core::Interface for MessageReceivedHandler {
    type Vtable = MessageReceivedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab0782_f6e2_4675_a045_d8e320c24808);
}
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct MessageReceivedHandler_box<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const MessageReceivedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::core::Result<()> + 'static> MessageReceivedHandler_box<F> {
    const VTABLE: MessageReceivedHandler_abi = MessageReceivedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <ProximityDevice as ::windows::core::Abi>::Abi as *const <ProximityDevice as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <ProximityMessage as ::windows::core::Abi>::Abi as *const <ProximityMessage as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MessageTransmittedHandler(::windows::core::IUnknown);
impl MessageTransmittedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = MessageTransmittedHandler_box::<F> { vtable: &MessageTransmittedHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ProximityDevice>>(&self, sender: Param0, messageid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), messageid).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MessageTransmittedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({efaa0b4a-f6e2-4d7d-856c-78fc8efc021e})");
}
unsafe impl ::windows::core::Interface for MessageTransmittedHandler {
    type Vtable = MessageTransmittedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefaa0b4a_f6e2_4d7d_856c_78fc8efc021e);
}
#[repr(C)]
#[doc(hidden)]
pub struct MessageTransmittedHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, messageid: i64) -> ::windows::core::HRESULT);
#[repr(C)]
struct MessageTransmittedHandler_box<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::core::Result<()> + 'static> {
    vtable: *const MessageTransmittedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::core::Result<()> + 'static> MessageTransmittedHandler_box<F> {
    const VTABLE: MessageTransmittedHandler_abi = MessageTransmittedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, messageid: i64) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <ProximityDevice as ::windows::core::Abi>::Abi as *const <ProximityDevice as ::windows::core::DefaultType>::DefaultType), messageid).into()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PeerDiscoveryTypes(pub u32);
impl PeerDiscoveryTypes {
    pub const None: PeerDiscoveryTypes = PeerDiscoveryTypes(0u32);
    pub const Browse: PeerDiscoveryTypes = PeerDiscoveryTypes(1u32);
    pub const Triggered: PeerDiscoveryTypes = PeerDiscoveryTypes(2u32);
}
impl ::core::convert::From<u32> for PeerDiscoveryTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PeerDiscoveryTypes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PeerDiscoveryTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerDiscoveryTypes;u4)");
}
impl ::windows::core::DefaultType for PeerDiscoveryTypes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PeerDiscoveryTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PeerDiscoveryTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PeerDiscoveryTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PeerDiscoveryTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PeerDiscoveryTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub struct PeerFinder {}
impl PeerFinder {
    pub fn AllowBluetooth() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetAllowBluetooth(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn AllowInfrastructure() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetAllowInfrastructure(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn AllowWiFiDirect() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetAllowWiFiDirect(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn DisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn SupportedDiscoveryTypes() -> ::windows::core::Result<PeerDiscoveryTypes> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: PeerDiscoveryTypes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerDiscoveryTypes>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateIdentities() -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        })
    }
    pub fn Start() -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn StartWithMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(peermessage: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), peermessage.into_param().abi()).ok() })
    }
    pub fn Stop() -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn TriggeredConnectionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TriggeredConnectionStateChangedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTriggeredConnectionStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ConnectionRequestedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllPeersAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, PeerInformation>>(peerinformation: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), peerinformation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>>(result__)
        })
    }
    pub fn Role() -> ::windows::core::Result<PeerRole> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: PeerRole = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerRole>(result__)
        })
    }
    pub fn SetRole(value: PeerRole) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscoveryData() -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDiscoveryData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn CreateWatcher() -> ::windows::core::Result<PeerWatcher> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerWatcher>(result__)
        })
    }
    pub fn IPeerFinderStatics<R, F: FnOnce(&IPeerFinderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PeerFinder, IPeerFinderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPeerFinderStatics2<R, F: FnOnce(&IPeerFinderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PeerFinder, IPeerFinderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PeerFinder {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerFinder";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PeerInformation(pub ::windows::core::IInspectable);
impl PeerInformation {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscoveryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn HostName(&self) -> ::windows::core::Result<super::HostName> {
        let this = &::windows::core::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    pub fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PeerInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.PeerInformation;{20024f08-9fff-45f4-b6e9-408b2ebef373})");
}
unsafe impl ::windows::core::Interface for PeerInformation {
    type Vtable = IPeerInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20024f08_9fff_45f4_b6e9_408b2ebef373);
}
impl ::windows::core::RuntimeName for PeerInformation {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerInformation";
}
impl ::core::convert::From<PeerInformation> for ::windows::core::IUnknown {
    fn from(value: PeerInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PeerInformation> for ::windows::core::IUnknown {
    fn from(value: &PeerInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PeerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PeerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PeerInformation> for ::windows::core::IInspectable {
    fn from(value: PeerInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PeerInformation> for ::windows::core::IInspectable {
    fn from(value: &PeerInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PeerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PeerInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PeerInformation {}
unsafe impl ::core::marker::Sync for PeerInformation {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PeerRole(pub i32);
impl PeerRole {
    pub const Peer: PeerRole = PeerRole(0i32);
    pub const Host: PeerRole = PeerRole(1i32);
    pub const Client: PeerRole = PeerRole(2i32);
}
impl ::core::convert::From<i32> for PeerRole {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PeerRole {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PeerRole {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerRole;i4)");
}
impl ::windows::core::DefaultType for PeerRole {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PeerWatcher(pub ::windows::core::IInspectable);
impl PeerWatcher {
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<PeerWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: PeerWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PeerWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.PeerWatcher;{3cee21f8-2fa6-4679-9691-03c94a420f34})");
}
unsafe impl ::windows::core::Interface for PeerWatcher {
    type Vtable = IPeerWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cee21f8_2fa6_4679_9691_03c94a420f34);
}
impl ::windows::core::RuntimeName for PeerWatcher {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerWatcher";
}
impl ::core::convert::From<PeerWatcher> for ::windows::core::IUnknown {
    fn from(value: PeerWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PeerWatcher> for ::windows::core::IUnknown {
    fn from(value: &PeerWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PeerWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PeerWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PeerWatcher> for ::windows::core::IInspectable {
    fn from(value: PeerWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PeerWatcher> for ::windows::core::IInspectable {
    fn from(value: &PeerWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PeerWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PeerWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PeerWatcher {}
unsafe impl ::core::marker::Sync for PeerWatcher {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PeerWatcherStatus(pub i32);
impl PeerWatcherStatus {
    pub const Created: PeerWatcherStatus = PeerWatcherStatus(0i32);
    pub const Started: PeerWatcherStatus = PeerWatcherStatus(1i32);
    pub const EnumerationCompleted: PeerWatcherStatus = PeerWatcherStatus(2i32);
    pub const Stopping: PeerWatcherStatus = PeerWatcherStatus(3i32);
    pub const Stopped: PeerWatcherStatus = PeerWatcherStatus(4i32);
    pub const Aborted: PeerWatcherStatus = PeerWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for PeerWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PeerWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PeerWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for PeerWatcherStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximityDevice(pub ::windows::core::IInspectable);
impl ProximityDevice {
    pub fn SubscribeForMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, MessageReceivedHandler>>(&self, messagetype: Param0, messagereceivedhandler: Param1) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), messagereceivedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn PublishMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, messagetype: Param0, message: Param1) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn PublishMessageWithCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, MessageTransmittedHandler>>(&self, messagetype: Param0, message: Param1, messagetransmittedhandler: Param2) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishBinaryMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, messagetype: Param0, message: Param1) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishBinaryMessageWithCallback<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, MessageTransmittedHandler>>(&self, messagetype: Param0, message: Param1, messagetransmittedhandler: Param2) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PublishUriMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, message: Param0) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PublishUriMessageWithCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, MessageTransmittedHandler>>(&self, message: Param0, messagetransmittedhandler: Param1) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn StopSubscribingForMessage(&self, subscriptionid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), subscriptionid).ok() }
    }
    pub fn StopPublishingMessage(&self, messageid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), messageid).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeviceArrived<'a, Param0: ::windows::core::IntoParam<'a, DeviceArrivedEventHandler>>(&self, arrivedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), arrivedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeviceDeparted<'a, Param0: ::windows::core::IntoParam<'a, DeviceDepartedEventHandler>>(&self, departedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), departedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceDeparted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn MaxMessageBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn BitsPerSecond(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDefault() -> ::windows::core::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProximityDevice>(result__)
        })
    }
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<ProximityDevice>(result__)
        })
    }
    pub fn IProximityDeviceStatics<R, F: FnOnce(&IProximityDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ProximityDevice, IProximityDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ProximityDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ProximityDevice;{efa8a552-f6e1-4329-a0fc-ab6b0fd28262})");
}
unsafe impl ::windows::core::Interface for ProximityDevice {
    type Vtable = IProximityDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8a552_f6e1_4329_a0fc_ab6b0fd28262);
}
impl ::windows::core::RuntimeName for ProximityDevice {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityDevice";
}
impl ::core::convert::From<ProximityDevice> for ::windows::core::IUnknown {
    fn from(value: ProximityDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProximityDevice> for ::windows::core::IUnknown {
    fn from(value: &ProximityDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximityDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximityDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProximityDevice> for ::windows::core::IInspectable {
    fn from(value: ProximityDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProximityDevice> for ::windows::core::IInspectable {
    fn from(value: &ProximityDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximityDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximityDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProximityDevice {}
unsafe impl ::core::marker::Sync for ProximityDevice {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximityMessage(pub ::windows::core::IInspectable);
impl ProximityMessage {
    pub fn MessageType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SubscriptionId(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn DataAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProximityMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ProximityMessage;{efab0782-f6e1-4675-a045-d8e320c24808})");
}
unsafe impl ::windows::core::Interface for ProximityMessage {
    type Vtable = IProximityMessage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab0782_f6e1_4675_a045_d8e320c24808);
}
impl ::windows::core::RuntimeName for ProximityMessage {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityMessage";
}
impl ::core::convert::From<ProximityMessage> for ::windows::core::IUnknown {
    fn from(value: ProximityMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProximityMessage> for ::windows::core::IUnknown {
    fn from(value: &ProximityMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProximityMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProximityMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProximityMessage> for ::windows::core::IInspectable {
    fn from(value: ProximityMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProximityMessage> for ::windows::core::IInspectable {
    fn from(value: &ProximityMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProximityMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProximityMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProximityMessage {}
unsafe impl ::core::marker::Sync for ProximityMessage {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TriggeredConnectState(pub i32);
impl TriggeredConnectState {
    pub const PeerFound: TriggeredConnectState = TriggeredConnectState(0i32);
    pub const Listening: TriggeredConnectState = TriggeredConnectState(1i32);
    pub const Connecting: TriggeredConnectState = TriggeredConnectState(2i32);
    pub const Completed: TriggeredConnectState = TriggeredConnectState(3i32);
    pub const Canceled: TriggeredConnectState = TriggeredConnectState(4i32);
    pub const Failed: TriggeredConnectState = TriggeredConnectState(5i32);
}
impl ::core::convert::From<i32> for TriggeredConnectState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TriggeredConnectState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TriggeredConnectState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.TriggeredConnectState;i4)");
}
impl ::windows::core::DefaultType for TriggeredConnectState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TriggeredConnectionStateChangedEventArgs(pub ::windows::core::IInspectable);
impl TriggeredConnectionStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<TriggeredConnectState> {
        let this = self;
        unsafe {
            let mut result__: TriggeredConnectState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TriggeredConnectState>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn Socket(&self) -> ::windows::core::Result<super::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Sockets::StreamSocket>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TriggeredConnectionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs;{c6a780ad-f6e1-4d54-96e2-33f620bca88a})");
}
unsafe impl ::windows::core::Interface for TriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6a780ad_f6e1_4d54_96e2_33f620bca88a);
}
impl ::windows::core::RuntimeName for TriggeredConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs";
}
impl ::core::convert::From<TriggeredConnectionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TriggeredConnectionStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TriggeredConnectionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TriggeredConnectionStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TriggeredConnectionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TriggeredConnectionStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TriggeredConnectionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TriggeredConnectionStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TriggeredConnectionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for TriggeredConnectionStateChangedEventArgs {}
