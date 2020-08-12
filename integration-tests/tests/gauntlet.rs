use tests::test_component::*;
use tests::windows::foundation::{IReference, IStringable, PropertyValue, Uri};
use winrt::ComInterface;

#[test]
fn test_self() -> winrt::Result<()> {
    TestRunner::test_self()?;

    Ok(())
}

#[test]
fn params() -> winrt::Result<()> {
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
            j: winrt::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
        };

        let mut b = Blittable::default();
        let c = tests.param13(&a, &a, &mut b)?;
        assert!(a == b && a == c);
    }

    {
        let object = PropertyValue::create_int64(1234)?;
        let pv: IReference<i64> = object.query();

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
        let pv: IReference<i64> = object.query();

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
                j: winrt::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
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

#[test]
fn arrays() -> winrt::Result<()> {
    let tests = TestRunner::make_tests()?;

    {
        let a: [bool; 3] = [true, false, true];
        let mut b = [false; 3];
        let mut c = winrt::Array::new();
        let d = tests.array1(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u8; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array2(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u16; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array3(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u32; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array4(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u64; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array5(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i16; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array6(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i32; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array7(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i64; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array8(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [f32; 3] = [1.0, 2.0, 3.0];
        let mut b = [0.0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array9(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [f64; 3] = [1.0, 2.0, 3.0];
        let mut b = [0.0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array10(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u16; 3] = [0x61, 0x62, 0x63]; // WinRT char e.g. L'a' , L'b', L'c'
        let mut b = [0; 3];
        let mut c = winrt::Array::new();
        let d = tests.array11(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [winrt::HString; 3] = ["apples".into(), "oranges".into(), "pears".into()];
        let mut b = [
            winrt::HString::new(),
            winrt::HString::new(),
            winrt::HString::new(),
        ];
        let mut c = winrt::Array::new();
        let d = tests.array12(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [Blittable; 3] = [
            Blittable {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: -5,
                f: -6,
                g: -7,
                h: 8.0,
                i: 9.0,
                j: winrt::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
            },
            Blittable {
                a: 10,
                b: 20,
                c: 30,
                d: 40,
                e: -50,
                f: -60,
                g: -70,
                h: 80.0,
                i: 90.0,
                j: winrt::Guid::from("914b6107-9a3a-4c0d-98df-aca11b016698"),
            },
            Blittable {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: -5,
                f: -6,
                g: -7,
                h: 8.0,
                i: 9.0,
                j: winrt::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
            },
        ];

        let mut b = [
            Blittable::default(),
            Blittable::default(),
            Blittable::default(),
        ];

        let mut c = winrt::Array::new();
        let d = tests.array13(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let object = PropertyValue::create_int64(123)?;
        let first: IReference<i64> = object.query();

        let object = PropertyValue::create_int64(456)?;
        let second: IReference<i64> = object.query();

        let object = PropertyValue::create_int64(789)?;
        let third: IReference<i64> = object.query();

        let a: [NonBlittable; 3] = [
            NonBlittable {
                a: false,
                b: 0x61, // WinRT char
                c: "first".into(),
                d: first,
            },
            NonBlittable {
                a: true,
                b: 0x62, // WinRT char
                c: "second".into(),
                d: second,
            },
            NonBlittable {
                a: false,
                b: 0x64, // WinRT char
                c: "third".into(),
                d: third,
            },
        ];

        let mut b = [
            NonBlittable::default(),
            NonBlittable::default(),
            NonBlittable::default(),
        ];

        let mut c = winrt::Array::new();
        let d = tests.array14(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let object = PropertyValue::create_int64(123)?;
        let first: IReference<i64> = object.query();

        let object = PropertyValue::create_int64(456)?;
        let second: IReference<i64> = object.query();

        let object = PropertyValue::create_int64(789)?;
        let third: IReference<i64> = object.query();

        let a: [Nested; 3] = [
            Nested {
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
                    j: winrt::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
                },
                non_blittable: NonBlittable {
                    a: false,
                    b: 0x61, // WinRT char
                    c: "first".into(),
                    d: first,
                },
            },
            Nested {
                blittable: Blittable {
                    a: 10,
                    b: 20,
                    c: 30,
                    d: 40,
                    e: -50,
                    f: -60,
                    g: -70,
                    h: 80.0,
                    i: 90.0,
                    j: winrt::Guid::from("914b6107-9a3a-4c0d-98df-aca11b016698"),
                },
                non_blittable: NonBlittable {
                    a: true,
                    b: 0x62, // WinRT char
                    c: "second".into(),
                    d: second,
                },
            },
            Nested {
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
                    j: winrt::Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
                },
                non_blittable: NonBlittable {
                    a: false,
                    b: 0x63, // WinRT char
                    c: "third".into(),
                    d: third,
                },
            },
        ];

        let mut b = [Nested::default(), Nested::default(), Nested::default()];

        let mut c = winrt::Array::new();
        let d = tests.array15(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [IStringable; 3] = [
            Uri::create_uri("http://kennykerr.ca/one")?.into(),
            Uri::create_uri("http://kennykerr.ca/two")?.into(),
            Uri::create_uri("http://kennykerr.ca/three")?.into(),
        ];

        let mut b = [
            IStringable::default(),
            IStringable::default(),
            IStringable::default(),
        ];

        let mut c = winrt::Array::new();
        let d = tests.array16(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);

        assert_eq!(c[0].to_string()?, "http://kennykerr.ca/one");
        assert_eq!(c[1].to_string()?, "http://kennykerr.ca/two");
        assert_eq!(c[2].to_string()?, "http://kennykerr.ca/three");
    }

    Ok(())
}

#[test]
fn delegate_params() -> winrt::Result<()> {
    let tests = TestRunner::make_tests()?;

    tests.param1_call(Param1Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param2_call(Param2Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param3_call(Param3Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param4_call(Param4Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param5_call(Param5Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param6_call(Param6Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param7_call(Param7Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param8_call(Param8Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param9_call(Param9Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param10_call(Param10Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param11_call(Param11Handler::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    tests.param12_call(Param12Handler::new(|a, b| {
        *b = a.clone();
        Ok(a.clone())
    }))?;

    tests.param13_call(Param13Handler::new(|a, b, c| {
        assert_eq!(a, b);
        *c = a.clone();
        Ok(a.clone())
    }))?;

    Ok(())
}

#[test]
fn collections() -> winrt::Result<()> {
    {
        let v = TestRunner::create_int32_vector()?;
        assert_eq!(v.size()?, 0);
        v.append(1)?;
        assert_eq!(v.size()?, 1);
        assert_eq!(v.get_at(0)?, 1);
        v.replace_all(&[1, 2, 3])?;
        assert_eq!(v.size()?, 3);
        assert_eq!(v.get_at(0)?, 1);
        assert_eq!(v.get_at(1)?, 2);
        assert_eq!(v.get_at(2)?, 3);
    }

    {
        let v = TestRunner::create_string_vector()?;
        assert_eq!(v.size()?, 0);
        v.append("one")?;
        assert_eq!(v.size()?, 1);
        assert_eq!(v.get_at(0)?, "one");
        v.replace_all(&["one".into(), "two".into(), "three".into()])?;
        assert_eq!(v.size()?, 3);
        assert_eq!(v.get_at(0)?, "one");
        assert_eq!(v.get_at(1)?, "two");
        assert_eq!(v.get_at(2)?, "three");
    }

    {
        let one: IStringable = Uri::create_uri("http://kennykerr.ca/one")?.into();
        let two: IStringable = Uri::create_uri("http://kennykerr.ca/two")?.into();
        let three: IStringable = Uri::create_uri("http://kennykerr.ca/three")?.into();

        let v = TestRunner::create_stringable_vector()?;
        assert_eq!(v.size()?, 0);
        v.append(&one)?;
        assert_eq!(v.size()?, 1);
        assert_eq!(v.get_at(0)?.to_string()?, "http://kennykerr.ca/one");
        v.replace_all(&[one, two, three])?;
        assert_eq!(v.size()?, 3);
        assert_eq!(v.get_at(0)?.to_string()?, "http://kennykerr.ca/one");
        assert_eq!(v.get_at(1)?.to_string()?, "http://kennykerr.ca/two");
        assert_eq!(v.get_at(2)?.to_string()?, "http://kennykerr.ca/three");
    }

    Ok(())
}

async fn async_await() -> winrt::Result<()> {
    let tests = TestRunner::make_tests()?;

    // Success and failure with no delay

    tests
        .async1(TestRunner::create_async_action(0)?, false)?
        .await?;
    assert_eq!(
        tests
            .async1(TestRunner::create_async_action(0)?, true)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    tests
        .async2(TestRunner::create_async_action(0)?, false, 0)?
        .await?;
    assert_eq!(
        tests
            .async2(TestRunner::create_async_action(0)?, true, 0)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    assert_eq!(
        tests
            .async3(TestRunner::create_async_action(0)?, false, 123)?
            .await?,
        123
    );
    assert_eq!(
        tests
            .async3(TestRunner::create_async_action(0)?, true, 123)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    assert_eq!(
        tests
            .async4(TestRunner::create_async_action(0)?, false, 123, 0)?
            .await?,
        123
    );
    assert_eq!(
        tests
            .async4(TestRunner::create_async_action(0)?, true, 123, 0)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    // Success and failure with initial delay

    tests
        .async1(TestRunner::create_async_action(200)?, false)?
        .await?;
    assert_eq!(
        tests
            .async1(TestRunner::create_async_action(200)?, true)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    tests
        .async2(TestRunner::create_async_action(200)?, false, 0)?
        .await?;
    assert_eq!(
        tests
            .async2(TestRunner::create_async_action(200)?, true, 0)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    assert_eq!(
        tests
            .async3(TestRunner::create_async_action(200)?, false, 123)?
            .await?,
        123
    );
    assert_eq!(
        tests
            .async3(TestRunner::create_async_action(200)?, true, 123)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    assert_eq!(
        tests
            .async4(TestRunner::create_async_action(200)?, false, 123, 0)?
            .await?,
        123
    );
    assert_eq!(
        tests
            .async4(TestRunner::create_async_action(200)?, true, 123, 0)?
            .await
            .unwrap_err()
            .message(),
        "test"
    );

    Ok(())
}

#[test]
fn test_async_await() -> winrt::Result<()> {
    futures::executor::block_on(async_await())
}
