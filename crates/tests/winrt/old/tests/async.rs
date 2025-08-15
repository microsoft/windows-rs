#[test]
fn async_get() -> windows::core::Result<()> {
    use windows::Storage::Streams::*;

    let stream = &InMemoryRandomAccessStream::new()?;

    let writer = DataWriter::CreateDataWriter(stream)?;
    writer.WriteByte(1)?;
    writer.WriteByte(2)?;
    writer.WriteByte(3)?;
    writer.StoreAsync()?.join()?;

    stream.Seek(0)?;
    let reader = DataReader::CreateDataReader(stream)?;
    reader.LoadAsync(3)?.join()?;

    let mut bytes: [u8; 3] = [0; 3];
    reader.ReadBytes(&mut bytes)?;

    assert!(bytes[0] == 1);
    assert!(bytes[1] == 2);
    assert!(bytes[2] == 3);

    Ok(())
}
