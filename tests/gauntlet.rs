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
        let a: bool = true;
        let mut b = false;
        let c = tests.param1(a, &mut b)?;
        assert!(b && c);
    }

    {
        let a: u8 = 123;
        let mut b = 0;
        let c = tests.param2(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: u16 = 123;
        let mut b = 0;
        let c = tests.param3(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: u32 = 123;
        let mut b = 0;
        let c = tests.param4(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: u64 = 123;
        let mut b = 0;
        let c = tests.param5(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: i16 = 123;
        let mut b = 0;
        let c = tests.param6(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: i32 = 123;
        let mut b = 0;
        let c = tests.param7(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: i64 = 123;
        let mut b = 0;
        let c = tests.param8(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: f32 = 12.3;
        let mut b = 0.0;
        let c = tests.param9(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: f64 = 12.3;
        let mut b = 0.0;
        let c = tests.param10(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: u16 = 0x57; // WinRT char e.g. L'W'
        let mut b = 0;
        let c = tests.param11(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: winrt::HString = "WinRT".into();
        let mut b = winrt::HString::new();
        let c = tests.param12(&a, &mut b)?;
        assert!(a == b && a == c);
    }

    Ok(())
}
