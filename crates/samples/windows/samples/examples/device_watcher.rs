fn main() -> windows::core::Result<()> {
    use windows::Devices::Enumeration::*;

    let watcher = DeviceInformation::CreateWatcher()?;

    let _added = watcher.Added(|_, info| {
        if let Some(info) = info.as_ref() {
            println!("{:?}", info.Name().expect("name"));
        }
    })?;

    let _completed = watcher.EnumerationCompleted(|_, _| {
        println!("done!");
    })?;

    watcher.Start()?;
    std::thread::sleep(std::time::Duration::new(10, 0));
    Ok(())
}
