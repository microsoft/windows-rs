include!(concat!(env!("OUT_DIR"), "/winrt.rs"));

fn main() -> winrt::Result<()> {
    use microsoft::ai::machine_learning::*;
    use windows::foundation::Uri;

    let tensor = TensorBoolean::create()?;
    assert!(tensor.tensor_kind()? == TensorKind::Boolean);
    println!("Tensor: {:?}", tensor);
    let uri = Uri::create_uri("http://kennykerr.ca:80")?;
    println!("Port: {:?}", uri.port()?);

    Ok(())
}
