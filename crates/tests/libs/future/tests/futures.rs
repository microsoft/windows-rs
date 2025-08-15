// Tests that the async interfaces can be used with Rust futures because `IntoFuture` and `Future` implementations
// are provided by the `windows` crate.

use windows::{core::*, Storage::Streams::*, System::Threading::*};

// A simple example of blocking synchronously with the `get` method.
#[test]
fn simple_sync() -> Result<()> {
    ThreadPool::RunAsync(&WorkItemHandler::new(|_| Ok(())))?.join()
}

// A simple example of awaiting with an async function.
#[test]
fn simple_async() -> Result<()> {
    futures::executor::block_on(async {
        ThreadPool::RunAsync(&WorkItemHandler::new(|_| Ok(())))?.await
    })
}

// A representative example of a WinRT async API.
#[test]
fn stream_async() -> Result<()> {
    futures::executor::block_on(async {
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
    })
}

#[test]
#[expect(clippy::async_yields_async)] // don't want to disturb concurrency testing
fn switch_context() -> Result<()> {
    use futures::{executor::LocalPool, future, task::SpawnExt};
    use std::future::IntoFuture;

    let mut pool = LocalPool::new();
    let (sender, receiver) = std::sync::mpsc::channel::<()>();

    let async_future = ThreadPool::RunAsync(&WorkItemHandler::new(move |_| {
        receiver.recv().unwrap();
        Ok(())
    }))?
    .into_future();

    let select = pool
        .spawner()
        .spawn_with_handle(async move {
            // Poll the future on a given context
            match future::select(async_future, future::ready(())).await {
                future::Either::Left(_) => panic!("async_future should be blocked"),
                future::Either::Right(((), future)) => future,
            }
        })
        .unwrap();

    let async_future = pool.run_until(select);

    pool.spawner()
        .spawn(async move {
            // Poll the future on a different context
            let (result, ()) = future::join(async_future, async {
                sender.send(()).unwrap();
            })
            .await;
            result.unwrap();
        })
        .unwrap();

    pool.run();

    Ok(())
}
