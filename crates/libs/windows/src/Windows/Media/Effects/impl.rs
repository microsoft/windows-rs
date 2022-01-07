#[cfg(feature = "implement_exclusive")]
pub trait IAudioCaptureEffectsManagerImpl: Sized {
    fn AudioCaptureEffectsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioCaptureEffectsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAudioCaptureEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioCaptureEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.IAudioCaptureEffectsManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioCaptureEffectsManagerVtbl {
    pub const fn new<Impl: IAudioCaptureEffectsManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioCaptureEffectsManagerVtbl {
        unsafe extern "system" fn AudioCaptureEffectsChanged<Impl: IAudioCaptureEffectsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioCaptureEffectsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioCaptureEffectsChanged<Impl: IAudioCaptureEffectsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAudioCaptureEffectsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAudioCaptureEffects<Impl: IAudioCaptureEffectsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAudioCaptureEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioCaptureEffectsManager>, base.5, AudioCaptureEffectsChanged::<Impl, OFFSET>, RemoveAudioCaptureEffectsChanged::<Impl, OFFSET>, GetAudioCaptureEffects::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEffectImpl: Sized {
    fn AudioEffectType(&self) -> ::windows::core::Result<AudioEffectType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffect";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEffectVtbl {
    pub const fn new<Impl: IAudioEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEffectVtbl {
        unsafe extern "system" fn AudioEffectType<Impl: IAudioEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioEffectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioEffectType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEffect>, base.5, AudioEffectType::<Impl, OFFSET>)
    }
}
pub trait IAudioEffectDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
impl ::windows::core::RuntimeName for IAudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectDefinition";
}
impl IAudioEffectDefinitionVtbl {
    pub const fn new<Impl: IAudioEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEffectDefinitionVtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IAudioEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IAudioEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEffectDefinition>, base.5, ActivatableClassId::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioEffectDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<AudioEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEffectDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectDefinitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEffectDefinitionFactoryVtbl {
    pub const fn new<Impl: IAudioEffectDefinitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEffectDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAudioEffectDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithProperties<Impl: IAudioEffectDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithProperties(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&props as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEffectDefinitionFactory>, base.5, Create::<Impl, OFFSET>, CreateWithProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEffectsManagerStaticsImpl: Sized {
    fn CreateAudioRenderEffectsManager(&self, deviceid: &::windows::core::HSTRING, category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioRenderEffectsManager>;
    fn CreateAudioRenderEffectsManagerWithMode(&self, deviceid: &::windows::core::HSTRING, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioRenderEffectsManager>;
    fn CreateAudioCaptureEffectsManager(&self, deviceid: &::windows::core::HSTRING, category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioCaptureEffectsManager>;
    fn CreateAudioCaptureEffectsManagerWithMode(&self, deviceid: &::windows::core::HSTRING, category: super::Capture::MediaCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioCaptureEffectsManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEffectsManagerStatics {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectsManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEffectsManagerStaticsVtbl {
    pub const fn new<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEffectsManagerStaticsVtbl {
        unsafe extern "system" fn CreateAudioRenderEffectsManager<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAudioRenderEffectsManager(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioRenderEffectsManagerWithMode<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAudioRenderEffectsManagerWithMode(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category, mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioCaptureEffectsManager<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAudioCaptureEffectsManager(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioCaptureEffectsManagerWithMode<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Capture::MediaCategory, mode: super::AudioProcessing, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAudioCaptureEffectsManagerWithMode(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category, mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEffectsManagerStatics>, base.5, CreateAudioRenderEffectsManager::<Impl, OFFSET>, CreateAudioRenderEffectsManagerWithMode::<Impl, OFFSET>, CreateAudioCaptureEffectsManager::<Impl, OFFSET>, CreateAudioCaptureEffectsManagerWithMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioRenderEffectsManagerImpl: Sized {
    fn AudioRenderEffectsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioRenderEffectsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAudioRenderEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioRenderEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.IAudioRenderEffectsManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioRenderEffectsManagerVtbl {
    pub const fn new<Impl: IAudioRenderEffectsManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioRenderEffectsManagerVtbl {
        unsafe extern "system" fn AudioRenderEffectsChanged<Impl: IAudioRenderEffectsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioRenderEffectsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioRenderEffectsChanged<Impl: IAudioRenderEffectsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAudioRenderEffectsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAudioRenderEffects<Impl: IAudioRenderEffectsManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAudioRenderEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioRenderEffectsManager>, base.5, AudioRenderEffectsChanged::<Impl, OFFSET>, RemoveAudioRenderEffectsChanged::<Impl, OFFSET>, GetAudioRenderEffects::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAudioRenderEffectsManager2Impl: Sized {
    fn EffectsProviderThumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn EffectsProviderSettingsLabel(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShowSettingsUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioRenderEffectsManager2 {
    const NAME: &'static str = "Windows.Media.Effects.IAudioRenderEffectsManager2";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IAudioRenderEffectsManager2Vtbl {
    pub const fn new<Impl: IAudioRenderEffectsManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioRenderEffectsManager2Vtbl {
        unsafe extern "system" fn EffectsProviderThumbnail<Impl: IAudioRenderEffectsManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EffectsProviderThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EffectsProviderSettingsLabel<Impl: IAudioRenderEffectsManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EffectsProviderSettingsLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowSettingsUI<Impl: IAudioRenderEffectsManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowSettingsUI().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioRenderEffectsManager2>, base.5, EffectsProviderThumbnail::<Impl, OFFSET>, EffectsProviderSettingsLabel::<Impl, OFFSET>, ShowSettingsUI::<Impl, OFFSET>)
    }
}
pub trait IBasicAudioEffectImpl: Sized + IMediaExtensionImpl {
    fn UseInputFrameForOutput(&self) -> ::windows::core::Result<bool>;
    fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>;
    fn SetEncodingProperties(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn ProcessFrame(&self, context: &::core::option::Option<ProcessAudioFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBasicAudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.IBasicAudioEffect";
}
impl IBasicAudioEffectVtbl {
    pub const fn new<Impl: IBasicAudioEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBasicAudioEffectVtbl {
        unsafe extern "system" fn UseInputFrameForOutput<Impl: IBasicAudioEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseInputFrameForOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncodingProperties<Impl: IBasicAudioEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Impl: IBasicAudioEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEncodingProperties(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessFrame<Impl: IBasicAudioEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProcessFrame(&*(&context as *const <ProcessAudioFrameContext as ::windows::core::Abi>::Abi as *const <ProcessAudioFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IBasicAudioEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IBasicAudioEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBasicAudioEffect>, base.5, UseInputFrameForOutput::<Impl, OFFSET>, SupportedEncodingProperties::<Impl, OFFSET>, SetEncodingProperties::<Impl, OFFSET>, ProcessFrame::<Impl, OFFSET>, Close::<Impl, OFFSET>, DiscardQueuedFrames::<Impl, OFFSET>)
    }
}
pub trait IBasicVideoEffectImpl: Sized + IMediaExtensionImpl {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SupportedMemoryTypes(&self) -> ::windows::core::Result<MediaMemoryTypes>;
    fn TimeIndependent(&self) -> ::windows::core::Result<bool>;
    fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>;
    fn SetEncodingProperties(&self, encodingproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn ProcessFrame(&self, context: &::core::option::Option<ProcessVideoFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBasicVideoEffect {
    const NAME: &'static str = "Windows.Media.Effects.IBasicVideoEffect";
}
impl IBasicVideoEffectVtbl {
    pub const fn new<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBasicVideoEffectVtbl {
        unsafe extern "system" fn IsReadOnly<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMemoryTypes<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaMemoryTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedMemoryTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeIndependent<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimeIndependent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncodingProperties<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .SetEncodingProperties(&*(&encodingproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&device as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn ProcessFrame<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProcessFrame(&*(&context as *const <ProcessVideoFrameContext as ::windows::core::Abi>::Abi as *const <ProcessVideoFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IBasicVideoEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBasicVideoEffect>, base.5, IsReadOnly::<Impl, OFFSET>, SupportedMemoryTypes::<Impl, OFFSET>, TimeIndependent::<Impl, OFFSET>, SupportedEncodingProperties::<Impl, OFFSET>, SetEncodingProperties::<Impl, OFFSET>, ProcessFrame::<Impl, OFFSET>, Close::<Impl, OFFSET>, DiscardQueuedFrames::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeVideoFrameContextImpl: Sized {
    fn SurfacesToOverlay(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>;
    fn BackgroundFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn GetOverlayForSurface(&self, surfacetooverlay: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<super::Editing::MediaOverlay>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositeVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ICompositeVideoFrameContext";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositeVideoFrameContextVtbl {
    pub const fn new<Impl: ICompositeVideoFrameContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositeVideoFrameContextVtbl {
        unsafe extern "system" fn SurfacesToOverlay<Impl: ICompositeVideoFrameContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SurfacesToOverlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundFrame<Impl: ICompositeVideoFrameContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BackgroundFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputFrame<Impl: ICompositeVideoFrameContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayForSurface<Impl: ICompositeVideoFrameContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surfacetooverlay: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOverlayForSurface(&*(&surfacetooverlay as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositeVideoFrameContext>, base.5, SurfacesToOverlay::<Impl, OFFSET>, BackgroundFrame::<Impl, OFFSET>, OutputFrame::<Impl, OFFSET>, GetOverlayForSurface::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessAudioFrameContextImpl: Sized {
    fn InputFrame(&self) -> ::windows::core::Result<super::AudioFrame>;
    fn OutputFrame(&self) -> ::windows::core::Result<super::AudioFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessAudioFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.IProcessAudioFrameContext";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessAudioFrameContextVtbl {
    pub const fn new<Impl: IProcessAudioFrameContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessAudioFrameContextVtbl {
        unsafe extern "system" fn InputFrame<Impl: IProcessAudioFrameContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputFrame<Impl: IProcessAudioFrameContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessAudioFrameContext>, base.5, InputFrame::<Impl, OFFSET>, OutputFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessVideoFrameContextImpl: Sized {
    fn InputFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.IProcessVideoFrameContext";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessVideoFrameContextVtbl {
    pub const fn new<Impl: IProcessVideoFrameContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessVideoFrameContextVtbl {
        unsafe extern "system" fn InputFrame<Impl: IProcessVideoFrameContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputFrame<Impl: IProcessVideoFrameContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessVideoFrameContext>, base.5, InputFrame::<Impl, OFFSET>, OutputFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlowMotionEffectDefinitionImpl: Sized + IVideoEffectDefinitionImpl {
    fn TimeStretchRate(&self) -> ::windows::core::Result<f64>;
    fn SetTimeStretchRate(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISlowMotionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.ISlowMotionEffectDefinition";
}
#[cfg(feature = "implement_exclusive")]
impl ISlowMotionEffectDefinitionVtbl {
    pub const fn new<Impl: ISlowMotionEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISlowMotionEffectDefinitionVtbl {
        unsafe extern "system" fn TimeStretchRate<Impl: ISlowMotionEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimeStretchRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeStretchRate<Impl: ISlowMotionEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTimeStretchRate(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISlowMotionEffectDefinition>, base.5, TimeStretchRate::<Impl, OFFSET>, SetTimeStretchRate::<Impl, OFFSET>)
    }
}
pub trait IVideoCompositorImpl: Sized + IMediaExtensionImpl {
    fn TimeIndependent(&self) -> ::windows::core::Result<bool>;
    fn SetEncodingProperties(&self, backgroundproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn CompositeFrame(&self, context: &::core::option::Option<CompositeVideoFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVideoCompositor {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositor";
}
impl IVideoCompositorVtbl {
    pub const fn new<Impl: IVideoCompositorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoCompositorVtbl {
        unsafe extern "system" fn TimeIndependent<Impl: IVideoCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimeIndependent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Impl: IVideoCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, backgroundproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .SetEncodingProperties(&*(&backgroundproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&device as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn CompositeFrame<Impl: IVideoCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CompositeFrame(&*(&context as *const <CompositeVideoFrameContext as ::windows::core::Abi>::Abi as *const <CompositeVideoFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IVideoCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IVideoCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoCompositor>, base.5, TimeIndependent::<Impl, OFFSET>, SetEncodingProperties::<Impl, OFFSET>, CompositeFrame::<Impl, OFFSET>, Close::<Impl, OFFSET>, DiscardQueuedFrames::<Impl, OFFSET>)
    }
}
pub trait IVideoCompositorDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
impl ::windows::core::RuntimeName for IVideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositorDefinition";
}
impl IVideoCompositorDefinitionVtbl {
    pub const fn new<Impl: IVideoCompositorDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoCompositorDefinitionVtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IVideoCompositorDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IVideoCompositorDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoCompositorDefinition>, base.5, ActivatableClassId::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoCompositorDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<VideoCompositorDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<VideoCompositorDefinition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoCompositorDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositorDefinitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoCompositorDefinitionFactoryVtbl {
    pub const fn new<Impl: IVideoCompositorDefinitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoCompositorDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVideoCompositorDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithProperties<Impl: IVideoCompositorDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithProperties(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&props as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoCompositorDefinitionFactory>, base.5, Create::<Impl, OFFSET>, CreateWithProperties::<Impl, OFFSET>)
    }
}
pub trait IVideoEffectDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
impl ::windows::core::RuntimeName for IVideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoEffectDefinition";
}
impl IVideoEffectDefinitionVtbl {
    pub const fn new<Impl: IVideoEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEffectDefinitionVtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IVideoEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IVideoEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEffectDefinition>, base.5, ActivatableClassId::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<VideoEffectDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<VideoEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEffectDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Effects.IVideoEffectDefinitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEffectDefinitionFactoryVtbl {
    pub const fn new<Impl: IVideoEffectDefinitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEffectDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVideoEffectDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithProperties<Impl: IVideoEffectDefinitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithProperties(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&props as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEffectDefinitionFactory>, base.5, Create::<Impl, OFFSET>, CreateWithProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTransformEffectDefinitionImpl: Sized + IVideoEffectDefinitionImpl {
    fn PaddingColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetPaddingColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn OutputSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetOutputSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn CropRectangle(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetCropRectangle(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<super::MediaProperties::MediaRotation>;
    fn SetRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()>;
    fn Mirror(&self) -> ::windows::core::Result<super::MediaProperties::MediaMirroringOptions>;
    fn SetMirror(&self, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::core::Result<()>;
    fn SetProcessingAlgorithm(&self, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()>;
    fn ProcessingAlgorithm(&self) -> ::windows::core::Result<super::Transcoding::MediaVideoProcessingAlgorithm>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTransformEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoTransformEffectDefinition";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTransformEffectDefinitionVtbl {
    pub const fn new<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoTransformEffectDefinitionVtbl {
        unsafe extern "system" fn PaddingColor<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PaddingColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPaddingColor<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPaddingColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutputSize<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutputSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputSize<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutputSize(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CropRectangle<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CropRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCropRectangle<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCropRectangle(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rotation<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn Mirror<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaMirroringOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mirror() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMirror<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMirror(value).into()
        }
        unsafe extern "system" fn SetProcessingAlgorithm<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProcessingAlgorithm(value).into()
        }
        unsafe extern "system" fn ProcessingAlgorithm<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessingAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVideoTransformEffectDefinition>,
            base.5,
            PaddingColor::<Impl, OFFSET>,
            SetPaddingColor::<Impl, OFFSET>,
            OutputSize::<Impl, OFFSET>,
            SetOutputSize::<Impl, OFFSET>,
            CropRectangle::<Impl, OFFSET>,
            SetCropRectangle::<Impl, OFFSET>,
            Rotation::<Impl, OFFSET>,
            SetRotation::<Impl, OFFSET>,
            Mirror::<Impl, OFFSET>,
            SetMirror::<Impl, OFFSET>,
            SetProcessingAlgorithm::<Impl, OFFSET>,
            ProcessingAlgorithm::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTransformEffectDefinition2Impl: Sized {
    fn SphericalProjection(&self) -> ::windows::core::Result<VideoTransformSphericalProjection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTransformEffectDefinition2 {
    const NAME: &'static str = "Windows.Media.Effects.IVideoTransformEffectDefinition2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTransformEffectDefinition2Vtbl {
    pub const fn new<Impl: IVideoTransformEffectDefinition2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoTransformEffectDefinition2Vtbl {
        unsafe extern "system" fn SphericalProjection<Impl: IVideoTransformEffectDefinition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SphericalProjection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoTransformEffectDefinition2>, base.5, SphericalProjection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTransformSphericalProjectionImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn FrameFormat(&self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat>;
    fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()>;
    fn ProjectionMode(&self) -> ::windows::core::Result<super::Playback::SphericalVideoProjectionMode>;
    fn SetProjectionMode(&self, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::core::Result<()>;
    fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
    fn ViewOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetViewOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTransformSphericalProjection {
    const NAME: &'static str = "Windows.Media.Effects.IVideoTransformSphericalProjection";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTransformSphericalProjectionVtbl {
    pub const fn new<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoTransformSphericalProjectionVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn FrameFormat<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameFormat<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrameFormat(value).into()
        }
        unsafe extern "system" fn ProjectionMode<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Playback::SphericalVideoProjectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProjectionMode<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProjectionMode(value).into()
        }
        unsafe extern "system" fn HorizontalFieldOfViewInDegrees<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalFieldOfViewInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalFieldOfViewInDegrees<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalFieldOfViewInDegrees(value).into()
        }
        unsafe extern "system" fn ViewOrientation<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewOrientation<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetViewOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoTransformSphericalProjection>, base.5, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>, FrameFormat::<Impl, OFFSET>, SetFrameFormat::<Impl, OFFSET>, ProjectionMode::<Impl, OFFSET>, SetProjectionMode::<Impl, OFFSET>, HorizontalFieldOfViewInDegrees::<Impl, OFFSET>, SetHorizontalFieldOfViewInDegrees::<Impl, OFFSET>, ViewOrientation::<Impl, OFFSET>, SetViewOrientation::<Impl, OFFSET>)
    }
}
