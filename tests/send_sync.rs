winrt::import!(
    dependencies
        os
    types
        windows::foundation::Uri
);

use std::thread;
use windows::foundation::Uri;

// Simple test to validate that types with MarshalingType.Agile are marked Send and Sync
// (if this compiles it worked)
#[test]
fn send_sync() -> winrt::Result<()> {
    let url = Uri::create_uri("http://kennykerr.ca")?;

    thread::spawn(move || {
        assert_eq!("http://kennykerr.ca/", &format!("{:?}", url));
    });

    Ok(())
}
