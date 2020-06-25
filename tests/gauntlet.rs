import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
        nuget: KennyKerr.Windows.TestWinRT
    types
        test_component::*
        windows::foundation::*
);

use test_component::*;
use windows::foundation::*;
use winrt::*;

#[test]
fn test_self() -> Result<()> {
    TestRunner::test_self()?;

    Ok(())
}

#[test]
fn tests() -> Result<()> {
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
        let a: HString = "WinRT".into();
        let mut b = HString::new();
        let c = tests.param12(&a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = Blittable {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: -5,
            f: -6,
            g: -7,
            h: 8.0,
            i: 9.0,
            j: Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
        };

        let mut b = Blittable::default();
        let c = tests.param13(&a, &a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let object = PropertyValue::create_int64(1234)?;
        let pv: IReference<i64> = object.try_into()?;

        let a = NonBlittable {
            a: false,
            b: 0x57, // WinRT char
            c: "WinRT".into(),
            d: pv,
        };

        let mut b = NonBlittable::default();
        let c = tests.param14(&a, &a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let object = PropertyValue::create_int64(1234)?;
        let pv: IReference<i64> = object.try_into()?;

        let a = Nested {
            blittable: Blittable {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: -5,
                f: -6,
                g: -7,
                h: 8.0,
                i: 9.0,
                j: Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
            },
            non_blittable: NonBlittable {
                a: false,
                b: 0x57, // WinRT char
                c: "WinRT".into(),
                d: pv,
            },
        };

        let mut b = Nested::default();
        let c = tests.param15(&a, &a, &mut b)?;
        assert!(a == b && a == c);
    }

    Ok(())
}
