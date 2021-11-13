#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMidiChannelPressureMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiChannelPressureMessage {}
impl ::core::clone::Clone for IMidiChannelPressureMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiChannelPressureMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiChannelPressureMessageFactory {}
impl ::core::clone::Clone for IMidiChannelPressureMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiControlChangeMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiControlChangeMessage {}
impl ::core::clone::Clone for IMidiControlChangeMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiControlChangeMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiControlChangeMessageFactory {}
impl ::core::clone::Clone for IMidiControlChangeMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiInPort(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiInPort {}
impl ::core::clone::Clone for IMidiInPort {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiInPortStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiInPortStatics {}
impl ::core::clone::Clone for IMidiInPortStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiMessage {}
impl ::core::clone::Clone for IMidiMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiMessageReceivedEventArgs {}
impl ::core::clone::Clone for IMidiMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiNoteOffMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiNoteOffMessage {}
impl ::core::clone::Clone for IMidiNoteOffMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiNoteOffMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiNoteOffMessageFactory {}
impl ::core::clone::Clone for IMidiNoteOffMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiNoteOnMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiNoteOnMessage {}
impl ::core::clone::Clone for IMidiNoteOnMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiNoteOnMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiNoteOnMessageFactory {}
impl ::core::clone::Clone for IMidiNoteOnMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiOutPort(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiOutPort {}
impl ::core::clone::Clone for IMidiOutPort {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiOutPortStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiOutPortStatics {}
impl ::core::clone::Clone for IMidiOutPortStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiPitchBendChangeMessage {}
impl ::core::clone::Clone for IMidiPitchBendChangeMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiPitchBendChangeMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiPitchBendChangeMessageFactory {}
impl ::core::clone::Clone for IMidiPitchBendChangeMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiPolyphonicKeyPressureMessage {}
impl ::core::clone::Clone for IMidiPolyphonicKeyPressureMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiPolyphonicKeyPressureMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiPolyphonicKeyPressureMessageFactory {}
impl ::core::clone::Clone for IMidiPolyphonicKeyPressureMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiProgramChangeMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiProgramChangeMessage {}
impl ::core::clone::Clone for IMidiProgramChangeMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiProgramChangeMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiProgramChangeMessageFactory {}
impl ::core::clone::Clone for IMidiProgramChangeMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiSongPositionPointerMessage {}
impl ::core::clone::Clone for IMidiSongPositionPointerMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiSongPositionPointerMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiSongPositionPointerMessageFactory {}
impl ::core::clone::Clone for IMidiSongPositionPointerMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiSongSelectMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiSongSelectMessage {}
impl ::core::clone::Clone for IMidiSongSelectMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiSongSelectMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiSongSelectMessageFactory {}
impl ::core::clone::Clone for IMidiSongSelectMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiSynthesizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiSynthesizer {}
impl ::core::clone::Clone for IMidiSynthesizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiSynthesizerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiSynthesizerStatics {}
impl ::core::clone::Clone for IMidiSynthesizerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiSystemExclusiveMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiSystemExclusiveMessageFactory {}
impl ::core::clone::Clone for IMidiSystemExclusiveMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiTimeCodeMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiTimeCodeMessage {}
impl ::core::clone::Clone for IMidiTimeCodeMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMidiTimeCodeMessageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMidiTimeCodeMessageFactory {}
impl ::core::clone::Clone for IMidiTimeCodeMessageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiActiveSensingMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiActiveSensingMessage {}
impl ::core::clone::Clone for MidiActiveSensingMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiChannelPressureMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiChannelPressureMessage {}
impl ::core::clone::Clone for MidiChannelPressureMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiContinueMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiContinueMessage {}
impl ::core::clone::Clone for MidiContinueMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiControlChangeMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiControlChangeMessage {}
impl ::core::clone::Clone for MidiControlChangeMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiInPort(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiInPort {}
impl ::core::clone::Clone for MidiInPort {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiMessageReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiMessageReceivedEventArgs {}
impl ::core::clone::Clone for MidiMessageReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MidiNoteOffMessage {}
impl ::core::clone::Clone for MidiNoteOffMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiNoteOnMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiNoteOnMessage {}
impl ::core::clone::Clone for MidiNoteOnMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiOutPort(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiOutPort {}
impl ::core::clone::Clone for MidiOutPort {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiPitchBendChangeMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiPitchBendChangeMessage {}
impl ::core::clone::Clone for MidiPitchBendChangeMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiPolyphonicKeyPressureMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiPolyphonicKeyPressureMessage {}
impl ::core::clone::Clone for MidiPolyphonicKeyPressureMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiProgramChangeMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiProgramChangeMessage {}
impl ::core::clone::Clone for MidiProgramChangeMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiSongPositionPointerMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiSongPositionPointerMessage {}
impl ::core::clone::Clone for MidiSongPositionPointerMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiSongSelectMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiSongSelectMessage {}
impl ::core::clone::Clone for MidiSongSelectMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiStartMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiStartMessage {}
impl ::core::clone::Clone for MidiStartMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiStopMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiStopMessage {}
impl ::core::clone::Clone for MidiStopMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiSynthesizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiSynthesizer {}
impl ::core::clone::Clone for MidiSynthesizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiSystemExclusiveMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiSystemExclusiveMessage {}
impl ::core::clone::Clone for MidiSystemExclusiveMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiSystemResetMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiSystemResetMessage {}
impl ::core::clone::Clone for MidiSystemResetMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiTimeCodeMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiTimeCodeMessage {}
impl ::core::clone::Clone for MidiTimeCodeMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiTimingClockMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiTimingClockMessage {}
impl ::core::clone::Clone for MidiTimingClockMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MidiTuneRequestMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MidiTuneRequestMessage {}
impl ::core::clone::Clone for MidiTuneRequestMessage {
    fn clone(&self) -> Self {
        *self
    }
}
