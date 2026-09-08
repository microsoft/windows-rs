fn main() -> windows::core::Result<()> {
    use windows::Devices::Enumeration::*;

    let watcher = DeviceInformation::CreateWatcherDeviceClass(DeviceClass::AudioRender)?;

    let _added = watcher.Added(|_, info| {
        if let Some(info) = info.as_ref() {
            println!("{:?}", info.Name().unwrap());
        }
    })?;

    let _completed = watcher.EnumerationCompleted(|_, _| println!("done"))?;

    watcher.Start()?;
    std::thread::sleep(std::time::Duration::from_secs(3));
    Ok(())
}
