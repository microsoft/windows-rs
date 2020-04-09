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

    let _uri = Uri::default();

    //     let uri = &Uri::create_uri("http://kennykerr.ca")?;
    //     assert!(uri.domain()? == "kennykerr.ca");
    //     assert!(uri.port()? == 80);

    //     // Calls QueryInterface followed by IStringable::ToString under the hood
    //     assert!(uri.to_string()? == "http://kennykerr.ca/");

    //     let default: IUriRuntimeClass = uri.into();
    //     assert!(default.domain()? == uri.domain()?);

    //     let stringable: IStringable = uri.into();
    //     assert!(stringable.to_string()? == uri.to_string()?);

    Ok(())
}
