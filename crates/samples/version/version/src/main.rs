use windows_version::*;

fn main() {
    let version = OsVersion::current();

    println!("version:  {version:?}");
    println!("revision: {}", revision());
    println!(
        "product:  {}",
        if is_server() {
            "Windows Server"
        } else {
            "Windows Workstation"
        }
    );

    const WINDOWS_11: OsVersion = OsVersion::new(10, 0, 0, 22_000);
    println!("Windows 11 or newer: {}", version >= WINDOWS_11);
}
