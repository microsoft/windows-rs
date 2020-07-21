winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::foundation::Uri
);

fn main() -> winrt::Result<()> {
    use windows::foundation::Uri;

    let uri = Uri::create_uri("http://kennykerr.ca:80")?;
    println!("Port: {:?}", uri.port()?);

    Ok(())
}
