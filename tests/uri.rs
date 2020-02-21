import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

use windows::foundation::*;
use winrt::*;

#[test]
fn uri() -> Result<()> {
    let uri = Uri::create_uri("http://kennykerr.ca")?;
    assert!(uri.domain()? == "kennykerr.ca");
    assert!(uri.port()? == 80);
    assert!(uri.to_string()? == "http://kennykerr.ca/");
    Ok(())
}
