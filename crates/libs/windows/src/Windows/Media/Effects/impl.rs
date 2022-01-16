#[cfg(feature = "Foundation_Collections")]
pub trait IAudioEffectDefinition_Impl: Sized {
    fn ActivatableClassId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IAudioEffectDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IAudioEffectDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEffectDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IAudioEffectDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IAudioEffectDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioEffectDefinition, BASE_OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
pub trait IBasicAudioEffect_Impl: Sized + super::IMediaExtension_Impl {
    fn UseInputFrameForOutput(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedEncodingProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>;
    fn SetEncodingProperties(&mut self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn ProcessFrame(&mut self, context: &::core::option::Option<ProcessAudioFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&mut self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IBasicAudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.IBasicAudioEffect";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
impl IBasicAudioEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicAudioEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBasicAudioEffect_Vtbl {
        unsafe extern "system" fn UseInputFrameForOutput<Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedEncodingProperties<Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEncodingProperties<Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncodingProperties(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessFrame<Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessFrame(&*(&context as *const <ProcessAudioFrameContext as ::windows::core::Abi>::Abi as *const <ProcessAudioFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IBasicAudioEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBasicAudioEffect, BASE_OFFSET>(),
            UseInputFrameForOutput: UseInputFrameForOutput::<Impl, IMPL_OFFSET>,
            SupportedEncodingProperties: SupportedEncodingProperties::<Impl, IMPL_OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Impl, IMPL_OFFSET>,
            ProcessFrame: ProcessFrame::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicAudioEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IBasicVideoEffect_Impl: Sized + super::IMediaExtension_Impl {
    fn IsReadOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedMemoryTypes(&mut self) -> ::windows::core::Result<MediaMemoryTypes>;
    fn TimeIndependent(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedEncodingProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>;
    fn SetEncodingProperties(&mut self, encodingproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn ProcessFrame(&mut self, context: &::core::option::Option<ProcessVideoFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&mut self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IBasicVideoEffect {
    const NAME: &'static str = "Windows.Media.Effects.IBasicVideoEffect";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl IBasicVideoEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBasicVideoEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBasicVideoEffect_Vtbl {
        unsafe extern "system" fn IsReadOnly<Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedMemoryTypes<Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaMemoryTypes) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimeIndependent<Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedEncodingProperties<Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEncodingProperties<Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetEncodingProperties(&*(&encodingproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&device as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn ProcessFrame<Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessFrame(&*(&context as *const <ProcessVideoFrameContext as ::windows::core::Abi>::Abi as *const <ProcessVideoFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IBasicVideoEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBasicVideoEffect, BASE_OFFSET>(),
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
            SupportedMemoryTypes: SupportedMemoryTypes::<Impl, IMPL_OFFSET>,
            TimeIndependent: TimeIndependent::<Impl, IMPL_OFFSET>,
            SupportedEncodingProperties: SupportedEncodingProperties::<Impl, IMPL_OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Impl, IMPL_OFFSET>,
            ProcessFrame: ProcessFrame::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBasicVideoEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
pub trait IVideoCompositor_Impl: Sized + super::IMediaExtension_Impl {
    fn TimeIndependent(&mut self) -> ::windows::core::Result<bool>;
    fn SetEncodingProperties(&mut self, backgroundproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn CompositeFrame(&mut self, context: &::core::option::Option<CompositeVideoFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&mut self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IVideoCompositor {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositor";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
impl IVideoCompositor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoCompositor_Vtbl {
        unsafe extern "system" fn TimeIndependent<Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEncodingProperties<Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, backgroundproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetEncodingProperties(&*(&backgroundproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType), &*(&device as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn CompositeFrame<Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompositeFrame(&*(&context as *const <CompositeVideoFrameContext as ::windows::core::Abi>::Abi as *const <CompositeVideoFrameContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Close<Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: MediaEffectClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close(reason).into()
        }
        unsafe extern "system" fn DiscardQueuedFrames<Impl: IVideoCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DiscardQueuedFrames().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoCompositor, BASE_OFFSET>(),
            TimeIndependent: TimeIndependent::<Impl, IMPL_OFFSET>,
            SetEncodingProperties: SetEncodingProperties::<Impl, IMPL_OFFSET>,
            CompositeFrame: CompositeFrame::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            DiscardQueuedFrames: DiscardQueuedFrames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoCompositor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoCompositorDefinition_Impl: Sized {
    fn ActivatableClassId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoCompositorDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IVideoCompositorDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoCompositorDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoCompositorDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IVideoCompositorDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IVideoCompositorDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoCompositorDefinition, BASE_OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoCompositorDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVideoEffectDefinition_Impl: Sized {
    fn ActivatableClassId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.IVideoEffectDefinition";
}
#[cfg(feature = "Foundation_Collections")]
impl IVideoEffectDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEffectDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEffectDefinition_Vtbl {
        unsafe extern "system" fn ActivatableClassId<Impl: IVideoEffectDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IVideoEffectDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEffectDefinition, BASE_OFFSET>(),
            ActivatableClassId: ActivatableClassId::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEffectDefinition as ::windows::core::Interface>::IID
    }
}
