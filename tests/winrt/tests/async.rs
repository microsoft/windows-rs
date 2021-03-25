#[test]
fn async_get() -> windows::Result<()> {
    use test_winrt::Windows::Storage::Streams::*;

    let stream = &InMemoryRandomAccessStream::new()?;

    let writer = DataWriter::CreateDataWriter(stream)?;
    writer.WriteByte(1)?;
    writer.WriteByte(2)?;
    writer.WriteByte(3)?;
    writer.StoreAsync()?.get()?;

    stream.Seek(0)?;
    let reader = DataReader::CreateDataReader(stream)?;
    reader.LoadAsync(3)?.get()?;

    let mut bytes: [u8; 3] = [0; 3];
    reader.ReadBytes(&mut bytes)?;

    assert!(bytes[0] == 1);
    assert!(bytes[1] == 2);
    assert!(bytes[2] == 3);

    Ok(())
}

async fn async_await() -> windows::Result<()> {
    use test_winrt::Windows::Storage::Streams::*;

    let stream = &InMemoryRandomAccessStream::new()?;

    let writer = DataWriter::CreateDataWriter(stream)?;
    writer.WriteByte(1)?;
    writer.WriteByte(2)?;
    writer.WriteByte(3)?;
    writer.StoreAsync()?.await?;

    stream.Seek(0)?;
    let reader = DataReader::CreateDataReader(stream)?;
    reader.LoadAsync(3)?.await?;

    let mut bytes: [u8; 3] = [0; 3];
    reader.ReadBytes(&mut bytes)?;

    assert!(bytes[0] == 1);
    assert!(bytes[1] == 2);
    assert!(bytes[2] == 3);

    Ok(())
}

#[test]
fn test_async_await() -> windows::Result<()> {
    futures::executor::block_on(async_await())
}
