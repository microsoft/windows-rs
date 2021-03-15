use std::thread;
use test_winrt::windows::foundation::*;
use test_winrt::windows::storage::streams::*;
use windows::{ErrorCode, Interface};

// Simple test to validate that types with MarshalingType.Agile are marked Send and Sync
// (if this compiles it worked)
#[test]
fn send_sync() -> windows::Result<()> {
    let url = Uri::create_uri("http://kennykerr.ca")?;

    thread::spawn(move || {
        assert_eq!("http://kennykerr.ca/", url.to_string().unwrap());
    });

    Ok(())
}

#[test]
fn send_async() {
    let stream = InMemoryRandomAccessStream::new().unwrap();

    let writer = DataWriter::create_data_writer(&stream).unwrap();
    writer.write_byte(1).unwrap();
    writer.write_byte(2).unwrap();
    writer.write_byte(3).unwrap();
    let store_async = writer.store_async().unwrap();

    let wait = thread::spawn(move || {
        store_async.get().unwrap();

        stream.seek(0).unwrap();
        let reader = DataReader::create_data_reader(&stream).unwrap();
        let load_async = reader.load_async(3).unwrap();

        let wait = thread::spawn(move || {
            load_async.get().unwrap();

            let mut bytes: [u8; 3] = [0; 3];
            reader.read_bytes(&mut bytes).unwrap();

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

    let writer = DataWriter::create_data_writer(&stream).unwrap();
    writer.write_byte(1).unwrap();
    writer.write_byte(2).unwrap();
    writer.write_byte(3).unwrap();
    let store_async: IAsyncOperation<u32> = writer.store_async().unwrap().cast().unwrap();

    let wait = thread::spawn(move || {
        store_async.get().unwrap();

        stream.seek(0).unwrap();
        let reader = DataReader::create_data_reader(&stream).unwrap();
        let load_async: IAsyncOperation<u32> = reader.load_async(3).unwrap().cast().unwrap();

        let wait = thread::spawn(move || {
            load_async.get().unwrap();

            let mut bytes: [u8; 3] = [0; 3];
            reader.read_bytes(&mut bytes).unwrap();

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
    let err = Uri::create_uri("BADURI").unwrap_err();
    let code = err.code();

    let wait = thread::spawn(move || {
        assert_eq!(err.message(), "BADURI is not a valid absolute URI.");
        assert_eq!(code, ErrorCode(0x8007_0057));
    });

    wait.join().unwrap();
}
