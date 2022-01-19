use test_winrt_async::*;
use windows::core::*;
use Component::Async::*;

async fn action() -> Result<()> {
    Test::CreateAsyncAction(None, false)?.await?;

    assert!("test" == Test::CreateAsyncAction(None, true)?.await.unwrap_err().message());

    Ok(())
}

async fn action_with_progress() -> Result<()> {
    Test::CreateAsyncActionWithProgress(None, false)?.await?;

    assert!("test" == Test::CreateAsyncActionWithProgress(None, true)?.await.unwrap_err().message());

    Ok(())
}

async fn operation() -> Result<()> {
    assert!(123 == Test::CreateAsyncOperation(None, false, 123)?.await?);

    assert!("test" == Test::CreateAsyncOperation(None, true, 0)?.await.unwrap_err().message());

    Ok(())
}

async fn operation_with_progress() -> Result<()> {
    assert!(123 == Test::CreateAsyncOperationWithProgress(None, false, 123)?.await?);

    assert!("test" == Test::CreateAsyncOperationWithProgress(None, true, 0)?.await.unwrap_err().message());

    Ok(())
}

#[test]
fn test_async_await() -> Result<()> {
    futures::executor::block_on(action())?;
    futures::executor::block_on(action_with_progress())?;
    futures::executor::block_on(operation())?;
    futures::executor::block_on(operation_with_progress())?;

    Ok(())
}
