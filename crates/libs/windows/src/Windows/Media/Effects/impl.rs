#[cfg(feature = "Foundation_Collections")]
pub trait IAudioEffectDefinition_Impl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IAudioEffectDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectDefinition_Impl, const OFFSET: isize>() -> IAudioEffectDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioEffectDefinition, OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
pub trait IBasicAudioEffect_Impl: Sized + super::IMediaExtension_Impl {
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
impl IBasicAudioEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffect_Impl, const OFFSET: isize>() -> IBasicAudioEffect_Vtbl {
        unsafe extern "system" fn UseInputFrameForOutput<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UseInputFrameForOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncodingProperties<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEncodingProperties(::core::mem::transmute(&encodingproperties)).into()
        }
        unsafe extern "system" fn ProcessFrame<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessFrame(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBasicAudioEffect, OFFSET>(),
            UseInputFrameForOutput: UseInputFrameForOutput::<Identity, Impl, OFFSET>,
            SupportedEncodingProperties: SupportedEncodingProperties::<Identity, Impl, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, Impl, OFFSET>,
            ProcessFrame: ProcessFrame::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicAudioEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IBasicVideoEffect_Impl: Sized + super::IMediaExtension_Impl {
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
impl IBasicVideoEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>() -> IBasicVideoEffect_Vtbl {
        unsafe extern "system" fn IsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMemoryTypes<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaMemoryTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedMemoryTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeIndependent<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TimeIndependent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedEncodingProperties<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEncodingProperties(::core::mem::transmute(&encodingproperties), ::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn ProcessFrame<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessFrame(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBasicVideoEffect, OFFSET>(),
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            SupportedMemoryTypes: SupportedMemoryTypes::<Identity, Impl, OFFSET>,
            TimeIndependent: TimeIndependent::<Identity, Impl, OFFSET>,
            SupportedEncodingProperties: SupportedEncodingProperties::<Identity, Impl, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, Impl, OFFSET>,
            ProcessFrame: ProcessFrame::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicVideoEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IVideoCompositor_Impl: Sized + super::IMediaExtension_Impl {
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
impl IVideoCompositor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositor_Impl, const OFFSET: isize>() -> IVideoCompositor_Vtbl {
        unsafe extern "system" fn TimeIndependent<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TimeIndependent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingProperties<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backgroundproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEncodingProperties(::core::mem::transmute(&backgroundproperties), ::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn CompositeFrame<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompositeFrame(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoCompositor, OFFSET>(),
            TimeIndependent: TimeIndependent::<Identity, Impl, OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Identity, Impl, OFFSET>,
            CompositeFrame: CompositeFrame::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoCompositor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoCompositorDefinition_Impl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositorDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IVideoCompositorDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositorDefinition_Impl, const OFFSET: isize>() -> IVideoCompositorDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositorDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositorDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoCompositorDefinition, OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoCompositorDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoEffectDefinition_Impl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoEffectDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IVideoEffectDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEffectDefinition_Impl, const OFFSET: isize>() -> IVideoEffectDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEffectDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActivatableClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEffectDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEffectDefinition, OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEffectDefinition as ::windows::core::Interface>::IID
    }
}
