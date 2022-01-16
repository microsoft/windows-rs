#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioInputNode_Impl: Sized + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn OutgoingConnections(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>;
    fn AddOutgoingConnection(&mut self, destination: &::core::option::Option<IAudioNode>) -> ::windows::core::Result<()>;
    fn AddOutgoingConnectionWithGain(&mut self, destination: &::core::option::Option<IAudioNode>, gain: f64) -> ::windows::core::Result<()>;
    fn RemoveOutgoingConnection(&mut self, destination: &::core::option::Option<IAudioNode>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioInputNode";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioInputNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioInputNode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioInputNode_Vtbl {
        unsafe extern "system" fn OutgoingConnections<Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOutgoingConnection<Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOutgoingConnection(&*(&destination as *const <IAudioNode as ::windows::core::Abi>::Abi as *const <IAudioNode as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddOutgoingConnectionWithGain<Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, gain: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOutgoingConnectionWithGain(&*(&destination as *const <IAudioNode as ::windows::core::Abi>::Abi as *const <IAudioNode as ::windows::core::DefaultType>::DefaultType), gain).into()
        }
        unsafe extern "system" fn RemoveOutgoingConnection<Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOutgoingConnection(&*(&destination as *const <IAudioNode as ::windows::core::Abi>::Abi as *const <IAudioNode as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioInputNode, BASE_OFFSET>(),
            OutgoingConnections: OutgoingConnections::<Impl, IMPL_OFFSET>,
            AddOutgoingConnection: AddOutgoingConnection::<Impl, IMPL_OFFSET>,
            AddOutgoingConnectionWithGain: AddOutgoingConnectionWithGain::<Impl, IMPL_OFFSET>,
            RemoveOutgoingConnection: RemoveOutgoingConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioInputNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioInputNode2_Impl: Sized + IAudioInputNode_Impl + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn Emitter(&mut self) -> ::windows::core::Result<AudioNodeEmitter>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IAudioInputNode2 {
    const NAME: &'static str = "Windows.Media.Audio.IAudioInputNode2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioInputNode2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioInputNode2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioInputNode2_Vtbl {
        unsafe extern "system" fn Emitter<Impl: IAudioInputNode2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Emitter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioInputNode2, BASE_OFFSET>(), Emitter: Emitter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioInputNode2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioNode_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn EffectDefinitions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn SetOutgoingGain(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn OutgoingGain(&mut self) -> ::windows::core::Result<f64>;
    fn EncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn ConsumeInput(&mut self) -> ::windows::core::Result<bool>;
    fn SetConsumeInput(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn DisableEffectsByDefinition(&mut self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<()>;
    fn EnableEffectsByDefinition(&mut self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IAudioNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNode";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioNode_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioNode_Vtbl {
        unsafe extern "system" fn EffectDefinitions<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingGain<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingGain(value).into()
        }
        unsafe extern "system" fn OutgoingGain<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingGain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingProperties<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConsumeInput<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConsumeInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConsumeInput<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConsumeInput(value).into()
        }
        unsafe extern "system" fn Start<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Reset<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn DisableEffectsByDefinition<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableEffectsByDefinition(&*(&definition as *const <super::Effects::IAudioEffectDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IAudioEffectDefinition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableEffectsByDefinition<Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableEffectsByDefinition(&*(&definition as *const <super::Effects::IAudioEffectDefinition as ::windows::core::Abi>::Abi as *const <super::Effects::IAudioEffectDefinition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioNode, BASE_OFFSET>(),
            EffectDefinitions: EffectDefinitions::<Impl, IMPL_OFFSET>,
            SetOutgoingGain: SetOutgoingGain::<Impl, IMPL_OFFSET>,
            OutgoingGain: OutgoingGain::<Impl, IMPL_OFFSET>,
            EncodingProperties: EncodingProperties::<Impl, IMPL_OFFSET>,
            ConsumeInput: ConsumeInput::<Impl, IMPL_OFFSET>,
            SetConsumeInput: SetConsumeInput::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            DisableEffectsByDefinition: DisableEffectsByDefinition::<Impl, IMPL_OFFSET>,
            EnableEffectsByDefinition: EnableEffectsByDefinition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioNodeWithListener_Impl: Sized + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn SetListener(&mut self, value: &::core::option::Option<AudioNodeListener>) -> ::windows::core::Result<()>;
    fn Listener(&mut self) -> ::windows::core::Result<AudioNodeListener>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IAudioNodeWithListener {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeWithListener";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioNodeWithListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioNodeWithListener_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioNodeWithListener_Vtbl {
        unsafe extern "system" fn SetListener<Impl: IAudioNodeWithListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListener(&*(&value as *const <AudioNodeListener as ::windows::core::Abi>::Abi as *const <AudioNodeListener as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Listener<Impl: IAudioNodeWithListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Listener() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioNodeWithListener, BASE_OFFSET>(),
            SetListener: SetListener::<Impl, IMPL_OFFSET>,
            Listener: Listener::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioNodeWithListener as ::windows::core::Interface>::IID
    }
}
