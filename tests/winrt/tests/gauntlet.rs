use test_winrt::TestComponent::*;
use test_winrt::Windows::Foundation::{IReference, IStringable, PropertyValue, Uri};
use windows::Interface;

#[test]
fn test_self() -> windows::Result<()> {
    TestRunner::TestSelf()?;

    Ok(())
}

#[test]
fn params() -> windows::Result<()> {
    let tests = TestRunner::MakeTests()?;

    {
        let a: bool = true;
        let mut b = false;
        let c = tests.Param1(a, &mut b)?;
        assert!(b && c);
    }

    {
        let a: u8 = 123;
        let mut b = 0;
        let c = tests.Param2(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: u16 = 123;
        let mut b = 0;
        let c = tests.Param3(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: u32 = 123;
        let mut b = 0;
        let c = tests.Param4(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: u64 = 123;
        let mut b = 0;
        let c = tests.Param5(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: i16 = 123;
        let mut b = 0;
        let c = tests.Param6(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: i32 = 123;
        let mut b = 0;
        let c = tests.Param7(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: i64 = 123;
        let mut b = 0;
        let c = tests.Param8(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: f32 = 12.3;
        let mut b = 0.0;
        let c = tests.Param9(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: f64 = 12.3;
        let mut b = 0.0;
        let c = tests.Param10(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: u16 = 0x57; // WinRT char e.g. L'W'
        let mut b = 0;
        let c = tests.Param11(a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a: windows::HSTRING = "WinRT".into();
        let mut b = windows::HSTRING::new();
        let c = tests.Param12(&a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let a = Blittable {
            A: 1,
            B: 2,
            C: 3,
            D: 4,
            E: -5,
            F: -6,
            G: -7,
            H: 8.0,
            I: 9.0,
            J: windows::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
        };

        let mut b = Blittable::default();
        let c = tests.Param13(&a, &a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let object = PropertyValue::CreateInt64(1234)?;
        let pv: IReference<i64> = object.cast()?;

        let a = NonBlittable {
            A: false,
            B: 0x57, // WinRT char
            C: "WinRT".into(),
            D: Some(pv),
        };

        let mut b = NonBlittable::default();
        let c = tests.Param14(&a, &a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let object = PropertyValue::CreateInt64(1234)?;
        let pv: IReference<i64> = object.cast()?;

        let a = Nested {
            Blittable: Blittable {
                A: 1,
                B: 2,
                C: 3,
                D: 4,
                E: -5,
                F: -6,
                G: -7,
                H: 8.0,
                I: 9.0,
                J: windows::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
            },
            NonBlittable: NonBlittable {
                A: false,
                B: 0x57, // WinRT char
                C: "WinRT".into(),
                D: Some(pv),
            },
        };

        let mut b = Nested::default();
        let c = tests.Param15(&a, &a, &mut b)?;
        assert!(a == b && a == c);
    }

    Ok(())
}

#[test]
fn arrays() -> windows::Result<()> {
    let tests = TestRunner::MakeTests()?;

    {
        let a: [bool; 3] = [true, false, true];
        let mut b = [false; 3];
        let mut c = windows::Array::new();
        let d = tests.Array1(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u8; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array2(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u16; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array3(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u32; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array4(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u64; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array5(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i16; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array6(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i32; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array7(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i64; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array8(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [f32; 3] = [1.0, 2.0, 3.0];
        let mut b = [0.0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array9(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [f64; 3] = [1.0, 2.0, 3.0];
        let mut b = [0.0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array10(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u16; 3] = [0x61, 0x62, 0x63]; // WinRT char e.g. L'a' , L'b', L'c'
        let mut b = [0; 3];
        let mut c = windows::Array::new();
        let d = tests.Array11(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [windows::HSTRING; 3] = ["apples".into(), "oranges".into(), "pears".into()];
        let mut b = [
            windows::HSTRING::new(),
            windows::HSTRING::new(),
            windows::HSTRING::new(),
        ];
        let mut c = windows::Array::new();
        let d = tests.Array12(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [Blittable; 3] = [
            Blittable {
                A: 1,
                B: 2,
                C: 3,
                D: 4,
                E: -5,
                F: -6,
                G: -7,
                H: 8.0,
                I: 9.0,
                J: windows::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
            },
            Blittable {
                A: 10,
                B: 20,
                C: 30,
                D: 40,
                E: -50,
                F: -60,
                G: -70,
                H: 80.0,
                I: 90.0,
                J: windows::Guid::from("914b6107-9a3a-4c0d-98df-aca11b016698"),
            },
            Blittable {
                A: 1,
                B: 2,
                C: 3,
                D: 4,
                E: -5,
                F: -6,
                G: -7,
                H: 8.0,
                I: 9.0,
                J: windows::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
            },
        ];

        let mut b = [
            Blittable::default(),
            Blittable::default(),
            Blittable::default(),
        ];

        let mut c = windows::Array::new();
        let d = tests.Array13(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let object = PropertyValue::CreateInt64(123)?;
        let first: IReference<i64> = object.cast()?;

        let object = PropertyValue::CreateInt64(456)?;
        let second: IReference<i64> = object.cast()?;

        let object = PropertyValue::CreateInt64(789)?;
        let third: IReference<i64> = object.cast()?;

        let a: [NonBlittable; 3] = [
            NonBlittable {
                A: false,
                B: 0x61, // WinRT char
                C: "first".into(),
                D: Some(first),
            },
            NonBlittable {
                A: true,
                B: 0x62, // WinRT char
                C: "second".into(),
                D: Some(second),
            },
            NonBlittable {
                A: false,
                B: 0x64, // WinRT char
                C: "third".into(),
                D: Some(third),
            },
        ];

        let mut b = [
            NonBlittable::default(),
            NonBlittable::default(),
            NonBlittable::default(),
        ];

        let mut c = windows::Array::new();
        let d = tests.Array14(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let object = PropertyValue::CreateInt64(123)?;
        let first: IReference<i64> = object.cast()?;

        let object = PropertyValue::CreateInt64(456)?;
        let second: IReference<i64> = object.cast()?;

        let object = PropertyValue::CreateInt64(789)?;
        let third: IReference<i64> = object.cast()?;

        let a: [Nested; 3] = [
            Nested {
                Blittable: Blittable {
                    A: 1,
                    B: 2,
                    C: 3,
                    D: 4,
                    E: -5,
                    F: -6,
                    G: -7,
                    H: 8.0,
                    I: 9.0,
                    J: windows::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
                },
                NonBlittable: NonBlittable {
                    A: false,
                    B: 0x61, // WinRT char
                    C: "first".into(),
                    D: Some(first),
                },
            },
            Nested {
                Blittable: Blittable {
                    A: 10,
                    B: 20,
                    C: 30,
                    D: 40,
                    E: -50,
                    F: -60,
                    G: -70,
                    H: 80.0,
                    I: 90.0,
                    J: windows::Guid::from("914b6107-9a3a-4c0d-98df-aca11b016698"),
                },
                NonBlittable: NonBlittable {
                    A: true,
                    B: 0x62, // WinRT char
                    C: "second".into(),
                    D: Some(second),
                },
            },
            Nested {
                Blittable: Blittable {
                    A: 1,
                    B: 2,
                    C: 3,
                    D: 4,
                    E: -5,
                    F: -6,
                    G: -7,
                    H: 8.0,
                    I: 9.0,
                    J: windows::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
                },
                NonBlittable: NonBlittable {
                    A: false,
                    B: 0x63, // WinRT char
                    C: "third".into(),
                    D: Some(third),
                },
            },
        ];

        let mut b = [Nested::default(), Nested::default(), Nested::default()];

        let mut c = windows::Array::new();
        let d = tests.Array15(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [Option<IStringable>; 3] = [
            Some(Uri::CreateUri("http://kennykerr.ca/one")?.into()),
            Some(Uri::CreateUri("http://kennykerr.ca/two")?.into()),
            Some(Uri::CreateUri("http://kennykerr.ca/three")?.into()),
        ];

        let mut b = [None, None, None];

        let mut c = windows::Array::new();
        let d = tests.Array16(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);

        assert_eq!(
            c[0].as_ref().unwrap().ToString()?,
            "http://kennykerr.ca/one"
        );
        assert_eq!(
            c[1].as_ref().unwrap().ToString()?,
            "http://kennykerr.ca/two"
        );
        assert_eq!(
            c[2].as_ref().unwrap().ToString()?,
            "http://kennykerr.ca/three"
        );
    }

    Ok(())
}

#[test]
fn delegate_params() -> windows::Result<()> {
    let tests = TestRunner::MakeTests()?;

    tests.Param1Call(Param1Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param2Call(Param2Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param3Call(Param3Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param4Call(Param4Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param5Call(Param5Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param6Call(Param6Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param7Call(Param7Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param8Call(Param8Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param9Call(Param9Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param10Call(Param10Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param11Call(Param11Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.Param12Call(Param12Handler::new(|a, b| {
        *b = a.clone();
        Ok(a.clone())
    }))?;

    tests.Param13Call(Param13Handler::new(|a, b, c| {
        assert_eq!(a, b);
        *c = a.clone();
        Ok(a.clone())
    }))?;

    Ok(())
}

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
