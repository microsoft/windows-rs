winrt::import!(
    dependencies
        os
    modules
        "windows.media.speech_synthesis"
);

#[test]
fn media() -> winrt::Result<()> {
    Ok(())
}
