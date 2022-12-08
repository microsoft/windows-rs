#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiChannelPressureMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiChannelPressureMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe1fa860_62b4_4d52_a37e_92e54d35b909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiChannelPressureMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiChannelPressureMessageFactory {
    type Vtable = IMidiChannelPressureMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiChannelPressureMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6218ed2f_2284_412a_94cf_10fb04842c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiChannelPressureMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, pressure: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiControlChangeMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiControlChangeMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7e15f83_780d_405f_b781_3e1598c97f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ControlValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiControlChangeMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiControlChangeMessageFactory {
    type Vtable = IMidiControlChangeMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiControlChangeMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ab14321_956c_46ad_9752_f87f55052fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiControlChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, controller: u8, controlvalue: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiInPort(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiInPort {
    type Vtable = IMidiInPort_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiInPort {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5c1d9db_971a_4eaf_a23d_ea19fe607ff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPort_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiInPortStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiInPortStatics {
    type Vtable = IMidiInPortStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiInPortStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44c439dc_67ff_4a6e_8bac_fdb6610cf296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPortStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct IMidiMessage(::windows::core::IUnknown);
impl IMidiMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IMidiMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IMidiMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMidiMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMidiMessage {}
impl ::core::fmt::Debug for IMidiMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMidiMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMidiMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{79767945-1094-4283-9be0-289fc0ee8334}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IMidiMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MidiMessageType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76566e56_f328_4b51_907d_b3a8ce96bf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessageReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOffMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiNoteOffMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16fd8af4_198e_4d8f_a654_d305a293548f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOffMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiNoteOffMessageFactory {
    type Vtable = IMidiNoteOffMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiNoteOffMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6b240e0_a749_425f_8af4_a4d979cc15b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiNoteOffMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOnMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiNoteOnMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0224af5_6181_46dd_afa2_410004c057aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOnMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiNoteOnMessageFactory {
    type Vtable = IMidiNoteOnMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiNoteOnMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b4280a0_59c1_420e_b517_15a10aa9606b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiNoteOnMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct IMidiOutPort(::windows::core::IUnknown);
impl IMidiOutPort {
    pub fn SendMessage<P0, E0>(&self, midimessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IMidiMessage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SendMessage)(::windows::core::Vtable::as_raw(this), midimessage.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0, E0>(&self, mididata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SendBuffer)(::windows::core::Vtable::as_raw(this), mididata.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IMidiOutPort, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IMidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IMidiOutPort) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IMidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMidiOutPort) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IMidiOutPort> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMidiOutPort) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IMidiOutPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMidiOutPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMidiOutPort {}
impl ::core::fmt::Debug for IMidiOutPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMidiOutPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMidiOutPort {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{931d6d9f-57a2-4a3a-adb8-4640886f6693}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IMidiOutPort {
    type Vtable = IMidiOutPort_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiOutPort {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x931d6d9f_57a2_4a3a_adb8_4640886f6693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPort_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, midimessage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SendBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mididata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendBuffer: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiOutPortStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiOutPortStatics {
    type Vtable = IMidiOutPortStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiOutPortStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x065cc3e9_0f88_448b_9b64_a95826c65b8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPortStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiPitchBendChangeMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29df4cb1_2e9f_4faf_8c2b_9cb82a9079ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Bend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiPitchBendChangeMessageFactory {
    type Vtable = IMidiPitchBendChangeMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiPitchBendChangeMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5eedf55_cfc8_4926_b30e_a3622393306c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiPitchBendChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, bend: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiPolyphonicKeyPressureMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f7337fe_ace8_48a0_868e_7cdbf20f04d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiPolyphonicKeyPressureMessageFactory {
    type Vtable = IMidiPolyphonicKeyPressureMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiPolyphonicKeyPressureMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe98f483e_c4b3_4dd2_917c_e349815a1b3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiPolyphonicKeyPressureMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, pressure: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiProgramChangeMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiProgramChangeMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbb3c78_7a3e_4327_aa98_20b8e4485af8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Program: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiProgramChangeMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiProgramChangeMessageFactory {
    type Vtable = IMidiProgramChangeMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiProgramChangeMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b04387_524b_4104_9c99_6572bfd2e261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiProgramChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, program: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiSongPositionPointerMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ca50c56_ec5e_4ae4_a115_88dc57cc2b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Beats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiSongPositionPointerMessageFactory {
    type Vtable = IMidiSongPositionPointerMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiSongPositionPointerMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c00e996_f10b_4fea_b395_f5d6cf80f64e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiSongPositionPointerMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beats: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongSelectMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiSongSelectMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49f0f27f_6d83_4741_a5bf_4629f6be974f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Song: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongSelectMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiSongSelectMessageFactory {
    type Vtable = IMidiSongSelectMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiSongSelectMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x848878e4_8748_4129_a66c_a05493f75daa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiSongSelectMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, song: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSynthesizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiSynthesizer {
    type Vtable = IMidiSynthesizer_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiSynthesizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0da155e_db90_405f_b8ae_21d2e17f2e45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub AudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    AudioDevice: usize,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSynthesizerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiSynthesizerStatics {
    type Vtable = IMidiSynthesizerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiSynthesizerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4224eaa8_6629_4d6b_aa8f_d4521a5a31ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub CreateFromAudioDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiodevice: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    CreateFromAudioDeviceAsync: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub IsSynthesizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mididevice: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    IsSynthesizer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSystemExclusiveMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiSystemExclusiveMessageFactory {
    type Vtable = IMidiSystemExclusiveMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiSystemExclusiveMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x083de222_3b74_4320_9b42_0ca8545f8a24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSystemExclusiveMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateMidiSystemExclusiveMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateMidiSystemExclusiveMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiTimeCodeMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiTimeCodeMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bf7087d_fa63_4a1c_8deb_c0e87796a6d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FrameType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiTimeCodeMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMidiTimeCodeMessageFactory {
    type Vtable = IMidiTimeCodeMessageFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMidiTimeCodeMessageFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb3099c5_771c_40de_b961_175a7489a85e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessageFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateMidiTimeCodeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frametype: u8, values: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiActiveSensingMessage(::windows::core::IUnknown);
impl MidiActiveSensingMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiActiveSensingMessage, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MidiActiveSensingMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiActiveSensingMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiActiveSensingMessage {}
impl ::core::fmt::Debug for MidiActiveSensingMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiActiveSensingMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiActiveSensingMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiActiveSensingMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiActiveSensingMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiActiveSensingMessage {
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiActiveSensingMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiActiveSensingMessage";
}
::windows::core::interface_hierarchy!(MidiActiveSensingMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiActiveSensingMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiActiveSensingMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiActiveSensingMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiActiveSensingMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiActiveSensingMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiActiveSensingMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiActiveSensingMessage {}
unsafe impl ::core::marker::Sync for MidiActiveSensingMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiChannelPressureMessage(::windows::core::IUnknown);
impl MidiChannelPressureMessage {
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Channel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pressure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiChannelPressureMessage(channel: u8, pressure: u8) -> ::windows::core::Result<MidiChannelPressureMessage> {
        Self::IMidiChannelPressureMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiChannelPressureMessage)(::windows::core::Vtable::as_raw(this), channel, pressure, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMidiChannelPressureMessageFactory<R, F: FnOnce(&IMidiChannelPressureMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiChannelPressureMessage, IMidiChannelPressureMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiChannelPressureMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiChannelPressureMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiChannelPressureMessage {}
impl ::core::fmt::Debug for MidiChannelPressureMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiChannelPressureMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiChannelPressureMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiChannelPressureMessage;{be1fa860-62b4-4d52-a37e-92e54d35b909})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiChannelPressureMessage {
    const IID: ::windows::core::GUID = <IMidiChannelPressureMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiChannelPressureMessage";
}
::windows::core::interface_hierarchy!(MidiChannelPressureMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiChannelPressureMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiChannelPressureMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiChannelPressureMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiChannelPressureMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiChannelPressureMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiChannelPressureMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiChannelPressureMessage {}
unsafe impl ::core::marker::Sync for MidiChannelPressureMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiContinueMessage(::windows::core::IUnknown);
impl MidiContinueMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiContinueMessage, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MidiContinueMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiContinueMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiContinueMessage {}
impl ::core::fmt::Debug for MidiContinueMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiContinueMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiContinueMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiContinueMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiContinueMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiContinueMessage {
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiContinueMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiContinueMessage";
}
::windows::core::interface_hierarchy!(MidiContinueMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiContinueMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiContinueMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiContinueMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiContinueMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiContinueMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiContinueMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiContinueMessage {}
unsafe impl ::core::marker::Sync for MidiContinueMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiControlChangeMessage(::windows::core::IUnknown);
impl MidiControlChangeMessage {
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Channel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Controller(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Controller)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ControlValue(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ControlValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiControlChangeMessage(channel: u8, controller: u8, controlvalue: u8) -> ::windows::core::Result<MidiControlChangeMessage> {
        Self::IMidiControlChangeMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiControlChangeMessage)(::windows::core::Vtable::as_raw(this), channel, controller, controlvalue, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMidiControlChangeMessageFactory<R, F: FnOnce(&IMidiControlChangeMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiControlChangeMessage, IMidiControlChangeMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiControlChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiControlChangeMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiControlChangeMessage {}
impl ::core::fmt::Debug for MidiControlChangeMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiControlChangeMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiControlChangeMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiControlChangeMessage;{b7e15f83-780d-405f-b781-3e1598c97f40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiControlChangeMessage {
    const IID: ::windows::core::GUID = <IMidiControlChangeMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiControlChangeMessage";
}
::windows::core::interface_hierarchy!(MidiControlChangeMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiControlChangeMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiControlChangeMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiControlChangeMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiControlChangeMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiControlChangeMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiControlChangeMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiControlChangeMessage {}
unsafe impl ::core::marker::Sync for MidiControlChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiInPort(::windows::core::IUnknown);
impl MidiInPort {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMessageReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiInPort>> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiInPortStatics<R, F: FnOnce(&IMidiInPortStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiInPort, IMidiInPortStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiInPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiInPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiInPort {}
impl ::core::fmt::Debug for MidiInPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiInPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiInPort {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiInPort;{d5c1d9db-971a-4eaf-a23d-ea19fe607ff9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiInPort {
    type Vtable = IMidiInPort_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiInPort {
    const IID: ::windows::core::GUID = <IMidiInPort as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiInPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiInPort";
}
::windows::core::interface_hierarchy!(MidiInPort, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MidiInPort> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiInPort) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiInPort> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiInPort) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiInPort> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiInPort) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiInPort {}
unsafe impl ::core::marker::Sync for MidiInPort {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiMessageReceivedEventArgs(::windows::core::IUnknown);
impl MidiMessageReceivedEventArgs {
    pub fn Message(&self) -> ::windows::core::Result<IMidiMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MidiMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiMessageReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiMessageReceivedEventArgs {}
impl ::core::fmt::Debug for MidiMessageReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiMessageReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiMessageReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiMessageReceivedEventArgs;{76566e56-f328-4b51-907d-b3a8ce96bf80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiMessageReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMidiMessageReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Midi.MidiMessageReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MidiMessageReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MidiMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MidiMessageReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiNoteOffMessage(::windows::core::IUnknown);
impl MidiNoteOffMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Channel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Note(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Note)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Velocity(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Velocity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiNoteOffMessage(channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOffMessage> {
        Self::IMidiNoteOffMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiNoteOffMessage)(::windows::core::Vtable::as_raw(this), channel, note, velocity, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiNoteOffMessageFactory<R, F: FnOnce(&IMidiNoteOffMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiNoteOffMessage, IMidiNoteOffMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiNoteOffMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiNoteOffMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiNoteOffMessage {}
impl ::core::fmt::Debug for MidiNoteOffMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiNoteOffMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiNoteOffMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiNoteOffMessage;{16fd8af4-198e-4d8f-a654-d305a293548f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiNoteOffMessage {
    const IID: ::windows::core::GUID = <IMidiNoteOffMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOffMessage";
}
::windows::core::interface_hierarchy!(MidiNoteOffMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiNoteOffMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiNoteOffMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiNoteOffMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiNoteOffMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiNoteOffMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiNoteOffMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiNoteOffMessage {}
unsafe impl ::core::marker::Sync for MidiNoteOffMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiNoteOnMessage(::windows::core::IUnknown);
impl MidiNoteOnMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Channel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Note(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Note)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Velocity(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Velocity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiNoteOnMessage(channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOnMessage> {
        Self::IMidiNoteOnMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiNoteOnMessage)(::windows::core::Vtable::as_raw(this), channel, note, velocity, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiNoteOnMessageFactory<R, F: FnOnce(&IMidiNoteOnMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiNoteOnMessage, IMidiNoteOnMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiNoteOnMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiNoteOnMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiNoteOnMessage {}
impl ::core::fmt::Debug for MidiNoteOnMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiNoteOnMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiNoteOnMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiNoteOnMessage;{e0224af5-6181-46dd-afa2-410004c057aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiNoteOnMessage {
    const IID: ::windows::core::GUID = <IMidiNoteOnMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOnMessage";
}
::windows::core::interface_hierarchy!(MidiNoteOnMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiNoteOnMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiNoteOnMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiNoteOnMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiNoteOnMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiNoteOnMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiNoteOnMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiNoteOnMessage {}
unsafe impl ::core::marker::Sync for MidiNoteOnMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiOutPort(::windows::core::IUnknown);
impl MidiOutPort {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SendMessage<P0, E0>(&self, midimessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IMidiMessage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SendMessage)(::windows::core::Vtable::as_raw(this), midimessage.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0, E0>(&self, mididata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SendBuffer)(::windows::core::Vtable::as_raw(this), mididata.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiOutPortStatics<R, F: FnOnce(&IMidiOutPortStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiOutPort, IMidiOutPortStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiOutPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiOutPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiOutPort {}
impl ::core::fmt::Debug for MidiOutPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiOutPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiOutPort {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiOutPort;{931d6d9f-57a2-4a3a-adb8-4640886f6693})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiOutPort {
    type Vtable = IMidiOutPort_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiOutPort {
    const IID: ::windows::core::GUID = <IMidiOutPort as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiOutPort";
}
::windows::core::interface_hierarchy!(MidiOutPort, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiOutPort) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiOutPort) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiOutPort> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiOutPort) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<MidiOutPort> for IMidiOutPort {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiOutPort) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiOutPort> for IMidiOutPort {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiOutPort) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiOutPort> for ::windows::core::InParam<IMidiOutPort> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiOutPort) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiOutPort {}
unsafe impl ::core::marker::Sync for MidiOutPort {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiPitchBendChangeMessage(::windows::core::IUnknown);
impl MidiPitchBendChangeMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Channel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Bend(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bend)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiPitchBendChangeMessage(channel: u8, bend: u16) -> ::windows::core::Result<MidiPitchBendChangeMessage> {
        Self::IMidiPitchBendChangeMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiPitchBendChangeMessage)(::windows::core::Vtable::as_raw(this), channel, bend, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiPitchBendChangeMessageFactory<R, F: FnOnce(&IMidiPitchBendChangeMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiPitchBendChangeMessage, IMidiPitchBendChangeMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiPitchBendChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiPitchBendChangeMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiPitchBendChangeMessage {}
impl ::core::fmt::Debug for MidiPitchBendChangeMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiPitchBendChangeMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiPitchBendChangeMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiPitchBendChangeMessage;{29df4cb1-2e9f-4faf-8c2b-9cb82a9079ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiPitchBendChangeMessage {
    const IID: ::windows::core::GUID = <IMidiPitchBendChangeMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPitchBendChangeMessage";
}
::windows::core::interface_hierarchy!(MidiPitchBendChangeMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiPitchBendChangeMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiPitchBendChangeMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiPitchBendChangeMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiPitchBendChangeMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiPitchBendChangeMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiPitchBendChangeMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiPitchBendChangeMessage {}
unsafe impl ::core::marker::Sync for MidiPitchBendChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiPolyphonicKeyPressureMessage(::windows::core::IUnknown);
impl MidiPolyphonicKeyPressureMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Channel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Note(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Note)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pressure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiPolyphonicKeyPressureMessage(channel: u8, note: u8, pressure: u8) -> ::windows::core::Result<MidiPolyphonicKeyPressureMessage> {
        Self::IMidiPolyphonicKeyPressureMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiPolyphonicKeyPressureMessage)(::windows::core::Vtable::as_raw(this), channel, note, pressure, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiPolyphonicKeyPressureMessageFactory<R, F: FnOnce(&IMidiPolyphonicKeyPressureMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiPolyphonicKeyPressureMessage, IMidiPolyphonicKeyPressureMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiPolyphonicKeyPressureMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiPolyphonicKeyPressureMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiPolyphonicKeyPressureMessage {}
impl ::core::fmt::Debug for MidiPolyphonicKeyPressureMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiPolyphonicKeyPressureMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiPolyphonicKeyPressureMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage;{1f7337fe-ace8-48a0-868e-7cdbf20f04d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiPolyphonicKeyPressureMessage {
    const IID: ::windows::core::GUID = <IMidiPolyphonicKeyPressureMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage";
}
::windows::core::interface_hierarchy!(MidiPolyphonicKeyPressureMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiPolyphonicKeyPressureMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiPolyphonicKeyPressureMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiPolyphonicKeyPressureMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiPolyphonicKeyPressureMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiPolyphonicKeyPressureMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiPolyphonicKeyPressureMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiPolyphonicKeyPressureMessage {}
unsafe impl ::core::marker::Sync for MidiPolyphonicKeyPressureMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiProgramChangeMessage(::windows::core::IUnknown);
impl MidiProgramChangeMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Channel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Program(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Program)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiProgramChangeMessage(channel: u8, program: u8) -> ::windows::core::Result<MidiProgramChangeMessage> {
        Self::IMidiProgramChangeMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiProgramChangeMessage)(::windows::core::Vtable::as_raw(this), channel, program, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiProgramChangeMessageFactory<R, F: FnOnce(&IMidiProgramChangeMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiProgramChangeMessage, IMidiProgramChangeMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiProgramChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiProgramChangeMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiProgramChangeMessage {}
impl ::core::fmt::Debug for MidiProgramChangeMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiProgramChangeMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiProgramChangeMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiProgramChangeMessage;{9cbb3c78-7a3e-4327-aa98-20b8e4485af8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiProgramChangeMessage {
    const IID: ::windows::core::GUID = <IMidiProgramChangeMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiProgramChangeMessage";
}
::windows::core::interface_hierarchy!(MidiProgramChangeMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiProgramChangeMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiProgramChangeMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiProgramChangeMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiProgramChangeMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiProgramChangeMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiProgramChangeMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiProgramChangeMessage {}
unsafe impl ::core::marker::Sync for MidiProgramChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSongPositionPointerMessage(::windows::core::IUnknown);
impl MidiSongPositionPointerMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Beats(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Beats)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiSongPositionPointerMessage(beats: u16) -> ::windows::core::Result<MidiSongPositionPointerMessage> {
        Self::IMidiSongPositionPointerMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiSongPositionPointerMessage)(::windows::core::Vtable::as_raw(this), beats, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSongPositionPointerMessageFactory<R, F: FnOnce(&IMidiSongPositionPointerMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiSongPositionPointerMessage, IMidiSongPositionPointerMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiSongPositionPointerMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiSongPositionPointerMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiSongPositionPointerMessage {}
impl ::core::fmt::Debug for MidiSongPositionPointerMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiSongPositionPointerMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiSongPositionPointerMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSongPositionPointerMessage;{4ca50c56-ec5e-4ae4-a115-88dc57cc2b79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiSongPositionPointerMessage {
    const IID: ::windows::core::GUID = <IMidiSongPositionPointerMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongPositionPointerMessage";
}
::windows::core::interface_hierarchy!(MidiSongPositionPointerMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiSongPositionPointerMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiSongPositionPointerMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiSongPositionPointerMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSongPositionPointerMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiSongPositionPointerMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSongPositionPointerMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiSongPositionPointerMessage {}
unsafe impl ::core::marker::Sync for MidiSongPositionPointerMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSongSelectMessage(::windows::core::IUnknown);
impl MidiSongSelectMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Song(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Song)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiSongSelectMessage(song: u8) -> ::windows::core::Result<MidiSongSelectMessage> {
        Self::IMidiSongSelectMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiSongSelectMessage)(::windows::core::Vtable::as_raw(this), song, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSongSelectMessageFactory<R, F: FnOnce(&IMidiSongSelectMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiSongSelectMessage, IMidiSongSelectMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiSongSelectMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiSongSelectMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiSongSelectMessage {}
impl ::core::fmt::Debug for MidiSongSelectMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiSongSelectMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiSongSelectMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSongSelectMessage;{49f0f27f-6d83-4741-a5bf-4629f6be974f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiSongSelectMessage {
    const IID: ::windows::core::GUID = <IMidiSongSelectMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongSelectMessage";
}
::windows::core::interface_hierarchy!(MidiSongSelectMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiSongSelectMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiSongSelectMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiSongSelectMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSongSelectMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiSongSelectMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSongSelectMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiSongSelectMessage {}
unsafe impl ::core::marker::Sync for MidiSongSelectMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiStartMessage(::windows::core::IUnknown);
impl MidiStartMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiStartMessage, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MidiStartMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiStartMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiStartMessage {}
impl ::core::fmt::Debug for MidiStartMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiStartMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiStartMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiStartMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiStartMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiStartMessage {
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiStartMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStartMessage";
}
::windows::core::interface_hierarchy!(MidiStartMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiStartMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiStartMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiStartMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiStartMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiStartMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiStartMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiStartMessage {}
unsafe impl ::core::marker::Sync for MidiStartMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiStopMessage(::windows::core::IUnknown);
impl MidiStopMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiStopMessage, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MidiStopMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiStopMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiStopMessage {}
impl ::core::fmt::Debug for MidiStopMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiStopMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiStopMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiStopMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiStopMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiStopMessage {
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiStopMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStopMessage";
}
::windows::core::interface_hierarchy!(MidiStopMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiStopMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiStopMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiStopMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiStopMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiStopMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiStopMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiStopMessage {}
unsafe impl ::core::marker::Sync for MidiStopMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSynthesizer(::windows::core::IUnknown);
impl MidiSynthesizer {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SendMessage<P0, E0>(&self, midimessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IMidiMessage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SendMessage)(::windows::core::Vtable::as_raw(this), midimessage.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0, E0>(&self, mididata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SendBuffer)(::windows::core::Vtable::as_raw(this), mididata.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn AudioDevice(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioDevice)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Volume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVolume)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn CreateFromAudioDeviceAsync(audiodevice: &super::Enumeration::DeviceInformation) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromAudioDeviceAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(audiodevice), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn IsSynthesizer(mididevice: &super::Enumeration::DeviceInformation) -> ::windows::core::Result<bool> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSynthesizer)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mididevice), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSynthesizerStatics<R, F: FnOnce(&IMidiSynthesizerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiSynthesizer, IMidiSynthesizerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiSynthesizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiSynthesizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiSynthesizer {}
impl ::core::fmt::Debug for MidiSynthesizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiSynthesizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiSynthesizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSynthesizer;{f0da155e-db90-405f-b8ae-21d2e17f2e45})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiSynthesizer {
    type Vtable = IMidiSynthesizer_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiSynthesizer {
    const IID: ::windows::core::GUID = <IMidiSynthesizer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSynthesizer";
}
::windows::core::interface_hierarchy!(MidiSynthesizer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MidiSynthesizer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiSynthesizer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiSynthesizer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSynthesizer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MidiSynthesizer> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSynthesizer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<MidiSynthesizer> for IMidiOutPort {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiSynthesizer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiSynthesizer> for IMidiOutPort {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSynthesizer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiSynthesizer> for ::windows::core::InParam<IMidiOutPort> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSynthesizer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiSynthesizer {}
unsafe impl ::core::marker::Sync for MidiSynthesizer {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSystemExclusiveMessage(::windows::core::IUnknown);
impl MidiSystemExclusiveMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateMidiSystemExclusiveMessage<P0, E0>(rawdata: P0) -> ::windows::core::Result<MidiSystemExclusiveMessage>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMidiSystemExclusiveMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiSystemExclusiveMessage)(::windows::core::Vtable::as_raw(this), rawdata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSystemExclusiveMessageFactory<R, F: FnOnce(&IMidiSystemExclusiveMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiSystemExclusiveMessage, IMidiSystemExclusiveMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiSystemExclusiveMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiSystemExclusiveMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiSystemExclusiveMessage {}
impl ::core::fmt::Debug for MidiSystemExclusiveMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiSystemExclusiveMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiSystemExclusiveMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSystemExclusiveMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiSystemExclusiveMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiSystemExclusiveMessage {
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSystemExclusiveMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemExclusiveMessage";
}
::windows::core::interface_hierarchy!(MidiSystemExclusiveMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiSystemExclusiveMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiSystemExclusiveMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiSystemExclusiveMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSystemExclusiveMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiSystemExclusiveMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSystemExclusiveMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiSystemExclusiveMessage {}
unsafe impl ::core::marker::Sync for MidiSystemExclusiveMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSystemResetMessage(::windows::core::IUnknown);
impl MidiSystemResetMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiSystemResetMessage, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MidiSystemResetMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiSystemResetMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiSystemResetMessage {}
impl ::core::fmt::Debug for MidiSystemResetMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiSystemResetMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiSystemResetMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSystemResetMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiSystemResetMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiSystemResetMessage {
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSystemResetMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemResetMessage";
}
::windows::core::interface_hierarchy!(MidiSystemResetMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiSystemResetMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiSystemResetMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiSystemResetMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSystemResetMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiSystemResetMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiSystemResetMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiSystemResetMessage {}
unsafe impl ::core::marker::Sync for MidiSystemResetMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiTimeCodeMessage(::windows::core::IUnknown);
impl MidiTimeCodeMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FrameType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Values(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Values)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateMidiTimeCodeMessage(frametype: u8, values: u8) -> ::windows::core::Result<MidiTimeCodeMessage> {
        Self::IMidiTimeCodeMessageFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMidiTimeCodeMessage)(::windows::core::Vtable::as_raw(this), frametype, values, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiTimeCodeMessageFactory<R, F: FnOnce(&IMidiTimeCodeMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiTimeCodeMessage, IMidiTimeCodeMessageFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MidiTimeCodeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiTimeCodeMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiTimeCodeMessage {}
impl ::core::fmt::Debug for MidiTimeCodeMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiTimeCodeMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiTimeCodeMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTimeCodeMessage;{0bf7087d-fa63-4a1c-8deb-c0e87796a6d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiTimeCodeMessage {
    const IID: ::windows::core::GUID = <IMidiTimeCodeMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimeCodeMessage";
}
::windows::core::interface_hierarchy!(MidiTimeCodeMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiTimeCodeMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiTimeCodeMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiTimeCodeMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiTimeCodeMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiTimeCodeMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiTimeCodeMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiTimeCodeMessage {}
unsafe impl ::core::marker::Sync for MidiTimeCodeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiTimingClockMessage(::windows::core::IUnknown);
impl MidiTimingClockMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiTimingClockMessage, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MidiTimingClockMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiTimingClockMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiTimingClockMessage {}
impl ::core::fmt::Debug for MidiTimingClockMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiTimingClockMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiTimingClockMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTimingClockMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiTimingClockMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiTimingClockMessage {
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiTimingClockMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimingClockMessage";
}
::windows::core::interface_hierarchy!(MidiTimingClockMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiTimingClockMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiTimingClockMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiTimingClockMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiTimingClockMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiTimingClockMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiTimingClockMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiTimingClockMessage {}
unsafe impl ::core::marker::Sync for MidiTimingClockMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiTuneRequestMessage(::windows::core::IUnknown);
impl MidiTuneRequestMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MidiTuneRequestMessage, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MidiTuneRequestMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MidiTuneRequestMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MidiTuneRequestMessage {}
impl ::core::fmt::Debug for MidiTuneRequestMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiTuneRequestMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiTuneRequestMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTuneRequestMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MidiTuneRequestMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for MidiTuneRequestMessage {
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiTuneRequestMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTuneRequestMessage";
}
::windows::core::interface_hierarchy!(MidiTuneRequestMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MidiTuneRequestMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: MidiTuneRequestMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MidiTuneRequestMessage> for IMidiMessage {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiTuneRequestMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MidiTuneRequestMessage> for ::windows::core::InParam<IMidiMessage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MidiTuneRequestMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MidiTuneRequestMessage {}
unsafe impl ::core::marker::Sync for MidiTuneRequestMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MidiMessageType(pub i32);
impl MidiMessageType {
    pub const None: Self = Self(0i32);
    pub const NoteOff: Self = Self(128i32);
    pub const NoteOn: Self = Self(144i32);
    pub const PolyphonicKeyPressure: Self = Self(160i32);
    pub const ControlChange: Self = Self(176i32);
    pub const ProgramChange: Self = Self(192i32);
    pub const ChannelPressure: Self = Self(208i32);
    pub const PitchBendChange: Self = Self(224i32);
    pub const SystemExclusive: Self = Self(240i32);
    pub const MidiTimeCode: Self = Self(241i32);
    pub const SongPositionPointer: Self = Self(242i32);
    pub const SongSelect: Self = Self(243i32);
    pub const TuneRequest: Self = Self(246i32);
    pub const EndSystemExclusive: Self = Self(247i32);
    pub const TimingClock: Self = Self(248i32);
    pub const Start: Self = Self(250i32);
    pub const Continue: Self = Self(251i32);
    pub const Stop: Self = Self(252i32);
    pub const ActiveSensing: Self = Self(254i32);
    pub const SystemReset: Self = Self(255i32);
}
impl ::core::marker::Copy for MidiMessageType {}
impl ::core::clone::Clone for MidiMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MidiMessageType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MidiMessageType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MidiMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiMessageType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MidiMessageType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Midi.MidiMessageType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
