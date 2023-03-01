#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioCaptureEffectsManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_Vtbl;
}
impl ::core::clone::Clone for IAudioCaptureEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioCaptureEffectsManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f85c271_038d_4393_8298_540110608eef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureEffectsManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AudioCaptureEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioCaptureEffectsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioCaptureEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioCaptureEffectsChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAudioCaptureEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAudioCaptureEffects: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEffect {
    type Vtable = IAudioEffect_Vtbl;
}
impl ::core::clone::Clone for IAudioEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34aafa51_9207_4055_be93_6e5734a86ae4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AudioEffectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioEffectType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct IAudioEffectDefinition(::windows::core::IUnknown);
impl IAudioEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAudioEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IAudioEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectDefinition {}
impl ::core::fmt::Debug for IAudioEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAudioEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e4d7f974-7d80-4f73-9089-e31c9db9c294}");
}
unsafe impl ::windows::core::Interface for IAudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for IAudioEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioEffectDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4d7f974_7d80_4f73_9089_e31c9db9c294);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEffectDefinitionFactory {
    type Vtable = IAudioEffectDefinitionFactory_Vtbl;
}
impl ::core::clone::Clone for IAudioEffectDefinitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioEffectDefinitionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e1da646_e705_45ed_8a2b_fc4e4f405a97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectDefinitionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioEffectsManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioEffectsManagerStatics {
    type Vtable = IAudioEffectsManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IAudioEffectsManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioEffectsManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66406c04_86fa_47cc_a315_f489d8c3fe10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Render")]
    pub CreateAudioRenderEffectsManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, category: super::Render::AudioRenderCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateAudioRenderEffectsManager: usize,
    #[cfg(feature = "Media_Render")]
    pub CreateAudioRenderEffectsManagerWithMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateAudioRenderEffectsManagerWithMode: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateAudioCaptureEffectsManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, category: super::Capture::MediaCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateAudioCaptureEffectsManager: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateAudioCaptureEffectsManagerWithMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, category: super::Capture::MediaCategory, mode: super::AudioProcessing, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateAudioCaptureEffectsManagerWithMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioRenderEffectsManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_Vtbl;
}
impl ::core::clone::Clone for IAudioRenderEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioRenderEffectsManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dc98966_8751_42b2_bfcb_39ca7864bd47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AudioRenderEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioRenderEffectsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioRenderEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioRenderEffectsChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAudioRenderEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAudioRenderEffects: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IAudioRenderEffectsManager2(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IAudioRenderEffectsManager2 {
    type Vtable = IAudioRenderEffectsManager2_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IAudioRenderEffectsManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IAudioRenderEffectsManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa844cd09_5ecc_44b3_bb4e_1db07287139c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderEffectsManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub EffectsProviderThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    EffectsProviderThumbnail: usize,
    #[cfg(feature = "deprecated")]
    pub EffectsProviderSettingsLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EffectsProviderSettingsLabel: usize,
    #[cfg(feature = "deprecated")]
    pub ShowSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowSettingsUI: usize,
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct IBasicAudioEffect(::windows::core::IUnknown);
impl IBasicAudioEffect {
    pub fn UseInputFrameForOutput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).UseInputFrameForOutput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>();
            (::windows::core::Interface::vtable(this).SupportedEncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties(&self, encodingproperties: &super::MediaProperties::AudioEncodingProperties) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncodingProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(encodingproperties)).ok() }
    }
    pub fn ProcessFrame(&self, context: &ProcessAudioFrameContext) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessFrame)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(context)).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DiscardQueuedFrames)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows::core::ComInterface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProperties)(::windows::core::Interface::as_raw(this), configuration.try_into_param()?.abi()).ok() }
    }
}
::windows::imp::interface_hierarchy!(IBasicAudioEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<super::IMediaExtension> for IBasicAudioEffect {}
impl ::core::cmp::PartialEq for IBasicAudioEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBasicAudioEffect {}
impl ::core::fmt::Debug for IBasicAudioEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBasicAudioEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBasicAudioEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{8c062c53-6bc0-48b8-a99a-4b41550f1359}");
}
unsafe impl ::windows::core::Interface for IBasicAudioEffect {
    type Vtable = IBasicAudioEffect_Vtbl;
}
impl ::core::clone::Clone for IBasicAudioEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBasicAudioEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c062c53_6bc0_48b8_a99a_4b41550f1359);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicAudioEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UseInputFrameForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SupportedEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SupportedEncodingProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetEncodingProperties: usize,
    pub ProcessFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct IBasicVideoEffect(::windows::core::IUnknown);
impl IBasicVideoEffect {
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportedMemoryTypes(&self) -> ::windows::core::Result<MediaMemoryTypes> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MediaMemoryTypes>();
            (::windows::core::Interface::vtable(this).SupportedMemoryTypes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TimeIndependent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TimeIndependent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>();
            (::windows::core::Interface::vtable(this).SupportedEncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub fn SetEncodingProperties<P0>(&self, encodingproperties: &super::MediaProperties::VideoEncodingProperties, device: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncodingProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(encodingproperties), device.try_into_param()?.abi()).ok() }
    }
    pub fn ProcessFrame(&self, context: &ProcessVideoFrameContext) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProcessFrame)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(context)).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DiscardQueuedFrames)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows::core::ComInterface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProperties)(::windows::core::Interface::as_raw(this), configuration.try_into_param()?.abi()).ok() }
    }
}
::windows::imp::interface_hierarchy!(IBasicVideoEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<super::IMediaExtension> for IBasicVideoEffect {}
impl ::core::cmp::PartialEq for IBasicVideoEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBasicVideoEffect {}
impl ::core::fmt::Debug for IBasicVideoEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBasicVideoEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBasicVideoEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{8262c7ef-b360-40be-949b-2ff42ff35693}");
}
unsafe impl ::windows::core::Interface for IBasicVideoEffect {
    type Vtable = IBasicVideoEffect_Vtbl;
}
impl ::core::clone::Clone for IBasicVideoEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBasicVideoEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8262c7ef_b360_40be_949b_2ff42ff35693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicVideoEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SupportedMemoryTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaMemoryTypes) -> ::windows::core::HRESULT,
    pub TimeIndependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SupportedEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SupportedEncodingProperties: usize,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))]
    SetEncodingProperties: usize,
    pub ProcessFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositeVideoFrameContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_Vtbl;
}
impl ::core::clone::Clone for ICompositeVideoFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompositeVideoFrameContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c30024b_f514_4278_a5f7_b9188049d110);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeVideoFrameContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub SurfacesToOverlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))]
    SurfacesToOverlay: usize,
    pub BackgroundFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    pub GetOverlayForSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surfacetooverlay: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing")))]
    GetOverlayForSurface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessAudioFrameContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_Vtbl;
}
impl ::core::clone::Clone for IProcessAudioFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProcessAudioFrameContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cd92946_1222_4a27_a586_fb3e20273255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessAudioFrameContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessVideoFrameContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_Vtbl;
}
impl ::core::clone::Clone for IProcessVideoFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProcessVideoFrameContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276f0e2b_6461_401e_ba78_0fdad6114eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessVideoFrameContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlowMotionEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for ISlowMotionEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISlowMotionEffectDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35053cd0_176c_4763_82c4_1b02dbe31737);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlowMotionEffectDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TimeStretchRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetTimeStretchRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct IVideoCompositor(::windows::core::IUnknown);
impl IVideoCompositor {
    pub fn TimeIndependent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TimeIndependent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub fn SetEncodingProperties<P0>(&self, backgroundproperties: &super::MediaProperties::VideoEncodingProperties, device: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncodingProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(backgroundproperties), device.try_into_param()?.abi()).ok() }
    }
    pub fn CompositeFrame(&self, context: &CompositeVideoFrameContext) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CompositeFrame)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(context)).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DiscardQueuedFrames)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows::core::ComInterface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProperties)(::windows::core::Interface::as_raw(this), configuration.try_into_param()?.abi()).ok() }
    }
}
::windows::imp::interface_hierarchy!(IVideoCompositor, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<super::IMediaExtension> for IVideoCompositor {}
impl ::core::cmp::PartialEq for IVideoCompositor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoCompositor {}
impl ::core::fmt::Debug for IVideoCompositor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoCompositor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IVideoCompositor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{8510b43e-420c-420f-96c7-7c98bba1fc55}");
}
unsafe impl ::windows::core::Interface for IVideoCompositor {
    type Vtable = IVideoCompositor_Vtbl;
}
impl ::core::clone::Clone for IVideoCompositor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoCompositor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8510b43e_420c_420f_96c7_7c98bba1fc55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TimeIndependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backgroundproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))]
    SetEncodingProperties: usize,
    pub CompositeFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct IVideoCompositorDefinition(::windows::core::IUnknown);
impl IVideoCompositorDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IVideoCompositorDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IVideoCompositorDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoCompositorDefinition {}
impl ::core::fmt::Debug for IVideoCompositorDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoCompositorDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IVideoCompositorDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{7946b8d0-2010-4ae3-9ab2-2cef42edd4d2}");
}
unsafe impl ::windows::core::Interface for IVideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_Vtbl;
}
impl ::core::clone::Clone for IVideoCompositorDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoCompositorDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7946b8d0_2010_4ae3_9ab2_2cef42edd4d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositorDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoCompositorDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoCompositorDefinitionFactory {
    type Vtable = IVideoCompositorDefinitionFactory_Vtbl;
}
impl ::core::clone::Clone for IVideoCompositorDefinitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoCompositorDefinitionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4366fd10_68b8_4d52_89b6_02a968cca899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoCompositorDefinitionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct IVideoEffectDefinition(::windows::core::IUnknown);
impl IVideoEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IVideoEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IVideoEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoEffectDefinition {}
impl ::core::fmt::Debug for IVideoEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IVideoEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{39f38cf0-8d0f-4f3e-84fc-2d46a5297943}");
}
unsafe impl ::windows::core::Interface for IVideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for IVideoEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoEffectDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ActivatableClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEffectDefinitionFactory {
    type Vtable = IVideoEffectDefinitionFactory_Vtbl;
}
impl ::core::clone::Clone for IVideoEffectDefinitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoEffectDefinitionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81439b4e_6e33_428f_9d21_b5aafef7617c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectDefinitionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTransformEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoTransformEffectDefinition {
    type Vtable = IVideoTransformEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for IVideoTransformEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoTransformEffectDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9664bb6a_1ea6_4aa6_8074_abe8851ecae2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub PaddingColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    PaddingColor: usize,
    #[cfg(feature = "UI")]
    pub SetPaddingColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetPaddingColor: usize,
    #[cfg(feature = "Foundation")]
    pub OutputSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutputSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetOutputSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOutputSize: usize,
    #[cfg(feature = "Foundation")]
    pub CropRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CropRectangle: usize,
    #[cfg(feature = "Foundation")]
    pub SetCropRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCropRectangle: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Rotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetRotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub Mirror: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaMirroringOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Mirror: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetMirror: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetMirror: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub SetProcessingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    SetProcessingAlgorithm: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub ProcessingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    ProcessingAlgorithm: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTransformEffectDefinition2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoTransformEffectDefinition2 {
    type Vtable = IVideoTransformEffectDefinition2_Vtbl;
}
impl ::core::clone::Clone for IVideoTransformEffectDefinition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoTransformEffectDefinition2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a8089f_66c8_4694_9fd9_1136abf7444a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformEffectDefinition2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SphericalProjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTransformSphericalProjection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_Vtbl;
}
impl ::core::clone::Clone for IVideoTransformSphericalProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoTransformSphericalProjection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4401f0_9bf2_4c39_9f41_e022514a8468);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTransformSphericalProjection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetFrameFormat: usize,
    #[cfg(feature = "Media_Playback")]
    pub ProjectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Playback::SphericalVideoProjectionMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    ProjectionMode: usize,
    #[cfg(feature = "Media_Playback")]
    pub SetProjectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    SetProjectionMode: usize,
    pub HorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetHorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ViewOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ViewOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetViewOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetViewOrientation: usize,
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct AudioCaptureEffectsManager(::windows::core::IUnknown);
impl AudioCaptureEffectsManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioCaptureEffectsChanged(&self, handler: &super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AudioCaptureEffectsChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioCaptureEffectsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAudioCaptureEffectsChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAudioCaptureEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioEffect>>();
            (::windows::core::Interface::vtable(this).GetAudioCaptureEffects)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioCaptureEffectsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioCaptureEffectsManager {}
impl ::core::fmt::Debug for AudioCaptureEffectsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioCaptureEffectsManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioCaptureEffectsManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioCaptureEffectsManager;{8f85c271-038d-4393-8298-540110608eef})");
}
impl ::core::clone::Clone for AudioCaptureEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioCaptureEffectsManager {
    const IID: ::windows::core::GUID = <IAudioCaptureEffectsManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioCaptureEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioCaptureEffectsManager";
}
::windows::imp::interface_hierarchy!(AudioCaptureEffectsManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioCaptureEffectsManager {}
unsafe impl ::core::marker::Sync for AudioCaptureEffectsManager {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct AudioEffect(::windows::core::IUnknown);
impl AudioEffect {
    pub fn AudioEffectType(&self) -> ::windows::core::Result<AudioEffectType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioEffectType>();
            (::windows::core::Interface::vtable(this).AudioEffectType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioEffect {}
impl ::core::fmt::Debug for AudioEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioEffect;{34aafa51-9207-4055-be93-6e5734a86ae4})");
}
impl ::core::clone::Clone for AudioEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioEffect {
    type Vtable = IAudioEffect_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioEffect {
    const IID: ::windows::core::GUID = <IAudioEffect as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffect";
}
::windows::imp::interface_hierarchy!(AudioEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioEffect {}
unsafe impl ::core::marker::Sync for AudioEffect {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct AudioEffectDefinition(::windows::core::IUnknown);
impl AudioEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioEffectDefinition> {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioEffectDefinition>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<P0>(activatableclassid: &::windows::core::HSTRING, props: P0) -> ::windows::core::Result<AudioEffectDefinition>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioEffectDefinition>();
            (::windows::core::Interface::vtable(this).CreateWithProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), props.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioEffectDefinitionFactory<R, F: FnOnce(&IAudioEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioEffectDefinition, IAudioEffectDefinitionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioEffectDefinition {}
impl ::core::fmt::Debug for AudioEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioEffectDefinition;{e4d7f974-7d80-4f73-9089-e31c9db9c294})");
}
impl ::core::clone::Clone for AudioEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioEffectDefinition {
    const IID: ::windows::core::GUID = <IAudioEffectDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectDefinition";
}
::windows::imp::interface_hierarchy!(AudioEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioEffectDefinition> for AudioEffectDefinition {}
unsafe impl ::core::marker::Send for AudioEffectDefinition {}
unsafe impl ::core::marker::Sync for AudioEffectDefinition {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
pub struct AudioEffectsManager;
impl AudioEffectsManager {
    #[doc = "*Required features: `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn CreateAudioRenderEffectsManager(deviceid: &::windows::core::HSTRING, category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioRenderEffectsManager>();
            (::windows::core::Interface::vtable(this).CreateAudioRenderEffectsManager)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), category, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn CreateAudioRenderEffectsManagerWithMode(deviceid: &::windows::core::HSTRING, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioRenderEffectsManager>();
            (::windows::core::Interface::vtable(this).CreateAudioRenderEffectsManagerWithMode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), category, mode, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn CreateAudioCaptureEffectsManager(deviceid: &::windows::core::HSTRING, category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioCaptureEffectsManager>();
            (::windows::core::Interface::vtable(this).CreateAudioCaptureEffectsManager)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), category, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn CreateAudioCaptureEffectsManagerWithMode(deviceid: &::windows::core::HSTRING, category: super::Capture::MediaCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioCaptureEffectsManager>();
            (::windows::core::Interface::vtable(this).CreateAudioCaptureEffectsManagerWithMode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), category, mode, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioEffectsManagerStatics<R, F: FnOnce(&IAudioEffectsManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioEffectsManager, IAudioEffectsManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AudioEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectsManager";
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct AudioRenderEffectsManager(::windows::core::IUnknown);
impl AudioRenderEffectsManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioRenderEffectsChanged(&self, handler: &super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AudioRenderEffectsChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioRenderEffectsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAudioRenderEffectsChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAudioRenderEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioEffect>>();
            (::windows::core::Interface::vtable(this).GetAudioRenderEffects)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn EffectsProviderThumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = &::windows::core::ComInterface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>();
            (::windows::core::Interface::vtable(this).EffectsProviderThumbnail)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn EffectsProviderSettingsLabel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).EffectsProviderSettingsLabel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowSettingsUI(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ShowSettingsUI)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioRenderEffectsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioRenderEffectsManager {}
impl ::core::fmt::Debug for AudioRenderEffectsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioRenderEffectsManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioRenderEffectsManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.AudioRenderEffectsManager;{4dc98966-8751-42b2-bfcb-39ca7864bd47})");
}
impl ::core::clone::Clone for AudioRenderEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioRenderEffectsManager {
    const IID: ::windows::core::GUID = <IAudioRenderEffectsManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioRenderEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioRenderEffectsManager";
}
::windows::imp::interface_hierarchy!(AudioRenderEffectsManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioRenderEffectsManager {}
unsafe impl ::core::marker::Sync for AudioRenderEffectsManager {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct CompositeVideoFrameContext(::windows::core::IUnknown);
impl CompositeVideoFrameContext {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn SurfacesToOverlay(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>();
            (::windows::core::Interface::vtable(this).SurfacesToOverlay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BackgroundFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::VideoFrame>();
            (::windows::core::Interface::vtable(this).BackgroundFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::VideoFrame>();
            (::windows::core::Interface::vtable(this).OutputFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`, `\"Media_Editing\"`*"]
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    pub fn GetOverlayForSurface<P0>(&self, surfacetooverlay: P0) -> ::windows::core::Result<super::Editing::MediaOverlay>
    where
        P0: ::windows::core::TryIntoParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Editing::MediaOverlay>();
            (::windows::core::Interface::vtable(this).GetOverlayForSurface)(::windows::core::Interface::as_raw(this), surfacetooverlay.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CompositeVideoFrameContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositeVideoFrameContext {}
impl ::core::fmt::Debug for CompositeVideoFrameContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositeVideoFrameContext").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CompositeVideoFrameContext {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.CompositeVideoFrameContext;{6c30024b-f514-4278-a5f7-b9188049d110})");
}
impl ::core::clone::Clone for CompositeVideoFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CompositeVideoFrameContext {
    const IID: ::windows::core::GUID = <ICompositeVideoFrameContext as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CompositeVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.CompositeVideoFrameContext";
}
::windows::imp::interface_hierarchy!(CompositeVideoFrameContext, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CompositeVideoFrameContext {}
unsafe impl ::core::marker::Sync for CompositeVideoFrameContext {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct ProcessAudioFrameContext(::windows::core::IUnknown);
impl ProcessAudioFrameContext {
    pub fn InputFrame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::AudioFrame>();
            (::windows::core::Interface::vtable(this).InputFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::AudioFrame>();
            (::windows::core::Interface::vtable(this).OutputFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ProcessAudioFrameContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessAudioFrameContext {}
impl ::core::fmt::Debug for ProcessAudioFrameContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessAudioFrameContext").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ProcessAudioFrameContext {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.ProcessAudioFrameContext;{4cd92946-1222-4a27-a586-fb3e20273255})");
}
impl ::core::clone::Clone for ProcessAudioFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ProcessAudioFrameContext {
    const IID: ::windows::core::GUID = <IProcessAudioFrameContext as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ProcessAudioFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessAudioFrameContext";
}
::windows::imp::interface_hierarchy!(ProcessAudioFrameContext, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProcessAudioFrameContext {}
unsafe impl ::core::marker::Sync for ProcessAudioFrameContext {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct ProcessVideoFrameContext(::windows::core::IUnknown);
impl ProcessVideoFrameContext {
    pub fn InputFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::VideoFrame>();
            (::windows::core::Interface::vtable(this).InputFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::VideoFrame>();
            (::windows::core::Interface::vtable(this).OutputFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ProcessVideoFrameContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessVideoFrameContext {}
impl ::core::fmt::Debug for ProcessVideoFrameContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessVideoFrameContext").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ProcessVideoFrameContext {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.ProcessVideoFrameContext;{276f0e2b-6461-401e-ba78-0fdad6114eec})");
}
impl ::core::clone::Clone for ProcessVideoFrameContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ProcessVideoFrameContext {
    const IID: ::windows::core::GUID = <IProcessVideoFrameContext as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ProcessVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessVideoFrameContext";
}
::windows::imp::interface_hierarchy!(ProcessVideoFrameContext, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProcessVideoFrameContext {}
unsafe impl ::core::marker::Sync for ProcessVideoFrameContext {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct SlowMotionEffectDefinition(::windows::core::IUnknown);
impl SlowMotionEffectDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SlowMotionEffectDefinition, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TimeStretchRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).TimeStretchRate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTimeStretchRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTimeStretchRate)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::ComInterface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SlowMotionEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlowMotionEffectDefinition {}
impl ::core::fmt::Debug for SlowMotionEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlowMotionEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SlowMotionEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.SlowMotionEffectDefinition;{35053cd0-176c-4763-82c4-1b02dbe31737})");
}
impl ::core::clone::Clone for SlowMotionEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SlowMotionEffectDefinition {
    const IID: ::windows::core::GUID = <ISlowMotionEffectDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SlowMotionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.SlowMotionEffectDefinition";
}
::windows::imp::interface_hierarchy!(SlowMotionEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IVideoEffectDefinition> for SlowMotionEffectDefinition {}
unsafe impl ::core::marker::Send for SlowMotionEffectDefinition {}
unsafe impl ::core::marker::Sync for SlowMotionEffectDefinition {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct VideoCompositorDefinition(::windows::core::IUnknown);
impl VideoCompositorDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<VideoCompositorDefinition> {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<VideoCompositorDefinition>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<P0>(activatableclassid: &::windows::core::HSTRING, props: P0) -> ::windows::core::Result<VideoCompositorDefinition>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<VideoCompositorDefinition>();
            (::windows::core::Interface::vtable(this).CreateWithProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), props.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoCompositorDefinitionFactory<R, F: FnOnce(&IVideoCompositorDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<VideoCompositorDefinition, IVideoCompositorDefinitionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VideoCompositorDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoCompositorDefinition {}
impl ::core::fmt::Debug for VideoCompositorDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoCompositorDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VideoCompositorDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoCompositorDefinition;{7946b8d0-2010-4ae3-9ab2-2cef42edd4d2})");
}
impl ::core::clone::Clone for VideoCompositorDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for VideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for VideoCompositorDefinition {
    const IID: ::windows::core::GUID = <IVideoCompositorDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for VideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoCompositorDefinition";
}
::windows::imp::interface_hierarchy!(VideoCompositorDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IVideoCompositorDefinition> for VideoCompositorDefinition {}
unsafe impl ::core::marker::Send for VideoCompositorDefinition {}
unsafe impl ::core::marker::Sync for VideoCompositorDefinition {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct VideoEffectDefinition(::windows::core::IUnknown);
impl VideoEffectDefinition {
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<VideoEffectDefinition> {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<VideoEffectDefinition>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<P0>(activatableclassid: &::windows::core::HSTRING, props: P0) -> ::windows::core::Result<VideoEffectDefinition>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<VideoEffectDefinition>();
            (::windows::core::Interface::vtable(this).CreateWithProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), props.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoEffectDefinitionFactory<R, F: FnOnce(&IVideoEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<VideoEffectDefinition, IVideoEffectDefinitionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VideoEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoEffectDefinition {}
impl ::core::fmt::Debug for VideoEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VideoEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
impl ::core::clone::Clone for VideoEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for VideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for VideoEffectDefinition {
    const IID: ::windows::core::GUID = <IVideoEffectDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for VideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoEffectDefinition";
}
::windows::imp::interface_hierarchy!(VideoEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IVideoEffectDefinition> for VideoEffectDefinition {}
unsafe impl ::core::marker::Send for VideoEffectDefinition {}
unsafe impl ::core::marker::Sync for VideoEffectDefinition {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct VideoTransformEffectDefinition(::windows::core::IUnknown);
impl VideoTransformEffectDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<VideoTransformEffectDefinition, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn PaddingColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Color>();
            (::windows::core::Interface::vtable(this).PaddingColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetPaddingColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPaddingColor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OutputSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).OutputSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOutputSize(&self, value: super::super::Foundation::Size) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutputSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CropRectangle(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).CropRectangle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCropRectangle(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCropRectangle)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Rotation(&self) -> ::windows::core::Result<super::MediaProperties::MediaRotation> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::MediaRotation>();
            (::windows::core::Interface::vtable(this).Rotation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRotation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Mirror(&self) -> ::windows::core::Result<super::MediaProperties::MediaMirroringOptions> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::MediaMirroringOptions>();
            (::windows::core::Interface::vtable(this).Mirror)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetMirror(&self, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMirror)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    #[cfg(feature = "Media_Transcoding")]
    pub fn SetProcessingAlgorithm(&self, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProcessingAlgorithm)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    #[cfg(feature = "Media_Transcoding")]
    pub fn ProcessingAlgorithm(&self) -> ::windows::core::Result<super::Transcoding::MediaVideoProcessingAlgorithm> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Transcoding::MediaVideoProcessingAlgorithm>();
            (::windows::core::Interface::vtable(this).ProcessingAlgorithm)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SphericalProjection(&self) -> ::windows::core::Result<VideoTransformSphericalProjection> {
        let this = &::windows::core::ComInterface::cast::<IVideoTransformEffectDefinition2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<VideoTransformSphericalProjection>();
            (::windows::core::Interface::vtable(this).SphericalProjection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoTransformEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTransformEffectDefinition {}
impl ::core::fmt::Debug for VideoTransformEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTransformEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VideoTransformEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoTransformEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
impl ::core::clone::Clone for VideoTransformEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for VideoTransformEffectDefinition {
    type Vtable = IVideoEffectDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for VideoTransformEffectDefinition {
    const IID: ::windows::core::GUID = <IVideoEffectDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for VideoTransformEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformEffectDefinition";
}
::windows::imp::interface_hierarchy!(VideoTransformEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IVideoEffectDefinition> for VideoTransformEffectDefinition {}
unsafe impl ::core::marker::Send for VideoTransformEffectDefinition {}
unsafe impl ::core::marker::Sync for VideoTransformEffectDefinition {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
pub struct VideoTransformSphericalProjection(::windows::core::IUnknown);
impl VideoTransformSphericalProjection {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameFormat(&self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::SphericalVideoFrameFormat>();
            (::windows::core::Interface::vtable(this).FrameFormat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFrameFormat)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Playback\"`*"]
    #[cfg(feature = "Media_Playback")]
    pub fn ProjectionMode(&self) -> ::windows::core::Result<super::Playback::SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Playback::SphericalVideoProjectionMode>();
            (::windows::core::Interface::vtable(this).ProjectionMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Playback\"`*"]
    #[cfg(feature = "Media_Playback")]
    pub fn SetProjectionMode(&self, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProjectionMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).HorizontalFieldOfViewInDegrees)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHorizontalFieldOfViewInDegrees)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ViewOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Numerics::Quaternion>();
            (::windows::core::Interface::vtable(this).ViewOrientation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetViewOrientation(&self, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetViewOrientation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for VideoTransformSphericalProjection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTransformSphericalProjection {}
impl ::core::fmt::Debug for VideoTransformSphericalProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTransformSphericalProjection").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VideoTransformSphericalProjection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Effects.VideoTransformSphericalProjection;{cf4401f0-9bf2-4c39-9f41-e022514a8468})");
}
impl ::core::clone::Clone for VideoTransformSphericalProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for VideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_Vtbl;
}
unsafe impl ::windows::core::ComInterface for VideoTransformSphericalProjection {
    const IID: ::windows::core::GUID = <IVideoTransformSphericalProjection as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for VideoTransformSphericalProjection {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformSphericalProjection";
}
::windows::imp::interface_hierarchy!(VideoTransformSphericalProjection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VideoTransformSphericalProjection {}
unsafe impl ::core::marker::Sync for VideoTransformSphericalProjection {}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioEffectType(pub i32);
impl AudioEffectType {
    pub const Other: Self = Self(0i32);
    pub const AcousticEchoCancellation: Self = Self(1i32);
    pub const NoiseSuppression: Self = Self(2i32);
    pub const AutomaticGainControl: Self = Self(3i32);
    pub const BeamForming: Self = Self(4i32);
    pub const ConstantToneRemoval: Self = Self(5i32);
    pub const Equalizer: Self = Self(6i32);
    pub const LoudnessEqualizer: Self = Self(7i32);
    pub const BassBoost: Self = Self(8i32);
    pub const VirtualSurround: Self = Self(9i32);
    pub const VirtualHeadphones: Self = Self(10i32);
    pub const SpeakerFill: Self = Self(11i32);
    pub const RoomCorrection: Self = Self(12i32);
    pub const BassManagement: Self = Self(13i32);
    pub const EnvironmentalEffects: Self = Self(14i32);
    pub const SpeakerProtection: Self = Self(15i32);
    pub const SpeakerCompensation: Self = Self(16i32);
    pub const DynamicRangeCompression: Self = Self(17i32);
    pub const FarFieldBeamForming: Self = Self(18i32);
    pub const DeepNoiseSuppression: Self = Self(19i32);
}
impl ::core::marker::Copy for AudioEffectType {}
impl ::core::clone::Clone for AudioEffectType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioEffectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioEffectType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioEffectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEffectType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioEffectType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.AudioEffectType;i4)");
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaEffectClosedReason(pub i32);
impl MediaEffectClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const UnsupportedEncodingFormat: Self = Self(2i32);
    pub const EffectCurrentlyUnloaded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaEffectClosedReason {}
impl ::core::clone::Clone for MediaEffectClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaEffectClosedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MediaEffectClosedReason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MediaEffectClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaEffectClosedReason").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MediaEffectClosedReason {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaEffectClosedReason;i4)");
}
#[doc = "*Required features: `\"Media_Effects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaMemoryTypes(pub i32);
impl MediaMemoryTypes {
    pub const Gpu: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const GpuAndCpu: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaMemoryTypes {}
impl ::core::clone::Clone for MediaMemoryTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaMemoryTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MediaMemoryTypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MediaMemoryTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaMemoryTypes").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MediaMemoryTypes {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaMemoryTypes;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
