#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ConnectionRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl ConnectionRequestedEventArgs {
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn PeerInformation(&self) -> ::windows::runtime::Result<PeerInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerInformation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ConnectionRequestedEventArgs;{eb6891ae-4f1e-4c66-bd0d-46924a942e08})");
}
unsafe impl ::windows::runtime::Interface for ConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3949498798, 20254, 19558, [189, 13, 70, 146, 74, 148, 46, 8]);
}
impl ::windows::runtime::RuntimeName for ConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.ConnectionRequestedEventArgs";
}
impl ::core::convert::From<ConnectionRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ConnectionRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ConnectionRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ConnectionRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ConnectionRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ConnectionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ConnectionRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ConnectionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ConnectionRequestedEventArgs {}
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceArrivedEventHandler(::windows::runtime::IUnknown);
impl DeviceArrivedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DeviceArrivedEventHandler_box::<F> {
            vtable: &DeviceArrivedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ProximityDevice>>(&self, sender: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceArrivedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({efa9da69-f6e1-49c9-a49e-8e0fc58fb911})");
}
unsafe impl ::windows::runtime::Interface for DeviceArrivedEventHandler {
    type Vtable = DeviceArrivedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020886121, 63201, 18889, [164, 158, 142, 15, 197, 143, 185, 17]);
}
#[repr(C)]
#[doc(hidden)]
pub struct DeviceArrivedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DeviceArrivedEventHandler_box<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DeviceArrivedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::runtime::Result<()> + 'static> DeviceArrivedEventHandler_box<F> {
    const VTABLE: DeviceArrivedEventHandler_abi = DeviceArrivedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DeviceArrivedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <ProximityDevice as ::windows::runtime::Abi>::Abi as *const <ProximityDevice as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceDepartedEventHandler(::windows::runtime::IUnknown);
impl DeviceDepartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DeviceDepartedEventHandler_box::<F> {
            vtable: &DeviceDepartedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ProximityDevice>>(&self, sender: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceDepartedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({efa9da69-f6e2-49c9-a49e-8e0fc58fb911})");
}
unsafe impl ::windows::runtime::Interface for DeviceDepartedEventHandler {
    type Vtable = DeviceDepartedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020886121, 63202, 18889, [164, 158, 142, 15, 197, 143, 185, 17]);
}
#[repr(C)]
#[doc(hidden)]
pub struct DeviceDepartedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DeviceDepartedEventHandler_box<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DeviceDepartedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>) -> ::windows::runtime::Result<()> + 'static> DeviceDepartedEventHandler_box<F> {
    const VTABLE: DeviceDepartedEventHandler_abi = DeviceDepartedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DeviceDepartedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <ProximityDevice as ::windows::runtime::Abi>::Abi as *const <ProximityDevice as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IConnectionRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3949498798, 20254, 19558, [189, 13, 70, 146, 74, 148, 46, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerFinderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPeerFinderStatics {
    type Vtable = IPeerFinderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2437626721, 63201, 18372, [161, 76, 20, 138, 25, 3, 208, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFinderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PeerDiscoveryTypes) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peermessage: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peerinformation: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerFinderStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPeerFinderStatics2 {
    type Vtable = IPeerFinderStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3605478501, 64976, 19211, [147, 18, 134, 100, 8, 147, 93, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFinderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PeerRole) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PeerRole) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerInformation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPeerInformation {
    type Vtable = IPeerInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(537022216, 40959, 17908, [182, 233, 64, 139, 46, 190, 243, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerInformation3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPeerInformation3 {
    type Vtable = IPeerInformation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2987352362, 56272, 16632, [149, 189, 45, 66, 9, 199, 131, 111]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformation3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerInformationWithHostAndService(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPeerInformationWithHostAndService {
    type Vtable = IPeerInformationWithHostAndService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3972517037, 7024, 20107, [146, 219, 187, 231, 129, 65, 147, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformationWithHostAndService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPeerWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPeerWatcher {
    type Vtable = IPeerWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1022239224, 12198, 18041, [150, 145, 3, 201, 74, 66, 15, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PeerWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximityDevice(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProximityDevice {
    type Vtable = IProximityDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020806994, 63201, 17193, [160, 252, 171, 107, 15, 210, 130, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, messagereceivedhandler: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, messagetransmittedhandler: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, message: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, message: ::windows::runtime::RawPtr, messagetransmittedhandler: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, messagetransmittedhandler: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subscriptionid: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messageid: i64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, arrivedhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, departedhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximityDeviceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProximityDeviceStatics {
    type Vtable = IProximityDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2437652509, 63201, 18372, [161, 76, 20, 138, 25, 3, 208, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProximityMessage(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProximityMessage {
    type Vtable = IProximityMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020963202, 63201, 18037, [160, 69, 216, 227, 32, 194, 72, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITriggeredConnectionStateChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3332866221, 63201, 19796, [150, 226, 51, 246, 32, 188, 168, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggeredConnectionStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TriggeredConnectState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
);
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MessageReceivedHandler(::windows::runtime::IUnknown);
impl MessageReceivedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = MessageReceivedHandler_box::<F> {
            vtable: &MessageReceivedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ProximityDevice>, Param1: ::windows::runtime::IntoParam<'a, ProximityMessage>>(&self, sender: Param0, message: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), message.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MessageReceivedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({efab0782-f6e2-4675-a045-d8e320c24808})");
}
unsafe impl ::windows::runtime::Interface for MessageReceivedHandler {
    type Vtable = MessageReceivedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020963202, 63202, 18037, [160, 69, 216, 227, 32, 194, 72, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct MessageReceivedHandler_box<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const MessageReceivedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>, &::core::option::Option<ProximityMessage>) -> ::windows::runtime::Result<()> + 'static> MessageReceivedHandler_box<F> {
    const VTABLE: MessageReceivedHandler_abi = MessageReceivedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<MessageReceivedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <ProximityDevice as ::windows::runtime::Abi>::Abi as *const <ProximityDevice as ::windows::runtime::DefaultType>::DefaultType), &*(&message as *const <ProximityMessage as ::windows::runtime::Abi>::Abi as *const <ProximityMessage as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MessageTransmittedHandler(::windows::runtime::IUnknown);
impl MessageTransmittedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = MessageTransmittedHandler_box::<F> {
            vtable: &MessageTransmittedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ProximityDevice>>(&self, sender: Param0, messageid: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), messageid).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MessageTransmittedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({efaa0b4a-f6e2-4d7d-856c-78fc8efc021e})");
}
unsafe impl ::windows::runtime::Interface for MessageTransmittedHandler {
    type Vtable = MessageTransmittedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020898634, 63202, 19837, [133, 108, 120, 252, 142, 252, 2, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct MessageTransmittedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, messageid: i64) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct MessageTransmittedHandler_box<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const MessageTransmittedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<ProximityDevice>, i64) -> ::windows::runtime::Result<()> + 'static> MessageTransmittedHandler_box<F> {
    const VTABLE: MessageTransmittedHandler_abi = MessageTransmittedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<MessageTransmittedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, messageid: i64) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <ProximityDevice as ::windows::runtime::Abi>::Abi as *const <ProximityDevice as ::windows::runtime::DefaultType>::DefaultType), messageid).into()
    }
}
#[doc = "*Required features: `Networking_Proximity`*"]
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
unsafe impl ::windows::runtime::Abi for PeerDiscoveryTypes {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PeerDiscoveryTypes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerDiscoveryTypes;u4)");
}
impl ::windows::runtime::DefaultType for PeerDiscoveryTypes {
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
#[doc = "*Required features: `Networking_Proximity`*"]
pub struct PeerFinder {}
impl PeerFinder {
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn AllowBluetooth() -> ::windows::runtime::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn SetAllowBluetooth(value: bool) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn AllowInfrastructure() -> ::windows::runtime::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn SetAllowInfrastructure(value: bool) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn AllowWiFiDirect() -> ::windows::runtime::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn SetAllowWiFiDirect(value: bool) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn DisplayName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn SupportedDiscoveryTypes() -> ::windows::runtime::Result<PeerDiscoveryTypes> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: PeerDiscoveryTypes = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerDiscoveryTypes>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation_Collections`*"]
    pub fn AlternateIdentities() -> ::windows::runtime::Result<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Start() -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn StartWithMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(peermessage: Param0) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), peermessage.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Stop() -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn TriggeredConnectionStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::runtime::IInspectable, TriggeredConnectionStateChangedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveTriggeredConnectionStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn ConnectionRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::runtime::IInspectable, ConnectionRequestedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllPeersAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`, `Networking_Sockets`*"]
    pub fn ConnectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, PeerInformation>>(peerinformation: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), peerinformation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Role() -> ::windows::runtime::Result<PeerRole> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: PeerRole = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerRole>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn SetRole(value: PeerRole) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_Proximity`, `Storage_Streams`*"]
    pub fn DiscoveryData() -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_Proximity`, `Storage_Streams`*"]
    pub fn SetDiscoveryData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IPeerFinderStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn CreateWatcher() -> ::windows::runtime::Result<PeerWatcher> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerWatcher>(result__)
        })
    }
    pub fn IPeerFinderStatics<R, F: FnOnce(&IPeerFinderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PeerFinder, IPeerFinderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPeerFinderStatics2<R, F: FnOnce(&IPeerFinderStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PeerFinder, IPeerFinderStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PeerFinder {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerFinder";
}
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PeerInformation(pub ::windows::runtime::IInspectable);
impl PeerInformation {
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_Proximity`, `Storage_Streams`*"]
    pub fn DiscoveryData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn HostName(&self) -> ::windows::runtime::Result<super::HostName> {
        let this = &::windows::runtime::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn ServiceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PeerInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.PeerInformation;{20024f08-9fff-45f4-b6e9-408b2ebef373})");
}
unsafe impl ::windows::runtime::Interface for PeerInformation {
    type Vtable = IPeerInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(537022216, 40959, 17908, [182, 233, 64, 139, 46, 190, 243, 115]);
}
impl ::windows::runtime::RuntimeName for PeerInformation {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerInformation";
}
impl ::core::convert::From<PeerInformation> for ::windows::runtime::IUnknown {
    fn from(value: PeerInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PeerInformation> for ::windows::runtime::IUnknown {
    fn from(value: &PeerInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PeerInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PeerInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PeerInformation> for ::windows::runtime::IInspectable {
    fn from(value: PeerInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PeerInformation> for ::windows::runtime::IInspectable {
    fn from(value: &PeerInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PeerInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PeerInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PeerInformation {}
unsafe impl ::core::marker::Sync for PeerInformation {}
#[doc = "*Required features: `Networking_Proximity`*"]
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
unsafe impl ::windows::runtime::Abi for PeerRole {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PeerRole {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerRole;i4)");
}
impl ::windows::runtime::DefaultType for PeerRole {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PeerWatcher(pub ::windows::runtime::IInspectable);
impl PeerWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn Updated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PeerWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: PeerWatcherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PeerWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PeerWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.PeerWatcher;{3cee21f8-2fa6-4679-9691-03c94a420f34})");
}
unsafe impl ::windows::runtime::Interface for PeerWatcher {
    type Vtable = IPeerWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1022239224, 12198, 18041, [150, 145, 3, 201, 74, 66, 15, 52]);
}
impl ::windows::runtime::RuntimeName for PeerWatcher {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerWatcher";
}
impl ::core::convert::From<PeerWatcher> for ::windows::runtime::IUnknown {
    fn from(value: PeerWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PeerWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &PeerWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PeerWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PeerWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PeerWatcher> for ::windows::runtime::IInspectable {
    fn from(value: PeerWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PeerWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &PeerWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PeerWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PeerWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PeerWatcher {}
unsafe impl ::core::marker::Sync for PeerWatcher {}
#[doc = "*Required features: `Networking_Proximity`*"]
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
unsafe impl ::windows::runtime::Abi for PeerWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PeerWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for PeerWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximityDevice(pub ::windows::runtime::IInspectable);
impl ProximityDevice {
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn SubscribeForMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, MessageReceivedHandler>>(&self, messagetype: Param0, messagereceivedhandler: Param1) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), messagereceivedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn PublishMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, messagetype: Param0, message: Param1) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn PublishMessageWithCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, MessageTransmittedHandler>>(&self, messagetype: Param0, message: Param1, messagetransmittedhandler: Param2) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_Proximity`, `Storage_Streams`*"]
    pub fn PublishBinaryMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, messagetype: Param0, message: Param1) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_Proximity`, `Storage_Streams`*"]
    pub fn PublishBinaryMessageWithCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, MessageTransmittedHandler>>(&self, messagetype: Param0, message: Param1, messagetransmittedhandler: Param2) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), messagetype.into_param().abi(), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn PublishUriMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, message: Param0) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn PublishUriMessageWithCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, MessageTransmittedHandler>>(&self, message: Param0, messagetransmittedhandler: Param1) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), message.into_param().abi(), messagetransmittedhandler.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn StopSubscribingForMessage(&self, subscriptionid: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), subscriptionid).ok() }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn StopPublishingMessage(&self, messageid: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), messageid).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn DeviceArrived<'a, Param0: ::windows::runtime::IntoParam<'a, DeviceArrivedEventHandler>>(&self, arrivedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), arrivedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveDeviceArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn DeviceDeparted<'a, Param0: ::windows::runtime::IntoParam<'a, DeviceDepartedEventHandler>>(&self, departedhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), departedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_Proximity`, `Foundation`*"]
    pub fn RemoveDeviceDeparted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn MaxMessageBytes(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn BitsPerSecond(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ProximityDevice>(result__)
        })
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn FromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<ProximityDevice>(result__)
        })
    }
    pub fn IProximityDeviceStatics<R, F: FnOnce(&IProximityDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProximityDevice, IProximityDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProximityDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ProximityDevice;{efa8a552-f6e1-4329-a0fc-ab6b0fd28262})");
}
unsafe impl ::windows::runtime::Interface for ProximityDevice {
    type Vtable = IProximityDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020806994, 63201, 17193, [160, 252, 171, 107, 15, 210, 130, 98]);
}
impl ::windows::runtime::RuntimeName for ProximityDevice {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityDevice";
}
impl ::core::convert::From<ProximityDevice> for ::windows::runtime::IUnknown {
    fn from(value: ProximityDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProximityDevice> for ::windows::runtime::IUnknown {
    fn from(value: &ProximityDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProximityDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProximityDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProximityDevice> for ::windows::runtime::IInspectable {
    fn from(value: ProximityDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProximityDevice> for ::windows::runtime::IInspectable {
    fn from(value: &ProximityDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProximityDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProximityDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProximityDevice {}
unsafe impl ::core::marker::Sync for ProximityDevice {}
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProximityMessage(pub ::windows::runtime::IInspectable);
impl ProximityMessage {
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn SubscriptionId(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_Proximity`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn DataAsString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProximityMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ProximityMessage;{efab0782-f6e1-4675-a045-d8e320c24808})");
}
unsafe impl ::windows::runtime::Interface for ProximityMessage {
    type Vtable = IProximityMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020963202, 63201, 18037, [160, 69, 216, 227, 32, 194, 72, 8]);
}
impl ::windows::runtime::RuntimeName for ProximityMessage {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityMessage";
}
impl ::core::convert::From<ProximityMessage> for ::windows::runtime::IUnknown {
    fn from(value: ProximityMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProximityMessage> for ::windows::runtime::IUnknown {
    fn from(value: &ProximityMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProximityMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProximityMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProximityMessage> for ::windows::runtime::IInspectable {
    fn from(value: ProximityMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProximityMessage> for ::windows::runtime::IInspectable {
    fn from(value: &ProximityMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProximityMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProximityMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProximityMessage {}
unsafe impl ::core::marker::Sync for ProximityMessage {}
#[doc = "*Required features: `Networking_Proximity`*"]
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
unsafe impl ::windows::runtime::Abi for TriggeredConnectState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TriggeredConnectState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.TriggeredConnectState;i4)");
}
impl ::windows::runtime::DefaultType for TriggeredConnectState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_Proximity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TriggeredConnectionStateChangedEventArgs(pub ::windows::runtime::IInspectable);
impl TriggeredConnectionStateChangedEventArgs {
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn State(&self) -> ::windows::runtime::Result<TriggeredConnectState> {
        let this = self;
        unsafe {
            let mut result__: TriggeredConnectState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TriggeredConnectState>(result__)
        }
    }
    #[doc = "*Required features: `Networking_Proximity`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    #[doc = "*Required features: `Networking_Proximity`, `Networking_Sockets`*"]
    pub fn Socket(&self) -> ::windows::runtime::Result<super::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Sockets::StreamSocket>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TriggeredConnectionStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs;{c6a780ad-f6e1-4d54-96e2-33f620bca88a})");
}
unsafe impl ::windows::runtime::Interface for TriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3332866221, 63201, 19796, [150, 226, 51, 246, 32, 188, 168, 138]);
}
impl ::windows::runtime::RuntimeName for TriggeredConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs";
}
impl ::core::convert::From<TriggeredConnectionStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: TriggeredConnectionStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TriggeredConnectionStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &TriggeredConnectionStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TriggeredConnectionStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: TriggeredConnectionStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TriggeredConnectionStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &TriggeredConnectionStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TriggeredConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TriggeredConnectionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for TriggeredConnectionStateChangedEventArgs {}
