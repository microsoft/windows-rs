winrt::import!(
    dependencies
        os
    modules
        windows::media::speech_synthesis::SpeechSynthesisStream
);

#[test]
fn media() -> winrt::Result<()> {
    Ok(())
}
