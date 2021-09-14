use test_winrt::TestComponent::*;
use test_winrt::Windows::Foundation::{IStringable, Uri};

#[test]
fn collections() -> windows::Result<()> {
    {
        let v = TestRunner::CreateInt32Vector()?;
        assert_eq!(v.Size()?, 0);
        v.Append(1)?;
        assert_eq!(v.Size()?, 1);
        assert_eq!(v.GetAt(0)?, 1);
        v.ReplaceAll(&[1, 2, 3])?;
        assert_eq!(v.Size()?, 3);
        assert_eq!(v.GetAt(0)?, 1);
        assert_eq!(v.GetAt(1)?, 2);
        assert_eq!(v.GetAt(2)?, 3);

        v.SetAt(0, 10)?;
        assert_eq!(v.GetAt(0)?, 10);
    }

    {
        let v = TestRunner::CreateStringVector()?;
        assert_eq!(v.Size()?, 0);
        v.Append("one")?;
        assert_eq!(v.Size()?, 1);
        assert_eq!(v.GetAt(0)?, "one");
        v.ReplaceAll(&["one".into(), "two".into(), "three".into()])?;
        assert_eq!(v.Size()?, 3);
        assert_eq!(v.GetAt(0)?, "one");
        assert_eq!(v.GetAt(1)?, "two");
        assert_eq!(v.GetAt(2)?, "three");

        v.SetAt(0, "ONE")?;
        assert_eq!(v.GetAt(0)?, "ONE");
    }

    {
        let one: IStringable = Uri::CreateUri("http://kennykerr.ca/one")?.into();
        let two: IStringable = Uri::CreateUri("http://kennykerr.ca/two")?.into();
        let three: IStringable = Uri::CreateUri("http://kennykerr.ca/three")?.into();

        let v = TestRunner::CreateStringableVector()?;
        assert_eq!(v.Size()?, 0);
        v.Append(&one)?;
        assert_eq!(v.Size()?, 1);
        assert_eq!(v.GetAt(0)?.ToString()?, "http://kennykerr.ca/one");
        v.ReplaceAll(&[Some(one), Some(two), Some(three)])?;
        assert_eq!(v.Size()?, 3);
        assert_eq!(v.GetAt(0)?.ToString()?, "http://kennykerr.ca/one");
        assert_eq!(v.GetAt(1)?.ToString()?, "http://kennykerr.ca/two");
        assert_eq!(v.GetAt(2)?.ToString()?, "http://kennykerr.ca/three");

        v.SetAt(0, Uri::CreateUri("http://kennykerr.ca/ONE")?)?;
        assert_eq!(v.GetAt(0)?.ToString()?, "http://kennykerr.ca/ONE");
    }

    Ok(())
}

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
