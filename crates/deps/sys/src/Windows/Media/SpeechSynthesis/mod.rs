#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IInstalledVoicesStatic();
    fn IInstalledVoicesStatic2();
    fn ISpeechSynthesisStream();
    fn ISpeechSynthesizer();
    fn ISpeechSynthesizer2();
    fn ISpeechSynthesizerOptions();
    fn ISpeechSynthesizerOptions2();
    fn ISpeechSynthesizerOptions3();
    fn IVoiceInformation();
    fn SpeechAppendedSilence();
    fn SpeechPunctuationSilence();
    fn SpeechSynthesisStream();
    fn SpeechSynthesizer();
    fn SpeechSynthesizerOptions();
    fn VoiceGender();
    fn VoiceInformation();
}
