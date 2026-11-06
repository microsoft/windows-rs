#![cfg(windows)]

use windows::{Storage::Streams::*, System::Threading::*, core::*};

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
    let receiver = std::sync::Mutex::new(receiver);

    let async_future = ThreadPool::RunAsync(&WorkItemHandler::new(move |_| {
        receiver.lock().unwrap().recv().unwrap();
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

#[test]
fn waker_replacement_survives_future_drop() -> Result<()> {
    use std::{
        future::{Future, IntoFuture},
        sync::{Arc, Mutex, mpsc},
        task::{Context, Wake, Waker},
        time::Duration,
    };

    struct Notify {
        id: u8,
        wakes: mpsc::Sender<u8>,
        drops: mpsc::Sender<u8>,
    }

    impl Wake for Notify {
        fn wake(self: Arc<Self>) {
            self.wakes.send(self.id).unwrap();
        }
    }

    impl Drop for Notify {
        fn drop(&mut self) {
            self.drops.send(self.id).unwrap();
        }
    }

    let (start, wait) = mpsc::channel();
    let wait = Mutex::new(wait);
    let operation = ThreadPool::RunAsync(&WorkItemHandler::new(move |_| {
        wait.lock().unwrap().recv().unwrap();
        Ok(())
    }))?;
    let mut future = Box::pin(operation.clone().into_future());
    let (wakes, woke) = mpsc::channel();
    let (drops, dropped) = mpsc::channel();
    for id in 0..2 {
        let waker = Waker::from(Arc::new(Notify {
            id,
            wakes: wakes.clone(),
            drops: drops.clone(),
        }));
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
    }
    assert_eq!(dropped.recv_timeout(Duration::from_secs(10)).unwrap(), 0);
    drop(future);
    assert!(dropped.try_recv().is_err());
    start.send(()).unwrap();
    assert_eq!(woke.recv_timeout(Duration::from_secs(10)).unwrap(), 1);
    assert_eq!(dropped.recv_timeout(Duration::from_secs(10)).unwrap(), 1);
    assert!(woke.try_recv().is_err());
    operation.GetResults()
}
