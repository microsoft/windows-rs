#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiChannelPressureMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiChannelPressureMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiChannelPressureMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe1fa860_62b4_4d52_a37e_92e54d35b909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiChannelPressureMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiChannelPressureMessageFactory {
    type Vtable = IMidiChannelPressureMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiChannelPressureMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiChannelPressureMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6218ed2f_2284_412a_94cf_10fb04842c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiChannelPressureMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, pressure: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiControlChangeMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiControlChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiControlChangeMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7e15f83_780d_405f_b781_3e1598c97f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ControlValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiControlChangeMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiControlChangeMessageFactory {
    type Vtable = IMidiControlChangeMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiControlChangeMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiControlChangeMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ab14321_956c_46ad_9752_f87f55052fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiControlChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, controller: u8, controlvalue: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiInPort(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiInPort {
    type Vtable = IMidiInPort_Vtbl;
}
impl ::core::clone::Clone for IMidiInPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiInPort {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5c1d9db_971a_4eaf_a23d_ea19fe607ff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPort_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiInPortStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiInPortStatics {
    type Vtable = IMidiInPortStatics_Vtbl;
}
impl ::core::clone::Clone for IMidiInPortStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiInPortStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44c439dc_67ff_4a6e_8bac_fdb6610cf296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPortStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct IMidiMessage(::windows_core::IUnknown);
impl IMidiMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMidiMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IMidiMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{79767945-1094-4283-9be0-289fc0ee8334}");
}
unsafe impl ::windows_core::Interface for IMidiMessage {
    type Vtable = IMidiMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MidiMessageType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiMessageReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMidiMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiMessageReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76566e56_f328_4b51_907d_b3a8ce96bf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessageReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOffMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiNoteOffMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiNoteOffMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16fd8af4_198e_4d8f_a654_d305a293548f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOffMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiNoteOffMessageFactory {
    type Vtable = IMidiNoteOffMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiNoteOffMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiNoteOffMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6b240e0_a749_425f_8af4_a4d979cc15b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiNoteOffMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOnMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiNoteOnMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiNoteOnMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0224af5_6181_46dd_afa2_410004c057aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOnMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiNoteOnMessageFactory {
    type Vtable = IMidiNoteOnMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiNoteOnMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiNoteOnMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b4280a0_59c1_420e_b517_15a10aa9606b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiNoteOnMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct IMidiOutPort(::windows_core::IUnknown);
impl IMidiOutPort {
    pub fn SendMessage<P0>(&self, midimessage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IMidiMessage>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendMessage)(::windows_core::Interface::as_raw(this), midimessage.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0>(&self, mididata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendBuffer)(::windows_core::Interface::as_raw(this), mididata.try_into_param()?.abi()).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IMidiOutPort, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for IMidiOutPort {}
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
impl ::windows_core::RuntimeType for IMidiOutPort {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{931d6d9f-57a2-4a3a-adb8-4640886f6693}");
}
unsafe impl ::windows_core::Interface for IMidiOutPort {
    type Vtable = IMidiOutPort_Vtbl;
}
impl ::core::clone::Clone for IMidiOutPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiOutPort {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x931d6d9f_57a2_4a3a_adb8_4640886f6693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPort_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, midimessage: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SendBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mididata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendBuffer: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiOutPortStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiOutPortStatics {
    type Vtable = IMidiOutPortStatics_Vtbl;
}
impl ::core::clone::Clone for IMidiOutPortStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiOutPortStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x065cc3e9_0f88_448b_9b64_a95826c65b8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPortStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiPitchBendChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiPitchBendChangeMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29df4cb1_2e9f_4faf_8c2b_9cb82a9079ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Bend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiPitchBendChangeMessageFactory {
    type Vtable = IMidiPitchBendChangeMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiPitchBendChangeMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiPitchBendChangeMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5eedf55_cfc8_4926_b30e_a3622393306c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiPitchBendChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, bend: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiPolyphonicKeyPressureMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiPolyphonicKeyPressureMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f7337fe_ace8_48a0_868e_7cdbf20f04d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiPolyphonicKeyPressureMessageFactory {
    type Vtable = IMidiPolyphonicKeyPressureMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiPolyphonicKeyPressureMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiPolyphonicKeyPressureMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe98f483e_c4b3_4dd2_917c_e349815a1b3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiPolyphonicKeyPressureMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, pressure: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiProgramChangeMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiProgramChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiProgramChangeMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cbb3c78_7a3e_4327_aa98_20b8e4485af8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Program: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiProgramChangeMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiProgramChangeMessageFactory {
    type Vtable = IMidiProgramChangeMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiProgramChangeMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiProgramChangeMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6b04387_524b_4104_9c99_6572bfd2e261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiProgramChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, program: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiSongPositionPointerMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiSongPositionPointerMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ca50c56_ec5e_4ae4_a115_88dc57cc2b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Beats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiSongPositionPointerMessageFactory {
    type Vtable = IMidiSongPositionPointerMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiSongPositionPointerMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiSongPositionPointerMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c00e996_f10b_4fea_b395_f5d6cf80f64e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiSongPositionPointerMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beats: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongSelectMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiSongSelectMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiSongSelectMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49f0f27f_6d83_4741_a5bf_4629f6be974f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Song: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongSelectMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiSongSelectMessageFactory {
    type Vtable = IMidiSongSelectMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiSongSelectMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiSongSelectMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x848878e4_8748_4129_a66c_a05493f75daa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiSongSelectMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, song: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSynthesizer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiSynthesizer {
    type Vtable = IMidiSynthesizer_Vtbl;
}
impl ::core::clone::Clone for IMidiSynthesizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiSynthesizer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0da155e_db90_405f_b8ae_21d2e17f2e45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub AudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    AudioDevice: usize,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSynthesizerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiSynthesizerStatics {
    type Vtable = IMidiSynthesizerStatics_Vtbl;
}
impl ::core::clone::Clone for IMidiSynthesizerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiSynthesizerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4224eaa8_6629_4d6b_aa8f_d4521a5a31ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub CreateFromAudioDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiodevice: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    CreateFromAudioDeviceAsync: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub IsSynthesizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mididevice: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    IsSynthesizer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSystemExclusiveMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiSystemExclusiveMessageFactory {
    type Vtable = IMidiSystemExclusiveMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiSystemExclusiveMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiSystemExclusiveMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x083de222_3b74_4320_9b42_0ca8545f8a24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSystemExclusiveMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateMidiSystemExclusiveMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateMidiSystemExclusiveMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiTimeCodeMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_Vtbl;
}
impl ::core::clone::Clone for IMidiTimeCodeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiTimeCodeMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bf7087d_fa63_4a1c_8deb_c0e87796a6d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FrameType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiTimeCodeMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMidiTimeCodeMessageFactory {
    type Vtable = IMidiTimeCodeMessageFactory_Vtbl;
}
impl ::core::clone::Clone for IMidiTimeCodeMessageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMidiTimeCodeMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb3099c5_771c_40de_b961_175a7489a85e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMidiTimeCodeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frametype: u8, values: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiActiveSensingMessage(::windows_core::IUnknown);
impl MidiActiveSensingMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiActiveSensingMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MidiActiveSensingMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiActiveSensingMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
impl ::core::clone::Clone for MidiActiveSensingMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiActiveSensingMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiActiveSensingMessage {
    const IID: ::windows_core::GUID = <IMidiMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiActiveSensingMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiActiveSensingMessage";
}
::windows_core::imp::interface_hierarchy!(MidiActiveSensingMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiActiveSensingMessage {}
unsafe impl ::core::marker::Send for MidiActiveSensingMessage {}
unsafe impl ::core::marker::Sync for MidiActiveSensingMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiChannelPressureMessage(::windows_core::IUnknown);
impl MidiChannelPressureMessage {
    pub fn Channel(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pressure)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiChannelPressureMessage(channel: u8, pressure: u8) -> ::windows_core::Result<MidiChannelPressureMessage> {
        Self::IMidiChannelPressureMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiChannelPressureMessage)(::windows_core::Interface::as_raw(this), channel, pressure, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMidiChannelPressureMessageFactory<R, F: FnOnce(&IMidiChannelPressureMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiChannelPressureMessage, IMidiChannelPressureMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiChannelPressureMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiChannelPressureMessage;{be1fa860-62b4-4d52-a37e-92e54d35b909})");
}
impl ::core::clone::Clone for MidiChannelPressureMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiChannelPressureMessage {
    const IID: ::windows_core::GUID = <IMidiChannelPressureMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiChannelPressureMessage";
}
::windows_core::imp::interface_hierarchy!(MidiChannelPressureMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiChannelPressureMessage {}
unsafe impl ::core::marker::Send for MidiChannelPressureMessage {}
unsafe impl ::core::marker::Sync for MidiChannelPressureMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiContinueMessage(::windows_core::IUnknown);
impl MidiContinueMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiContinueMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MidiContinueMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiContinueMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
impl ::core::clone::Clone for MidiContinueMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiContinueMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiContinueMessage {
    const IID: ::windows_core::GUID = <IMidiMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiContinueMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiContinueMessage";
}
::windows_core::imp::interface_hierarchy!(MidiContinueMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiContinueMessage {}
unsafe impl ::core::marker::Send for MidiContinueMessage {}
unsafe impl ::core::marker::Sync for MidiContinueMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiControlChangeMessage(::windows_core::IUnknown);
impl MidiControlChangeMessage {
    pub fn Channel(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Controller(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Controller)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ControlValue(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ControlValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiControlChangeMessage(channel: u8, controller: u8, controlvalue: u8) -> ::windows_core::Result<MidiControlChangeMessage> {
        Self::IMidiControlChangeMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiControlChangeMessage)(::windows_core::Interface::as_raw(this), channel, controller, controlvalue, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMidiControlChangeMessageFactory<R, F: FnOnce(&IMidiControlChangeMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiControlChangeMessage, IMidiControlChangeMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiControlChangeMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiControlChangeMessage;{b7e15f83-780d-405f-b781-3e1598c97f40})");
}
impl ::core::clone::Clone for MidiControlChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiControlChangeMessage {
    const IID: ::windows_core::GUID = <IMidiControlChangeMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiControlChangeMessage";
}
::windows_core::imp::interface_hierarchy!(MidiControlChangeMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiControlChangeMessage {}
unsafe impl ::core::marker::Send for MidiControlChangeMessage {}
unsafe impl ::core::marker::Sync for MidiControlChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiInPort(::windows_core::IUnknown);
impl MidiInPort {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MessageReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMessageReceived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MidiInPort>> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiInPortStatics<R, F: FnOnce(&IMidiInPortStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiInPort, IMidiInPortStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiInPort {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiInPort;{d5c1d9db-971a-4eaf-a23d-ea19fe607ff9})");
}
impl ::core::clone::Clone for MidiInPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiInPort {
    type Vtable = IMidiInPort_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiInPort {
    const IID: ::windows_core::GUID = <IMidiInPort as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiInPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiInPort";
}
::windows_core::imp::interface_hierarchy!(MidiInPort, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MidiInPort {}
unsafe impl ::core::marker::Send for MidiInPort {}
unsafe impl ::core::marker::Sync for MidiInPort {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiMessageReceivedEventArgs(::windows_core::IUnknown);
impl MidiMessageReceivedEventArgs {
    pub fn Message(&self) -> ::windows_core::Result<IMidiMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MidiMessageReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiMessageReceivedEventArgs;{76566e56-f328-4b51-907d-b3a8ce96bf80})");
}
impl ::core::clone::Clone for MidiMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiMessageReceivedEventArgs {
    const IID: ::windows_core::GUID = <IMidiMessageReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Midi.MidiMessageReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MidiMessageReceivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MidiMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MidiMessageReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiNoteOffMessage(::windows_core::IUnknown);
impl MidiNoteOffMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Note(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Note)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Velocity(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Velocity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiNoteOffMessage(channel: u8, note: u8, velocity: u8) -> ::windows_core::Result<MidiNoteOffMessage> {
        Self::IMidiNoteOffMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiNoteOffMessage)(::windows_core::Interface::as_raw(this), channel, note, velocity, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiNoteOffMessageFactory<R, F: FnOnce(&IMidiNoteOffMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiNoteOffMessage, IMidiNoteOffMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiNoteOffMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiNoteOffMessage;{16fd8af4-198e-4d8f-a654-d305a293548f})");
}
impl ::core::clone::Clone for MidiNoteOffMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiNoteOffMessage {
    const IID: ::windows_core::GUID = <IMidiNoteOffMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOffMessage";
}
::windows_core::imp::interface_hierarchy!(MidiNoteOffMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiNoteOffMessage {}
unsafe impl ::core::marker::Send for MidiNoteOffMessage {}
unsafe impl ::core::marker::Sync for MidiNoteOffMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiNoteOnMessage(::windows_core::IUnknown);
impl MidiNoteOnMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Note(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Note)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Velocity(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Velocity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiNoteOnMessage(channel: u8, note: u8, velocity: u8) -> ::windows_core::Result<MidiNoteOnMessage> {
        Self::IMidiNoteOnMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiNoteOnMessage)(::windows_core::Interface::as_raw(this), channel, note, velocity, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiNoteOnMessageFactory<R, F: FnOnce(&IMidiNoteOnMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiNoteOnMessage, IMidiNoteOnMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiNoteOnMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiNoteOnMessage;{e0224af5-6181-46dd-afa2-410004c057aa})");
}
impl ::core::clone::Clone for MidiNoteOnMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiNoteOnMessage {
    const IID: ::windows_core::GUID = <IMidiNoteOnMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOnMessage";
}
::windows_core::imp::interface_hierarchy!(MidiNoteOnMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiNoteOnMessage {}
unsafe impl ::core::marker::Send for MidiNoteOnMessage {}
unsafe impl ::core::marker::Sync for MidiNoteOnMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiOutPort(::windows_core::IUnknown);
impl MidiOutPort {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SendMessage<P0>(&self, midimessage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IMidiMessage>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendMessage)(::windows_core::Interface::as_raw(this), midimessage.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0>(&self, mididata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendBuffer)(::windows_core::Interface::as_raw(this), mididata.try_into_param()?.abi()).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiOutPortStatics<R, F: FnOnce(&IMidiOutPortStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiOutPort, IMidiOutPortStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiOutPort {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiOutPort;{931d6d9f-57a2-4a3a-adb8-4640886f6693})");
}
impl ::core::clone::Clone for MidiOutPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiOutPort {
    type Vtable = IMidiOutPort_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiOutPort {
    const IID: ::windows_core::GUID = <IMidiOutPort as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiOutPort";
}
::windows_core::imp::interface_hierarchy!(MidiOutPort, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MidiOutPort {}
impl ::windows_core::CanTryInto<IMidiOutPort> for MidiOutPort {}
unsafe impl ::core::marker::Send for MidiOutPort {}
unsafe impl ::core::marker::Sync for MidiOutPort {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiPitchBendChangeMessage(::windows_core::IUnknown);
impl MidiPitchBendChangeMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Bend(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bend)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiPitchBendChangeMessage(channel: u8, bend: u16) -> ::windows_core::Result<MidiPitchBendChangeMessage> {
        Self::IMidiPitchBendChangeMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiPitchBendChangeMessage)(::windows_core::Interface::as_raw(this), channel, bend, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiPitchBendChangeMessageFactory<R, F: FnOnce(&IMidiPitchBendChangeMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiPitchBendChangeMessage, IMidiPitchBendChangeMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiPitchBendChangeMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiPitchBendChangeMessage;{29df4cb1-2e9f-4faf-8c2b-9cb82a9079ca})");
}
impl ::core::clone::Clone for MidiPitchBendChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiPitchBendChangeMessage {
    const IID: ::windows_core::GUID = <IMidiPitchBendChangeMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPitchBendChangeMessage";
}
::windows_core::imp::interface_hierarchy!(MidiPitchBendChangeMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiPitchBendChangeMessage {}
unsafe impl ::core::marker::Send for MidiPitchBendChangeMessage {}
unsafe impl ::core::marker::Sync for MidiPitchBendChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiPolyphonicKeyPressureMessage(::windows_core::IUnknown);
impl MidiPolyphonicKeyPressureMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Note(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Note)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pressure)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiPolyphonicKeyPressureMessage(channel: u8, note: u8, pressure: u8) -> ::windows_core::Result<MidiPolyphonicKeyPressureMessage> {
        Self::IMidiPolyphonicKeyPressureMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiPolyphonicKeyPressureMessage)(::windows_core::Interface::as_raw(this), channel, note, pressure, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiPolyphonicKeyPressureMessageFactory<R, F: FnOnce(&IMidiPolyphonicKeyPressureMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiPolyphonicKeyPressureMessage, IMidiPolyphonicKeyPressureMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiPolyphonicKeyPressureMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage;{1f7337fe-ace8-48a0-868e-7cdbf20f04d6})");
}
impl ::core::clone::Clone for MidiPolyphonicKeyPressureMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiPolyphonicKeyPressureMessage {
    const IID: ::windows_core::GUID = <IMidiPolyphonicKeyPressureMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage";
}
::windows_core::imp::interface_hierarchy!(MidiPolyphonicKeyPressureMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiPolyphonicKeyPressureMessage {}
unsafe impl ::core::marker::Send for MidiPolyphonicKeyPressureMessage {}
unsafe impl ::core::marker::Sync for MidiPolyphonicKeyPressureMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiProgramChangeMessage(::windows_core::IUnknown);
impl MidiProgramChangeMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Channel(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Program(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Program)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiProgramChangeMessage(channel: u8, program: u8) -> ::windows_core::Result<MidiProgramChangeMessage> {
        Self::IMidiProgramChangeMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiProgramChangeMessage)(::windows_core::Interface::as_raw(this), channel, program, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiProgramChangeMessageFactory<R, F: FnOnce(&IMidiProgramChangeMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiProgramChangeMessage, IMidiProgramChangeMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiProgramChangeMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiProgramChangeMessage;{9cbb3c78-7a3e-4327-aa98-20b8e4485af8})");
}
impl ::core::clone::Clone for MidiProgramChangeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiProgramChangeMessage {
    const IID: ::windows_core::GUID = <IMidiProgramChangeMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiProgramChangeMessage";
}
::windows_core::imp::interface_hierarchy!(MidiProgramChangeMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiProgramChangeMessage {}
unsafe impl ::core::marker::Send for MidiProgramChangeMessage {}
unsafe impl ::core::marker::Sync for MidiProgramChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSongPositionPointerMessage(::windows_core::IUnknown);
impl MidiSongPositionPointerMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Beats(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Beats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiSongPositionPointerMessage(beats: u16) -> ::windows_core::Result<MidiSongPositionPointerMessage> {
        Self::IMidiSongPositionPointerMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiSongPositionPointerMessage)(::windows_core::Interface::as_raw(this), beats, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSongPositionPointerMessageFactory<R, F: FnOnce(&IMidiSongPositionPointerMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiSongPositionPointerMessage, IMidiSongPositionPointerMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiSongPositionPointerMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSongPositionPointerMessage;{4ca50c56-ec5e-4ae4-a115-88dc57cc2b79})");
}
impl ::core::clone::Clone for MidiSongPositionPointerMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiSongPositionPointerMessage {
    const IID: ::windows_core::GUID = <IMidiSongPositionPointerMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongPositionPointerMessage";
}
::windows_core::imp::interface_hierarchy!(MidiSongPositionPointerMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiSongPositionPointerMessage {}
unsafe impl ::core::marker::Send for MidiSongPositionPointerMessage {}
unsafe impl ::core::marker::Sync for MidiSongPositionPointerMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSongSelectMessage(::windows_core::IUnknown);
impl MidiSongSelectMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Song(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Song)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiSongSelectMessage(song: u8) -> ::windows_core::Result<MidiSongSelectMessage> {
        Self::IMidiSongSelectMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiSongSelectMessage)(::windows_core::Interface::as_raw(this), song, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSongSelectMessageFactory<R, F: FnOnce(&IMidiSongSelectMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiSongSelectMessage, IMidiSongSelectMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiSongSelectMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSongSelectMessage;{49f0f27f-6d83-4741-a5bf-4629f6be974f})");
}
impl ::core::clone::Clone for MidiSongSelectMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiSongSelectMessage {
    const IID: ::windows_core::GUID = <IMidiSongSelectMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongSelectMessage";
}
::windows_core::imp::interface_hierarchy!(MidiSongSelectMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiSongSelectMessage {}
unsafe impl ::core::marker::Send for MidiSongSelectMessage {}
unsafe impl ::core::marker::Sync for MidiSongSelectMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiStartMessage(::windows_core::IUnknown);
impl MidiStartMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiStartMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MidiStartMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiStartMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
impl ::core::clone::Clone for MidiStartMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiStartMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiStartMessage {
    const IID: ::windows_core::GUID = <IMidiMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiStartMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStartMessage";
}
::windows_core::imp::interface_hierarchy!(MidiStartMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiStartMessage {}
unsafe impl ::core::marker::Send for MidiStartMessage {}
unsafe impl ::core::marker::Sync for MidiStartMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiStopMessage(::windows_core::IUnknown);
impl MidiStopMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiStopMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MidiStopMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiStopMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
impl ::core::clone::Clone for MidiStopMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiStopMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiStopMessage {
    const IID: ::windows_core::GUID = <IMidiMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiStopMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStopMessage";
}
::windows_core::imp::interface_hierarchy!(MidiStopMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiStopMessage {}
unsafe impl ::core::marker::Send for MidiStopMessage {}
unsafe impl ::core::marker::Sync for MidiStopMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSynthesizer(::windows_core::IUnknown);
impl MidiSynthesizer {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SendMessage<P0>(&self, midimessage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IMidiMessage>,
    {
        let this = &::windows_core::ComInterface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SendMessage)(::windows_core::Interface::as_raw(this), midimessage.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<P0>(&self, mididata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SendBuffer)(::windows_core::Interface::as_raw(this), mididata.try_into_param()?.abi()).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMidiOutPort>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn AudioDevice(&self) -> ::windows_core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioDevice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Volume(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Volume)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVolume)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn CreateFromAudioDeviceAsync<P0>(audiodevice: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>
    where
        P0: ::windows_core::IntoParam<super::Enumeration::DeviceInformation>,
    {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromAudioDeviceAsync)(::windows_core::Interface::as_raw(this), audiodevice.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn IsSynthesizer<P0>(mididevice: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::Enumeration::DeviceInformation>,
    {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSynthesizer)(::windows_core::Interface::as_raw(this), mididevice.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSynthesizerStatics<R, F: FnOnce(&IMidiSynthesizerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiSynthesizer, IMidiSynthesizerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiSynthesizer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSynthesizer;{f0da155e-db90-405f-b8ae-21d2e17f2e45})");
}
impl ::core::clone::Clone for MidiSynthesizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiSynthesizer {
    type Vtable = IMidiSynthesizer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiSynthesizer {
    const IID: ::windows_core::GUID = <IMidiSynthesizer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSynthesizer";
}
::windows_core::imp::interface_hierarchy!(MidiSynthesizer, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MidiSynthesizer {}
impl ::windows_core::CanTryInto<IMidiOutPort> for MidiSynthesizer {}
unsafe impl ::core::marker::Send for MidiSynthesizer {}
unsafe impl ::core::marker::Sync for MidiSynthesizer {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSystemExclusiveMessage(::windows_core::IUnknown);
impl MidiSystemExclusiveMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateMidiSystemExclusiveMessage<P0>(rawdata: P0) -> ::windows_core::Result<MidiSystemExclusiveMessage>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IMidiSystemExclusiveMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiSystemExclusiveMessage)(::windows_core::Interface::as_raw(this), rawdata.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSystemExclusiveMessageFactory<R, F: FnOnce(&IMidiSystemExclusiveMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiSystemExclusiveMessage, IMidiSystemExclusiveMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiSystemExclusiveMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSystemExclusiveMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
impl ::core::clone::Clone for MidiSystemExclusiveMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiSystemExclusiveMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiSystemExclusiveMessage {
    const IID: ::windows_core::GUID = <IMidiMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiSystemExclusiveMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemExclusiveMessage";
}
::windows_core::imp::interface_hierarchy!(MidiSystemExclusiveMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiSystemExclusiveMessage {}
unsafe impl ::core::marker::Send for MidiSystemExclusiveMessage {}
unsafe impl ::core::marker::Sync for MidiSystemExclusiveMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSystemResetMessage(::windows_core::IUnknown);
impl MidiSystemResetMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiSystemResetMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MidiSystemResetMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSystemResetMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
impl ::core::clone::Clone for MidiSystemResetMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiSystemResetMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiSystemResetMessage {
    const IID: ::windows_core::GUID = <IMidiMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiSystemResetMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemResetMessage";
}
::windows_core::imp::interface_hierarchy!(MidiSystemResetMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiSystemResetMessage {}
unsafe impl ::core::marker::Send for MidiSystemResetMessage {}
unsafe impl ::core::marker::Sync for MidiSystemResetMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiTimeCodeMessage(::windows_core::IUnknown);
impl MidiTimeCodeMessage {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = &::windows_core::ComInterface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FrameType(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Values(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Values)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateMidiTimeCodeMessage(frametype: u8, values: u8) -> ::windows_core::Result<MidiTimeCodeMessage> {
        Self::IMidiTimeCodeMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMidiTimeCodeMessage)(::windows_core::Interface::as_raw(this), frametype, values, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiTimeCodeMessageFactory<R, F: FnOnce(&IMidiTimeCodeMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiTimeCodeMessage, IMidiTimeCodeMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for MidiTimeCodeMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTimeCodeMessage;{0bf7087d-fa63-4a1c-8deb-c0e87796a6d7})");
}
impl ::core::clone::Clone for MidiTimeCodeMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiTimeCodeMessage {
    const IID: ::windows_core::GUID = <IMidiTimeCodeMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimeCodeMessage";
}
::windows_core::imp::interface_hierarchy!(MidiTimeCodeMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiTimeCodeMessage {}
unsafe impl ::core::marker::Send for MidiTimeCodeMessage {}
unsafe impl ::core::marker::Sync for MidiTimeCodeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiTimingClockMessage(::windows_core::IUnknown);
impl MidiTimingClockMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiTimingClockMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MidiTimingClockMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTimingClockMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
impl ::core::clone::Clone for MidiTimingClockMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiTimingClockMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiTimingClockMessage {
    const IID: ::windows_core::GUID = <IMidiMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiTimingClockMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimingClockMessage";
}
::windows_core::imp::interface_hierarchy!(MidiTimingClockMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiTimingClockMessage {}
unsafe impl ::core::marker::Send for MidiTimingClockMessage {}
unsafe impl ::core::marker::Sync for MidiTimingClockMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiTuneRequestMessage(::windows_core::IUnknown);
impl MidiTuneRequestMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MidiTuneRequestMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RawData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for MidiTuneRequestMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTuneRequestMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
impl ::core::clone::Clone for MidiTuneRequestMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MidiTuneRequestMessage {
    type Vtable = IMidiMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MidiTuneRequestMessage {
    const IID: ::windows_core::GUID = <IMidiMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MidiTuneRequestMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTuneRequestMessage";
}
::windows_core::imp::interface_hierarchy!(MidiTuneRequestMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMidiMessage> for MidiTuneRequestMessage {}
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
impl ::windows_core::TypeKind for MidiMessageType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MidiMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiMessageType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MidiMessageType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Midi.MidiMessageType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
