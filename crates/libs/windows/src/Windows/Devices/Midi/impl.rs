#[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IMidiMessage_Impl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Type(&self) -> ::windows::core::Result<MidiMessageType>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IMidiMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IMidiMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: isize>() -> IMidiMessage_Vtbl {
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RawData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MidiMessageType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IMidiMessage, OFFSET>(),
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            RawData: RawData::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiMessage as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IMidiOutPort_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn SendMessage(&self, midimessage: ::core::option::Option<&IMidiMessage>) -> ::windows::core::Result<()>;
    fn SendBuffer(&self, mididata: ::core::option::Option<&super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IMidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiOutPort";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IMidiOutPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: isize>() -> IMidiOutPort_Vtbl {
        unsafe extern "system" fn SendMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, midimessage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMessage(::windows::core::from_raw_borrowed(&midimessage)).into()
        }
        unsafe extern "system" fn SendBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mididata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendBuffer(::windows::core::from_raw_borrowed(&mididata)).into()
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IMidiOutPort, OFFSET>(),
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
            SendBuffer: SendBuffer::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiOutPort as ::windows::core::ComInterface>::IID
    }
}
