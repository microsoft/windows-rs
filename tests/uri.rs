import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);

use winrt::*;

#[test]
fn uri() -> Result<()> {
    use windows::foundation::*;

    let uri = Uri::create_uri("http://kennykerr.ca")?;
    assert!(uri.domain()? == "kennykerr.ca");
    assert!(uri.port()? == 80);

    // Calls IStringable::ToString under the hood (via QueryInterface)
    assert!(uri.to_string()? == "http://kennykerr.ca/");

    Ok(())
}
