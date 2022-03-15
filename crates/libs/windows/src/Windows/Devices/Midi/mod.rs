#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiChannelPressureMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe1fa860_62b4_4d52_a37e_92e54d35b909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiChannelPressureMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiChannelPressureMessageFactory {
    type Vtable = IMidiChannelPressureMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6218ed2f_2284_412a_94cf_10fb04842c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiChannelPressureMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, pressure: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiControlChangeMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7e15f83_780d_405f_b781_3e1598c97f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ControlValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiControlChangeMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiControlChangeMessageFactory {
    type Vtable = IMidiControlChangeMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ab14321_956c_46ad_9752_f87f55052fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiControlChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, controller: u8, controlvalue: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiInPort(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiInPort {
    type Vtable = IMidiInPort_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5c1d9db_971a_4eaf_a23d_ea19fe607ff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPort_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiInPortStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiInPortStatics {
    type Vtable = IMidiInPortStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44c439dc_67ff_4a6e_8bac_fdb6610cf296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPortStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct IMidiMessage(::windows::core::IUnknown);
impl IMidiMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
impl ::core::convert::From<IMidiMessage> for ::windows::core::IUnknown {
    fn from(value: IMidiMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMidiMessage> for ::windows::core::IUnknown {
    fn from(value: &IMidiMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMidiMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMidiMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMidiMessage> for ::windows::core::IInspectable {
    fn from(value: IMidiMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMidiMessage> for ::windows::core::IInspectable {
    fn from(value: &IMidiMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMidiMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMidiMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IMidiMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79767945_1094_4283_9be0_289fc0ee8334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MidiMessageType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiMessageReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76566e56_f328_4b51_907d_b3a8ce96bf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessageReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOffMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16fd8af4_198e_4d8f_a654_d305a293548f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOffMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiNoteOffMessageFactory {
    type Vtable = IMidiNoteOffMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6b240e0_a749_425f_8af4_a4d979cc15b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiNoteOffMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOnMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0224af5_6181_46dd_afa2_410004c057aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Velocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiNoteOnMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiNoteOnMessageFactory {
    type Vtable = IMidiNoteOnMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b4280a0_59c1_420e_b517_15a10aa9606b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiNoteOnMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct IMidiOutPort(::windows::core::IUnknown);
impl IMidiOutPort {
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn SendMessage<'a, Param0: ::windows::core::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SendMessage)(::core::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SendBuffer)(::core::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IMidiOutPort> for ::windows::core::IUnknown {
    fn from(value: IMidiOutPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMidiOutPort> for ::windows::core::IUnknown {
    fn from(value: &IMidiOutPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMidiOutPort> for ::windows::core::IInspectable {
    fn from(value: IMidiOutPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMidiOutPort> for ::windows::core::IInspectable {
    fn from(value: &IMidiOutPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IMidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IMidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
unsafe impl ::windows::core::Interface for IMidiOutPort {
    type Vtable = IMidiOutPort_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x931d6d9f_57a2_4a3a_adb8_4640886f6693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPort_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, midimessage: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SendBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mididata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendBuffer: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiOutPortStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiOutPortStatics {
    type Vtable = IMidiOutPortStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x065cc3e9_0f88_448b_9b64_a95826c65b8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPortStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29df4cb1_2e9f_4faf_8c2b_9cb82a9079ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Bend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiPitchBendChangeMessageFactory {
    type Vtable = IMidiPitchBendChangeMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5eedf55_cfc8_4926_b30e_a3622393306c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiPitchBendChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, bend: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f7337fe_ace8_48a0_868e_7cdbf20f04d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiPolyphonicKeyPressureMessageFactory {
    type Vtable = IMidiPolyphonicKeyPressureMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe98f483e_c4b3_4dd2_917c_e349815a1b3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiPolyphonicKeyPressureMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, note: u8, pressure: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiProgramChangeMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbb3c78_7a3e_4327_aa98_20b8e4485af8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Program: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiProgramChangeMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiProgramChangeMessageFactory {
    type Vtable = IMidiProgramChangeMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b04387_524b_4104_9c99_6572bfd2e261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiProgramChangeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: u8, program: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ca50c56_ec5e_4ae4_a115_88dc57cc2b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Beats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiSongPositionPointerMessageFactory {
    type Vtable = IMidiSongPositionPointerMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c00e996_f10b_4fea_b395_f5d6cf80f64e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiSongPositionPointerMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beats: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongSelectMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49f0f27f_6d83_4741_a5bf_4629f6be974f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Song: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSongSelectMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiSongSelectMessageFactory {
    type Vtable = IMidiSongSelectMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x848878e4_8748_4129_a66c_a05493f75daa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiSongSelectMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, song: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSynthesizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiSynthesizer {
    type Vtable = IMidiSynthesizer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0da155e_db90_405f_b8ae_21d2e17f2e45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub AudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    AudioDevice: usize,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSynthesizerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiSynthesizerStatics {
    type Vtable = IMidiSynthesizerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4224eaa8_6629_4d6b_aa8f_d4521a5a31ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub CreateFromAudioDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiodevice: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    CreateFromAudioDeviceAsync: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub IsSynthesizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mididevice: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    IsSynthesizer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiSystemExclusiveMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiSystemExclusiveMessageFactory {
    type Vtable = IMidiSystemExclusiveMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x083de222_3b74_4320_9b42_0ca8545f8a24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSystemExclusiveMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateMidiSystemExclusiveMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateMidiSystemExclusiveMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiTimeCodeMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bf7087d_fa63_4a1c_8deb_c0e87796a6d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FrameType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMidiTimeCodeMessageFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMidiTimeCodeMessageFactory {
    type Vtable = IMidiTimeCodeMessageFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb3099c5_771c_40de_b961_175a7489a85e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessageFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateMidiTimeCodeMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frametype: u8, values: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiActiveSensingMessage(::windows::core::IUnknown);
impl MidiActiveSensingMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiActiveSensingMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
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
unsafe impl ::windows::core::Interface for MidiActiveSensingMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiActiveSensingMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiActiveSensingMessage";
}
impl ::core::convert::From<MidiActiveSensingMessage> for ::windows::core::IUnknown {
    fn from(value: MidiActiveSensingMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiActiveSensingMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiActiveSensingMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiActiveSensingMessage> for ::windows::core::IInspectable {
    fn from(value: MidiActiveSensingMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiActiveSensingMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiActiveSensingMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiActiveSensingMessage {}
unsafe impl ::core::marker::Sync for MidiActiveSensingMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiChannelPressureMessage(::windows::core::IUnknown);
impl MidiChannelPressureMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Pressure(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pressure)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiChannelPressureMessage(channel: u8, pressure: u8) -> ::windows::core::Result<MidiChannelPressureMessage> {
        Self::IMidiChannelPressureMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiChannelPressureMessage)(::core::mem::transmute_copy(this), channel, pressure, &mut result__).from_abi::<MidiChannelPressureMessage>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMidiChannelPressureMessageFactory<R, F: FnOnce(&IMidiChannelPressureMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiChannelPressureMessage, IMidiChannelPressureMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiChannelPressureMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiChannelPressureMessage";
}
impl ::core::convert::From<MidiChannelPressureMessage> for ::windows::core::IUnknown {
    fn from(value: MidiChannelPressureMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiChannelPressureMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiChannelPressureMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiChannelPressureMessage> for ::windows::core::IInspectable {
    fn from(value: MidiChannelPressureMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiChannelPressureMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiChannelPressureMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiContinueMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
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
unsafe impl ::windows::core::Interface for MidiContinueMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiContinueMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiContinueMessage";
}
impl ::core::convert::From<MidiContinueMessage> for ::windows::core::IUnknown {
    fn from(value: MidiContinueMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiContinueMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiContinueMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiContinueMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiContinueMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiContinueMessage> for ::windows::core::IInspectable {
    fn from(value: MidiContinueMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiContinueMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiContinueMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiContinueMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiContinueMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiContinueMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiContinueMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiContinueMessage {}
unsafe impl ::core::marker::Sync for MidiContinueMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiControlChangeMessage(::windows::core::IUnknown);
impl MidiControlChangeMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Controller(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Controller)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn ControlValue(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ControlValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiControlChangeMessage(channel: u8, controller: u8, controlvalue: u8) -> ::windows::core::Result<MidiControlChangeMessage> {
        Self::IMidiControlChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiControlChangeMessage)(::core::mem::transmute_copy(this), channel, controller, controlvalue, &mut result__).from_abi::<MidiControlChangeMessage>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMidiControlChangeMessageFactory<R, F: FnOnce(&IMidiControlChangeMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiControlChangeMessage, IMidiControlChangeMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiControlChangeMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiControlChangeMessage";
}
impl ::core::convert::From<MidiControlChangeMessage> for ::windows::core::IUnknown {
    fn from(value: MidiControlChangeMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiControlChangeMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiControlChangeMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiControlChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiControlChangeMessage> for ::windows::core::IInspectable {
    fn from(value: MidiControlChangeMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiControlChangeMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiControlChangeMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiControlChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiControlChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiControlChangeMessage {}
unsafe impl ::core::marker::Sync for MidiControlChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiInPort(::windows::core::IUnknown);
impl MidiInPort {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MessageReceived)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMessageReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiInPort>> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiInPort>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiInPortStatics<R, F: FnOnce(&IMidiInPortStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiInPort, IMidiInPortStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiInPort {
    type Vtable = IMidiInPort_Vtbl;
    const IID: ::windows::core::GUID = <IMidiInPort as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiInPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiInPort";
}
impl ::core::convert::From<MidiInPort> for ::windows::core::IUnknown {
    fn from(value: MidiInPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiInPort> for ::windows::core::IUnknown {
    fn from(value: &MidiInPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiInPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiInPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiInPort> for ::windows::core::IInspectable {
    fn from(value: MidiInPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiInPort> for ::windows::core::IInspectable {
    fn from(value: &MidiInPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiInPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiInPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MidiInPort {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MidiInPort {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiInPort {}
unsafe impl ::core::marker::Sync for MidiInPort {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiMessageReceivedEventArgs(::windows::core::IUnknown);
impl MidiMessageReceivedEventArgs {
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<IMidiMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMidiMessage>(result__)
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
unsafe impl ::windows::core::Interface for MidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessageReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Midi.MidiMessageReceivedEventArgs";
}
impl ::core::convert::From<MidiMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MidiMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiMessageReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MidiMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiMessageReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MidiMessageReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiMessageReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MidiMessageReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MidiMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MidiMessageReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiNoteOffMessage(::windows::core::IUnknown);
impl MidiNoteOffMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Note(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Note)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Velocity(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiNoteOffMessage(channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOffMessage> {
        Self::IMidiNoteOffMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiNoteOffMessage)(::core::mem::transmute_copy(this), channel, note, velocity, &mut result__).from_abi::<MidiNoteOffMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiNoteOffMessageFactory<R, F: FnOnce(&IMidiNoteOffMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiNoteOffMessage, IMidiNoteOffMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiNoteOffMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOffMessage";
}
impl ::core::convert::From<MidiNoteOffMessage> for ::windows::core::IUnknown {
    fn from(value: MidiNoteOffMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiNoteOffMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiNoteOffMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiNoteOffMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiNoteOffMessage> for ::windows::core::IInspectable {
    fn from(value: MidiNoteOffMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiNoteOffMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiNoteOffMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiNoteOffMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiNoteOffMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiNoteOffMessage {}
unsafe impl ::core::marker::Sync for MidiNoteOffMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiNoteOnMessage(::windows::core::IUnknown);
impl MidiNoteOnMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Note(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Note)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Velocity(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiNoteOnMessage(channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOnMessage> {
        Self::IMidiNoteOnMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiNoteOnMessage)(::core::mem::transmute_copy(this), channel, note, velocity, &mut result__).from_abi::<MidiNoteOnMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiNoteOnMessageFactory<R, F: FnOnce(&IMidiNoteOnMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiNoteOnMessage, IMidiNoteOnMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiNoteOnMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOnMessage";
}
impl ::core::convert::From<MidiNoteOnMessage> for ::windows::core::IUnknown {
    fn from(value: MidiNoteOnMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiNoteOnMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiNoteOnMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiNoteOnMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiNoteOnMessage> for ::windows::core::IInspectable {
    fn from(value: MidiNoteOnMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiNoteOnMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiNoteOnMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiNoteOnMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiNoteOnMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiNoteOnMessage {}
unsafe impl ::core::marker::Sync for MidiNoteOnMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiOutPort(::windows::core::IUnknown);
impl MidiOutPort {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn SendMessage<'a, Param0: ::windows::core::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SendMessage)(::core::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SendBuffer)(::core::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IMidiOutPort>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiOutPortStatics<R, F: FnOnce(&IMidiOutPortStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiOutPort, IMidiOutPortStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiOutPort {
    type Vtable = IMidiOutPort_Vtbl;
    const IID: ::windows::core::GUID = <IMidiOutPort as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiOutPort";
}
impl ::core::convert::From<MidiOutPort> for ::windows::core::IUnknown {
    fn from(value: MidiOutPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiOutPort> for ::windows::core::IUnknown {
    fn from(value: &MidiOutPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiOutPort> for ::windows::core::IInspectable {
    fn from(value: MidiOutPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiOutPort> for ::windows::core::IInspectable {
    fn from(value: &MidiOutPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, IMidiOutPort> for MidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiOutPort> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiOutPort> for &MidiOutPort {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiOutPort> {
        ::core::convert::TryInto::<IMidiOutPort>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiOutPort {}
unsafe impl ::core::marker::Sync for MidiOutPort {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiPitchBendChangeMessage(::windows::core::IUnknown);
impl MidiPitchBendChangeMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Bend(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bend)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiPitchBendChangeMessage(channel: u8, bend: u16) -> ::windows::core::Result<MidiPitchBendChangeMessage> {
        Self::IMidiPitchBendChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiPitchBendChangeMessage)(::core::mem::transmute_copy(this), channel, bend, &mut result__).from_abi::<MidiPitchBendChangeMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiPitchBendChangeMessageFactory<R, F: FnOnce(&IMidiPitchBendChangeMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiPitchBendChangeMessage, IMidiPitchBendChangeMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiPitchBendChangeMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPitchBendChangeMessage";
}
impl ::core::convert::From<MidiPitchBendChangeMessage> for ::windows::core::IUnknown {
    fn from(value: MidiPitchBendChangeMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiPitchBendChangeMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiPitchBendChangeMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiPitchBendChangeMessage> for ::windows::core::IInspectable {
    fn from(value: MidiPitchBendChangeMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiPitchBendChangeMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiPitchBendChangeMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiPitchBendChangeMessage {}
unsafe impl ::core::marker::Sync for MidiPitchBendChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiPolyphonicKeyPressureMessage(::windows::core::IUnknown);
impl MidiPolyphonicKeyPressureMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Note(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Note)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Pressure(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pressure)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiPolyphonicKeyPressureMessage(channel: u8, note: u8, pressure: u8) -> ::windows::core::Result<MidiPolyphonicKeyPressureMessage> {
        Self::IMidiPolyphonicKeyPressureMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiPolyphonicKeyPressureMessage)(::core::mem::transmute_copy(this), channel, note, pressure, &mut result__).from_abi::<MidiPolyphonicKeyPressureMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiPolyphonicKeyPressureMessageFactory<R, F: FnOnce(&IMidiPolyphonicKeyPressureMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiPolyphonicKeyPressureMessage, IMidiPolyphonicKeyPressureMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiPolyphonicKeyPressureMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage";
}
impl ::core::convert::From<MidiPolyphonicKeyPressureMessage> for ::windows::core::IUnknown {
    fn from(value: MidiPolyphonicKeyPressureMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiPolyphonicKeyPressureMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiPolyphonicKeyPressureMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiPolyphonicKeyPressureMessage> for ::windows::core::IInspectable {
    fn from(value: MidiPolyphonicKeyPressureMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiPolyphonicKeyPressureMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiPolyphonicKeyPressureMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiPolyphonicKeyPressureMessage {}
unsafe impl ::core::marker::Sync for MidiPolyphonicKeyPressureMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiProgramChangeMessage(::windows::core::IUnknown);
impl MidiProgramChangeMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Program(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Program)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiProgramChangeMessage(channel: u8, program: u8) -> ::windows::core::Result<MidiProgramChangeMessage> {
        Self::IMidiProgramChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiProgramChangeMessage)(::core::mem::transmute_copy(this), channel, program, &mut result__).from_abi::<MidiProgramChangeMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiProgramChangeMessageFactory<R, F: FnOnce(&IMidiProgramChangeMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiProgramChangeMessage, IMidiProgramChangeMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiProgramChangeMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiProgramChangeMessage";
}
impl ::core::convert::From<MidiProgramChangeMessage> for ::windows::core::IUnknown {
    fn from(value: MidiProgramChangeMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiProgramChangeMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiProgramChangeMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiProgramChangeMessage> for ::windows::core::IInspectable {
    fn from(value: MidiProgramChangeMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiProgramChangeMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiProgramChangeMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiProgramChangeMessage {}
unsafe impl ::core::marker::Sync for MidiProgramChangeMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSongPositionPointerMessage(::windows::core::IUnknown);
impl MidiSongPositionPointerMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Beats(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Beats)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiSongPositionPointerMessage(beats: u16) -> ::windows::core::Result<MidiSongPositionPointerMessage> {
        Self::IMidiSongPositionPointerMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiSongPositionPointerMessage)(::core::mem::transmute_copy(this), beats, &mut result__).from_abi::<MidiSongPositionPointerMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSongPositionPointerMessageFactory<R, F: FnOnce(&IMidiSongPositionPointerMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiSongPositionPointerMessage, IMidiSongPositionPointerMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiSongPositionPointerMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongPositionPointerMessage";
}
impl ::core::convert::From<MidiSongPositionPointerMessage> for ::windows::core::IUnknown {
    fn from(value: MidiSongPositionPointerMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSongPositionPointerMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiSongPositionPointerMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiSongPositionPointerMessage> for ::windows::core::IInspectable {
    fn from(value: MidiSongPositionPointerMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSongPositionPointerMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiSongPositionPointerMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiSongPositionPointerMessage {}
unsafe impl ::core::marker::Sync for MidiSongPositionPointerMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSongSelectMessage(::windows::core::IUnknown);
impl MidiSongSelectMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Song(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Song)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiSongSelectMessage(song: u8) -> ::windows::core::Result<MidiSongSelectMessage> {
        Self::IMidiSongSelectMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiSongSelectMessage)(::core::mem::transmute_copy(this), song, &mut result__).from_abi::<MidiSongSelectMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSongSelectMessageFactory<R, F: FnOnce(&IMidiSongSelectMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiSongSelectMessage, IMidiSongSelectMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiSongSelectMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongSelectMessage";
}
impl ::core::convert::From<MidiSongSelectMessage> for ::windows::core::IUnknown {
    fn from(value: MidiSongSelectMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSongSelectMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiSongSelectMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiSongSelectMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiSongSelectMessage> for ::windows::core::IInspectable {
    fn from(value: MidiSongSelectMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSongSelectMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiSongSelectMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiSongSelectMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiSongSelectMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiStartMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
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
unsafe impl ::windows::core::Interface for MidiStartMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiStartMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStartMessage";
}
impl ::core::convert::From<MidiStartMessage> for ::windows::core::IUnknown {
    fn from(value: MidiStartMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiStartMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiStartMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiStartMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiStartMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiStartMessage> for ::windows::core::IInspectable {
    fn from(value: MidiStartMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiStartMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiStartMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiStartMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiStartMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiStartMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiStartMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiStopMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
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
unsafe impl ::windows::core::Interface for MidiStopMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiStopMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStopMessage";
}
impl ::core::convert::From<MidiStopMessage> for ::windows::core::IUnknown {
    fn from(value: MidiStopMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiStopMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiStopMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiStopMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiStopMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiStopMessage> for ::windows::core::IInspectable {
    fn from(value: MidiStopMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiStopMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiStopMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiStopMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiStopMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiStopMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiStopMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiStopMessage {}
unsafe impl ::core::marker::Sync for MidiStopMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSynthesizer(::windows::core::IUnknown);
impl MidiSynthesizer {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn SendMessage<'a, Param0: ::windows::core::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SendMessage)(::core::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SendBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SendBuffer)(::core::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMidiOutPort>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn AudioDevice(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Volume)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn SetVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVolume)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn CreateFromAudioDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Enumeration::DeviceInformation>>(audiodevice: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromAudioDeviceAsync)(::core::mem::transmute_copy(this), audiodevice.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn IsSynthesizer<'a, Param0: ::windows::core::IntoParam<'a, super::Enumeration::DeviceInformation>>(mididevice: Param0) -> ::windows::core::Result<bool> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSynthesizer)(::core::mem::transmute_copy(this), mididevice.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSynthesizerStatics<R, F: FnOnce(&IMidiSynthesizerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiSynthesizer, IMidiSynthesizerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiSynthesizer {
    type Vtable = IMidiSynthesizer_Vtbl;
    const IID: ::windows::core::GUID = <IMidiSynthesizer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSynthesizer";
}
impl ::core::convert::From<MidiSynthesizer> for ::windows::core::IUnknown {
    fn from(value: MidiSynthesizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSynthesizer> for ::windows::core::IUnknown {
    fn from(value: &MidiSynthesizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiSynthesizer> for ::windows::core::IInspectable {
    fn from(value: MidiSynthesizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSynthesizer> for ::windows::core::IInspectable {
    fn from(value: &MidiSynthesizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MidiSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MidiSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, IMidiOutPort> for MidiSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiOutPort> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiOutPort> for &MidiSynthesizer {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiOutPort> {
        ::core::convert::TryInto::<IMidiOutPort>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiSynthesizer {}
unsafe impl ::core::marker::Sync for MidiSynthesizer {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiSystemExclusiveMessage(::windows::core::IUnknown);
impl MidiSystemExclusiveMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateMidiSystemExclusiveMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(rawdata: Param0) -> ::windows::core::Result<MidiSystemExclusiveMessage> {
        Self::IMidiSystemExclusiveMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiSystemExclusiveMessage)(::core::mem::transmute_copy(this), rawdata.into_param().abi(), &mut result__).from_abi::<MidiSystemExclusiveMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiSystemExclusiveMessageFactory<R, F: FnOnce(&IMidiSystemExclusiveMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiSystemExclusiveMessage, IMidiSystemExclusiveMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiSystemExclusiveMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSystemExclusiveMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemExclusiveMessage";
}
impl ::core::convert::From<MidiSystemExclusiveMessage> for ::windows::core::IUnknown {
    fn from(value: MidiSystemExclusiveMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSystemExclusiveMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiSystemExclusiveMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiSystemExclusiveMessage> for ::windows::core::IInspectable {
    fn from(value: MidiSystemExclusiveMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSystemExclusiveMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiSystemExclusiveMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiSystemResetMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
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
unsafe impl ::windows::core::Interface for MidiSystemResetMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiSystemResetMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemResetMessage";
}
impl ::core::convert::From<MidiSystemResetMessage> for ::windows::core::IUnknown {
    fn from(value: MidiSystemResetMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSystemResetMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiSystemResetMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiSystemResetMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiSystemResetMessage> for ::windows::core::IInspectable {
    fn from(value: MidiSystemResetMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiSystemResetMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiSystemResetMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiSystemResetMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiSystemResetMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiSystemResetMessage {}
unsafe impl ::core::marker::Sync for MidiSystemResetMessage {}
#[doc = "*Required features: `\"Devices_Midi\"`*"]
#[repr(transparent)]
pub struct MidiTimeCodeMessage(::windows::core::IUnknown);
impl MidiTimeCodeMessage {
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = &::windows::core::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn FrameType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Values(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Values)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn CreateMidiTimeCodeMessage(frametype: u8, values: u8) -> ::windows::core::Result<MidiTimeCodeMessage> {
        Self::IMidiTimeCodeMessageFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMidiTimeCodeMessage)(::core::mem::transmute_copy(this), frametype, values, &mut result__).from_abi::<MidiTimeCodeMessage>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMidiTimeCodeMessageFactory<R, F: FnOnce(&IMidiTimeCodeMessageFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiTimeCodeMessage, IMidiTimeCodeMessageFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for MidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiTimeCodeMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimeCodeMessage";
}
impl ::core::convert::From<MidiTimeCodeMessage> for ::windows::core::IUnknown {
    fn from(value: MidiTimeCodeMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiTimeCodeMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiTimeCodeMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiTimeCodeMessage> for ::windows::core::IInspectable {
    fn from(value: MidiTimeCodeMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiTimeCodeMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiTimeCodeMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiTimingClockMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
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
unsafe impl ::windows::core::Interface for MidiTimingClockMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiTimingClockMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimingClockMessage";
}
impl ::core::convert::From<MidiTimingClockMessage> for ::windows::core::IUnknown {
    fn from(value: MidiTimingClockMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiTimingClockMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiTimingClockMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiTimingClockMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiTimingClockMessage> for ::windows::core::IInspectable {
    fn from(value: MidiTimingClockMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiTimingClockMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiTimingClockMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiTimingClockMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiTimingClockMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MidiTuneRequestMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Midi\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
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
unsafe impl ::windows::core::Interface for MidiTuneRequestMessage {
    type Vtable = IMidiMessage_Vtbl;
    const IID: ::windows::core::GUID = <IMidiMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MidiTuneRequestMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTuneRequestMessage";
}
impl ::core::convert::From<MidiTuneRequestMessage> for ::windows::core::IUnknown {
    fn from(value: MidiTuneRequestMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiTuneRequestMessage> for ::windows::core::IUnknown {
    fn from(value: &MidiTuneRequestMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MidiTuneRequestMessage> for ::windows::core::IInspectable {
    fn from(value: MidiTuneRequestMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MidiTuneRequestMessage> for ::windows::core::IInspectable {
    fn from(value: &MidiTuneRequestMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMidiMessage> for &MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IMidiMessage> {
        ::core::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MidiTuneRequestMessage {}
unsafe impl ::core::marker::Sync for MidiTuneRequestMessage {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
