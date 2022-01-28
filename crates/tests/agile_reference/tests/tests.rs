use windows::core::AgileReference;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

#[test]
fn test() {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap();
    let reference = AgileReference::<GlobalSystemMediaTransportControlsSessionManager>::new(manager).unwrap();

    let handle = std::thread::spawn(move || {
        let manager = reference.resolve().unwrap();

        manager.GetSessions().unwrap();
    });
    handle.join().unwrap();
}
