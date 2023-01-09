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
impl ::core::default::Default for MidiMessageType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MidiMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MidiMessageType").field(&self.0).finish()
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
