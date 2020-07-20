include!(concat!(env!("OUT_DIR"), "/winrt.rs"));

fn main() -> winrt::Result<()> {
    use windows::foundation::Uri;

    let uri = Uri::create_uri("http://kennykerr.ca:80")?;
    println!("Port: {:?}", uri.port()?);

    Ok(())
}
