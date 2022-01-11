#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAudioCaptureEffectsManagerImpl: Sized {
    fn AudioCaptureEffectsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioCaptureEffectsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAudioCaptureEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioCaptureEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.IAudioCaptureEffectsManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAudioCaptureEffectsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioCaptureEffectsManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioCaptureEffectsManagerVtbl {
        unsafe extern "system" fn AudioCaptureEffectsChanged<Impl: IAudioCaptureEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioCaptureEffectsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioCaptureEffectsChanged<Impl: IAudioCaptureEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioCaptureEffectsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAudioCaptureEffects<Impl: IAudioCaptureEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioCaptureEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioCaptureEffectsManager>, ::windows::core::GetTrustLevel, AudioCaptureEffectsChanged::<Impl, IMPL_OFFSET>, RemoveAudioCaptureEffectsChanged::<Impl, IMPL_OFFSET>, GetAudioCaptureEffects::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioCaptureEffectsManager as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEffectVtbl {
        unsafe extern "system" fn AudioEffectType<Impl: IAudioEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AudioEffectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEffectType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEffect>, ::windows::core::GetTrustLevel, AudioEffectType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IAudioEffectDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IAudioEffectDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEffectDefinitionVtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IAudioEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IAudioEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEffectDefinition>, ::windows::core::GetTrustLevel, ActivatableClassId::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAudioEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioEffectDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<AudioEffectDefinition>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioEffectDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectDefinitionFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAudioEffectDefinitionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectDefinitionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEffectDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAudioEffectDefinitionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithProperties<Impl: IAudioEffectDefinitionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithProperties(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&props as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEffectDefinitionFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectDefinitionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Capture", feature = "Media_Render", feature = "implement_exclusive"))]
pub trait IAudioEffectsManagerStaticsImpl: Sized {
    fn CreateAudioRenderEffectsManager(&self, deviceid: &::windows::core::HSTRING, category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioRenderEffectsManager>;
    fn CreateAudioRenderEffectsManagerWithMode(&self, deviceid: &::windows::core::HSTRING, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioRenderEffectsManager>;
    fn CreateAudioCaptureEffectsManager(&self, deviceid: &::windows::core::HSTRING, category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioCaptureEffectsManager>;
    fn CreateAudioCaptureEffectsManagerWithMode(&self, deviceid: &::windows::core::HSTRING, category: super::Capture::MediaCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioCaptureEffectsManager>;
}
#[cfg(all(feature = "Media_Capture", feature = "Media_Render", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioEffectsManagerStatics {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectsManagerStatics";
}
#[cfg(all(feature = "Media_Capture", feature = "Media_Render", feature = "implement_exclusive"))]
impl IAudioEffectsManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectsManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEffectsManagerStaticsVtbl {
        unsafe extern "system" fn CreateAudioRenderEffectsManager<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAudioRenderEffectsManager(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioRenderEffectsManagerWithMode<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAudioRenderEffectsManagerWithMode(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category, mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioCaptureEffectsManager<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAudioCaptureEffectsManager(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioCaptureEffectsManagerWithMode<Impl: IAudioEffectsManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: super::Capture::MediaCategory, mode: super::AudioProcessing, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAudioCaptureEffectsManagerWithMode(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), category, mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAudioEffectsManagerStatics>,
            ::windows::core::GetTrustLevel,
            CreateAudioRenderEffectsManager::<Impl, IMPL_OFFSET>,
            CreateAudioRenderEffectsManagerWithMode::<Impl, IMPL_OFFSET>,
            CreateAudioCaptureEffectsManager::<Impl, IMPL_OFFSET>,
            CreateAudioCaptureEffectsManagerWithMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectsManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAudioRenderEffectsManagerImpl: Sized {
    fn AudioRenderEffectsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioRenderEffectsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAudioRenderEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioRenderEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.IAudioRenderEffectsManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAudioRenderEffectsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioRenderEffectsManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioRenderEffectsManagerVtbl {
        unsafe extern "system" fn AudioRenderEffectsChanged<Impl: IAudioRenderEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioRenderEffectsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAudioRenderEffectsChanged<Impl: IAudioRenderEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAudioRenderEffectsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAudioRenderEffects<Impl: IAudioRenderEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioRenderEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioRenderEffectsManager>, ::windows::core::GetTrustLevel, AudioRenderEffectsChanged::<Impl, IMPL_OFFSET>, RemoveAudioRenderEffectsChanged::<Impl, IMPL_OFFSET>, GetAudioRenderEffects::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioRenderEffectsManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAudioRenderEffectsManager2Impl: Sized {
    fn EffectsProviderThumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn EffectsProviderSettingsLabel(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShowSettingsUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioRenderEffectsManager2 {
    const NAME: &'static str = "Windows.Media.Effects.IAudioRenderEffectsManager2";
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl IAudioRenderEffectsManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioRenderEffectsManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioRenderEffectsManager2Vtbl {
        unsafe extern "system" fn EffectsProviderThumbnail<Impl: IAudioRenderEffectsManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectsProviderThumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EffectsProviderSettingsLabel<Impl: IAudioRenderEffectsManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectsProviderSettingsLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowSettingsUI<Impl: IAudioRenderEffectsManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowSettingsUI().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioRenderEffectsManager2>, ::windows::core::GetTrustLevel, EffectsProviderThumbnail::<Impl, IMPL_OFFSET>, EffectsProviderSettingsLabel::<Impl, IMPL_OFFSET>, ShowSettingsUI::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioRenderEffectsManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
pub trait IBasicAudioEffectImpl: Sized + IMediaExtensionImpl {
    fn UseInputFrameForOutput(&self) -> ::windows::core::Result<bool>;
    fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>;
    fn SetEncodingProperties(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn ProcessFrame(&self, context: &::core::option::Option<ProcessAudioFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IBasicAudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.IBasicAudioEffect";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
impl IBasicAudioEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBasicAudioEffectVtbl {
        unsafe extern "system" fn UseInputFrameForOutput<Impl: IBasicAudioEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseInputFrameForOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncodingProperties<Impl: IBasicAudioEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Impl: IBasicAudioEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncodingProperties(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessFrame<Impl: IBasicAudioEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessFrame(&*(&context as *const <ProcessAudioFrameContext as ::windows::core::Abi>::Abi as *const <ProcessAudioFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IBasicAudioEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IBasicAudioEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBasicAudioEffect>,
            ::windows::core::GetTrustLevel,
            UseInputFrameForOutput::<Impl, IMPL_OFFSET>,
            SupportedEncodingProperties::<Impl, IMPL_OFFSET>,
            SetEncodingProperties::<Impl, IMPL_OFFSET>,
            ProcessFrame::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            DiscardQueuedFrames::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicAudioEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
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
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IBasicVideoEffect {
    const NAME: &'static str = "Windows.Media.Effects.IBasicVideoEffect";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl IBasicVideoEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBasicVideoEffectVtbl {
        unsafe extern "system" fn IsReadOnly<Impl: IBasicVideoEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMemoryTypes<Impl: IBasicVideoEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaMemoryTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedMemoryTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeIndependent<Impl: IBasicVideoEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeIndependent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncodingProperties<Impl: IBasicVideoEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Impl: IBasicVideoEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetEncodingProperties(&*(&encodingproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&device as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn ProcessFrame<Impl: IBasicVideoEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessFrame(&*(&context as *const <ProcessVideoFrameContext as ::windows::core::Abi>::Abi as *const <ProcessVideoFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IBasicVideoEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IBasicVideoEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBasicVideoEffect>,
            ::windows::core::GetTrustLevel,
            IsReadOnly::<Impl, IMPL_OFFSET>,
            SupportedMemoryTypes::<Impl, IMPL_OFFSET>,
            TimeIndependent::<Impl, IMPL_OFFSET>,
            SupportedEncodingProperties::<Impl, IMPL_OFFSET>,
            SetEncodingProperties::<Impl, IMPL_OFFSET>,
            ProcessFrame::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            DiscardQueuedFrames::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicVideoEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing", feature = "implement_exclusive"))]
pub trait ICompositeVideoFrameContextImpl: Sized {
    fn SurfacesToOverlay(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>;
    fn BackgroundFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn GetOverlayForSurface(&self, surfacetooverlay: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<super::Editing::MediaOverlay>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompositeVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ICompositeVideoFrameContext";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing", feature = "implement_exclusive"))]
impl ICompositeVideoFrameContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositeVideoFrameContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositeVideoFrameContextVtbl {
        unsafe extern "system" fn SurfacesToOverlay<Impl: ICompositeVideoFrameContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SurfacesToOverlay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundFrame<Impl: ICompositeVideoFrameContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputFrame<Impl: ICompositeVideoFrameContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayForSurface<Impl: ICompositeVideoFrameContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surfacetooverlay: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverlayForSurface(&*(&surfacetooverlay as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompositeVideoFrameContext>, ::windows::core::GetTrustLevel, SurfacesToOverlay::<Impl, IMPL_OFFSET>, BackgroundFrame::<Impl, IMPL_OFFSET>, OutputFrame::<Impl, IMPL_OFFSET>, GetOverlayForSurface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositeVideoFrameContext as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessAudioFrameContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessAudioFrameContextVtbl {
        unsafe extern "system" fn InputFrame<Impl: IProcessAudioFrameContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputFrame<Impl: IProcessAudioFrameContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessAudioFrameContext>, ::windows::core::GetTrustLevel, InputFrame::<Impl, IMPL_OFFSET>, OutputFrame::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessAudioFrameContext as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessVideoFrameContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessVideoFrameContextVtbl {
        unsafe extern "system" fn InputFrame<Impl: IProcessVideoFrameContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputFrame<Impl: IProcessVideoFrameContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessVideoFrameContext>, ::windows::core::GetTrustLevel, InputFrame::<Impl, IMPL_OFFSET>, OutputFrame::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessVideoFrameContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISlowMotionEffectDefinitionImpl: Sized + IVideoEffectDefinitionImpl {
    fn TimeStretchRate(&self) -> ::windows::core::Result<f64>;
    fn SetTimeStretchRate(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISlowMotionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.ISlowMotionEffectDefinition";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISlowMotionEffectDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISlowMotionEffectDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISlowMotionEffectDefinitionVtbl {
        unsafe extern "system" fn TimeStretchRate<Impl: ISlowMotionEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeStretchRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeStretchRate<Impl: ISlowMotionEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeStretchRate(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISlowMotionEffectDefinition>, ::windows::core::GetTrustLevel, TimeStretchRate::<Impl, IMPL_OFFSET>, SetTimeStretchRate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISlowMotionEffectDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IVideoCompositorImpl: Sized + IMediaExtensionImpl {
    fn TimeIndependent(&self) -> ::windows::core::Result<bool>;
    fn SetEncodingProperties(&self, backgroundproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn CompositeFrame(&self, context: &::core::option::Option<CompositeVideoFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IVideoCompositor {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositor";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl IVideoCompositorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoCompositorVtbl {
        unsafe extern "system" fn TimeIndependent<Impl: IVideoCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeIndependent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Impl: IVideoCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backgroundproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetEncodingProperties(&*(&backgroundproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&device as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn CompositeFrame<Impl: IVideoCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompositeFrame(&*(&context as *const <CompositeVideoFrameContext as ::windows::core::Abi>::Abi as *const <CompositeVideoFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IVideoCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IVideoCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoCompositor>, ::windows::core::GetTrustLevel, TimeIndependent::<Impl, IMPL_OFFSET>, SetEncodingProperties::<Impl, IMPL_OFFSET>, CompositeFrame::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>, DiscardQueuedFrames::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoCompositor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoCompositorDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositorDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IVideoCompositorDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositorDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoCompositorDefinitionVtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IVideoCompositorDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IVideoCompositorDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoCompositorDefinition>, ::windows::core::GetTrustLevel, ActivatableClassId::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoCompositorDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVideoCompositorDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<VideoCompositorDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<VideoCompositorDefinition>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoCompositorDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositorDefinitionFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVideoCompositorDefinitionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositorDefinitionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoCompositorDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVideoCompositorDefinitionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithProperties<Impl: IVideoCompositorDefinitionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithProperties(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&props as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoCompositorDefinitionFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoCompositorDefinitionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoEffectDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoEffectDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IVideoEffectDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEffectDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEffectDefinitionVtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IVideoEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IVideoEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoEffectDefinition>, ::windows::core::GetTrustLevel, ActivatableClassId::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEffectDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVideoEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<VideoEffectDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<VideoEffectDefinition>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoEffectDefinitionFactory {
    const NAME: &'static str = "Windows.Media.Effects.IVideoEffectDefinitionFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVideoEffectDefinitionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEffectDefinitionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEffectDefinitionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVideoEffectDefinitionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithProperties<Impl: IVideoEffectDefinitionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithProperties(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&props as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoEffectDefinitionFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEffectDefinitionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "UI", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoTransformEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoTransformEffectDefinition";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "UI", feature = "implement_exclusive"))]
impl IVideoTransformEffectDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoTransformEffectDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoTransformEffectDefinitionVtbl {
        unsafe extern "system" fn PaddingColor<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PaddingColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPaddingColor<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPaddingColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutputSize<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputSize<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputSize(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CropRectangle<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CropRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCropRectangle<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCropRectangle(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rotation<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn Mirror<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaMirroringOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mirror() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMirror<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMirror(value).into()
        }
        unsafe extern "system" fn SetProcessingAlgorithm<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProcessingAlgorithm(value).into()
        }
        unsafe extern "system" fn ProcessingAlgorithm<Impl: IVideoTransformEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVideoTransformEffectDefinition>,
            ::windows::core::GetTrustLevel,
            PaddingColor::<Impl, IMPL_OFFSET>,
            SetPaddingColor::<Impl, IMPL_OFFSET>,
            OutputSize::<Impl, IMPL_OFFSET>,
            SetOutputSize::<Impl, IMPL_OFFSET>,
            CropRectangle::<Impl, IMPL_OFFSET>,
            SetCropRectangle::<Impl, IMPL_OFFSET>,
            Rotation::<Impl, IMPL_OFFSET>,
            SetRotation::<Impl, IMPL_OFFSET>,
            Mirror::<Impl, IMPL_OFFSET>,
            SetMirror::<Impl, IMPL_OFFSET>,
            SetProcessingAlgorithm::<Impl, IMPL_OFFSET>,
            ProcessingAlgorithm::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoTransformEffectDefinition as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoTransformEffectDefinition2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoTransformEffectDefinition2Vtbl {
        unsafe extern "system" fn SphericalProjection<Impl: IVideoTransformEffectDefinition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SphericalProjection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoTransformEffectDefinition2>, ::windows::core::GetTrustLevel, SphericalProjection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoTransformEffectDefinition2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoTransformSphericalProjection {
    const NAME: &'static str = "Windows.Media.Effects.IVideoTransformSphericalProjection";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
impl IVideoTransformSphericalProjectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoTransformSphericalProjectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoTransformSphericalProjectionVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn FrameFormat<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameFormat<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameFormat(value).into()
        }
        unsafe extern "system" fn ProjectionMode<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Playback::SphericalVideoProjectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProjectionMode<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProjectionMode(value).into()
        }
        unsafe extern "system" fn HorizontalFieldOfViewInDegrees<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalFieldOfViewInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalFieldOfViewInDegrees<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalFieldOfViewInDegrees(value).into()
        }
        unsafe extern "system" fn ViewOrientation<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewOrientation<Impl: IVideoTransformSphericalProjectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewOrientation(&*(&value as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVideoTransformSphericalProjection>,
            ::windows::core::GetTrustLevel,
            IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled::<Impl, IMPL_OFFSET>,
            FrameFormat::<Impl, IMPL_OFFSET>,
            SetFrameFormat::<Impl, IMPL_OFFSET>,
            ProjectionMode::<Impl, IMPL_OFFSET>,
            SetProjectionMode::<Impl, IMPL_OFFSET>,
            HorizontalFieldOfViewInDegrees::<Impl, IMPL_OFFSET>,
            SetHorizontalFieldOfViewInDegrees::<Impl, IMPL_OFFSET>,
            ViewOrientation::<Impl, IMPL_OFFSET>,
            SetViewOrientation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoTransformSphericalProjection as ::windows::core::Interface>::IID
    }
}
