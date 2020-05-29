winrt::import!(
    dependencies
        os
    types
        windows::media::speech_synthesis::SpeechSynthesisStream
);

#[test]
fn media() -> winrt::Result<()> {
    Ok(())
}
