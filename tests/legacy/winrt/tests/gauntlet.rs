use test_winrt::TestComponent::*;
use test_winrt::Windows::Foundation::{IStringable, Uri};

async fn async_await() -> windows::Result<()> {
    let tests = TestRunner::MakeTests()?;

    // Success and failure with no delay

    tests
        .Async1(TestRunner::CreateAsyncAction(0)?, false)?
        .await?;
    assert_eq!(
        tests
            .Async1(TestRunner::CreateAsyncAction(0)?, true)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    tests
        .Async2(TestRunner::CreateAsyncAction(0)?, false, 0)?
        .await?;
    assert_eq!(
        tests
            .Async2(TestRunner::CreateAsyncAction(0)?, true, 0)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    assert_eq!(
        tests
            .Async3(TestRunner::CreateAsyncAction(0)?, false, 123)?
            .await?,
        123
    );
    assert_eq!(
        tests
            .Async3(TestRunner::CreateAsyncAction(0)?, true, 123)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    assert_eq!(
        tests
            .Async4(TestRunner::CreateAsyncAction(0)?, false, 123, 0)?
            .await?,
        123
    );
    assert_eq!(
        tests
            .Async4(TestRunner::CreateAsyncAction(0)?, true, 123, 0)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    // Success and failure with initial delay

    tests
        .Async1(TestRunner::CreateAsyncAction(200)?, false)?
        .await?;
    assert_eq!(
        tests
            .Async1(TestRunner::CreateAsyncAction(200)?, true)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    tests
        .Async2(TestRunner::CreateAsyncAction(200)?, false, 0)?
        .await?;
    assert_eq!(
        tests
            .Async2(TestRunner::CreateAsyncAction(200)?, true, 0)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    assert_eq!(
        tests
            .Async3(TestRunner::CreateAsyncAction(200)?, false, 123)?
            .await?,
        123
    );
    assert_eq!(
        tests
            .Async3(TestRunner::CreateAsyncAction(200)?, true, 123)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    assert_eq!(
        tests
            .Async4(TestRunner::CreateAsyncAction(200)?, false, 123, 0)?
            .await?,
        123
    );
    assert_eq!(
        tests
            .Async4(TestRunner::CreateAsyncAction(200)?, true, 123, 0)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    Ok(())
}

#[test]
fn test_async_await() -> windows::Result<()> {
    futures::executor::block_on(async_await())
}
