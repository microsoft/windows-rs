use windows::core::{AgileReference, Result};
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

#[test]
fn test() -> Result<()> {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.get()?;
    let reference = AgileReference::<GlobalSystemMediaTransportControlsSessionManager>::new(&manager)?;

    let handle = std::thread::spawn(move || {
        let manager = reference.resolve()?;

        manager.GetSessions()?;
        Ok(())
    });
    handle.join().unwrap()
}
