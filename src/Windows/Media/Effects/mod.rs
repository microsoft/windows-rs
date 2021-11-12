#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioCaptureEffectsManager(pub ::windows::core::IInspectable);
impl AudioCaptureEffectsManager {
    #[cfg(feature = "Foundation")]
    pub fn AudioCaptureEffectsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioCaptureEffectsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAudioCaptureEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioEffect>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioCaptureEffectsManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioCaptureEffectsManager;{8f85c271-038d-4393-8298-540110608eef})");
}
unsafe impl ::windows::core::Interface for AudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f85c271_038d_4393_8298_540110608eef);
}
impl ::windows::core::RuntimeName for AudioCaptureEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioCaptureEffectsManager";
}
impl ::core::convert::From<AudioCaptureEffectsManager> for ::windows::core::IUnknown {
    fn from(value: AudioCaptureEffectsManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioCaptureEffectsManager> for ::windows::core::IUnknown {
    fn from(value: &AudioCaptureEffectsManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioCaptureEffectsManager> for ::windows::core::IInspectable {
    fn from(value: AudioCaptureEffectsManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioCaptureEffectsManager> for ::windows::core::IInspectable {
    fn from(value: &AudioCaptureEffectsManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioCaptureEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioCaptureEffectsManager {}
unsafe impl ::core::marker::Sync for AudioCaptureEffectsManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioEffect(pub ::windows::core::IInspectable);
impl AudioEffect {
    pub fn AudioEffectType(&self) -> ::windows::core::Result<AudioEffectType> {
        let this = self;
        unsafe {
            let mut result__: AudioEffectType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioEffectType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioEffect;{34aafa51-9207-4055-be93-6e5734a86ae4})");
}
unsafe impl ::windows::core::Interface for AudioEffect {
    type Vtable = IAudioEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34aafa51_9207_4055_be93_6e5734a86ae4);
}
impl ::windows::core::RuntimeName for AudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffect";
}
impl ::core::convert::From<AudioEffect> for ::windows::core::IUnknown {
    fn from(value: AudioEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioEffect> for ::windows::core::IUnknown {
    fn from(value: &AudioEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioEffect> for ::windows::core::IInspectable {
    fn from(value: AudioEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioEffect> for ::windows::core::IInspectable {
    fn from(value: &AudioEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioEffect {}
unsafe impl ::core::marker::Sync for AudioEffect {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioEffectDefinition(pub ::windows::core::IInspectable);
impl AudioEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(activatableclassid: Param0) -> ::windows::core::Result<AudioEffectDefinition> {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), &mut result__).from_abi::<AudioEffectDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows::core::Result<AudioEffectDefinition> {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), props.into_param().abi(), &mut result__).from_abi::<AudioEffectDefinition>(result__)
        })
    }
    pub fn IAudioEffectDefinitionFactory<R, F: FnOnce(&IAudioEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioEffectDefinition, IAudioEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioEffectDefinition;{e4d7f974-7d80-4f73-9089-e31c9db9c294})");
}
unsafe impl ::windows::core::Interface for AudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4d7f974_7d80_4f73_9089_e31c9db9c294);
}
impl ::windows::core::RuntimeName for AudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectDefinition";
}
impl ::core::convert::From<AudioEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: AudioEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &AudioEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: AudioEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &AudioEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AudioEffectDefinition> for IAudioEffectDefinition {
    fn from(value: AudioEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioEffectDefinition> for IAudioEffectDefinition {
    fn from(value: &AudioEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioEffectDefinition> for AudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioEffectDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioEffectDefinition> for &AudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioEffectDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioEffectDefinition {}
unsafe impl ::core::marker::Sync for AudioEffectDefinition {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioEffectType(pub i32);
impl AudioEffectType {
    pub const Other: AudioEffectType = AudioEffectType(0i32);
    pub const AcousticEchoCancellation: AudioEffectType = AudioEffectType(1i32);
    pub const NoiseSuppression: AudioEffectType = AudioEffectType(2i32);
    pub const AutomaticGainControl: AudioEffectType = AudioEffectType(3i32);
    pub const BeamForming: AudioEffectType = AudioEffectType(4i32);
    pub const ConstantToneRemoval: AudioEffectType = AudioEffectType(5i32);
    pub const Equalizer: AudioEffectType = AudioEffectType(6i32);
    pub const LoudnessEqualizer: AudioEffectType = AudioEffectType(7i32);
    pub const BassBoost: AudioEffectType = AudioEffectType(8i32);
    pub const VirtualSurround: AudioEffectType = AudioEffectType(9i32);
    pub const VirtualHeadphones: AudioEffectType = AudioEffectType(10i32);
    pub const SpeakerFill: AudioEffectType = AudioEffectType(11i32);
    pub const RoomCorrection: AudioEffectType = AudioEffectType(12i32);
    pub const BassManagement: AudioEffectType = AudioEffectType(13i32);
    pub const EnvironmentalEffects: AudioEffectType = AudioEffectType(14i32);
    pub const SpeakerProtection: AudioEffectType = AudioEffectType(15i32);
    pub const SpeakerCompensation: AudioEffectType = AudioEffectType(16i32);
    pub const DynamicRangeCompression: AudioEffectType = AudioEffectType(17i32);
    pub const FarFieldBeamForming: AudioEffectType = AudioEffectType(18i32);
    pub const DeepNoiseSuppression: AudioEffectType = AudioEffectType(19i32);
}
impl ::core::convert::From<i32> for AudioEffectType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioEffectType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioEffectType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.AudioEffectType;i4)");
}
impl ::windows::core::DefaultType for AudioEffectType {
    type DefaultType = Self;
}
pub struct AudioEffectsManager {}
impl AudioEffectsManager {
    #[cfg(feature = "Media_Render")]
    pub fn CreateAudioRenderEffectsManager<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0, category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), category, &mut result__).from_abi::<AudioRenderEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateAudioRenderEffectsManagerWithMode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), category, mode, &mut result__).from_abi::<AudioRenderEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateAudioCaptureEffectsManager<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0, category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), category, &mut result__).from_abi::<AudioCaptureEffectsManager>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateAudioCaptureEffectsManagerWithMode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0, category: super::Capture::MediaCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), category, mode, &mut result__).from_abi::<AudioCaptureEffectsManager>(result__)
        })
    }
    pub fn IAudioEffectsManagerStatics<R, F: FnOnce(&IAudioEffectsManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioEffectsManager, IAudioEffectsManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AudioEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectsManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioRenderEffectsManager(pub ::windows::core::IInspectable);
impl AudioRenderEffectsManager {
    #[cfg(feature = "Foundation")]
    pub fn AudioRenderEffectsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioRenderEffectsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAudioRenderEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioEffect>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage_Streams")]
    pub fn EffectsProviderThumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = &::windows::core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn EffectsProviderSettingsLabel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShowSettingsUI(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioRenderEffectsManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioRenderEffectsManager;{4dc98966-8751-42b2-bfcb-39ca7864bd47})");
}
unsafe impl ::windows::core::Interface for AudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dc98966_8751_42b2_bfcb_39ca7864bd47);
}
impl ::windows::core::RuntimeName for AudioRenderEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioRenderEffectsManager";
}
impl ::core::convert::From<AudioRenderEffectsManager> for ::windows::core::IUnknown {
    fn from(value: AudioRenderEffectsManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioRenderEffectsManager> for ::windows::core::IUnknown {
    fn from(value: &AudioRenderEffectsManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioRenderEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioRenderEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioRenderEffectsManager> for ::windows::core::IInspectable {
    fn from(value: AudioRenderEffectsManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioRenderEffectsManager> for ::windows::core::IInspectable {
    fn from(value: &AudioRenderEffectsManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioRenderEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioRenderEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioRenderEffectsManager {}
unsafe impl ::core::marker::Sync for AudioRenderEffectsManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompositeVideoFrameContext(pub ::windows::core::IInspectable);
impl CompositeVideoFrameContext {
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn SurfacesToOverlay(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(result__)
        }
    }
    pub fn BackgroundFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    pub fn GetOverlayForSurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(&self, surfacetooverlay: Param0) -> ::windows::core::Result<super::Editing::MediaOverlay> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), surfacetooverlay.into_param().abi(), &mut result__).from_abi::<super::Editing::MediaOverlay>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CompositeVideoFrameContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.CompositeVideoFrameContext;{6c30024b-f514-4278-a5f7-b9188049d110})");
}
unsafe impl ::windows::core::Interface for CompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c30024b_f514_4278_a5f7_b9188049d110);
}
impl ::windows::core::RuntimeName for CompositeVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.CompositeVideoFrameContext";
}
impl ::core::convert::From<CompositeVideoFrameContext> for ::windows::core::IUnknown {
    fn from(value: CompositeVideoFrameContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositeVideoFrameContext> for ::windows::core::IUnknown {
    fn from(value: &CompositeVideoFrameContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositeVideoFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompositeVideoFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositeVideoFrameContext> for ::windows::core::IInspectable {
    fn from(value: CompositeVideoFrameContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositeVideoFrameContext> for ::windows::core::IInspectable {
    fn from(value: &CompositeVideoFrameContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositeVideoFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompositeVideoFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CompositeVideoFrameContext {}
unsafe impl ::core::marker::Sync for CompositeVideoFrameContext {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioCaptureEffectsManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f85c271_038d_4393_8298_540110608eef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureEffectsManager_abi(
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
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioEffect {
    type Vtable = IAudioEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34aafa51_9207_4055_be93_6e5734a86ae4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioEffectType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEffectDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4d7f974_7d80_4f73_9089_e31c9db9c294);
}
impl IAudioEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e4d7f974-7d80-4f73-9089-e31c9db9c294}");
}
impl ::core::convert::From<IAudioEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: IAudioEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAudioEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &IAudioEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAudioEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: IAudioEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &IAudioEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioEffectDefinitionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioEffectDefinitionFactory {
    type Vtable = IAudioEffectDefinitionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e1da646_e705_45ed_8a2b_fc4e4f405a97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioEffectsManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioEffectsManagerStatics {
    type Vtable = IAudioEffectsManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66406c04_86fa_47cc_a315_f489d8c3fe10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Capture::MediaCategory, mode: super::AudioProcessing, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dc98966_8751_42b2_bfcb_39ca7864bd47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager_abi(
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
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioRenderEffectsManager2 {
    type Vtable = IAudioRenderEffectsManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa844cd09_5ecc_44b3_bb4e_1db07287139c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBasicAudioEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBasicAudioEffect {
    type Vtable = IBasicAudioEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c062c53_6bc0_48b8_a99a_4b41550f1359);
}
impl IBasicAudioEffect {
    pub fn UseInputFrameForOutput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi()).ok() }
    }
    pub fn ProcessFrame<'a, Param0: ::windows::core::IntoParam<'a, ProcessAudioFrameContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), context.into_param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IBasicAudioEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8c062c53-6bc0-48b8-a99a-4b41550f1359}");
}
impl ::core::convert::From<IBasicAudioEffect> for ::windows::core::IUnknown {
    fn from(value: IBasicAudioEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBasicAudioEffect> for ::windows::core::IUnknown {
    fn from(value: &IBasicAudioEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBasicAudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBasicAudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBasicAudioEffect> for ::windows::core::IInspectable {
    fn from(value: IBasicAudioEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBasicAudioEffect> for ::windows::core::IInspectable {
    fn from(value: &IBasicAudioEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBasicAudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBasicAudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IBasicAudioEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: IBasicAudioEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBasicAudioEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBasicAudioEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for IBasicAudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for &IBasicAudioEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicAudioEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBasicVideoEffect(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBasicVideoEffect {
    type Vtable = IBasicVideoEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8262c7ef_b360_40be_949b_2ff42ff35693);
}
impl IBasicVideoEffect {
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SupportedMemoryTypes(&self) -> ::windows::core::Result<MediaMemoryTypes> {
        let this = self;
        unsafe {
            let mut result__: MediaMemoryTypes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaMemoryTypes>(result__)
        }
    }
    pub fn TimeIndependent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub fn SetEncodingProperties<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>>(&self, encodingproperties: Param0, device: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), device.into_param().abi()).ok() }
    }
    pub fn ProcessFrame<'a, Param0: ::windows::core::IntoParam<'a, ProcessVideoFrameContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), context.into_param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IBasicVideoEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8262c7ef-b360-40be-949b-2ff42ff35693}");
}
impl ::core::convert::From<IBasicVideoEffect> for ::windows::core::IUnknown {
    fn from(value: IBasicVideoEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBasicVideoEffect> for ::windows::core::IUnknown {
    fn from(value: &IBasicVideoEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBasicVideoEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBasicVideoEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBasicVideoEffect> for ::windows::core::IInspectable {
    fn from(value: IBasicVideoEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBasicVideoEffect> for ::windows::core::IInspectable {
    fn from(value: &IBasicVideoEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBasicVideoEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBasicVideoEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IBasicVideoEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: IBasicVideoEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBasicVideoEffect> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBasicVideoEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for IBasicVideoEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for &IBasicVideoEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicVideoEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaMemoryTypes) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositeVideoFrameContext(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c30024b_f514_4278_a5f7_b9188049d110);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeVideoFrameContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, surfacetooverlay: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessAudioFrameContext(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cd92946_1222_4a27_a586_fb3e20273255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessAudioFrameContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IProcessVideoFrameContext(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276f0e2b_6461_401e_ba78_0fdad6114eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessVideoFrameContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISlowMotionEffectDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35053cd0_176c_4763_82c4_1b02dbe31737);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlowMotionEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVideoCompositor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoCompositor {
    type Vtable = IVideoCompositor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8510b43e_420c_420f_96c7_7c98bba1fc55);
}
impl IVideoCompositor {
    pub fn TimeIndependent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub fn SetEncodingProperties<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::VideoEncodingProperties>, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>>(&self, backgroundproperties: Param0, device: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), backgroundproperties.into_param().abi(), device.into_param().abi()).ok() }
    }
    pub fn CompositeFrame<'a, Param0: ::windows::core::IntoParam<'a, CompositeVideoFrameContext>>(&self, context: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), context.into_param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IVideoCompositor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8510b43e-420c-420f-96c7-7c98bba1fc55}");
}
impl ::core::convert::From<IVideoCompositor> for ::windows::core::IUnknown {
    fn from(value: IVideoCompositor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVideoCompositor> for ::windows::core::IUnknown {
    fn from(value: &IVideoCompositor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVideoCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVideoCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVideoCompositor> for ::windows::core::IInspectable {
    fn from(value: IVideoCompositor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVideoCompositor> for ::windows::core::IInspectable {
    fn from(value: &IVideoCompositor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVideoCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVideoCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IVideoCompositor> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: IVideoCompositor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IVideoCompositor> for super::IMediaExtension {
    type Error = ::windows::core::Error;
    fn try_from(value: &IVideoCompositor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for IVideoCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaExtension> for &IVideoCompositor {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaExtension> {
        ::core::convert::TryInto::<super::IMediaExtension>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, backgroundproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVideoCompositorDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7946b8d0_2010_4ae3_9ab2_2cef42edd4d2);
}
impl IVideoCompositorDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVideoCompositorDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7946b8d0-2010-4ae3-9ab2-2cef42edd4d2}");
}
impl ::core::convert::From<IVideoCompositorDefinition> for ::windows::core::IUnknown {
    fn from(value: IVideoCompositorDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVideoCompositorDefinition> for ::windows::core::IUnknown {
    fn from(value: &IVideoCompositorDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVideoCompositorDefinition> for ::windows::core::IInspectable {
    fn from(value: IVideoCompositorDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVideoCompositorDefinition> for ::windows::core::IInspectable {
    fn from(value: &IVideoCompositorDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositorDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoCompositorDefinitionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoCompositorDefinitionFactory {
    type Vtable = IVideoCompositorDefinitionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4366fd10_68b8_4d52_89b6_02a968cca899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositorDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVideoEffectDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
}
impl IVideoEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVideoEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{39f38cf0-8d0f-4f3e-84fc-2d46a5297943}");
}
impl ::core::convert::From<IVideoEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: IVideoEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVideoEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &IVideoEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVideoEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: IVideoEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVideoEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &IVideoEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoEffectDefinitionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoEffectDefinitionFactory {
    type Vtable = IVideoEffectDefinitionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81439b4e_6e33_428f_9d21_b5aafef7617c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoTransformEffectDefinition {
    type Vtable = IVideoTransformEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9664bb6a_1ea6_4aa6_8074_abe8851ecae2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::MediaMirroringOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_Transcoding")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))] usize,
    #[cfg(feature = "Media_Transcoding")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoTransformEffectDefinition2 {
    type Vtable = IVideoTransformEffectDefinition2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a8089f_66c8_4694_9fd9_1136abf7444a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition2_abi(
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
pub struct IVideoTransformSphericalProjection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4401f0_9bf2_4c39_9f41_e022514a8468);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformSphericalProjection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Playback::SphericalVideoProjectionMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaEffectClosedReason(pub i32);
impl MediaEffectClosedReason {
    pub const Done: MediaEffectClosedReason = MediaEffectClosedReason(0i32);
    pub const UnknownError: MediaEffectClosedReason = MediaEffectClosedReason(1i32);
    pub const UnsupportedEncodingFormat: MediaEffectClosedReason = MediaEffectClosedReason(2i32);
    pub const EffectCurrentlyUnloaded: MediaEffectClosedReason = MediaEffectClosedReason(3i32);
}
impl ::core::convert::From<i32> for MediaEffectClosedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaEffectClosedReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaEffectClosedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaEffectClosedReason;i4)");
}
impl ::windows::core::DefaultType for MediaEffectClosedReason {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaMemoryTypes(pub i32);
impl MediaMemoryTypes {
    pub const Gpu: MediaMemoryTypes = MediaMemoryTypes(0i32);
    pub const Cpu: MediaMemoryTypes = MediaMemoryTypes(1i32);
    pub const GpuAndCpu: MediaMemoryTypes = MediaMemoryTypes(2i32);
}
impl ::core::convert::From<i32> for MediaMemoryTypes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaMemoryTypes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaMemoryTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaMemoryTypes;i4)");
}
impl ::windows::core::DefaultType for MediaMemoryTypes {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessAudioFrameContext(pub ::windows::core::IInspectable);
impl ProcessAudioFrameContext {
    pub fn InputFrame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioFrame>(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioFrame>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessAudioFrameContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.ProcessAudioFrameContext;{4cd92946-1222-4a27-a586-fb3e20273255})");
}
unsafe impl ::windows::core::Interface for ProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cd92946_1222_4a27_a586_fb3e20273255);
}
impl ::windows::core::RuntimeName for ProcessAudioFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessAudioFrameContext";
}
impl ::core::convert::From<ProcessAudioFrameContext> for ::windows::core::IUnknown {
    fn from(value: ProcessAudioFrameContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessAudioFrameContext> for ::windows::core::IUnknown {
    fn from(value: &ProcessAudioFrameContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessAudioFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessAudioFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessAudioFrameContext> for ::windows::core::IInspectable {
    fn from(value: ProcessAudioFrameContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessAudioFrameContext> for ::windows::core::IInspectable {
    fn from(value: &ProcessAudioFrameContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessAudioFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessAudioFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessAudioFrameContext {}
unsafe impl ::core::marker::Sync for ProcessAudioFrameContext {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessVideoFrameContext(pub ::windows::core::IInspectable);
impl ProcessVideoFrameContext {
    pub fn InputFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessVideoFrameContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.ProcessVideoFrameContext;{276f0e2b-6461-401e-ba78-0fdad6114eec})");
}
unsafe impl ::windows::core::Interface for ProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276f0e2b_6461_401e_ba78_0fdad6114eec);
}
impl ::windows::core::RuntimeName for ProcessVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessVideoFrameContext";
}
impl ::core::convert::From<ProcessVideoFrameContext> for ::windows::core::IUnknown {
    fn from(value: ProcessVideoFrameContext) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessVideoFrameContext> for ::windows::core::IUnknown {
    fn from(value: &ProcessVideoFrameContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessVideoFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessVideoFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessVideoFrameContext> for ::windows::core::IInspectable {
    fn from(value: ProcessVideoFrameContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessVideoFrameContext> for ::windows::core::IInspectable {
    fn from(value: &ProcessVideoFrameContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessVideoFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessVideoFrameContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessVideoFrameContext {}
unsafe impl ::core::marker::Sync for ProcessVideoFrameContext {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SlowMotionEffectDefinition(pub ::windows::core::IInspectable);
impl SlowMotionEffectDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SlowMotionEffectDefinition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TimeStretchRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetTimeStretchRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SlowMotionEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.SlowMotionEffectDefinition;{35053cd0-176c-4763-82c4-1b02dbe31737})");
}
unsafe impl ::windows::core::Interface for SlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35053cd0_176c_4763_82c4_1b02dbe31737);
}
impl ::windows::core::RuntimeName for SlowMotionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.SlowMotionEffectDefinition";
}
impl ::core::convert::From<SlowMotionEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: SlowMotionEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SlowMotionEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &SlowMotionEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SlowMotionEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: SlowMotionEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SlowMotionEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &SlowMotionEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<SlowMotionEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: SlowMotionEffectDefinition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlowMotionEffectDefinition> for IVideoEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlowMotionEffectDefinition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVideoEffectDefinition> for SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IVideoEffectDefinition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVideoEffectDefinition> for &SlowMotionEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IVideoEffectDefinition> {
        ::core::convert::TryInto::<IVideoEffectDefinition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SlowMotionEffectDefinition {}
unsafe impl ::core::marker::Sync for SlowMotionEffectDefinition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoCompositorDefinition(pub ::windows::core::IInspectable);
impl VideoCompositorDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(activatableclassid: Param0) -> ::windows::core::Result<VideoCompositorDefinition> {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), &mut result__).from_abi::<VideoCompositorDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows::core::Result<VideoCompositorDefinition> {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), props.into_param().abi(), &mut result__).from_abi::<VideoCompositorDefinition>(result__)
        })
    }
    pub fn IVideoCompositorDefinitionFactory<R, F: FnOnce(&IVideoCompositorDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoCompositorDefinition, IVideoCompositorDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VideoCompositorDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoCompositorDefinition;{7946b8d0-2010-4ae3-9ab2-2cef42edd4d2})");
}
unsafe impl ::windows::core::Interface for VideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7946b8d0_2010_4ae3_9ab2_2cef42edd4d2);
}
impl ::windows::core::RuntimeName for VideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoCompositorDefinition";
}
impl ::core::convert::From<VideoCompositorDefinition> for ::windows::core::IUnknown {
    fn from(value: VideoCompositorDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoCompositorDefinition> for ::windows::core::IUnknown {
    fn from(value: &VideoCompositorDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoCompositorDefinition> for ::windows::core::IInspectable {
    fn from(value: VideoCompositorDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoCompositorDefinition> for ::windows::core::IInspectable {
    fn from(value: &VideoCompositorDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<VideoCompositorDefinition> for IVideoCompositorDefinition {
    fn from(value: VideoCompositorDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoCompositorDefinition> for IVideoCompositorDefinition {
    fn from(value: &VideoCompositorDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVideoCompositorDefinition> for VideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IVideoCompositorDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVideoCompositorDefinition> for &VideoCompositorDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IVideoCompositorDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoCompositorDefinition {}
unsafe impl ::core::marker::Sync for VideoCompositorDefinition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoEffectDefinition(pub ::windows::core::IInspectable);
impl VideoEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(activatableclassid: Param0) -> ::windows::core::Result<VideoEffectDefinition> {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), &mut result__).from_abi::<VideoEffectDefinition>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(activatableclassid: Param0, props: Param1) -> ::windows::core::Result<VideoEffectDefinition> {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), props.into_param().abi(), &mut result__).from_abi::<VideoEffectDefinition>(result__)
        })
    }
    pub fn IVideoEffectDefinitionFactory<R, F: FnOnce(&IVideoEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoEffectDefinition, IVideoEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VideoEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
unsafe impl ::windows::core::Interface for VideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
}
impl ::windows::core::RuntimeName for VideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoEffectDefinition";
}
impl ::core::convert::From<VideoEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: VideoEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &VideoEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: VideoEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &VideoEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<VideoEffectDefinition> for IVideoEffectDefinition {
    fn from(value: VideoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoEffectDefinition> for IVideoEffectDefinition {
    fn from(value: &VideoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVideoEffectDefinition> for VideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IVideoEffectDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVideoEffectDefinition> for &VideoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IVideoEffectDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoEffectDefinition {}
unsafe impl ::core::marker::Sync for VideoEffectDefinition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoTransformEffectDefinition(pub ::windows::core::IInspectable);
impl VideoTransformEffectDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoTransformEffectDefinition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn PaddingColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetPaddingColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OutputSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetOutputSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CropRectangle(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetCropRectangle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Rotation(&self) -> ::windows::core::Result<super::MediaProperties::MediaRotation> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::MediaProperties::MediaRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaRotation>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Mirror(&self) -> ::windows::core::Result<super::MediaProperties::MediaMirroringOptions> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::MediaProperties::MediaMirroringOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaMirroringOptions>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetMirror(&self, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Transcoding")]
    pub fn SetProcessingAlgorithm(&self, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Transcoding")]
    pub fn ProcessingAlgorithm(&self) -> ::windows::core::Result<super::Transcoding::MediaVideoProcessingAlgorithm> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__: super::Transcoding::MediaVideoProcessingAlgorithm = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Transcoding::MediaVideoProcessingAlgorithm>(result__)
        }
    }
    pub fn SphericalProjection(&self) -> ::windows::core::Result<VideoTransformSphericalProjection> {
        let this = &::windows::core::Interface::cast::<IVideoTransformEffectDefinition2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoTransformSphericalProjection>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VideoTransformEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoTransformEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
unsafe impl ::windows::core::Interface for VideoTransformEffectDefinition {
    type Vtable = IVideoEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
}
impl ::windows::core::RuntimeName for VideoTransformEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformEffectDefinition";
}
impl ::core::convert::From<VideoTransformEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: VideoTransformEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoTransformEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &VideoTransformEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoTransformEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: VideoTransformEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoTransformEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &VideoTransformEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<VideoTransformEffectDefinition> for IVideoEffectDefinition {
    fn from(value: VideoTransformEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoTransformEffectDefinition> for IVideoEffectDefinition {
    fn from(value: &VideoTransformEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVideoEffectDefinition> for VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IVideoEffectDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVideoEffectDefinition> for &VideoTransformEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IVideoEffectDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoTransformEffectDefinition {}
unsafe impl ::core::marker::Sync for VideoTransformEffectDefinition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoTransformSphericalProjection(pub ::windows::core::IInspectable);
impl VideoTransformSphericalProjection {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameFormat(&self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__: super::MediaProperties::SphericalVideoFrameFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::SphericalVideoFrameFormat>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn ProjectionMode(&self) -> ::windows::core::Result<super::Playback::SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__: super::Playback::SphericalVideoProjectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Playback::SphericalVideoProjectionMode>(result__)
        }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn SetProjectionMode(&self, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ViewOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetViewOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VideoTransformSphericalProjection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoTransformSphericalProjection;{cf4401f0-9bf2-4c39-9f41-e022514a8468})");
}
unsafe impl ::windows::core::Interface for VideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4401f0_9bf2_4c39_9f41_e022514a8468);
}
impl ::windows::core::RuntimeName for VideoTransformSphericalProjection {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformSphericalProjection";
}
impl ::core::convert::From<VideoTransformSphericalProjection> for ::windows::core::IUnknown {
    fn from(value: VideoTransformSphericalProjection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoTransformSphericalProjection> for ::windows::core::IUnknown {
    fn from(value: &VideoTransformSphericalProjection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoTransformSphericalProjection> for ::windows::core::IInspectable {
    fn from(value: VideoTransformSphericalProjection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoTransformSphericalProjection> for ::windows::core::IInspectable {
    fn from(value: &VideoTransformSphericalProjection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoTransformSphericalProjection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VideoTransformSphericalProjection {}
unsafe impl ::core::marker::Sync for VideoTransformSphericalProjection {}
