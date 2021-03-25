use std::thread;
use test_winrt::Windows::Foundation::*;
use test_winrt::Windows::Storage::Streams::*;
use windows::{ErrorCode, Interface};

// Simple test to validate that types with MarshalingType.Agile are marked Send and Sync
// (if this compiles it worked)
#[test]
fn send_sync() -> windows::Result<()> {
    let url = Uri::CreateUri("http://kennykerr.ca")?;

    thread::spawn(move || {
        assert_eq!("http://kennykerr.ca/", url.ToString().unwrap());
    });

    Ok(())
}

#[test]
fn send_async() {
    let stream = InMemoryRandomAccessStream::new().unwrap();

    let writer = DataWriter::CreateDataWriter(&stream).unwrap();
    writer.WriteByte(1).unwrap();
    writer.WriteByte(2).unwrap();
    writer.WriteByte(3).unwrap();
    let store_async = writer.StoreAsync().unwrap();

    let wait = thread::spawn(move || {
        store_async.get().unwrap();

        stream.Seek(0).unwrap();
        let reader = DataReader::CreateDataReader(&stream).unwrap();
        let load_async = reader.LoadAsync(3).unwrap();

        let wait = thread::spawn(move || {
            load_async.get().unwrap();

            let mut bytes: [u8; 3] = [0; 3];
            reader.ReadBytes(&mut bytes).unwrap();

            assert!(bytes[0] == 1);
            assert!(bytes[1] == 2);
            assert!(bytes[2] == 3);
        });

        wait.join().unwrap();
    });

    wait.join().unwrap();
}

#[test]
fn send_async_no_class() {
    let stream = InMemoryRandomAccessStream::new().unwrap();

    let writer = DataWriter::CreateDataWriter(&stream).unwrap();
    writer.WriteByte(1).unwrap();
    writer.WriteByte(2).unwrap();
    writer.WriteByte(3).unwrap();
    let store_async: IAsyncOperation<u32> = writer.StoreAsync().unwrap().cast().unwrap();

    let wait = thread::spawn(move || {
        store_async.get().unwrap();

        stream.Seek(0).unwrap();
        let reader = DataReader::CreateDataReader(&stream).unwrap();
        let load_async: IAsyncOperation<u32> = reader.LoadAsync(3).unwrap().cast().unwrap();

        let wait = thread::spawn(move || {
            load_async.get().unwrap();

            let mut bytes: [u8; 3] = [0; 3];
            reader.ReadBytes(&mut bytes).unwrap();

            assert!(bytes[0] == 1);
            assert!(bytes[1] == 2);
            assert!(bytes[2] == 3);
        });

        wait.join().unwrap();
    });

    wait.join().unwrap();
}

#[test]
fn send_sync_err() {
    let err = Uri::CreateUri("BADURI").unwrap_err();
    let code = err.code();

    let wait = thread::spawn(move || {
        assert_eq!(err.message(), "BADURI is not a valid absolute URI.");
        assert_eq!(code, ErrorCode(0x8007_0057));
    });

    wait.join().unwrap();
}
