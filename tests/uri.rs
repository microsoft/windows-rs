use windows::foundation::*;
use winrt::*;

import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

#[test]
fn uri() -> Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca")?;
    assert!(uri.domain()? == "kennykerr.ca");
    assert!(uri.port()? == 80);
    assert!(uri.to_string()? == "http://kennykerr.ca/");
    Ok(())
}
