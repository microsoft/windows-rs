#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioRoutingEndpoint(pub i32);
impl AudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Earpiece: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const WiredHeadset: Self = Self(4i32);
    pub const WiredHeadsetSpeakerOnly: Self = Self(5i32);
    pub const BluetoothWithNoiseAndEchoCancellation: Self = Self(6i32);
    pub const BluetoothPreferred: Self = Self(7i32);
}
impl ::core::marker::Copy for AudioRoutingEndpoint {}
impl ::core::clone::Clone for AudioRoutingEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioRoutingEndpoint {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioRoutingEndpoint {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioRoutingEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioRoutingEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioRoutingEndpoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AudioRoutingEndpoint;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
#[repr(transparent)]
pub struct AudioRoutingManager(::windows::core::IUnknown);
impl AudioRoutingManager {
    #[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
    pub fn GetAudioEndpoint(&self) -> ::windows::core::Result<AudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__: AudioRoutingEndpoint = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAudioEndpoint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioRoutingEndpoint>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
    pub fn SetAudioEndpoint(&self, endpoint: AudioRoutingEndpoint) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioEndpoint)(::core::mem::transmute_copy(this), endpoint).ok() }
    }
    #[doc = "*Required features: `\"Phone_Media_Devices\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioEndpointChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, ::windows::core::IInspectable>>>(&self, endpointchangehandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioEndpointChanged)(::core::mem::transmute_copy(this), endpointchangehandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Media_Devices\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioEndpointChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAudioEndpointChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
    pub fn AvailableAudioEndpoints(&self) -> ::windows::core::Result<AvailableAudioRoutingEndpoints> {
        let this = self;
        unsafe {
            let mut result__: AvailableAudioRoutingEndpoints = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AvailableAudioEndpoints)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AvailableAudioRoutingEndpoints>(result__)
        }
    }
    #[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<AudioRoutingManager> {
        Self::IAudioRoutingManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioRoutingManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioRoutingManagerStatics<R, F: FnOnce(&IAudioRoutingManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioRoutingManager, IAudioRoutingManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioRoutingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioRoutingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioRoutingManager {}
impl ::core::fmt::Debug for AudioRoutingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioRoutingManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioRoutingManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Phone.Media.Devices.AudioRoutingManager;{79340d20-71cc-4526-9f29-fc8d2486418b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioRoutingManager {
    type Vtable = IAudioRoutingManager_Vtbl;
    const IID: ::windows::core::GUID = <IAudioRoutingManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioRoutingManager {
    const NAME: &'static str = "Windows.Phone.Media.Devices.AudioRoutingManager";
}
impl ::core::convert::From<AudioRoutingManager> for ::windows::core::IUnknown {
    fn from(value: AudioRoutingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioRoutingManager> for ::windows::core::IUnknown {
    fn from(value: &AudioRoutingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioRoutingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioRoutingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioRoutingManager> for ::windows::core::IInspectable {
    fn from(value: AudioRoutingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioRoutingManager> for ::windows::core::IInspectable {
    fn from(value: &AudioRoutingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioRoutingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioRoutingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioRoutingManager {}
unsafe impl ::core::marker::Sync for AudioRoutingManager {}
#[doc = "*Required features: `\"Phone_Media_Devices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AvailableAudioRoutingEndpoints(pub u32);
impl AvailableAudioRoutingEndpoints {
    pub const None: Self = Self(0u32);
    pub const Earpiece: Self = Self(1u32);
    pub const Speakerphone: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
}
impl ::core::marker::Copy for AvailableAudioRoutingEndpoints {}
impl ::core::clone::Clone for AvailableAudioRoutingEndpoints {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AvailableAudioRoutingEndpoints {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AvailableAudioRoutingEndpoints {
    type Abi = Self;
}
impl ::core::fmt::Debug for AvailableAudioRoutingEndpoints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AvailableAudioRoutingEndpoints").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AvailableAudioRoutingEndpoints {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AvailableAudioRoutingEndpoints {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for AvailableAudioRoutingEndpoints {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AvailableAudioRoutingEndpoints;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioRoutingManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioRoutingManager {
    type Vtable = IAudioRoutingManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79340d20_71cc_4526_9f29_fc8d2486418b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRoutingManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioRoutingEndpoint) -> ::windows::core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: AudioRoutingEndpoint) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioEndpointChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointchangehandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioEndpointChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioEndpointChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioEndpointChanged: usize,
    pub AvailableAudioEndpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AvailableAudioRoutingEndpoints) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioRoutingManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioRoutingManagerStatics {
    type Vtable = IAudioRoutingManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x977fb2a4_5590_4a6f_adde_6a3d0ad58250);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRoutingManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
