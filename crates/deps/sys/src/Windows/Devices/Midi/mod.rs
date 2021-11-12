#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMidiChannelPressureMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiChannelPressureMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiControlChangeMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiControlChangeMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiInPort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiInPortStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiNoteOffMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiNoteOffMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiNoteOnMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiNoteOnMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiOutPort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiOutPortStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiProgramChangeMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiProgramChangeMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiSongSelectMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiSongSelectMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiSynthesizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiSynthesizerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiSystemExclusiveMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiTimeCodeMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMidiTimeCodeMessageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiActiveSensingMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiChannelPressureMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiContinueMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiControlChangeMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiInPort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct MidiNoteOffMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiNoteOnMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiOutPort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiPitchBendChangeMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiPolyphonicKeyPressureMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiProgramChangeMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiSongPositionPointerMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiSongSelectMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiStartMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiStopMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiSynthesizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiSystemExclusiveMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiSystemResetMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiTimeCodeMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiTimingClockMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MidiTuneRequestMessage(pub *mut ::core::ffi::c_void);
