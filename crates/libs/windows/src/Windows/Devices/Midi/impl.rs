#[cfg(feature = "implement_exclusive")]
pub trait IMidiChannelPressureMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Pressure(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiChannelPressureMessageFactoryImpl: Sized {
    fn CreateMidiChannelPressureMessage(&self, channel: u8, pressure: u8) -> ::windows::core::Result<MidiChannelPressureMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiControlChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Controller(&self) -> ::windows::core::Result<u8>;
    fn ControlValue(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiControlChangeMessageFactoryImpl: Sized {
    fn CreateMidiControlChangeMessage(&self, channel: u8, controller: u8, controlvalue: u8) -> ::windows::core::Result<MidiControlChangeMessage>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiInPortImpl: Sized + IClosableImpl {
    fn MessageReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MidiInPort, MidiMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiInPortStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiInPort>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IMidiMessageImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Type(&self) -> ::windows::core::Result<MidiMessageType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiMessageReceivedEventArgsImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<IMidiMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOffMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Velocity(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOffMessageFactoryImpl: Sized {
    fn CreateMidiNoteOffMessage(&self, channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOffMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOnMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Velocity(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiNoteOnMessageFactoryImpl: Sized {
    fn CreateMidiNoteOnMessage(&self, channel: u8, note: u8, velocity: u8) -> ::windows::core::Result<MidiNoteOnMessage>;
}
#[cfg(feature = "Foundation")]
pub trait IMidiOutPortImpl: Sized + IClosableImpl {
    fn SendMessage(&self, midimessage: &::core::option::Option<IMidiMessage>) -> ::windows::core::Result<()>;
    fn SendBuffer(&self, mididata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiOutPortStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IMidiOutPort>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPitchBendChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Bend(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPitchBendChangeMessageFactoryImpl: Sized {
    fn CreateMidiPitchBendChangeMessage(&self, channel: u8, bend: u16) -> ::windows::core::Result<MidiPitchBendChangeMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPolyphonicKeyPressureMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Note(&self) -> ::windows::core::Result<u8>;
    fn Pressure(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiPolyphonicKeyPressureMessageFactoryImpl: Sized {
    fn CreateMidiPolyphonicKeyPressureMessage(&self, channel: u8, note: u8, pressure: u8) -> ::windows::core::Result<MidiPolyphonicKeyPressureMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiProgramChangeMessageImpl: Sized + IMidiMessageImpl {
    fn Channel(&self) -> ::windows::core::Result<u8>;
    fn Program(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiProgramChangeMessageFactoryImpl: Sized {
    fn CreateMidiProgramChangeMessage(&self, channel: u8, program: u8) -> ::windows::core::Result<MidiProgramChangeMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongPositionPointerMessageImpl: Sized + IMidiMessageImpl {
    fn Beats(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongPositionPointerMessageFactoryImpl: Sized {
    fn CreateMidiSongPositionPointerMessage(&self, beats: u16) -> ::windows::core::Result<MidiSongPositionPointerMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongSelectMessageImpl: Sized + IMidiMessageImpl {
    fn Song(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSongSelectMessageFactoryImpl: Sized {
    fn CreateMidiSongSelectMessage(&self, song: u8) -> ::windows::core::Result<MidiSongSelectMessage>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMidiSynthesizerImpl: Sized + IClosableImpl + IMidiOutPortImpl {
    fn AudioDevice(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSynthesizerStaticsImpl: Sized {
    fn CreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>;
    fn CreateFromAudioDeviceAsync(&self, audiodevice: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MidiSynthesizer>>;
    fn IsSynthesizer(&self, mididevice: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiSystemExclusiveMessageFactoryImpl: Sized {
    fn CreateMidiSystemExclusiveMessage(&self, rawdata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<MidiSystemExclusiveMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiTimeCodeMessageImpl: Sized + IMidiMessageImpl {
    fn FrameType(&self) -> ::windows::core::Result<u8>;
    fn Values(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMidiTimeCodeMessageFactoryImpl: Sized {
    fn CreateMidiTimeCodeMessage(&self, frametype: u8, values: u8) -> ::windows::core::Result<MidiTimeCodeMessage>;
}
