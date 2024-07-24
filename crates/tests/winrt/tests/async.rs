use std::future::IntoFuture;

use futures::{executor::LocalPool, future, task::SpawnExt};
use windows::{
    Storage::Streams::*,
    System::Threading::{ThreadPool, WorkItemHandler},
};

#[test]
fn async_get() -> windows::core::Result<()> {
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

async fn async_await() -> windows::core::Result<()> {
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
fn test_async_await() -> windows::core::Result<()> {
    futures::executor::block_on(async_await())
}

#[test]
fn test_async_updates_waker() -> windows::core::Result<()> {
    let mut pool = LocalPool::new();

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let winrt_future = ThreadPool::RunAsync(&WorkItemHandler::new(move |_| {
        rx.recv().unwrap();
        Ok(())
    }))?
    .into_future();

    let task = pool
        .spawner()
        .spawn_with_handle(async move {
            // Poll the future once on a LocalPool task
            match future::select(winrt_future, future::ready(())).await {
                future::Either::Left(_) => panic!("threadpool action can't finish yet"),
                future::Either::Right(((), future)) => future,
            }
        })
        .unwrap();
    let winrt_future = pool.run_until(task);

    pool.spawner()
        .spawn(async move {
            // Now run the future to completion on a *different* LocalPool task.
            // This will hang unless winrt_future properly updates its saved waker to the new task.
            let (result, ()) = future::join(winrt_future, async {
                tx.send(()).unwrap();
            })
            .await;
            result.unwrap();
        })
        .unwrap();
    pool.run();

    Ok(())
}
