//! Exercises the `interface_decl!` and `implement_decl!` macro_rules! alternatives to
//! `#[interface]` / `#[implement]`.
//!
//! Both sides — interface declaration and implementer wiring — are done with declarative
//! macros, so this test doubles as proof that the fully proc-macro-free path produces an
//! object that is layout- and ABI-compatible with what the proc-macros would have emitted.
//! Coverage spans the non-generic and `impl<T>` arms, the IInspectable identity, and
//! cross-interface `QueryInterface`.

#![expect(non_snake_case)]

use windows_core::*;

const S_OK: HRESULT = HRESULT(0);
const S_FALSE: HRESULT = HRESULT(1);
const E_INVALIDARG: HRESULT = HRESULT(0x80070057_u32 as _);

interface_decl! {
    unsafe trait ITest(ITest_Vtbl, ITest_Impl) : IUnknown
        = 0x09428a59_5b40_4e4c_9175_e7a78514316d
    {
        unsafe fn Void(&self);
        unsafe fn Result(&self, code: HRESULT) -> Result<()>;
    }
}

interface_decl! {
    unsafe trait IOther(IOther_Vtbl, IOther_Impl) : IUnknown
        = 0x1a2b3c4d_5e6f_7081_92a3_b4c5d6e7f809
    {
        unsafe fn Sum(&self, a: i32, b: i32, out: *mut i32) -> Result<()>;
    }
}

// Interface modeled after `IWindowNative` from WinUI: a method that returns a raw
// `HRESULT` rather than `Result<()>`. The safe wrapper, impl trait, and vtable entry
// all carry the declared return type through verbatim.
interface_decl! {
    unsafe trait IRaw(IRaw_Vtbl, IRaw_Impl) : IUnknown
        = 0xeecdbf0e_bae9_4cb6_a68e_9598e1cb57bb
    {
        unsafe fn Echo(&self, code: HRESULT, out: *mut HRESULT) -> HRESULT;
    }
}

pub struct Test;

implement_decl! {
    impl Test as pub Test_Impl: [ITest, IOther, IRaw]
}

impl ITest_Impl for Test_Impl {
    unsafe fn Void(&self) {}
    unsafe fn Result(&self, code: HRESULT) -> Result<()> {
        code.ok()
    }
}

impl IOther_Impl for Test_Impl {
    unsafe fn Sum(&self, a: i32, b: i32, out: *mut i32) -> Result<()> {
        unsafe { *out = a + b };
        Ok(())
    }
}

impl IRaw_Impl for Test_Impl {
    unsafe fn Echo(&self, code: HRESULT, out: *mut HRESULT) -> HRESULT {
        unsafe { *out = code };
        code
    }
}

// A generic implementer, exercising the `impl<T>` arm of `implement_decl!`. Each `T`
// contributes an offset so the wrapped value is observably used per monomorphization.
trait Offset {
    fn offset(&self) -> i32;
}

impl Offset for i32 {
    fn offset(&self) -> i32 {
        *self
    }
}

impl Offset for &'static str {
    fn offset(&self) -> i32 {
        self.len() as i32
    }
}

struct Generic<T>(T);

implement_decl! {
    impl<T> Generic as Generic_Impl: [ITest, IOther]
    where T: Offset + 'static
}

impl<T> ITest_Impl for Generic_Impl<T>
where
    T: Offset + 'static,
{
    unsafe fn Void(&self) {}
    unsafe fn Result(&self, code: HRESULT) -> Result<()> {
        code.ok()
    }
}

impl<T> IOther_Impl for Generic_Impl<T>
where
    T: Offset + 'static,
{
    unsafe fn Sum(&self, a: i32, b: i32, out: *mut i32) -> Result<()> {
        unsafe { *out = a + b + self.0.offset() };
        Ok(())
    }
}

#[test]
fn test_itest() {
    unsafe {
        let test: ITest = Test.into();

        test.Void();

        assert!(test.Result(S_OK).is_ok());
        assert!(test.Result(S_FALSE).is_ok());
        assert_eq!(test.Result(E_INVALIDARG), E_INVALIDARG.ok());
    }
}

#[test]
fn test_iother() {
    unsafe {
        // Construct via the second interface to exercise its vtable slot.
        let other: IOther = Test.into();
        let mut value = 0;
        other.Sum(2, 3, &mut value).unwrap();
        assert_eq!(value, 5);
        other.Sum(-1, 1, &mut value).unwrap();
        assert_eq!(value, 0);

        // QueryInterface across to ITest and back to IOther.
        let test: ITest = other.cast().unwrap();
        test.Void();
        let other2: IOther = test.cast().unwrap();
        other2.Sum(7, 8, &mut value).unwrap();
        assert_eq!(value, 15);
    }
}

#[test]
fn test_iraw() {
    unsafe {
        let raw: IRaw = Test.into();
        let mut out = S_OK;
        // Method returns the raw `HRESULT` verbatim, no `Result` wrapping.
        assert_eq!(raw.Echo(S_FALSE, &mut out), S_FALSE);
        assert_eq!(out, S_FALSE);
        assert_eq!(raw.Echo(E_INVALIDARG, &mut out), E_INVALIDARG);
        assert_eq!(out, E_INVALIDARG);

        // QueryInterface across to the other interfaces is unaffected.
        let test: ITest = raw.cast().unwrap();
        test.Void();
    }
}

#[test]
fn iid_matches() {
    assert_eq!(
        ITest::IID,
        GUID::from_u128(0x09428a59_5b40_4e4c_9175_e7a78514316d)
    );
    assert!(ITest_Vtbl::matches(&ITest::IID));
    assert!(!ITest_Vtbl::matches(&IUnknown::IID));
}

#[test]
fn refcount_drops_to_zero() {
    // Construct an object, take an extra reference via cast, then drop both. If the
    // refcount math is wrong the boxed object would leak (caught by miri) or, if doubly
    // freed, cause a crash. This test is mainly here so that the `Release` path runs.
    let test: ITest = Test.into();
    let other: IOther = test.cast().unwrap();
    drop(test);
    drop(other);
}

#[test]
fn iinspectable_identity() {
    // `implement_decl!` always emits the IInspectable identity (`From`,
    // `ComObjectInterface<IInspectable>`, and the QI route), even for IUnknown-only
    // interfaces, with a fixed trust level of 0.
    let inspectable: IInspectable = Test.into();
    assert_eq!(inspectable.GetTrustLevel().unwrap(), 0);

    // QI(IID_IInspectable) succeeds from any interface and round-trips back.
    let test: ITest = Test.into();
    let inspectable: IInspectable = test.cast().unwrap();
    assert_eq!(inspectable.GetTrustLevel().unwrap(), 0);
    let test: ITest = inspectable.cast().unwrap();
    unsafe { test.Void() };

    // IUnknown <-> IInspectable in both directions.
    let unknown: IUnknown = inspectable.cast().unwrap();
    let _: IInspectable = unknown.cast().unwrap();
}

#[test]
fn generic_implementer() {
    unsafe {
        // T = i32: the wrapped value adds 10.
        let other: IOther = Generic(10_i32).into();
        let mut value = 0;
        other.Sum(2, 3, &mut value).unwrap();
        assert_eq!(value, 15);

        // QI across to ITest and to the IInspectable identity.
        let test: ITest = other.cast().unwrap();
        test.Void();
        let inspectable: IInspectable = other.cast().unwrap();
        assert_eq!(inspectable.GetTrustLevel().unwrap(), 0);

        // A second monomorphization (T = &str) wires up independently.
        let other: IOther = Generic("abcd").into();
        other.Sum(1, 1, &mut value).unwrap();
        assert_eq!(value, 6);
    }
}
