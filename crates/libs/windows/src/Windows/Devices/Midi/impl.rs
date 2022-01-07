#[cfg(feature = "implement_exclusive")]
pub trait IMidiChannelPressureMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Pressure(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiChannelPressureMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiChannelPressureMessageVtbl {
    pub const fn new<Impl: IMidiChannelPressureMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiChannelPressureMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiChannelPressureMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pressure<Impl: IMidiChannelPressureMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiChannelPressureMessage>, base.5, Channel::<Impl, OFFSET>, Pressure::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiChannelPressureMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiChannelPressureMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiChannelPressureMessage<Impl: IMidiChannelPressureMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: u8, pressure: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiChannelPressureMessage(channel, pressure) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiChannelPressureMessageFactory>, base.5, CreateMidiChannelPressureMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiControlChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Controller(&self) -> ::windows::core::Result<u8>;
    fn ControlValue(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiControlChangeMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiControlChangeMessageVtbl {
    pub const fn new<Impl: IMidiControlChangeMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiControlChangeMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiControlChangeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Controller<Impl: IMidiControlChangeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlValue<Impl: IMidiControlChangeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiControlChangeMessage>, base.5, Channel::<Impl, OFFSET>, Controller::<Impl, OFFSET>, ControlValue::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiControlChangeMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiControlChangeMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiControlChangeMessage<Impl: IMidiControlChangeMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: u8, controller: u8, controlvalue: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiControlChangeMessage(channel, controller, controlvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiControlChangeMessageFactory>, base.5, CreateMidiControlChangeMessage::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiInPortImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiInPortVtbl {
        unsafe extern "system" fn MessageReceived<Impl: IMidiInPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MessageReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceived<Impl: IMidiInPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceId<Impl: IMidiInPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiInPort>, base.5, MessageReceived::<Impl, OFFSET>, RemoveMessageReceived::<Impl, OFFSET>, DeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiInPortStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiInPort>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiInPortStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiInPortStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiInPortStaticsVtbl {
    pub const fn new<Impl: IMidiInPortStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiInPortStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IMidiInPortStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IMidiInPortStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiInPortStatics>, base.5, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
pub trait IMidiMessageImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Type(&self) -> ::windows::core::Result<MidiMessageType>;
}
impl ::windows::core::RuntimeName for IMidiMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiMessage";
}
impl IMidiMessageVtbl {
    pub const fn new<Impl: IMidiMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiMessageVtbl {
        unsafe extern "system" fn Timestamp<Impl: IMidiMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Impl: IMidiMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IMidiMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MidiMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiMessage>, base.5, Timestamp::<Impl, OFFSET>, RawData::<Impl, OFFSET>, Type::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiMessageReceivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiMessageReceivedEventArgsVtbl {
        unsafe extern "system" fn Message<Impl: IMidiMessageReceivedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiMessageReceivedEventArgs>, base.5, Message::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOffMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Velocity(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOffMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiNoteOffMessageVtbl {
    pub const fn new<Impl: IMidiNoteOffMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiNoteOffMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiNoteOffMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Note<Impl: IMidiNoteOffMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Note() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocity<Impl: IMidiNoteOffMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiNoteOffMessage>, base.5, Channel::<Impl, OFFSET>, Note::<Impl, OFFSET>, Velocity::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiNoteOffMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiNoteOffMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiNoteOffMessage<Impl: IMidiNoteOffMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiNoteOffMessage(channel, note, velocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiNoteOffMessageFactory>, base.5, CreateMidiNoteOffMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOnMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Velocity(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOnMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiNoteOnMessageVtbl {
    pub const fn new<Impl: IMidiNoteOnMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiNoteOnMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiNoteOnMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Note<Impl: IMidiNoteOnMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Note() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocity<Impl: IMidiNoteOnMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiNoteOnMessage>, base.5, Channel::<Impl, OFFSET>, Note::<Impl, OFFSET>, Velocity::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiNoteOnMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiNoteOnMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiNoteOnMessage<Impl: IMidiNoteOnMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiNoteOnMessage(channel, note, velocity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiNoteOnMessageFactory>, base.5, CreateMidiNoteOnMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Foundation")]
pub trait IMidiOutPortImpl: Sized + IClosableImpl {
    fn SendMessage(&self, midimessage: &::core::option::Option<IMidiMessage>) -> ::windows::core::Result<()>;
    fn SendBuffer(&self, mididata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IMidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiOutPort";
}
#[cfg(feature = "Foundation")]
impl IMidiOutPortVtbl {
    pub const fn new<Impl: IMidiOutPortImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiOutPortVtbl {
        unsafe extern "system" fn SendMessage<Impl: IMidiOutPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, midimessage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SendMessage(&*(&midimessage as *const <IMidiMessage as ::windows::core::Abi>::Abi as *const <IMidiMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendBuffer<Impl: IMidiOutPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mididata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SendBuffer(&*(&mididata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceId<Impl: IMidiOutPortImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiOutPort>, base.5, SendMessage::<Impl, OFFSET>, SendBuffer::<Impl, OFFSET>, DeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiOutPortStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiOutPortStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiOutPortStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiOutPortStaticsVtbl {
    pub const fn new<Impl: IMidiOutPortStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiOutPortStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IMidiOutPortStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IMidiOutPortStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiOutPortStatics>, base.5, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPitchBendChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Bend(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPitchBendChangeMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiPitchBendChangeMessageVtbl {
    pub const fn new<Impl: IMidiPitchBendChangeMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiPitchBendChangeMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiPitchBendChangeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bend<Impl: IMidiPitchBendChangeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiPitchBendChangeMessage>, base.5, Channel::<Impl, OFFSET>, Bend::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiPitchBendChangeMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiPitchBendChangeMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiPitchBendChangeMessage<Impl: IMidiPitchBendChangeMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: u8, bend: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiPitchBendChangeMessage(channel, bend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiPitchBendChangeMessageFactory>, base.5, CreateMidiPitchBendChangeMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPolyphonicKeyPressureMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Pressure(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPolyphonicKeyPressureMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiPolyphonicKeyPressureMessageVtbl {
    pub const fn new<Impl: IMidiPolyphonicKeyPressureMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiPolyphonicKeyPressureMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiPolyphonicKeyPressureMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Note<Impl: IMidiPolyphonicKeyPressureMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Note() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pressure<Impl: IMidiPolyphonicKeyPressureMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiPolyphonicKeyPressureMessage>, base.5, Channel::<Impl, OFFSET>, Note::<Impl, OFFSET>, Pressure::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiPolyphonicKeyPressureMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiPolyphonicKeyPressureMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiPolyphonicKeyPressureMessage<Impl: IMidiPolyphonicKeyPressureMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, pressure: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiPolyphonicKeyPressureMessage(channel, note, pressure) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiPolyphonicKeyPressureMessageFactory>, base.5, CreateMidiPolyphonicKeyPressureMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiProgramChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Program(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiProgramChangeMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiProgramChangeMessageVtbl {
    pub const fn new<Impl: IMidiProgramChangeMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiProgramChangeMessageVtbl {
        unsafe extern "system" fn Channel<Impl: IMidiProgramChangeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Channel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Program<Impl: IMidiProgramChangeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Program() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiProgramChangeMessage>, base.5, Channel::<Impl, OFFSET>, Program::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiProgramChangeMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiProgramChangeMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiProgramChangeMessage<Impl: IMidiProgramChangeMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: u8, program: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiProgramChangeMessage(channel, program) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiProgramChangeMessageFactory>, base.5, CreateMidiProgramChangeMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongPositionPointerMessageImpl: Sized + IMidiMessageImpl {
    fn Beats(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongPositionPointerMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiSongPositionPointerMessageVtbl {
    pub const fn new<Impl: IMidiSongPositionPointerMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiSongPositionPointerMessageVtbl {
        unsafe extern "system" fn Beats<Impl: IMidiSongPositionPointerMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Beats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiSongPositionPointerMessage>, base.5, Beats::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiSongPositionPointerMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiSongPositionPointerMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiSongPositionPointerMessage<Impl: IMidiSongPositionPointerMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beats: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiSongPositionPointerMessage(beats) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiSongPositionPointerMessageFactory>, base.5, CreateMidiSongPositionPointerMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongSelectMessageImpl: Sized + IMidiMessageImpl {
    fn Song(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongSelectMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiSongSelectMessageVtbl {
    pub const fn new<Impl: IMidiSongSelectMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiSongSelectMessageVtbl {
        unsafe extern "system" fn Song<Impl: IMidiSongSelectMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Song() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiSongSelectMessage>, base.5, Song::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiSongSelectMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiSongSelectMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiSongSelectMessage<Impl: IMidiSongSelectMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, song: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiSongSelectMessage(song) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiSongSelectMessageFactory>, base.5, CreateMidiSongSelectMessage::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiSynthesizerImpl: Sized + IClosableImpl + IMidiOutPortImpl {
    fn AudioDevice(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSynthesizer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiSynthesizerVtbl {
    pub const fn new<Impl: IMidiSynthesizerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiSynthesizerVtbl {
        unsafe extern "system" fn AudioDevice<Impl: IMidiSynthesizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Impl: IMidiSynthesizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IMidiSynthesizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiSynthesizer>, base.5, AudioDevice::<Impl, OFFSET>, Volume::<Impl, OFFSET>, SetVolume::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSynthesizerStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>;
    fn CreateFromAudioDeviceAsync(&self, audiodevice: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>;
    fn IsSynthesizer(&self, mididevice: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiSynthesizerStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSynthesizerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiSynthesizerStaticsVtbl {
    pub const fn new<Impl: IMidiSynthesizerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiSynthesizerStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IMidiSynthesizerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromAudioDeviceAsync<Impl: IMidiSynthesizerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiodevice: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromAudioDeviceAsync(&*(&audiodevice as *const <super::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSynthesizer<Impl: IMidiSynthesizerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mididevice: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSynthesizer(&*(&mididevice as *const <super::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiSynthesizerStatics>, base.5, CreateAsync::<Impl, OFFSET>, CreateFromAudioDeviceAsync::<Impl, OFFSET>, IsSynthesizer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSystemExclusiveMessageFactoryImpl: Sized {
    fn CreateMidiSystemExclusiveMessage(&self, rawdata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<MidiSystemExclusiveMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiSystemExclusiveMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSystemExclusiveMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiSystemExclusiveMessageFactoryVtbl {
    pub const fn new<Impl: IMidiSystemExclusiveMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiSystemExclusiveMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiSystemExclusiveMessage<Impl: IMidiSystemExclusiveMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rawdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiSystemExclusiveMessage(&*(&rawdata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiSystemExclusiveMessageFactory>, base.5, CreateMidiSystemExclusiveMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiTimeCodeMessageImpl: Sized + IMidiMessageImpl {
    fn FrameType(&self) -> ::windows::core::Result<u8>;
    fn Values(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiTimeCodeMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiTimeCodeMessageVtbl {
    pub const fn new<Impl: IMidiTimeCodeMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiTimeCodeMessageVtbl {
        unsafe extern "system" fn FrameType<Impl: IMidiTimeCodeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Impl: IMidiTimeCodeMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiTimeCodeMessage>, base.5, FrameType::<Impl, OFFSET>, Values::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMidiTimeCodeMessageFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMidiTimeCodeMessageFactoryVtbl {
        unsafe extern "system" fn CreateMidiTimeCodeMessage<Impl: IMidiTimeCodeMessageFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frametype: u8, values: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMidiTimeCodeMessage(frametype, values) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMidiTimeCodeMessageFactory>, base.5, CreateMidiTimeCodeMessage::<Impl, OFFSET>)
    }
}
