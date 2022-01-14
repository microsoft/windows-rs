#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiChannelPressureMessage_Impl: Sized + IMidiMessage_Impl {
    fn Channel(&mut self) -> ::windows::core::Result<u8>;
    fn Pressure(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiChannelPressureMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiChannelPressureMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiChannelPressureMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiChannelPressureMessage_Vtbl {
        unsafe extern "system" fn Channel<Impl: IMidiChannelPressureMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pressure<Impl: IMidiChannelPressureMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiChannelPressureMessage, BASE_OFFSET>(),
            Channel: Channel::<Impl, IMPL_OFFSET>,
            Pressure: Pressure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiChannelPressureMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiChannelPressureMessageFactory_Impl: Sized {
    fn CreateMidiChannelPressureMessage(&mut self, channel: u8, pressure: u8) -> ::windows::core::Result<MidiChannelPressureMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiChannelPressureMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiChannelPressureMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiChannelPressureMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiChannelPressureMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiChannelPressureMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiChannelPressureMessage<Impl: IMidiChannelPressureMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, pressure: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiChannelPressureMessageFactory, BASE_OFFSET>(),
            CreateMidiChannelPressureMessage: CreateMidiChannelPressureMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiChannelPressureMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiControlChangeMessage_Impl: Sized + IMidiMessage_Impl {
    fn Channel(&mut self) -> ::windows::core::Result<u8>;
    fn Controller(&mut self) -> ::windows::core::Result<u8>;
    fn ControlValue(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiControlChangeMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiControlChangeMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiControlChangeMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiControlChangeMessage_Vtbl {
        unsafe extern "system" fn Channel<Impl: IMidiControlChangeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Controller<Impl: IMidiControlChangeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ControlValue<Impl: IMidiControlChangeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiControlChangeMessage, BASE_OFFSET>(),
            Channel: Channel::<Impl, IMPL_OFFSET>,
            Controller: Controller::<Impl, IMPL_OFFSET>,
            ControlValue: ControlValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiControlChangeMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiControlChangeMessageFactory_Impl: Sized {
    fn CreateMidiControlChangeMessage(&mut self, channel: u8, controller: u8, controlvalue: u8) -> ::windows::core::Result<MidiControlChangeMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiControlChangeMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiControlChangeMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiControlChangeMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiControlChangeMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiControlChangeMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiControlChangeMessage<Impl: IMidiControlChangeMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, controller: u8, controlvalue: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiControlChangeMessageFactory, BASE_OFFSET>(),
            CreateMidiControlChangeMessage: CreateMidiControlChangeMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiControlChangeMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiInPort_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn MessageReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiInPort {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiInPort";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiInPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiInPort_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiInPort_Vtbl {
        unsafe extern "system" fn MessageReceived<Impl: IMidiInPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMessageReceived<Impl: IMidiInPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceId<Impl: IMidiInPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiInPort, BASE_OFFSET>(),
            MessageReceived: MessageReceived::<Impl, IMPL_OFFSET>,
            RemoveMessageReceived: RemoveMessageReceived::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiInPort as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiInPortStatics_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiInPort>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiInPortStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiInPortStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiInPortStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiInPortStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiInPortStatics_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IMidiInPortStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IMidiInPortStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiInPortStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiInPortStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IMidiMessage_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn RawData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Type(&mut self) -> ::windows::core::Result<MidiMessageType>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IMidiMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IMidiMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiMessage_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RawData<Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MidiMessageType) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiMessage, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            RawData: RawData::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiMessageReceivedEventArgs_Impl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<IMidiMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiMessageReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiMessageReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiMessageReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiMessageReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Message<Impl: IMidiMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiMessageReceivedEventArgs, BASE_OFFSET>(), Message: Message::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiMessageReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiNoteOffMessage_Impl: Sized + IMidiMessage_Impl {
    fn Channel(&mut self) -> ::windows::core::Result<u8>;
    fn Note(&mut self) -> ::windows::core::Result<u8>;
    fn Velocity(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOffMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiNoteOffMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiNoteOffMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiNoteOffMessage_Vtbl {
        unsafe extern "system" fn Channel<Impl: IMidiNoteOffMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Note<Impl: IMidiNoteOffMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Velocity<Impl: IMidiNoteOffMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiNoteOffMessage, BASE_OFFSET>(),
            Channel: Channel::<Impl, IMPL_OFFSET>,
            Note: Note::<Impl, IMPL_OFFSET>,
            Velocity: Velocity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiNoteOffMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOffMessageFactory_Impl: Sized {
    fn CreateMidiNoteOffMessage(&mut self, channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOffMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiNoteOffMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOffMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiNoteOffMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiNoteOffMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiNoteOffMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiNoteOffMessage<Impl: IMidiNoteOffMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiNoteOffMessageFactory, BASE_OFFSET>(),
            CreateMidiNoteOffMessage: CreateMidiNoteOffMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiNoteOffMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiNoteOnMessage_Impl: Sized + IMidiMessage_Impl {
    fn Channel(&mut self) -> ::windows::core::Result<u8>;
    fn Note(&mut self) -> ::windows::core::Result<u8>;
    fn Velocity(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOnMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiNoteOnMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiNoteOnMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiNoteOnMessage_Vtbl {
        unsafe extern "system" fn Channel<Impl: IMidiNoteOnMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Note<Impl: IMidiNoteOnMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Velocity<Impl: IMidiNoteOnMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiNoteOnMessage, BASE_OFFSET>(),
            Channel: Channel::<Impl, IMPL_OFFSET>,
            Note: Note::<Impl, IMPL_OFFSET>,
            Velocity: Velocity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiNoteOnMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOnMessageFactory_Impl: Sized {
    fn CreateMidiNoteOnMessage(&mut self, channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOnMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiNoteOnMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiNoteOnMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiNoteOnMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiNoteOnMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiNoteOnMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiNoteOnMessage<Impl: IMidiNoteOnMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiNoteOnMessageFactory, BASE_OFFSET>(),
            CreateMidiNoteOnMessage: CreateMidiNoteOnMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiNoteOnMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IMidiOutPort_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn SendMessage(&mut self, midimessage: &::core::option::Option<IMidiMessage>) -> ::windows::core::Result<()>;
    fn SendBuffer(&mut self, mididata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IMidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiOutPort";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IMidiOutPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiOutPort_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiOutPort_Vtbl {
        unsafe extern "system" fn SendMessage<Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, midimessage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMessage(&*(&midimessage as *const <IMidiMessage as ::windows::core::Abi>::Abi as *const <IMidiMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SendBuffer<Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mididata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendBuffer(&*(&mididata as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceId<Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiOutPort, BASE_OFFSET>(),
            SendMessage: SendMessage::<Impl, IMPL_OFFSET>,
            SendBuffer: SendBuffer::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiOutPort as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiOutPortStatics_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiOutPortStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiOutPortStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiOutPortStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiOutPortStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiOutPortStatics_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IMidiOutPortStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IMidiOutPortStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiOutPortStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiOutPortStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiPitchBendChangeMessage_Impl: Sized + IMidiMessage_Impl {
    fn Channel(&mut self) -> ::windows::core::Result<u8>;
    fn Bend(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPitchBendChangeMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiPitchBendChangeMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiPitchBendChangeMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiPitchBendChangeMessage_Vtbl {
        unsafe extern "system" fn Channel<Impl: IMidiPitchBendChangeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Bend<Impl: IMidiPitchBendChangeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiPitchBendChangeMessage, BASE_OFFSET>(),
            Channel: Channel::<Impl, IMPL_OFFSET>,
            Bend: Bend::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiPitchBendChangeMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPitchBendChangeMessageFactory_Impl: Sized {
    fn CreateMidiPitchBendChangeMessage(&mut self, channel: u8, bend: u16) -> ::windows::core::Result<MidiPitchBendChangeMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiPitchBendChangeMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPitchBendChangeMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiPitchBendChangeMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiPitchBendChangeMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiPitchBendChangeMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiPitchBendChangeMessage<Impl: IMidiPitchBendChangeMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, bend: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiPitchBendChangeMessageFactory, BASE_OFFSET>(),
            CreateMidiPitchBendChangeMessage: CreateMidiPitchBendChangeMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiPitchBendChangeMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiPolyphonicKeyPressureMessage_Impl: Sized + IMidiMessage_Impl {
    fn Channel(&mut self) -> ::windows::core::Result<u8>;
    fn Note(&mut self) -> ::windows::core::Result<u8>;
    fn Pressure(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPolyphonicKeyPressureMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiPolyphonicKeyPressureMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiPolyphonicKeyPressureMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiPolyphonicKeyPressureMessage_Vtbl {
        unsafe extern "system" fn Channel<Impl: IMidiPolyphonicKeyPressureMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Note<Impl: IMidiPolyphonicKeyPressureMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pressure<Impl: IMidiPolyphonicKeyPressureMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiPolyphonicKeyPressureMessage, BASE_OFFSET>(),
            Channel: Channel::<Impl, IMPL_OFFSET>,
            Note: Note::<Impl, IMPL_OFFSET>,
            Pressure: Pressure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiPolyphonicKeyPressureMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPolyphonicKeyPressureMessageFactory_Impl: Sized {
    fn CreateMidiPolyphonicKeyPressureMessage(&mut self, channel: u8, note: u8, pressure: u8) -> ::windows::core::Result<MidiPolyphonicKeyPressureMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiPolyphonicKeyPressureMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiPolyphonicKeyPressureMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiPolyphonicKeyPressureMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiPolyphonicKeyPressureMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiPolyphonicKeyPressureMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiPolyphonicKeyPressureMessage<Impl: IMidiPolyphonicKeyPressureMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, note: u8, pressure: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiPolyphonicKeyPressureMessageFactory, BASE_OFFSET>(),
            CreateMidiPolyphonicKeyPressureMessage: CreateMidiPolyphonicKeyPressureMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiPolyphonicKeyPressureMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiProgramChangeMessage_Impl: Sized + IMidiMessage_Impl {
    fn Channel(&mut self) -> ::windows::core::Result<u8>;
    fn Program(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiProgramChangeMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiProgramChangeMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiProgramChangeMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiProgramChangeMessage_Vtbl {
        unsafe extern "system" fn Channel<Impl: IMidiProgramChangeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Program<Impl: IMidiProgramChangeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiProgramChangeMessage, BASE_OFFSET>(),
            Channel: Channel::<Impl, IMPL_OFFSET>,
            Program: Program::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiProgramChangeMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiProgramChangeMessageFactory_Impl: Sized {
    fn CreateMidiProgramChangeMessage(&mut self, channel: u8, program: u8) -> ::windows::core::Result<MidiProgramChangeMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiProgramChangeMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiProgramChangeMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiProgramChangeMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiProgramChangeMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiProgramChangeMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiProgramChangeMessage<Impl: IMidiProgramChangeMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: u8, program: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiProgramChangeMessageFactory, BASE_OFFSET>(),
            CreateMidiProgramChangeMessage: CreateMidiProgramChangeMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiProgramChangeMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiSongPositionPointerMessage_Impl: Sized + IMidiMessage_Impl {
    fn Beats(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongPositionPointerMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiSongPositionPointerMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSongPositionPointerMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSongPositionPointerMessage_Vtbl {
        unsafe extern "system" fn Beats<Impl: IMidiSongPositionPointerMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiSongPositionPointerMessage, BASE_OFFSET>(), Beats: Beats::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSongPositionPointerMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongPositionPointerMessageFactory_Impl: Sized {
    fn CreateMidiSongPositionPointerMessage(&mut self, beats: u16) -> ::windows::core::Result<MidiSongPositionPointerMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiSongPositionPointerMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongPositionPointerMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiSongPositionPointerMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSongPositionPointerMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSongPositionPointerMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiSongPositionPointerMessage<Impl: IMidiSongPositionPointerMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, beats: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiSongPositionPointerMessageFactory, BASE_OFFSET>(),
            CreateMidiSongPositionPointerMessage: CreateMidiSongPositionPointerMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSongPositionPointerMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiSongSelectMessage_Impl: Sized + IMidiMessage_Impl {
    fn Song(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongSelectMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiSongSelectMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSongSelectMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSongSelectMessage_Vtbl {
        unsafe extern "system" fn Song<Impl: IMidiSongSelectMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiSongSelectMessage, BASE_OFFSET>(), Song: Song::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSongSelectMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongSelectMessageFactory_Impl: Sized {
    fn CreateMidiSongSelectMessage(&mut self, song: u8) -> ::windows::core::Result<MidiSongSelectMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiSongSelectMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSongSelectMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiSongSelectMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSongSelectMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSongSelectMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiSongSelectMessage<Impl: IMidiSongSelectMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, song: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiSongSelectMessageFactory, BASE_OFFSET>(),
            CreateMidiSongSelectMessage: CreateMidiSongSelectMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSongSelectMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiSynthesizer_Impl: Sized + super::super::Foundation::IClosable_Impl + IMidiOutPort_Impl {
    fn AudioDevice(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
    fn Volume(&mut self) -> ::windows::core::Result<f64>;
    fn SetVolume(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSynthesizer";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiSynthesizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSynthesizer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSynthesizer_Vtbl {
        unsafe extern "system" fn AudioDevice<Impl: IMidiSynthesizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Volume<Impl: IMidiSynthesizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVolume<Impl: IMidiSynthesizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiSynthesizer, BASE_OFFSET>(),
            AudioDevice: AudioDevice::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSynthesizer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiSynthesizerStatics_Impl: Sized {
    fn CreateAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>;
    fn CreateFromAudioDeviceAsync(&mut self, audiodevice: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>;
    fn IsSynthesizer(&mut self, mididevice: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSynthesizerStatics {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSynthesizerStatics";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl IMidiSynthesizerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSynthesizerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSynthesizerStatics_Vtbl {
        unsafe extern "system" fn CreateAsync<Impl: IMidiSynthesizerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromAudioDeviceAsync<Impl: IMidiSynthesizerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiodevice: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSynthesizer<Impl: IMidiSynthesizerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mididevice: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiSynthesizerStatics, BASE_OFFSET>(),
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
            CreateFromAudioDeviceAsync: CreateFromAudioDeviceAsync::<Impl, IMPL_OFFSET>,
            IsSynthesizer: IsSynthesizer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSynthesizerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiSystemExclusiveMessageFactory_Impl: Sized {
    fn CreateMidiSystemExclusiveMessage(&mut self, rawdata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<MidiSystemExclusiveMessage>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiSystemExclusiveMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiSystemExclusiveMessageFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiSystemExclusiveMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiSystemExclusiveMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiSystemExclusiveMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiSystemExclusiveMessage<Impl: IMidiSystemExclusiveMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rawdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiSystemExclusiveMessageFactory, BASE_OFFSET>(),
            CreateMidiSystemExclusiveMessage: CreateMidiSystemExclusiveMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiSystemExclusiveMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMidiTimeCodeMessage_Impl: Sized + IMidiMessage_Impl {
    fn FrameType(&mut self) -> ::windows::core::Result<u8>;
    fn Values(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiTimeCodeMessage";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMidiTimeCodeMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiTimeCodeMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiTimeCodeMessage_Vtbl {
        unsafe extern "system" fn FrameType<Impl: IMidiTimeCodeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Values<Impl: IMidiTimeCodeMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiTimeCodeMessage, BASE_OFFSET>(),
            FrameType: FrameType::<Impl, IMPL_OFFSET>,
            Values: Values::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiTimeCodeMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiTimeCodeMessageFactory_Impl: Sized {
    fn CreateMidiTimeCodeMessage(&mut self, frametype: u8, values: u8) -> ::windows::core::Result<MidiTimeCodeMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMidiTimeCodeMessageFactory {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiTimeCodeMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMidiTimeCodeMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMidiTimeCodeMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMidiTimeCodeMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMidiTimeCodeMessage<Impl: IMidiTimeCodeMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frametype: u8, values: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMidiTimeCodeMessageFactory, BASE_OFFSET>(),
            CreateMidiTimeCodeMessage: CreateMidiTimeCodeMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMidiTimeCodeMessageFactory as ::windows::core::Interface>::IID
    }
}
