#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioInputNode_Impl: Sized + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>;
    fn AddOutgoingConnection(&self, destination: &::core::option::Option<IAudioNode>) -> ::windows::core::Result<()>;
    fn AddOutgoingConnectionWithGain(&self, destination: &::core::option::Option<IAudioNode>, gain: f64) -> ::windows::core::Result<()>;
    fn RemoveOutgoingConnection(&self, destination: &::core::option::Option<IAudioNode>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioInputNode";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioInputNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>() -> IAudioInputNode_Vtbl {
        unsafe extern "system" fn OutgoingConnections<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingConnections() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOutgoingConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOutgoingConnection(::core::mem::transmute(&destination)).into()
        }
        unsafe extern "system" fn AddOutgoingConnectionWithGain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, gain: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOutgoingConnectionWithGain(::core::mem::transmute(&destination), gain).into()
        }
        unsafe extern "system" fn RemoveOutgoingConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveOutgoingConnection(::core::mem::transmute(&destination)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IAudioInputNode, OFFSET>(),
            OutgoingConnections: OutgoingConnections::<Identity, Impl, OFFSET>,
            AddOutgoingConnection: AddOutgoingConnection::<Identity, Impl, OFFSET>,
            AddOutgoingConnectionWithGain: AddOutgoingConnectionWithGain::<Identity, Impl, OFFSET>,
            RemoveOutgoingConnection: RemoveOutgoingConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioInputNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioInputNode2_Impl: Sized + IAudioInputNode_Impl + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IAudioInputNode2 {
    const NAME: &'static str = "Windows.Media.Audio.IAudioInputNode2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioInputNode2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode2_Impl, const OFFSET: isize>() -> IAudioInputNode2_Vtbl {
        unsafe extern "system" fn Emitter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputNode2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Emitter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, IAudioInputNode2, OFFSET>(), Emitter: Emitter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioInputNode2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioNode_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn OutgoingGain(&self) -> ::windows::core::Result<f64>;
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn ConsumeInput(&self) -> ::windows::core::Result<bool>;
    fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn DisableEffectsByDefinition(&self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<()>;
    fn EnableEffectsByDefinition(&self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IAudioNode {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNode";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>() -> IAudioNode_Vtbl {
        unsafe extern "system" fn EffectDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EffectDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingGain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutgoingGain(value).into()
        }
        unsafe extern "system" fn OutgoingGain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingGain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConsumeInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConsumeInput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConsumeInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConsumeInput(value).into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn DisableEffectsByDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableEffectsByDefinition(::core::mem::transmute(&definition)).into()
        }
        unsafe extern "system" fn EnableEffectsByDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableEffectsByDefinition(::core::mem::transmute(&definition)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IAudioNode, OFFSET>(),
            EffectDefinitions: EffectDefinitions::<Identity, Impl, OFFSET>,
            SetOutgoingGain: SetOutgoingGain::<Identity, Impl, OFFSET>,
            OutgoingGain: OutgoingGain::<Identity, Impl, OFFSET>,
            EncodingProperties: EncodingProperties::<Identity, Impl, OFFSET>,
            ConsumeInput: ConsumeInput::<Identity, Impl, OFFSET>,
            SetConsumeInput: SetConsumeInput::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            DisableEffectsByDefinition: DisableEffectsByDefinition::<Identity, Impl, OFFSET>,
            EnableEffectsByDefinition: EnableEffectsByDefinition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
pub trait IAudioNodeWithListener_Impl: Sized + IAudioNode_Impl + super::super::Foundation::IClosable_Impl {
    fn SetListener(&self, value: &::core::option::Option<AudioNodeListener>) -> ::windows::core::Result<()>;
    fn Listener(&self) -> ::windows::core::Result<AudioNodeListener>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IAudioNodeWithListener {
    const NAME: &'static str = "Windows.Media.Audio.IAudioNodeWithListener";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "Media_MediaProperties"))]
impl IAudioNodeWithListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: isize>() -> IAudioNodeWithListener_Vtbl {
        unsafe extern "system" fn SetListener<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetListener(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Listener<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioNodeWithListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Listener() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IAudioNodeWithListener, OFFSET>(),
            SetListener: SetListener::<Identity, Impl, OFFSET>,
            Listener: Listener::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioNodeWithListener as ::windows::core::Interface>::IID
    }
}
