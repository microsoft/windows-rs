#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IVoiceCommand();
    fn IVoiceCommandCompletedEventArgs();
    fn IVoiceCommandConfirmationResult();
    fn IVoiceCommandContentTile();
    fn IVoiceCommandDefinition();
    fn IVoiceCommandDefinitionManagerStatics();
    fn IVoiceCommandDisambiguationResult();
    fn IVoiceCommandResponse();
    fn IVoiceCommandResponseStatics();
    fn IVoiceCommandServiceConnection();
    fn IVoiceCommandServiceConnectionStatics();
    fn IVoiceCommandUserMessage();
    fn VoiceCommand();
    fn VoiceCommandCompletedEventArgs();
    fn VoiceCommandCompletionReason();
    fn VoiceCommandConfirmationResult();
    fn VoiceCommandContentTile();
    fn VoiceCommandContentTileType();
    fn VoiceCommandDefinition();
    fn VoiceCommandDefinitionManager();
    fn VoiceCommandDisambiguationResult();
    fn VoiceCommandResponse();
    fn VoiceCommandServiceConnection();
    fn VoiceCommandUserMessage();
}
