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
