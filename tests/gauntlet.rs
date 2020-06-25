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
fn params() -> Result<()> {
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

#[test]
fn arrays() -> Result<()> {
    let tests = TestRunner::make_tests()?;

    {
        let a: [bool; 3] = [true, false, true];
        let mut b = [false; 3];
        let mut c = Array::new();
        let d = tests.array1(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u8; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = tests.array2(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u16; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = tests.array3(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u32; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = tests.array4(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u64; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = tests.array5(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i16; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = tests.array6(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i32; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = tests.array7(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [i64; 3] = [1, 2, 3];
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = tests.array8(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [f32; 3] = [1.0, 2.0, 3.0];
        let mut b = [0.0; 3];
        let mut c = Array::new();
        let d = tests.array9(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [f64; 3] = [1.0, 2.0, 3.0];
        let mut b = [0.0; 3];
        let mut c = Array::new();
        let d = tests.array10(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [u16; 3] = [0x61, 0x62, 0x63]; // WinRT char e.g. L'a' , L'b', L'c'
        let mut b = [0; 3];
        let mut c = Array::new();
        let d = tests.array11(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let a: [HString; 3] = ["apples".into(), "oranges".into(), "pears".into()];
        let mut b = [HString::new(), HString::new(), HString::new()];
        let mut c = Array::new();
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
                j: Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
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
                j: Guid::from("914b6107-9a3a-4c0d-98df-aca11b016698"),
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
                j: Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
            },
        ];

        let mut b = [
            Blittable::default(),
            Blittable::default(),
            Blittable::default(),
        ];

        let mut c = Array::new();
        let d = tests.array13(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let object = PropertyValue::create_int64(123)?;
        let first: IReference<i64> = object.try_into()?;

        let object = PropertyValue::create_int64(456)?;
        let second: IReference<i64> = object.try_into()?;

        let object = PropertyValue::create_int64(789)?;
        let third: IReference<i64> = object.try_into()?;

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

        let mut c = Array::new();
        let d = tests.array14(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    {
        let object = PropertyValue::create_int64(123)?;
        let first: IReference<i64> = object.try_into()?;

        let object = PropertyValue::create_int64(456)?;
        let second: IReference<i64> = object.try_into()?;

        let object = PropertyValue::create_int64(789)?;
        let third: IReference<i64> = object.try_into()?;

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
                    j: Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
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
                    j: Guid::from("914b6107-9a3a-4c0d-98df-aca11b016698"),
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
                    j: Guid::from("CFF52E04-CCA6-4614-A17E-754910C84A99"),
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

        let mut c = Array::new();
        let d = tests.array15(&a, &mut b, &mut c)?;
        assert!(a == b);
        assert!(a == c[..]);
        assert!(a == d[..]);
    }

    Ok(())
}
