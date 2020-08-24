#[test]
fn async_get() -> winrt::Result<()> {
    use tests::windows::storage::streams::*;

    let stream = &InMemoryRandomAccessStream::new()?;

    let writer = DataWriter::create_data_writer(stream)?;
    writer.write_byte(1)?;
    writer.write_byte(2)?;
    writer.write_byte(3)?;
    writer.store_async()?.get()?;

    stream.seek(0)?;
    let reader = DataReader::create_data_reader(stream)?;
    reader.load_async(3)?.get()?;

    let mut bytes: [u8; 3] = [0; 3];
    reader.read_bytes(&mut bytes)?;

    assert!(bytes[0] == 1);
    assert!(bytes[1] == 2);
    assert!(bytes[2] == 3);

    Ok(())
}

async fn async_await() -> winrt::Result<()> {
    use tests::windows::storage::streams::*;

    let stream = &InMemoryRandomAccessStream::new()?;

    let writer = DataWriter::create_data_writer(stream)?;
    writer.write_byte(1)?;
    writer.write_byte(2)?;
    writer.write_byte(3)?;
    writer.store_async()?.await?;

    stream.seek(0)?;
    let reader = DataReader::create_data_reader(stream)?;
    reader.load_async(3)?.await?;

    let mut bytes: [u8; 3] = [0; 3];
    reader.read_bytes(&mut bytes)?;

    assert!(bytes[0] == 1);
    assert!(bytes[1] == 2);
    assert!(bytes[2] == 3);

    Ok(())
}

#[test]
fn test_async_await() -> winrt::Result<()> {
    futures::executor::block_on(async_await())
}
