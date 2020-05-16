winrt::import!(
    dependencies
        os
    modules
        "windows.storage.streams"
);

#[test]
fn async_get() -> winrt::Result<()> {
    use windows::storage::streams::*;

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
