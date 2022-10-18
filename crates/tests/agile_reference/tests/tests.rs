use windows::{core::*, Foundation::*, Media::Control::*};

#[test]
fn agile_send() -> Result<()> {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?;
    let reference = AgileReference::new(&manager)?;

    let handle = std::thread::spawn(move || {
        let manager = reference.resolve()?;

        manager.GetSessions()?;
        Ok(())
    });
    handle.join().unwrap()
}

#[test]
fn agile_debug() -> Result<()> {
    let uri = Uri::CreateUri(h!("http://kennykerr.ca"))?;
    assert!(format!("{:?}", uri).starts_with("Uri(IUnknown(0x"));

    let reference = AgileReference::new(&uri)?;
    assert!(format!("{:?}", reference).starts_with("AgileReference(IUnknown(0x"));
    Ok(())
}
