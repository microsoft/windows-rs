fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows_bindgen::builder()
        .filters([
            "DeviceInformation::{CreateWatcherDeviceClass,get_Name}",
            "DeviceClass::AudioRender",
            "DeviceWatcher::{Added,EnumerationCompleted,Start}",
        ])
        .write();
}
