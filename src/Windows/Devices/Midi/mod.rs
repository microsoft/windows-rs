#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3189745760, 25268, 19794, [163, 126, 146, 229, 77, 53, 185, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiChannelPressureMessageFactory {
    type Vtable = IMidiChannelPressureMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1645800751, 8836, 16682, [148, 207, 16, 251, 4, 132, 44, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiChannelPressureMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, pressure: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiControlChangeMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085000579, 30733, 16479, [183, 129, 62, 21, 152, 201, 127, 64]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiControlChangeMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiControlChangeMessageFactory {
    type Vtable = IMidiControlChangeMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(716260129, 38252, 18093, [151, 82, 248, 127, 85, 5, 47, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiControlChangeMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, controller: u8, controlvalue: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiInPort(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiInPort {
    type Vtable = IMidiInPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3586251227, 38682, 20143, [162, 61, 234, 25, 254, 96, 127, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiInPortStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiInPortStatics {
    type Vtable = IMidiInPortStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1153710556, 26623, 19054, [139, 172, 253, 182, 97, 12, 242, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiInPortStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Midi`*"]
pub struct IMidiMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl IMidiMessage {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMidiMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{79767945-1094-4283-9be0-289fc0ee8334}");
}
impl ::std::convert::From<IMidiMessage> for ::windows::runtime::IUnknown {
    fn from(value: IMidiMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMidiMessage> for ::windows::runtime::IUnknown {
    fn from(value: &IMidiMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMidiMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMidiMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMidiMessage> for ::windows::runtime::IInspectable {
    fn from(value: IMidiMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMidiMessage> for ::windows::runtime::IInspectable {
    fn from(value: &IMidiMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMidiMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMidiMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MidiMessageType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiMessageReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1985375830, 62248, 19281, [144, 125, 179, 168, 206, 150, 191, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiMessageReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiNoteOffMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(385714932, 6542, 19855, [166, 84, 211, 5, 162, 147, 84, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiNoteOffMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiNoteOffMessageFactory {
    type Vtable = IMidiNoteOffMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2796699872, 42825, 16991, [138, 244, 164, 217, 121, 204, 21, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOffMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiNoteOnMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3760343797, 24961, 18141, [175, 162, 65, 0, 4, 192, 87, 170]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiNoteOnMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiNoteOnMessageFactory {
    type Vtable = IMidiNoteOnMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2604826784, 22977, 16910, [181, 23, 21, 161, 10, 169, 96, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiNoteOnMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, note: u8, velocity: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Midi`*"]
pub struct IMidiOutPort(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiOutPort {
    type Vtable = IMidiOutPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2468179359, 22434, 19002, [173, 184, 70, 64, 136, 111, 102, 147]);
}
impl IMidiOutPort {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn SendMessage<'a, Param0: ::windows::runtime::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn SendBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMidiOutPort {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{931d6d9f-57a2-4a3a-adb8-4640886f6693}");
}
impl ::std::convert::From<IMidiOutPort> for ::windows::runtime::IUnknown {
    fn from(value: IMidiOutPort) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMidiOutPort> for ::windows::runtime::IUnknown {
    fn from(value: &IMidiOutPort) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMidiOutPort> for ::windows::runtime::IInspectable {
    fn from(value: IMidiOutPort) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMidiOutPort> for ::windows::runtime::IInspectable {
    fn from(value: &IMidiOutPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IMidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IMidiOutPort) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IMidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IMidiOutPort) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &IMidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, midimessage: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mididata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiOutPortStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiOutPortStatics {
    type Vtable = IMidiOutPortStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(106742761, 3976, 17547, [155, 100, 169, 88, 38, 198, 91, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiOutPortStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(702500017, 11935, 20399, [140, 43, 156, 184, 42, 144, 121, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiPitchBendChangeMessageFactory {
    type Vtable = IMidiPitchBendChangeMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4126072661, 53192, 18726, [179, 14, 163, 98, 35, 147, 48, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPitchBendChangeMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, bend: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(527644670, 44264, 18592, [134, 142, 124, 219, 242, 15, 4, 214]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiPolyphonicKeyPressureMessageFactory {
    type Vtable = IMidiPolyphonicKeyPressureMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3918481470, 50355, 19922, [145, 124, 227, 73, 129, 90, 27, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiPolyphonicKeyPressureMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, note: u8, pressure: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2629516408, 31294, 17191, [170, 152, 32, 184, 228, 72, 90, 248]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiProgramChangeMessageFactory {
    type Vtable = IMidiProgramChangeMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3601875847, 21067, 16644, [156, 153, 101, 114, 191, 210, 226, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiProgramChangeMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: u8, program: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1285885014, 60510, 19172, [161, 21, 136, 220, 87, 204, 43, 121]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSongPositionPointerMessageFactory {
    type Vtable = IMidiSongPositionPointerMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2617305494, 61707, 20458, [179, 149, 245, 214, 207, 128, 246, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongPositionPointerMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, beats: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiSongSelectMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1240527487, 28035, 18241, [165, 191, 70, 41, 246, 190, 151, 79]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiSongSelectMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSongSelectMessageFactory {
    type Vtable = IMidiSongSelectMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2223536356, 34632, 16681, [166, 108, 160, 84, 147, 247, 93, 170]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSongSelectMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, song: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiSynthesizer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSynthesizer {
    type Vtable = IMidiSynthesizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4040824158, 56208, 16479, [184, 174, 33, 210, 225, 127, 46, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiSynthesizerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSynthesizerStatics {
    type Vtable = IMidiSynthesizerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1109715624, 26153, 19819, [170, 143, 212, 82, 26, 90, 49, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSynthesizerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audiodevice: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mididevice: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiSystemExclusiveMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiSystemExclusiveMessageFactory {
    type Vtable = IMidiSystemExclusiveMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(138273314, 15220, 17184, [155, 66, 12, 168, 84, 95, 138, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiSystemExclusiveMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rawdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(200738941, 64099, 18972, [141, 235, 192, 232, 119, 150, 166, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMidiTimeCodeMessageFactory {
    type Vtable = IMidiTimeCodeMessageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3945830853, 30492, 16606, [185, 97, 23, 90, 116, 137, 168, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMidiTimeCodeMessageFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frametype: u8, values: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiActiveSensingMessage(::windows::runtime::IInspectable);
impl MidiActiveSensingMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiActiveSensingMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiActiveSensingMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiActiveSensingMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiActiveSensingMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl ::windows::runtime::RuntimeName for MidiActiveSensingMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiActiveSensingMessage";
}
impl ::std::convert::From<MidiActiveSensingMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiActiveSensingMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiActiveSensingMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiActiveSensingMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiActiveSensingMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiActiveSensingMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiActiveSensingMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiActiveSensingMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiActiveSensingMessage> for IMidiMessage {
    fn from(value: MidiActiveSensingMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiActiveSensingMessage> for IMidiMessage {
    fn from(value: &MidiActiveSensingMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiActiveSensingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for MidiActiveSensingMessage {}
unsafe impl ::std::marker::Sync for MidiActiveSensingMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiChannelPressureMessage(::windows::runtime::IInspectable);
impl MidiChannelPressureMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Pressure(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiChannelPressureMessage(channel: u8, pressure: u8) -> ::windows::runtime::Result<MidiChannelPressureMessage> {
        Self::IMidiChannelPressureMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), channel, pressure, &mut result__).from_abi::<MidiChannelPressureMessage>(result__)
        })
    }
    pub fn IMidiChannelPressureMessageFactory<R, F: FnOnce(&IMidiChannelPressureMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiChannelPressureMessage, IMidiChannelPressureMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiChannelPressureMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiChannelPressureMessage;{be1fa860-62b4-4d52-a37e-92e54d35b909})");
}
unsafe impl ::windows::runtime::Interface for MidiChannelPressureMessage {
    type Vtable = IMidiChannelPressureMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3189745760, 25268, 19794, [163, 126, 146, 229, 77, 53, 185, 9]);
}
impl ::windows::runtime::RuntimeName for MidiChannelPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiChannelPressureMessage";
}
impl ::std::convert::From<MidiChannelPressureMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiChannelPressureMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiChannelPressureMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiChannelPressureMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiChannelPressureMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiChannelPressureMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiChannelPressureMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiChannelPressureMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiChannelPressureMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiChannelPressureMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiChannelPressureMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiChannelPressureMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiChannelPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiChannelPressureMessage {}
unsafe impl ::std::marker::Sync for MidiChannelPressureMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiContinueMessage(::windows::runtime::IInspectable);
impl MidiContinueMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiContinueMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiContinueMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiContinueMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiContinueMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl ::windows::runtime::RuntimeName for MidiContinueMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiContinueMessage";
}
impl ::std::convert::From<MidiContinueMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiContinueMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiContinueMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiContinueMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiContinueMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiContinueMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiContinueMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiContinueMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiContinueMessage> for IMidiMessage {
    fn from(value: MidiContinueMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiContinueMessage> for IMidiMessage {
    fn from(value: &MidiContinueMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiContinueMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for MidiContinueMessage {}
unsafe impl ::std::marker::Sync for MidiContinueMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiControlChangeMessage(::windows::runtime::IInspectable);
impl MidiControlChangeMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Controller(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn ControlValue(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiControlChangeMessage(channel: u8, controller: u8, controlvalue: u8) -> ::windows::runtime::Result<MidiControlChangeMessage> {
        Self::IMidiControlChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), channel, controller, controlvalue, &mut result__).from_abi::<MidiControlChangeMessage>(result__)
        })
    }
    pub fn IMidiControlChangeMessageFactory<R, F: FnOnce(&IMidiControlChangeMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiControlChangeMessage, IMidiControlChangeMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiControlChangeMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiControlChangeMessage;{b7e15f83-780d-405f-b781-3e1598c97f40})");
}
unsafe impl ::windows::runtime::Interface for MidiControlChangeMessage {
    type Vtable = IMidiControlChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085000579, 30733, 16479, [183, 129, 62, 21, 152, 201, 127, 64]);
}
impl ::windows::runtime::RuntimeName for MidiControlChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiControlChangeMessage";
}
impl ::std::convert::From<MidiControlChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiControlChangeMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiControlChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiControlChangeMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiControlChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiControlChangeMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiControlChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiControlChangeMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiControlChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiControlChangeMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiControlChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiControlChangeMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiControlChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiControlChangeMessage {}
unsafe impl ::std::marker::Sync for MidiControlChangeMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiInPort(::windows::runtime::IInspectable);
impl MidiInPort {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn MessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MidiInPort>> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiInPort>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMidiInPortStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IMidiInPortStatics<R, F: FnOnce(&IMidiInPortStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiInPort, IMidiInPortStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiInPort {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiInPort;{d5c1d9db-971a-4eaf-a23d-ea19fe607ff9})");
}
unsafe impl ::windows::runtime::Interface for MidiInPort {
    type Vtable = IMidiInPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3586251227, 38682, 20143, [162, 61, 234, 25, 254, 96, 127, 249]);
}
impl ::windows::runtime::RuntimeName for MidiInPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiInPort";
}
impl ::std::convert::From<MidiInPort> for ::windows::runtime::IUnknown {
    fn from(value: MidiInPort) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiInPort> for ::windows::runtime::IUnknown {
    fn from(value: &MidiInPort) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiInPort> for ::windows::runtime::IInspectable {
    fn from(value: MidiInPort) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiInPort> for ::windows::runtime::IInspectable {
    fn from(value: &MidiInPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MidiInPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiInPort) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MidiInPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiInPort) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MidiInPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiInPort {}
unsafe impl ::std::marker::Sync for MidiInPort {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiMessageReceivedEventArgs(::windows::runtime::IInspectable);
impl MidiMessageReceivedEventArgs {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<IMidiMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IMidiMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiMessageReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiMessageReceivedEventArgs;{76566e56-f328-4b51-907d-b3a8ce96bf80})");
}
unsafe impl ::windows::runtime::Interface for MidiMessageReceivedEventArgs {
    type Vtable = IMidiMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1985375830, 62248, 19281, [144, 125, 179, 168, 206, 150, 191, 128]);
}
impl ::windows::runtime::RuntimeName for MidiMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Midi.MidiMessageReceivedEventArgs";
}
impl ::std::convert::From<MidiMessageReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MidiMessageReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiMessageReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MidiMessageReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiMessageReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MidiMessageReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiMessageReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MidiMessageReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MidiMessageReceivedEventArgs {}
unsafe impl ::std::marker::Sync for MidiMessageReceivedEventArgs {}
#[doc = "*Required features: `Devices_Midi`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MidiMessageType(pub i32);
impl MidiMessageType {
    pub const None: MidiMessageType = MidiMessageType(0i32);
    pub const NoteOff: MidiMessageType = MidiMessageType(128i32);
    pub const NoteOn: MidiMessageType = MidiMessageType(144i32);
    pub const PolyphonicKeyPressure: MidiMessageType = MidiMessageType(160i32);
    pub const ControlChange: MidiMessageType = MidiMessageType(176i32);
    pub const ProgramChange: MidiMessageType = MidiMessageType(192i32);
    pub const ChannelPressure: MidiMessageType = MidiMessageType(208i32);
    pub const PitchBendChange: MidiMessageType = MidiMessageType(224i32);
    pub const SystemExclusive: MidiMessageType = MidiMessageType(240i32);
    pub const MidiTimeCode: MidiMessageType = MidiMessageType(241i32);
    pub const SongPositionPointer: MidiMessageType = MidiMessageType(242i32);
    pub const SongSelect: MidiMessageType = MidiMessageType(243i32);
    pub const TuneRequest: MidiMessageType = MidiMessageType(246i32);
    pub const EndSystemExclusive: MidiMessageType = MidiMessageType(247i32);
    pub const TimingClock: MidiMessageType = MidiMessageType(248i32);
    pub const Start: MidiMessageType = MidiMessageType(250i32);
    pub const Continue: MidiMessageType = MidiMessageType(251i32);
    pub const Stop: MidiMessageType = MidiMessageType(252i32);
    pub const ActiveSensing: MidiMessageType = MidiMessageType(254i32);
    pub const SystemReset: MidiMessageType = MidiMessageType(255i32);
}
impl ::std::convert::From<i32> for MidiMessageType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MidiMessageType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MidiMessageType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Midi.MidiMessageType;i4)");
}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiNoteOffMessage(::windows::runtime::IInspectable);
impl MidiNoteOffMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Note(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Velocity(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiNoteOffMessage(channel: u8, note: u8, velocity: u8) -> ::windows::runtime::Result<MidiNoteOffMessage> {
        Self::IMidiNoteOffMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), channel, note, velocity, &mut result__).from_abi::<MidiNoteOffMessage>(result__)
        })
    }
    pub fn IMidiNoteOffMessageFactory<R, F: FnOnce(&IMidiNoteOffMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiNoteOffMessage, IMidiNoteOffMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiNoteOffMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiNoteOffMessage;{16fd8af4-198e-4d8f-a654-d305a293548f})");
}
unsafe impl ::windows::runtime::Interface for MidiNoteOffMessage {
    type Vtable = IMidiNoteOffMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(385714932, 6542, 19855, [166, 84, 211, 5, 162, 147, 84, 143]);
}
impl ::windows::runtime::RuntimeName for MidiNoteOffMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOffMessage";
}
impl ::std::convert::From<MidiNoteOffMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiNoteOffMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiNoteOffMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiNoteOffMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiNoteOffMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiNoteOffMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiNoteOffMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiNoteOffMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiNoteOffMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiNoteOffMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiNoteOffMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiNoteOffMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiNoteOffMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiNoteOffMessage {}
unsafe impl ::std::marker::Sync for MidiNoteOffMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiNoteOnMessage(::windows::runtime::IInspectable);
impl MidiNoteOnMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Note(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Velocity(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiNoteOnMessage(channel: u8, note: u8, velocity: u8) -> ::windows::runtime::Result<MidiNoteOnMessage> {
        Self::IMidiNoteOnMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), channel, note, velocity, &mut result__).from_abi::<MidiNoteOnMessage>(result__)
        })
    }
    pub fn IMidiNoteOnMessageFactory<R, F: FnOnce(&IMidiNoteOnMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiNoteOnMessage, IMidiNoteOnMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiNoteOnMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiNoteOnMessage;{e0224af5-6181-46dd-afa2-410004c057aa})");
}
unsafe impl ::windows::runtime::Interface for MidiNoteOnMessage {
    type Vtable = IMidiNoteOnMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3760343797, 24961, 18141, [175, 162, 65, 0, 4, 192, 87, 170]);
}
impl ::windows::runtime::RuntimeName for MidiNoteOnMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiNoteOnMessage";
}
impl ::std::convert::From<MidiNoteOnMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiNoteOnMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiNoteOnMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiNoteOnMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiNoteOnMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiNoteOnMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiNoteOnMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiNoteOnMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiNoteOnMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiNoteOnMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiNoteOnMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiNoteOnMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiNoteOnMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiNoteOnMessage {}
unsafe impl ::std::marker::Sync for MidiNoteOnMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiOutPort(::windows::runtime::IInspectable);
impl MidiOutPort {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn SendMessage<'a, Param0: ::windows::runtime::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn SendBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IMidiOutPort>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMidiOutPortStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IMidiOutPortStatics<R, F: FnOnce(&IMidiOutPortStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiOutPort, IMidiOutPortStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiOutPort {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiOutPort;{931d6d9f-57a2-4a3a-adb8-4640886f6693})");
}
unsafe impl ::windows::runtime::Interface for MidiOutPort {
    type Vtable = IMidiOutPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2468179359, 22434, 19002, [173, 184, 70, 64, 136, 111, 102, 147]);
}
impl ::windows::runtime::RuntimeName for MidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.MidiOutPort";
}
impl ::std::convert::From<MidiOutPort> for ::windows::runtime::IUnknown {
    fn from(value: MidiOutPort) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiOutPort> for ::windows::runtime::IUnknown {
    fn from(value: &MidiOutPort) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiOutPort> for ::windows::runtime::IInspectable {
    fn from(value: MidiOutPort) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiOutPort> for ::windows::runtime::IInspectable {
    fn from(value: &MidiOutPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiOutPort> for IMidiOutPort {
    fn from(value: MidiOutPort) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiOutPort> for IMidiOutPort {
    fn from(value: &MidiOutPort) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiOutPort> for MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiOutPort> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiOutPort>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiOutPort> for &MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiOutPort> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiOutPort>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiOutPort) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MidiOutPort> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiOutPort) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MidiOutPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiOutPort {}
unsafe impl ::std::marker::Sync for MidiOutPort {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiPitchBendChangeMessage(::windows::runtime::IInspectable);
impl MidiPitchBendChangeMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Bend(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiPitchBendChangeMessage(channel: u8, bend: u16) -> ::windows::runtime::Result<MidiPitchBendChangeMessage> {
        Self::IMidiPitchBendChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), channel, bend, &mut result__).from_abi::<MidiPitchBendChangeMessage>(result__)
        })
    }
    pub fn IMidiPitchBendChangeMessageFactory<R, F: FnOnce(&IMidiPitchBendChangeMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiPitchBendChangeMessage, IMidiPitchBendChangeMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiPitchBendChangeMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiPitchBendChangeMessage;{29df4cb1-2e9f-4faf-8c2b-9cb82a9079ca})");
}
unsafe impl ::windows::runtime::Interface for MidiPitchBendChangeMessage {
    type Vtable = IMidiPitchBendChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(702500017, 11935, 20399, [140, 43, 156, 184, 42, 144, 121, 202]);
}
impl ::windows::runtime::RuntimeName for MidiPitchBendChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPitchBendChangeMessage";
}
impl ::std::convert::From<MidiPitchBendChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiPitchBendChangeMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiPitchBendChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiPitchBendChangeMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiPitchBendChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiPitchBendChangeMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiPitchBendChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiPitchBendChangeMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiPitchBendChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiPitchBendChangeMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiPitchBendChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiPitchBendChangeMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiPitchBendChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiPitchBendChangeMessage {}
unsafe impl ::std::marker::Sync for MidiPitchBendChangeMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiPolyphonicKeyPressureMessage(::windows::runtime::IInspectable);
impl MidiPolyphonicKeyPressureMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Note(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Pressure(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiPolyphonicKeyPressureMessage(channel: u8, note: u8, pressure: u8) -> ::windows::runtime::Result<MidiPolyphonicKeyPressureMessage> {
        Self::IMidiPolyphonicKeyPressureMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), channel, note, pressure, &mut result__).from_abi::<MidiPolyphonicKeyPressureMessage>(result__)
        })
    }
    pub fn IMidiPolyphonicKeyPressureMessageFactory<R, F: FnOnce(&IMidiPolyphonicKeyPressureMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiPolyphonicKeyPressureMessage, IMidiPolyphonicKeyPressureMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiPolyphonicKeyPressureMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage;{1f7337fe-ace8-48a0-868e-7cdbf20f04d6})");
}
unsafe impl ::windows::runtime::Interface for MidiPolyphonicKeyPressureMessage {
    type Vtable = IMidiPolyphonicKeyPressureMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(527644670, 44264, 18592, [134, 142, 124, 219, 242, 15, 4, 214]);
}
impl ::windows::runtime::RuntimeName for MidiPolyphonicKeyPressureMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiPolyphonicKeyPressureMessage";
}
impl ::std::convert::From<MidiPolyphonicKeyPressureMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiPolyphonicKeyPressureMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiPolyphonicKeyPressureMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiPolyphonicKeyPressureMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiPolyphonicKeyPressureMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiPolyphonicKeyPressureMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiPolyphonicKeyPressureMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiPolyphonicKeyPressureMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiPolyphonicKeyPressureMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiPolyphonicKeyPressureMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiPolyphonicKeyPressureMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiPolyphonicKeyPressureMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiPolyphonicKeyPressureMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiPolyphonicKeyPressureMessage {}
unsafe impl ::std::marker::Sync for MidiPolyphonicKeyPressureMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiProgramChangeMessage(::windows::runtime::IInspectable);
impl MidiProgramChangeMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Program(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiProgramChangeMessage(channel: u8, program: u8) -> ::windows::runtime::Result<MidiProgramChangeMessage> {
        Self::IMidiProgramChangeMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), channel, program, &mut result__).from_abi::<MidiProgramChangeMessage>(result__)
        })
    }
    pub fn IMidiProgramChangeMessageFactory<R, F: FnOnce(&IMidiProgramChangeMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiProgramChangeMessage, IMidiProgramChangeMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiProgramChangeMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiProgramChangeMessage;{9cbb3c78-7a3e-4327-aa98-20b8e4485af8})");
}
unsafe impl ::windows::runtime::Interface for MidiProgramChangeMessage {
    type Vtable = IMidiProgramChangeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2629516408, 31294, 17191, [170, 152, 32, 184, 228, 72, 90, 248]);
}
impl ::windows::runtime::RuntimeName for MidiProgramChangeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiProgramChangeMessage";
}
impl ::std::convert::From<MidiProgramChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiProgramChangeMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiProgramChangeMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiProgramChangeMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiProgramChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiProgramChangeMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiProgramChangeMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiProgramChangeMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiProgramChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiProgramChangeMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiProgramChangeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiProgramChangeMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiProgramChangeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiProgramChangeMessage {}
unsafe impl ::std::marker::Sync for MidiProgramChangeMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiSongPositionPointerMessage(::windows::runtime::IInspectable);
impl MidiSongPositionPointerMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Beats(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiSongPositionPointerMessage(beats: u16) -> ::windows::runtime::Result<MidiSongPositionPointerMessage> {
        Self::IMidiSongPositionPointerMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), beats, &mut result__).from_abi::<MidiSongPositionPointerMessage>(result__)
        })
    }
    pub fn IMidiSongPositionPointerMessageFactory<R, F: FnOnce(&IMidiSongPositionPointerMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSongPositionPointerMessage, IMidiSongPositionPointerMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSongPositionPointerMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSongPositionPointerMessage;{4ca50c56-ec5e-4ae4-a115-88dc57cc2b79})");
}
unsafe impl ::windows::runtime::Interface for MidiSongPositionPointerMessage {
    type Vtable = IMidiSongPositionPointerMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1285885014, 60510, 19172, [161, 21, 136, 220, 87, 204, 43, 121]);
}
impl ::windows::runtime::RuntimeName for MidiSongPositionPointerMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongPositionPointerMessage";
}
impl ::std::convert::From<MidiSongPositionPointerMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiSongPositionPointerMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiSongPositionPointerMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSongPositionPointerMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiSongPositionPointerMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiSongPositionPointerMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiSongPositionPointerMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSongPositionPointerMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiSongPositionPointerMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiSongPositionPointerMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiSongPositionPointerMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiSongPositionPointerMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiSongPositionPointerMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiSongPositionPointerMessage {}
unsafe impl ::std::marker::Sync for MidiSongPositionPointerMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiSongSelectMessage(::windows::runtime::IInspectable);
impl MidiSongSelectMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Song(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiSongSelectMessage(song: u8) -> ::windows::runtime::Result<MidiSongSelectMessage> {
        Self::IMidiSongSelectMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), song, &mut result__).from_abi::<MidiSongSelectMessage>(result__)
        })
    }
    pub fn IMidiSongSelectMessageFactory<R, F: FnOnce(&IMidiSongSelectMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSongSelectMessage, IMidiSongSelectMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSongSelectMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSongSelectMessage;{49f0f27f-6d83-4741-a5bf-4629f6be974f})");
}
unsafe impl ::windows::runtime::Interface for MidiSongSelectMessage {
    type Vtable = IMidiSongSelectMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1240527487, 28035, 18241, [165, 191, 70, 41, 246, 190, 151, 79]);
}
impl ::windows::runtime::RuntimeName for MidiSongSelectMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSongSelectMessage";
}
impl ::std::convert::From<MidiSongSelectMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiSongSelectMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiSongSelectMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSongSelectMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiSongSelectMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiSongSelectMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiSongSelectMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSongSelectMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiSongSelectMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiSongSelectMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiSongSelectMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiSongSelectMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiSongSelectMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiSongSelectMessage {}
unsafe impl ::std::marker::Sync for MidiSongSelectMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiStartMessage(::windows::runtime::IInspectable);
impl MidiStartMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiStartMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiStartMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiStartMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiStartMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl ::windows::runtime::RuntimeName for MidiStartMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStartMessage";
}
impl ::std::convert::From<MidiStartMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiStartMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiStartMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiStartMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiStartMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiStartMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiStartMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiStartMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiStartMessage> for IMidiMessage {
    fn from(value: MidiStartMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiStartMessage> for IMidiMessage {
    fn from(value: &MidiStartMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiStartMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for MidiStartMessage {}
unsafe impl ::std::marker::Sync for MidiStartMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiStopMessage(::windows::runtime::IInspectable);
impl MidiStopMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiStopMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiStopMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiStopMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiStopMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl ::windows::runtime::RuntimeName for MidiStopMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiStopMessage";
}
impl ::std::convert::From<MidiStopMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiStopMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiStopMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiStopMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiStopMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiStopMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiStopMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiStopMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiStopMessage> for IMidiMessage {
    fn from(value: MidiStopMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiStopMessage> for IMidiMessage {
    fn from(value: &MidiStopMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiStopMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for MidiStopMessage {}
unsafe impl ::std::marker::Sync for MidiStopMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiSynthesizer(::windows::runtime::IInspectable);
impl MidiSynthesizer {
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Midi`, `Devices_Enumeration`*"]
    pub fn AudioDevice(&self) -> ::windows::runtime::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Volume(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn SetVolume(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn SendMessage<'a, Param0: ::windows::runtime::IntoParam<'a, IMidiMessage>>(&self, midimessage: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), midimessage.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn SendBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, mididata: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMidiOutPort>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mididata.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMidiOutPort>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn CreateAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Midi`, `Devices_Enumeration`, `Foundation`*"]
    pub fn CreateFromAudioDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Enumeration::DeviceInformation>>(audiodevice: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), audiodevice.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Midi`, `Devices_Enumeration`*"]
    pub fn IsSynthesizer<'a, Param0: ::windows::runtime::IntoParam<'a, super::Enumeration::DeviceInformation>>(mididevice: Param0) -> ::windows::runtime::Result<bool> {
        Self::IMidiSynthesizerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), mididevice.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IMidiSynthesizerStatics<R, F: FnOnce(&IMidiSynthesizerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSynthesizer, IMidiSynthesizerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSynthesizer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSynthesizer;{f0da155e-db90-405f-b8ae-21d2e17f2e45})");
}
unsafe impl ::windows::runtime::Interface for MidiSynthesizer {
    type Vtable = IMidiSynthesizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4040824158, 56208, 16479, [184, 174, 33, 210, 225, 127, 46, 69]);
}
impl ::windows::runtime::RuntimeName for MidiSynthesizer {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSynthesizer";
}
impl ::std::convert::From<MidiSynthesizer> for ::windows::runtime::IUnknown {
    fn from(value: MidiSynthesizer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiSynthesizer> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSynthesizer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiSynthesizer> for ::windows::runtime::IInspectable {
    fn from(value: MidiSynthesizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiSynthesizer> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSynthesizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiSynthesizer> for IMidiOutPort {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiSynthesizer) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiSynthesizer> for IMidiOutPort {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiSynthesizer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiOutPort> for MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiOutPort> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiOutPort> for &MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiOutPort> {
        ::std::convert::TryInto::<IMidiOutPort>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MidiSynthesizer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiSynthesizer) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MidiSynthesizer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiSynthesizer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MidiSynthesizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiSynthesizer {}
unsafe impl ::std::marker::Sync for MidiSynthesizer {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiSystemExclusiveMessage(::windows::runtime::IInspectable);
impl MidiSystemExclusiveMessage {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn CreateMidiSystemExclusiveMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(rawdata: Param0) -> ::windows::runtime::Result<MidiSystemExclusiveMessage> {
        Self::IMidiSystemExclusiveMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), rawdata.into_param().abi(), &mut result__).from_abi::<MidiSystemExclusiveMessage>(result__)
        })
    }
    pub fn IMidiSystemExclusiveMessageFactory<R, F: FnOnce(&IMidiSystemExclusiveMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSystemExclusiveMessage, IMidiSystemExclusiveMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSystemExclusiveMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSystemExclusiveMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiSystemExclusiveMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl ::windows::runtime::RuntimeName for MidiSystemExclusiveMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemExclusiveMessage";
}
impl ::std::convert::From<MidiSystemExclusiveMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiSystemExclusiveMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiSystemExclusiveMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSystemExclusiveMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiSystemExclusiveMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiSystemExclusiveMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiSystemExclusiveMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSystemExclusiveMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiSystemExclusiveMessage> for IMidiMessage {
    fn from(value: MidiSystemExclusiveMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiSystemExclusiveMessage> for IMidiMessage {
    fn from(value: &MidiSystemExclusiveMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiSystemExclusiveMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for MidiSystemExclusiveMessage {}
unsafe impl ::std::marker::Sync for MidiSystemExclusiveMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiSystemResetMessage(::windows::runtime::IInspectable);
impl MidiSystemResetMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiSystemResetMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiSystemResetMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiSystemResetMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiSystemResetMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl ::windows::runtime::RuntimeName for MidiSystemResetMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiSystemResetMessage";
}
impl ::std::convert::From<MidiSystemResetMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiSystemResetMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiSystemResetMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiSystemResetMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiSystemResetMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiSystemResetMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiSystemResetMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiSystemResetMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiSystemResetMessage> for IMidiMessage {
    fn from(value: MidiSystemResetMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiSystemResetMessage> for IMidiMessage {
    fn from(value: &MidiSystemResetMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiSystemResetMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for MidiSystemResetMessage {}
unsafe impl ::std::marker::Sync for MidiSystemResetMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiTimeCodeMessage(::windows::runtime::IInspectable);
impl MidiTimeCodeMessage {
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn FrameType(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Values(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = &::windows::runtime::Interface::cast::<IMidiMessage>(self)?;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn CreateMidiTimeCodeMessage(frametype: u8, values: u8) -> ::windows::runtime::Result<MidiTimeCodeMessage> {
        Self::IMidiTimeCodeMessageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), frametype, values, &mut result__).from_abi::<MidiTimeCodeMessage>(result__)
        })
    }
    pub fn IMidiTimeCodeMessageFactory<R, F: FnOnce(&IMidiTimeCodeMessageFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiTimeCodeMessage, IMidiTimeCodeMessageFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiTimeCodeMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTimeCodeMessage;{0bf7087d-fa63-4a1c-8deb-c0e87796a6d7})");
}
unsafe impl ::windows::runtime::Interface for MidiTimeCodeMessage {
    type Vtable = IMidiTimeCodeMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(200738941, 64099, 18972, [141, 235, 192, 232, 119, 150, 166, 215]);
}
impl ::windows::runtime::RuntimeName for MidiTimeCodeMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimeCodeMessage";
}
impl ::std::convert::From<MidiTimeCodeMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiTimeCodeMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiTimeCodeMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiTimeCodeMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiTimeCodeMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiTimeCodeMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiTimeCodeMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiTimeCodeMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<MidiTimeCodeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MidiTimeCodeMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MidiTimeCodeMessage> for IMidiMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MidiTimeCodeMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiTimeCodeMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::std::convert::TryInto::<IMidiMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MidiTimeCodeMessage {}
unsafe impl ::std::marker::Sync for MidiTimeCodeMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiTimingClockMessage(::windows::runtime::IInspectable);
impl MidiTimingClockMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiTimingClockMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiTimingClockMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTimingClockMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiTimingClockMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl ::windows::runtime::RuntimeName for MidiTimingClockMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTimingClockMessage";
}
impl ::std::convert::From<MidiTimingClockMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiTimingClockMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiTimingClockMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiTimingClockMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiTimingClockMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiTimingClockMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiTimingClockMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiTimingClockMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiTimingClockMessage> for IMidiMessage {
    fn from(value: MidiTimingClockMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiTimingClockMessage> for IMidiMessage {
    fn from(value: &MidiTimingClockMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiTimingClockMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for MidiTimingClockMessage {}
unsafe impl ::std::marker::Sync for MidiTimingClockMessage {}
#[doc = "*Required features: `Devices_Midi`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MidiTuneRequestMessage(::windows::runtime::IInspectable);
impl MidiTuneRequestMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MidiTuneRequestMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Midi`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Midi`, `Storage_Streams`*"]
    pub fn RawData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Midi`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<MidiMessageType> {
        let this = self;
        unsafe {
            let mut result__: MidiMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MidiMessageType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MidiTuneRequestMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Midi.MidiTuneRequestMessage;{79767945-1094-4283-9be0-289fc0ee8334})");
}
unsafe impl ::windows::runtime::Interface for MidiTuneRequestMessage {
    type Vtable = IMidiMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037807429, 4244, 17027, [155, 224, 40, 159, 192, 238, 131, 52]);
}
impl ::windows::runtime::RuntimeName for MidiTuneRequestMessage {
    const NAME: &'static str = "Windows.Devices.Midi.MidiTuneRequestMessage";
}
impl ::std::convert::From<MidiTuneRequestMessage> for ::windows::runtime::IUnknown {
    fn from(value: MidiTuneRequestMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiTuneRequestMessage> for ::windows::runtime::IUnknown {
    fn from(value: &MidiTuneRequestMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MidiTuneRequestMessage> for ::windows::runtime::IInspectable {
    fn from(value: MidiTuneRequestMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MidiTuneRequestMessage> for ::windows::runtime::IInspectable {
    fn from(value: &MidiTuneRequestMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<MidiTuneRequestMessage> for IMidiMessage {
    fn from(value: MidiTuneRequestMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MidiTuneRequestMessage> for IMidiMessage {
    fn from(value: &MidiTuneRequestMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMidiMessage> for &MidiTuneRequestMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMidiMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMidiMessage>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for MidiTuneRequestMessage {}
unsafe impl ::std::marker::Sync for MidiTuneRequestMessage {}
