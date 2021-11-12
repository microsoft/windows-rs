#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct MidiMessageType(i32);
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
