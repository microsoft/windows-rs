#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CurrentTimeChangeRequestedEventArgs(::windows::runtime::IInspectable);
impl CurrentTimeChangeRequestedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn Time(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CurrentTimeChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs;{99711324-edc7-4bf5-91f6-3c8627db59e5})");
}
unsafe impl ::windows::runtime::Interface for CurrentTimeChangeRequestedEventArgs {
    type Vtable = ICurrentTimeChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2574324516, 60871, 19445, [145, 246, 60, 134, 39, 219, 89, 229]);
}
impl ::windows::runtime::RuntimeName for CurrentTimeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICurrentTimeChangeRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICurrentTimeChangeRequestedEventArgs {
    type Vtable = ICurrentTimeChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2574324516, 60871, 19445, [145, 246, 60, 134, 39, 219, 89, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentTimeChangeRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMuteChangeRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMuteChangeRequestedEventArgs {
    type Vtable = IMuteChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3837064694, 44831, 20254, [180, 55, 125, 163, 36, 0, 225, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToConnection {
    type Vtable = IPlayToConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(288341960, 62005, 20446, [141, 65, 155, 242, 124, 158, 154, 64]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlayToConnectionState) -> ::windows::runtime::HRESULT,
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
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToConnectionErrorEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToConnectionErrorEventArgs {
    type Vtable = IPlayToConnectionErrorEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3210653094, 35046, 17503, [157, 64, 217, 185, 248, 147, 152, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionErrorEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlayToConnectionError) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToConnectionStateChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToConnectionStateChangedEventArgs {
    type Vtable = IPlayToConnectionStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1757721871, 3104, 18816, [134, 2, 88, 198, 34, 56, 212, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlayToConnectionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlayToConnectionState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToConnectionTransferredEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToConnectionTransferredEventArgs {
    type Vtable = IPlayToConnectionTransferredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4209187130, 1667, 18393, [141, 240, 24, 203, 180, 137, 132, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionTransferredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToManager {
    type Vtable = IPlayToManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4117373038, 7031, 17135, [143, 13, 185, 73, 248, 217, 178, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManager_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToManagerStatics {
    type Vtable = IPlayToManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1692838023, 14722, 20283, [186, 32, 97, 85, 228, 53, 50, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToReceiver(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToReceiver {
    type Vtable = IPlayToReceiver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2887110471, 41314, 19110, [175, 27, 58, 163, 95, 59, 144, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToReceiver_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volume: f64, mute: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rate: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currenttime: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, duration: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToSource {
    type Vtable = IPlayToSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2131986952, 64439, 19209, [131, 86, 170, 95, 78, 51, 92, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToSourceDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToSourceDeferral {
    type Vtable = IPlayToSourceDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1090554141, 10126, 20265, [133, 155, 169, 229, 1, 5, 62, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToSourceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToSourceRequest {
    type Vtable = IPlayToSourceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4166534757, 25844, 17568, [172, 13, 70, 141, 43, 143, 218, 131]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errorstring: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToSourceRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToSourceRequestedEventArgs {
    type Vtable = IPlayToSourceRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3318596400, 10719, 20166, [157, 169, 159, 189, 252, 252, 27, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToSourceSelectedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToSourceSelectedEventArgs {
    type Vtable = IPlayToSourceSelectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(211649809, 20994, 19915, [140, 103, 171, 218, 18, 187, 60, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceSelectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlayToSourceWithPreferredSourceUri(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayToSourceWithPreferredSourceUri {
    type Vtable = IPlayToSourceWithPreferredSourceUri_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2863813611, 13057, 19908, [175, 186, 178, 242, 237, 150, 53, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceWithPreferredSourceUri_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(257319342, 11400, 19658, [133, 64, 213, 134, 9, 93, 19, 165]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISourceChangeRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISourceChangeRequestedEventArgs {
    type Vtable = ISourceChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4215224982, 31398, 19083, [134, 231, 84, 246, 198, 211, 79, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceChangeRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IVolumeChangeRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVolumeChangeRequestedEventArgs {
    type Vtable = IVolumeChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1862430044, 53109, 19499, [145, 62, 109, 124, 108, 50, 145, 121]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeChangeRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MuteChangeRequestedEventArgs(::windows::runtime::IInspectable);
impl MuteChangeRequestedEventArgs {
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Mute(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MuteChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.MuteChangeRequestedEventArgs;{e4b4f5f6-af1f-4f1e-b437-7da32400e1d4})");
}
unsafe impl ::windows::runtime::Interface for MuteChangeRequestedEventArgs {
    type Vtable = IMuteChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3837064694, 44831, 20254, [180, 55, 125, 163, 36, 0, 225, 212]);
}
impl ::windows::runtime::RuntimeName for MuteChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.MuteChangeRequestedEventArgs";
}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToConnection(::windows::runtime::IInspectable);
impl PlayToConnection {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn State(&self) -> ::windows::runtime::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__: PlayToConnectionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnectionState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn Transferred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveTransferred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn Error<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveError<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnection;{112fbfc8-f235-4fde-8d41-9bf27c9e9a40})");
}
unsafe impl ::windows::runtime::Interface for PlayToConnection {
    type Vtable = IPlayToConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(288341960, 62005, 20446, [141, 65, 155, 242, 124, 158, 154, 64]);
}
impl ::windows::runtime::RuntimeName for PlayToConnection {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnection";
}
unsafe impl ::std::marker::Send for PlayToConnection {}
unsafe impl ::std::marker::Sync for PlayToConnection {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayToConnectionError(pub i32);
impl PlayToConnectionError {
    pub const None: PlayToConnectionError = PlayToConnectionError(0i32);
    pub const DeviceNotResponding: PlayToConnectionError = PlayToConnectionError(1i32);
    pub const DeviceError: PlayToConnectionError = PlayToConnectionError(2i32);
    pub const DeviceLocked: PlayToConnectionError = PlayToConnectionError(3i32);
    pub const ProtectedPlaybackFailed: PlayToConnectionError = PlayToConnectionError(4i32);
}
impl ::std::convert::From<i32> for PlayToConnectionError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlayToConnectionError {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlayToConnectionError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionError;i4)");
}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToConnectionErrorEventArgs(::windows::runtime::IInspectable);
impl PlayToConnectionErrorEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Code(&self) -> ::windows::runtime::Result<PlayToConnectionError> {
        let this = self;
        unsafe {
            let mut result__: PlayToConnectionError = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnectionError>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToConnectionErrorEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionErrorEventArgs;{bf5eada6-88e6-445f-9d40-d9b9f8939896})");
}
unsafe impl ::windows::runtime::Interface for PlayToConnectionErrorEventArgs {
    type Vtable = IPlayToConnectionErrorEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3210653094, 35046, 17503, [157, 64, 217, 185, 248, 147, 152, 150]);
}
impl ::windows::runtime::RuntimeName for PlayToConnectionErrorEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionErrorEventArgs";
}
unsafe impl ::std::marker::Send for PlayToConnectionErrorEventArgs {}
unsafe impl ::std::marker::Sync for PlayToConnectionErrorEventArgs {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayToConnectionState(pub i32);
impl PlayToConnectionState {
    pub const Disconnected: PlayToConnectionState = PlayToConnectionState(0i32);
    pub const Connected: PlayToConnectionState = PlayToConnectionState(1i32);
    pub const Rendering: PlayToConnectionState = PlayToConnectionState(2i32);
}
impl ::std::convert::From<i32> for PlayToConnectionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlayToConnectionState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlayToConnectionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionState;i4)");
}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToConnectionStateChangedEventArgs(::windows::runtime::IInspectable);
impl PlayToConnectionStateChangedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn PreviousState(&self) -> ::windows::runtime::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__: PlayToConnectionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnectionState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn CurrentState(&self) -> ::windows::runtime::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__: PlayToConnectionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnectionState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToConnectionStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs;{68c4b50f-0c20-4980-8602-58c62238d423})");
}
unsafe impl ::windows::runtime::Interface for PlayToConnectionStateChangedEventArgs {
    type Vtable = IPlayToConnectionStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1757721871, 3104, 18816, [134, 2, 88, 198, 34, 56, 212, 35]);
}
impl ::windows::runtime::RuntimeName for PlayToConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs";
}
unsafe impl ::std::marker::Send for PlayToConnectionStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for PlayToConnectionStateChangedEventArgs {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToConnectionTransferredEventArgs(::windows::runtime::IInspectable);
impl PlayToConnectionTransferredEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn PreviousSource(&self) -> ::windows::runtime::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSource>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn CurrentSource(&self) -> ::windows::runtime::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSource>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToConnectionTransferredEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs;{fae3193a-0683-47d9-8df0-18cbb48984d8})");
}
unsafe impl ::windows::runtime::Interface for PlayToConnectionTransferredEventArgs {
    type Vtable = IPlayToConnectionTransferredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4209187130, 1667, 18393, [141, 240, 24, 203, 180, 137, 132, 216]);
}
impl ::windows::runtime::RuntimeName for PlayToConnectionTransferredEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs";
}
unsafe impl ::std::marker::Send for PlayToConnectionTransferredEventArgs {}
unsafe impl ::std::marker::Sync for PlayToConnectionTransferredEventArgs {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToManager(::windows::runtime::IInspectable);
impl PlayToManager {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn SourceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveSourceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn SourceSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveSourceSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SetDefaultSourceSelection(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn DefaultSourceSelection(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<PlayToManager> {
        Self::IPlayToManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToManager>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn ShowPlayToUI() -> ::windows::runtime::Result<()> {
        Self::IPlayToManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() })
    }
    pub fn IPlayToManagerStatics<R, F: FnOnce(&IPlayToManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayToManager, IPlayToManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToManager;{f56a206e-1b77-42ef-8f0d-b949f8d9b260})");
}
unsafe impl ::windows::runtime::Interface for PlayToManager {
    type Vtable = IPlayToManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4117373038, 7031, 17135, [143, 13, 185, 73, 248, 217, 178, 96]);
}
impl ::windows::runtime::RuntimeName for PlayToManager {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToManager";
}
unsafe impl ::std::marker::Send for PlayToManager {}
unsafe impl ::std::marker::Sync for PlayToManager {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToReceiver(::windows::runtime::IInspectable);
impl PlayToReceiver {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayToReceiver, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn PlayRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemovePlayRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn PauseRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemovePauseRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn SourceChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveSourceChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn PlaybackRateChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemovePlaybackRateChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn CurrentTimeChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveCurrentTimeChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn MuteChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveMuteChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn VolumeChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveVolumeChangeRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn TimeUpdateRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveTimeUpdateRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn StopRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn RemoveStopRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifyVolumeChange(&self, volume: f64, mute: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), volume, mute).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifyRateChange(&self, rate: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), rate).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifyLoadedMetadata(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn NotifyTimeUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, currenttime: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), currenttime.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn NotifyDurationChange<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, duration: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), duration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifySeeking(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifySeeked(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifyPaused(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifyPlaying(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifyEnded(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifyError(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn NotifyStopped(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn FriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SetSupportsImage(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SupportsImage(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SetSupportsAudio(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SupportsAudio(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SetSupportsVideo(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SupportsVideo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn StopAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToReceiver {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToReceiver;{ac15cf47-a162-4aa6-af1b-3aa35f3b9069})");
}
unsafe impl ::windows::runtime::Interface for PlayToReceiver {
    type Vtable = IPlayToReceiver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2887110471, 41314, 19110, [175, 27, 58, 163, 95, 59, 144, 105]);
}
impl ::windows::runtime::RuntimeName for PlayToReceiver {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToReceiver";
}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToSource(::windows::runtime::IInspectable);
impl PlayToSource {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<PlayToConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnection>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Next(&self) -> ::windows::runtime::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSource>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SetNext<'a, Param0: ::windows::runtime::IntoParam<'a, PlayToSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn PlayNext(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn PreferredSourceUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn SetPreferredSourceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSource;{7f138a08-fbb7-4b09-8356-aa5f4e335c31})");
}
unsafe impl ::windows::runtime::Interface for PlayToSource {
    type Vtable = IPlayToSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2131986952, 64439, 19209, [131, 86, 170, 95, 78, 51, 92, 49]);
}
impl ::windows::runtime::RuntimeName for PlayToSource {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSource";
}
unsafe impl ::std::marker::Send for PlayToSource {}
unsafe impl ::std::marker::Sync for PlayToSource {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToSourceDeferral(::windows::runtime::IInspectable);
impl PlayToSourceDeferral {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToSourceDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceDeferral;{4100891d-278e-4f29-859b-a9e501053e7d})");
}
unsafe impl ::windows::runtime::Interface for PlayToSourceDeferral {
    type Vtable = IPlayToSourceDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1090554141, 10126, 20265, [133, 155, 169, 229, 1, 5, 62, 125]);
}
impl ::windows::runtime::RuntimeName for PlayToSourceDeferral {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceDeferral";
}
unsafe impl ::std::marker::Send for PlayToSourceDeferral {}
unsafe impl ::std::marker::Sync for PlayToSourceDeferral {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToSourceRequest(::windows::runtime::IInspectable);
impl PlayToSourceRequest {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn DisplayErrorString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, errorstring: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), errorstring.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<PlayToSourceDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSourceDeferral>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, PlayToSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToSourceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceRequest;{f8584665-64f4-44a0-ac0d-468d2b8fda83})");
}
unsafe impl ::windows::runtime::Interface for PlayToSourceRequest {
    type Vtable = IPlayToSourceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4166534757, 25844, 17568, [172, 13, 70, 141, 43, 143, 218, 131]);
}
impl ::windows::runtime::RuntimeName for PlayToSourceRequest {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequest";
}
unsafe impl ::std::marker::Send for PlayToSourceRequest {}
unsafe impl ::std::marker::Sync for PlayToSourceRequest {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToSourceRequestedEventArgs(::windows::runtime::IInspectable);
impl PlayToSourceRequestedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SourceRequest(&self) -> ::windows::runtime::Result<PlayToSourceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSourceRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToSourceRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceRequestedEventArgs;{c5cdc330-29df-4ec6-9da9-9fbdfcfc1b3e})");
}
unsafe impl ::windows::runtime::Interface for PlayToSourceRequestedEventArgs {
    type Vtable = IPlayToSourceRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3318596400, 10719, 20166, [157, 169, 159, 189, 252, 252, 27, 62]);
}
impl ::windows::runtime::RuntimeName for PlayToSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequestedEventArgs";
}
unsafe impl ::std::marker::Send for PlayToSourceRequestedEventArgs {}
unsafe impl ::std::marker::Sync for PlayToSourceRequestedEventArgs {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlayToSourceSelectedEventArgs(::windows::runtime::IInspectable);
impl PlayToSourceSelectedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn FriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_PlayTo`, `Storage_Streams`*"]
    pub fn Icon(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SupportsImage(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SupportsAudio(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn SupportsVideo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayToSourceSelectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceSelectedEventArgs;{0c9d8511-5202-4dcb-8c67-abda12bb3c12})");
}
unsafe impl ::windows::runtime::Interface for PlayToSourceSelectedEventArgs {
    type Vtable = IPlayToSourceSelectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(211649809, 20994, 19915, [140, 103, 171, 218, 18, 187, 60, 18]);
}
impl ::windows::runtime::RuntimeName for PlayToSourceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceSelectedEventArgs";
}
unsafe impl ::std::marker::Send for PlayToSourceSelectedEventArgs {}
unsafe impl ::std::marker::Sync for PlayToSourceSelectedEventArgs {}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PlaybackRateChangeRequestedEventArgs(::windows::runtime::IInspectable);
impl PlaybackRateChangeRequestedEventArgs {
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Rate(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs;{0f5661ae-2c88-4cca-8540-d586095d13a5})");
}
unsafe impl ::windows::runtime::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(257319342, 11400, 19658, [133, 64, 213, 134, 9, 93, 19, 165]);
}
impl ::windows::runtime::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs";
}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SourceChangeRequestedEventArgs(::windows::runtime::IInspectable);
impl SourceChangeRequestedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_PlayTo`, `Storage_Streams`*"]
    pub fn Stream(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Author(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Album(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Genre(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn Date(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_PlayTo`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation`*"]
    pub fn Rating(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_PlayTo`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SourceChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.SourceChangeRequestedEventArgs;{fb3f3a96-7aa6-4a8b-86e7-54f6c6d34f64})");
}
unsafe impl ::windows::runtime::Interface for SourceChangeRequestedEventArgs {
    type Vtable = ISourceChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4215224982, 31398, 19083, [134, 231, 84, 246, 198, 211, 79, 100]);
}
impl ::windows::runtime::RuntimeName for SourceChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.SourceChangeRequestedEventArgs";
}
#[doc = "*Required features: `Media_PlayTo`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct VolumeChangeRequestedEventArgs(::windows::runtime::IInspectable);
impl VolumeChangeRequestedEventArgs {
    #[doc = "*Required features: `Media_PlayTo`*"]
    pub fn Volume(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VolumeChangeRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.VolumeChangeRequestedEventArgs;{6f026d5c-cf75-4c2b-913e-6d7c6c329179})");
}
unsafe impl ::windows::runtime::Interface for VolumeChangeRequestedEventArgs {
    type Vtable = IVolumeChangeRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1862430044, 53109, 19499, [145, 62, 109, 124, 108, 50, 145, 121]);
}
impl ::windows::runtime::RuntimeName for VolumeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.VolumeChangeRequestedEventArgs";
}
