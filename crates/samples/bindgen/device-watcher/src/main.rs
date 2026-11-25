#[expect(nonstandard_style)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() -> windows_core::Result<()> {
    use bindings::*;

    let watcher = DeviceInformation::CreateWatcherDeviceClass(DeviceClass::AudioRender)?;

    let _added = watcher.Added(|_, info| {
        if let Some(info) = info.as_ref() {
            println!("{}", info.Name().unwrap());
        }
    })?;

    let _completed = watcher.EnumerationCompleted(|_, _| println!("done"))?;

    watcher.Start()?;
    std::thread::sleep(std::time::Duration::from_secs(3));
    Ok(())
}
