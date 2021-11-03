#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_Media_Devices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioRoutingEndpoint(pub i32);
impl AudioRoutingEndpoint {
    pub const Default: AudioRoutingEndpoint = AudioRoutingEndpoint(0i32);
    pub const Earpiece: AudioRoutingEndpoint = AudioRoutingEndpoint(1i32);
    pub const Speakerphone: AudioRoutingEndpoint = AudioRoutingEndpoint(2i32);
    pub const Bluetooth: AudioRoutingEndpoint = AudioRoutingEndpoint(3i32);
    pub const WiredHeadset: AudioRoutingEndpoint = AudioRoutingEndpoint(4i32);
    pub const WiredHeadsetSpeakerOnly: AudioRoutingEndpoint = AudioRoutingEndpoint(5i32);
    pub const BluetoothWithNoiseAndEchoCancellation: AudioRoutingEndpoint = AudioRoutingEndpoint(6i32);
    pub const BluetoothPreferred: AudioRoutingEndpoint = AudioRoutingEndpoint(7i32);
}
impl ::std::convert::From<i32> for AudioRoutingEndpoint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioRoutingEndpoint {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioRoutingEndpoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AudioRoutingEndpoint;i4)");
}
#[doc = "*Required features: `Phone_Media_Devices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AudioRoutingManager(::windows::runtime::IInspectable);
impl AudioRoutingManager {
    #[doc = "*Required features: `Phone_Media_Devices`*"]
    pub fn GetAudioEndpoint(&self) -> ::windows::runtime::Result<AudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__: AudioRoutingEndpoint = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AudioRoutingEndpoint>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Media_Devices`*"]
    pub fn SetAudioEndpoint(&self, endpoint: AudioRoutingEndpoint) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), endpoint).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Media_Devices`, `Foundation`*"]
    pub fn AudioEndpointChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::runtime::IInspectable>>>(&self, endpointchangehandler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), endpointchangehandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_Media_Devices`, `Foundation`*"]
    pub fn RemoveAudioEndpointChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Phone_Media_Devices`*"]
    pub fn AvailableAudioEndpoints(&self) -> ::windows::runtime::Result<AvailableAudioRoutingEndpoints> {
        let this = self;
        unsafe {
            let mut result__: AvailableAudioRoutingEndpoints = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AvailableAudioRoutingEndpoints>(result__)
        }
    }
    #[doc = "*Required features: `Phone_Media_Devices`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<AudioRoutingManager> {
        Self::IAudioRoutingManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AudioRoutingManager>(result__)
        })
    }
    pub fn IAudioRoutingManagerStatics<R, F: FnOnce(&IAudioRoutingManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AudioRoutingManager, IAudioRoutingManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioRoutingManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.Media.Devices.AudioRoutingManager;{79340d20-71cc-4526-9f29-fc8d2486418b})");
}
unsafe impl ::windows::runtime::Interface for AudioRoutingManager {
    type Vtable = IAudioRoutingManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2033454368, 29132, 17702, [159, 41, 252, 141, 36, 134, 65, 139]);
}
impl ::windows::runtime::RuntimeName for AudioRoutingManager {
    const NAME: &'static str = "Windows.Phone.Media.Devices.AudioRoutingManager";
}
unsafe impl ::std::marker::Send for AudioRoutingManager {}
unsafe impl ::std::marker::Sync for AudioRoutingManager {}
#[doc = "*Required features: `Phone_Media_Devices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AvailableAudioRoutingEndpoints(pub u32);
impl AvailableAudioRoutingEndpoints {
    pub const None: AvailableAudioRoutingEndpoints = AvailableAudioRoutingEndpoints(0u32);
    pub const Earpiece: AvailableAudioRoutingEndpoints = AvailableAudioRoutingEndpoints(1u32);
    pub const Speakerphone: AvailableAudioRoutingEndpoints = AvailableAudioRoutingEndpoints(2u32);
    pub const Bluetooth: AvailableAudioRoutingEndpoints = AvailableAudioRoutingEndpoints(4u32);
}
impl ::std::convert::From<u32> for AvailableAudioRoutingEndpoints {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AvailableAudioRoutingEndpoints {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AvailableAudioRoutingEndpoints {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AvailableAudioRoutingEndpoints;u4)");
}
impl ::std::ops::BitOr for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for AvailableAudioRoutingEndpoints {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for AvailableAudioRoutingEndpoints {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAudioRoutingManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioRoutingManager {
    type Vtable = IAudioRoutingManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2033454368, 29132, 17702, [159, 41, 252, 141, 36, 134, 65, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRoutingManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AudioRoutingEndpoint) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, endpoint: AudioRoutingEndpoint) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, endpointchangehandler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AvailableAudioRoutingEndpoints) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAudioRoutingManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioRoutingManagerStatics {
    type Vtable = IAudioRoutingManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2541728420, 21904, 19055, [173, 222, 106, 61, 10, 213, 130, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRoutingManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
