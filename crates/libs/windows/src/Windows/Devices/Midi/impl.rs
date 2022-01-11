#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiChannelPressureMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Pressure(&self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiChannelPressureMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiChannelPressureMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiChannelPressureMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiChannelPressureMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiChannelPressureMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pressure<Impl: IMidiChannelPressureMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiChannelPressureMessage>, ::windows::core::GetTrustLevel, Channel::<Impl, IMPL_OFFSET>, Pressure::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiChannelPressureMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiChannelPressureMessageFactoryImpl: Sized {
    fn CreateMidiChannelPressureMessage(&self, channel: u8, pressure: u8) -> ::windows::core::Result<MidiChannelPressureMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiChannelPressureMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiChannelPressureMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiChannelPressureMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiChannelPressureMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiChannelPressureMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiChannelPressureMessage<Impl: IMidiChannelPressureMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, pressure: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiChannelPressureMessage(channel, pressure) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiChannelPressureMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiChannelPressureMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiChannelPressureMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiControlChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Controller(&self) -> ::windows::core::Result<u8>;
    fn ControlValue(&self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiControlChangeMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiControlChangeMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiControlChangeMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiControlChangeMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiControlChangeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Controller<Impl: IMidiControlChangeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlValue<Impl: IMidiControlChangeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiControlChangeMessage>, ::windows::core::GetTrustLevel, Channel::<Impl, IMPL_OFFSET>, Controller::<Impl, IMPL_OFFSET>, ControlValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiControlChangeMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiControlChangeMessageFactoryImpl: Sized {
    fn CreateMidiControlChangeMessage(&self, channel: u8, controller: u8, controlvalue: u8) -> ::windows::core::Result<MidiControlChangeMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiControlChangeMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiControlChangeMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiControlChangeMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiControlChangeMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiControlChangeMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiControlChangeMessage<Impl: IMidiControlChangeMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, controller: u8, controlvalue: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiControlChangeMessage(channel, controller, controlvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiControlChangeMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiControlChangeMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiControlChangeMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiInPortImpl: Sized + IClosableImpl {
    fn MessageReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiInPort {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiInPort";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiInPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiInPortImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiInPortVtbl {
        unsafe extern "system" fn MessageReceived<Impl: IMidiInPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceived<Impl: IMidiInPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceId<Impl: IMidiInPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiInPort>, ::windows::core::GetTrustLevel, MessageReceived::<Impl, IMPL_OFFSET>, RemoveMessageReceived::<Impl, IMPL_OFFSET>, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiInPort as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiInPortStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiInPort>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiInPortStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiInPortStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiInPortStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiInPortStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiInPortStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IMidiInPortStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IMidiInPortStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiInPortStatics>, ::windows::core::GetTrustLevel, FromIdAsync::<Impl, IMPL_OFFSET>, GetDeviceSelector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiInPortStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IMidiMessageImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Type(&self) -> ::windows::core::Result<MidiMessageType>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IMidiMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IMidiMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiMessageVtbl {
        unsafe extern "system" fn Timestamp<Impl: IMidiMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Impl: IMidiMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IMidiMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MidiMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiMessage>, ::windows::core::GetTrustLevel, Timestamp::<Impl, IMPL_OFFSET>, RawData::<Impl, IMPL_OFFSET>, Type::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiMessageReceivedEventArgsImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<IMidiMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiMessageReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiMessageReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiMessageReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiMessageReceivedEventArgsVtbl {
        unsafe extern "system" fn Message<Impl: IMidiMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiMessageReceivedEventArgs>, ::windows::core::GetTrustLevel, Message::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiMessageReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiNoteOffMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Velocity(&self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOffMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiNoteOffMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiNoteOffMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiNoteOffMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiNoteOffMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Note<Impl: IMidiNoteOffMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Note() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocity<Impl: IMidiNoteOffMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiNoteOffMessage>, ::windows::core::GetTrustLevel, Channel::<Impl, IMPL_OFFSET>, Note::<Impl, IMPL_OFFSET>, Velocity::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiNoteOffMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOffMessageFactoryImpl: Sized {
    fn CreateMidiNoteOffMessage(&self, channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOffMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiNoteOffMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOffMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiNoteOffMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiNoteOffMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiNoteOffMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiNoteOffMessage<Impl: IMidiNoteOffMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiNoteOffMessage(channel, note, velocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiNoteOffMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiNoteOffMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiNoteOffMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiNoteOnMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Velocity(&self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOnMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiNoteOnMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiNoteOnMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiNoteOnMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiNoteOnMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Note<Impl: IMidiNoteOnMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Note() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocity<Impl: IMidiNoteOnMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiNoteOnMessage>, ::windows::core::GetTrustLevel, Channel::<Impl, IMPL_OFFSET>, Note::<Impl, IMPL_OFFSET>, Velocity::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiNoteOnMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOnMessageFactoryImpl: Sized {
    fn CreateMidiNoteOnMessage(&self, channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOnMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiNoteOnMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOnMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiNoteOnMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiNoteOnMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiNoteOnMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiNoteOnMessage<Impl: IMidiNoteOnMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiNoteOnMessage(channel, note, velocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiNoteOnMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiNoteOnMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiNoteOnMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IMidiOutPortImpl: Sized + IClosableImpl {
    fn SendMessage(&self, midimessage: &::core::option::Option<IMidiMessage>) -> ::windows::core::Result<()>;
    fn SendBuffer(&self, mididata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IMidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiOutPort";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IMidiOutPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiOutPortImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiOutPortVtbl {
        unsafe extern "system" fn SendMessage<Impl: IMidiOutPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, midimessage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessage(&*(&midimessage as *const <IMidiMessage as ::windows::core::Abi>::Abi as *const <IMidiMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendBuffer<Impl: IMidiOutPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mididata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendBuffer(&*(&mididata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceId<Impl: IMidiOutPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiOutPort>, ::windows::core::GetTrustLevel, SendMessage::<Impl, IMPL_OFFSET>, SendBuffer::<Impl, IMPL_OFFSET>, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiOutPort as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiOutPortStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiOutPortStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiOutPortStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiOutPortStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiOutPortStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiOutPortStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IMidiOutPortStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IMidiOutPortStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiOutPortStatics>, ::windows::core::GetTrustLevel, FromIdAsync::<Impl, IMPL_OFFSET>, GetDeviceSelector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiOutPortStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiPitchBendChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Bend(&self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPitchBendChangeMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiPitchBendChangeMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiPitchBendChangeMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiPitchBendChangeMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiPitchBendChangeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bend<Impl: IMidiPitchBendChangeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiPitchBendChangeMessage>, ::windows::core::GetTrustLevel, Channel::<Impl, IMPL_OFFSET>, Bend::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiPitchBendChangeMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPitchBendChangeMessageFactoryImpl: Sized {
    fn CreateMidiPitchBendChangeMessage(&self, channel: u8, bend: u16) -> ::windows::core::Result<MidiPitchBendChangeMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiPitchBendChangeMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPitchBendChangeMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiPitchBendChangeMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiPitchBendChangeMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiPitchBendChangeMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiPitchBendChangeMessage<Impl: IMidiPitchBendChangeMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, bend: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiPitchBendChangeMessage(channel, bend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiPitchBendChangeMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiPitchBendChangeMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiPitchBendChangeMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiPolyphonicKeyPressureMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Pressure(&self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPolyphonicKeyPressureMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiPolyphonicKeyPressureMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiPolyphonicKeyPressureMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiPolyphonicKeyPressureMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiPolyphonicKeyPressureMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Note<Impl: IMidiPolyphonicKeyPressureMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Note() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pressure<Impl: IMidiPolyphonicKeyPressureMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiPolyphonicKeyPressureMessage>, ::windows::core::GetTrustLevel, Channel::<Impl, IMPL_OFFSET>, Note::<Impl, IMPL_OFFSET>, Pressure::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiPolyphonicKeyPressureMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPolyphonicKeyPressureMessageFactoryImpl: Sized {
    fn CreateMidiPolyphonicKeyPressureMessage(&self, channel: u8, note: u8, pressure: u8) -> ::windows::core::Result<MidiPolyphonicKeyPressureMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiPolyphonicKeyPressureMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPolyphonicKeyPressureMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiPolyphonicKeyPressureMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiPolyphonicKeyPressureMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiPolyphonicKeyPressureMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiPolyphonicKeyPressureMessage<Impl: IMidiPolyphonicKeyPressureMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, pressure: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiPolyphonicKeyPressureMessage(channel, note, pressure) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiPolyphonicKeyPressureMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiPolyphonicKeyPressureMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiPolyphonicKeyPressureMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiProgramChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Program(&self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiProgramChangeMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiProgramChangeMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiProgramChangeMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiProgramChangeMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiProgramChangeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Program<Impl: IMidiProgramChangeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Program() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiProgramChangeMessage>, ::windows::core::GetTrustLevel, Channel::<Impl, IMPL_OFFSET>, Program::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiProgramChangeMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiProgramChangeMessageFactoryImpl: Sized {
    fn CreateMidiProgramChangeMessage(&self, channel: u8, program: u8) -> ::windows::core::Result<MidiProgramChangeMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiProgramChangeMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiProgramChangeMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiProgramChangeMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiProgramChangeMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiProgramChangeMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiProgramChangeMessage<Impl: IMidiProgramChangeMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, program: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiProgramChangeMessage(channel, program) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiProgramChangeMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiProgramChangeMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiProgramChangeMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiSongPositionPointerMessageImpl: Sized + IMidiMessageImpl {
    fn Beats(&self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongPositionPointerMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiSongPositionPointerMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSongPositionPointerMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSongPositionPointerMessageVtbl {
        unsafe extern "system" fn Beats<Impl: IMidiSongPositionPointerMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Beats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiSongPositionPointerMessage>, ::windows::core::GetTrustLevel, Beats::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSongPositionPointerMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongPositionPointerMessageFactoryImpl: Sized {
    fn CreateMidiSongPositionPointerMessage(&self, beats: u16) -> ::windows::core::Result<MidiSongPositionPointerMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiSongPositionPointerMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongPositionPointerMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiSongPositionPointerMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSongPositionPointerMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSongPositionPointerMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiSongPositionPointerMessage<Impl: IMidiSongPositionPointerMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beats: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiSongPositionPointerMessage(beats) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiSongPositionPointerMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiSongPositionPointerMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSongPositionPointerMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiSongSelectMessageImpl: Sized + IMidiMessageImpl {
    fn Song(&self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongSelectMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiSongSelectMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSongSelectMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSongSelectMessageVtbl {
        unsafe extern "system" fn Song<Impl: IMidiSongSelectMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Song() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiSongSelectMessage>, ::windows::core::GetTrustLevel, Song::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSongSelectMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongSelectMessageFactoryImpl: Sized {
    fn CreateMidiSongSelectMessage(&self, song: u8) -> ::windows::core::Result<MidiSongSelectMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiSongSelectMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongSelectMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiSongSelectMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSongSelectMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSongSelectMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiSongSelectMessage<Impl: IMidiSongSelectMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, song: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiSongSelectMessage(song) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiSongSelectMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiSongSelectMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSongSelectMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiSynthesizerImpl: Sized + IClosableImpl + IMidiOutPortImpl {
    fn AudioDevice(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSynthesizer";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiSynthesizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSynthesizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSynthesizerVtbl {
        unsafe extern "system" fn AudioDevice<Impl: IMidiSynthesizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Impl: IMidiSynthesizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IMidiSynthesizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiSynthesizer>, ::windows::core::GetTrustLevel, AudioDevice::<Impl, IMPL_OFFSET>, Volume::<Impl, IMPL_OFFSET>, SetVolume::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSynthesizer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiSynthesizerStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>;
    fn CreateFromAudioDeviceAsync(&self, audiodevice: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>;
    fn IsSynthesizer(&self, mididevice: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSynthesizerStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSynthesizerStatics";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiSynthesizerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSynthesizerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSynthesizerStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IMidiSynthesizerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromAudioDeviceAsync<Impl: IMidiSynthesizerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiodevice: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromAudioDeviceAsync(&*(&audiodevice as *const <super::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSynthesizer<Impl: IMidiSynthesizerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mididevice: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSynthesizer(&*(&mididevice as *const <super::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiSynthesizerStatics>, ::windows::core::GetTrustLevel, CreateAsync::<Impl, IMPL_OFFSET>, CreateFromAudioDeviceAsync::<Impl, IMPL_OFFSET>, IsSynthesizer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSynthesizerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiSystemExclusiveMessageFactoryImpl: Sized {
    fn CreateMidiSystemExclusiveMessage(&self, rawdata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<MidiSystemExclusiveMessage>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSystemExclusiveMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSystemExclusiveMessageFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiSystemExclusiveMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSystemExclusiveMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSystemExclusiveMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiSystemExclusiveMessage<Impl: IMidiSystemExclusiveMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rawdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiSystemExclusiveMessage(&*(&rawdata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiSystemExclusiveMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiSystemExclusiveMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSystemExclusiveMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiTimeCodeMessageImpl: Sized + IMidiMessageImpl {
    fn FrameType(&self) -> ::windows::core::Result<u8>;
    fn Values(&self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiTimeCodeMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiTimeCodeMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiTimeCodeMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiTimeCodeMessageVtbl {
        unsafe extern "system" fn FrameType<Impl: IMidiTimeCodeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Impl: IMidiTimeCodeMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiTimeCodeMessage>, ::windows::core::GetTrustLevel, FrameType::<Impl, IMPL_OFFSET>, Values::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiTimeCodeMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiTimeCodeMessageFactoryImpl: Sized {
    fn CreateMidiTimeCodeMessage(&self, frametype: u8, values: u8) -> ::windows::core::Result<MidiTimeCodeMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiTimeCodeMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiTimeCodeMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiTimeCodeMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiTimeCodeMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiTimeCodeMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiTimeCodeMessage<Impl: IMidiTimeCodeMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frametype: u8, values: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMidiTimeCodeMessage(frametype, values) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMidiTimeCodeMessageFactory>, ::windows::core::GetTrustLevel, CreateMidiTimeCodeMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiTimeCodeMessageFactory as ::windows::core::Interface>::IID
    }
}
