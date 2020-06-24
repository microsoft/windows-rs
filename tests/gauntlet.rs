winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
        nuget: KennyKerr.Windows.TestWinRT
    types
        test_component::*
        windows::foundation::*
);

use test_component::*;

#[test]
fn test_self() -> winrt::Result<()> {
    TestRunner::test_self()?;

    Ok(())
}

#[test]
fn tests() -> winrt::Result<()> {
    let tests = TestRunner::make_tests()?;

    {
        let a = true;
        let mut b = false;
        let c = tests.param1(a, &mut b)?;
        assert!(b && c);
    }

    {
        let a = 123;
        let mut b = 0;
        let c = tests.param2(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = 123;
        let mut b = 0;
        let c = tests.param3(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = 123;
        let mut b = 0;
        let c = tests.param4(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = 123;
        let mut b = 0;
        let c = tests.param5(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = 123;
        let mut b = 0;
        let c = tests.param6(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = 123;
        let mut b = 0;
        let c = tests.param7(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = 123;
        let mut b = 0;
        let c = tests.param8(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = 12.3;
        let mut b = 0.0;
        let c = tests.param9(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = 12.3;
        let mut b = 0.0;
        let c = tests.param10(a, &mut b)?;
        assert!(a == b && a == c);
    }

    Ok(())
}
