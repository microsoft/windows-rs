#![cfg(windows)]
#![expect(non_snake_case)]

use std::sync::atomic::{AtomicBool, Ordering};
use test_implement::*;
use windows_core::*;

#[derive(Default)]
struct TestData {
    GetData: AtomicBool,
    GetDataHere: AtomicBool,
    QueryGetData: AtomicBool,
    GetCanonicalFormatEtc: AtomicBool,
    SetData: AtomicBool,
    EnumFormatEtc: AtomicBool,
    DAdvise: AtomicBool,
    DUnadvise: AtomicBool,
    EnumDAdvise: AtomicBool,
}

#[implement(IDataObject)]
#[derive(Default)]
struct Test(TestData);

impl IDataObject_Impl for Test_Impl {
    fn GetData(&self, _: *const FORMATETC, medium: *mut STGMEDIUM) -> HRESULT {
        unsafe {
            self.0.GetData.store(true, Ordering::Relaxed);
            medium.write(STGMEDIUM::default());
            S_OK
        }
    }

    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> HRESULT {
        self.0.GetDataHere.store(true, Ordering::Relaxed);
        S_OK
    }

    fn QueryGetData(&self, _: *const FORMATETC) -> HRESULT {
        self.0.QueryGetData.store(true, Ordering::Relaxed);
        S_OK
    }

    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC, _: *mut FORMATETC) -> HRESULT {
        self.0.GetCanonicalFormatEtc.store(true, Ordering::Relaxed);
        S_OK
    }

    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> HRESULT {
        self.0.SetData.store(true, Ordering::Relaxed);
        S_OK
    }

    fn EnumFormatEtc(&self, _: u32) -> Result<IEnumFORMATETC> {
        self.0.EnumFormatEtc.store(true, Ordering::Relaxed);
        Err(Error::empty())
    }

    fn DAdvise(
        &self,
        _: *const FORMATETC,
        _: u32,
        _: Ref<IAdviseSink>,
        connection: *mut u32,
    ) -> HRESULT {
        unsafe {
            self.0.DAdvise.store(true, Ordering::Relaxed);
            *connection = 0;
            S_OK
        }
    }

    fn DUnadvise(&self, _: u32) -> HRESULT {
        self.0.DUnadvise.store(true, Ordering::Relaxed);
        S_OK
    }

    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA> {
        self.0.EnumDAdvise.store(true, Ordering::Relaxed);
        Err(Error::empty())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let d: IDataObject = Test::default().into();
        d.GetData(&Default::default(), &mut Default::default())
            .ok()?;
        d.GetDataHere(&Default::default(), &mut Default::default())
            .ok()?;
        d.QueryGetData(Some(&Default::default())).ok()?;
        d.GetCanonicalFormatEtc(Some(&Default::default()), &mut Default::default())
            .ok()?;
        d.SetData(&Default::default(), &Default::default(), false)
            .ok()?;

        // EnumFormatEtc returns a null result value with a successful (S_OK) return code.
        let r = d.EnumFormatEtc(0);
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert!(e.code() == S_OK);
        assert!(e.as_ptr().is_null());

        let mut connection = 0;
        d.DAdvise(&Default::default(), 0, None, &mut connection)
            .ok()?;

        let i = &d.as_impl().0;
        assert!(i.GetData.load(Ordering::Relaxed));
        assert!(i.GetDataHere.load(Ordering::Relaxed));
        assert!(i.QueryGetData.load(Ordering::Relaxed));
        assert!(i.GetCanonicalFormatEtc.load(Ordering::Relaxed));
        assert!(i.SetData.load(Ordering::Relaxed));
        assert!(i.EnumFormatEtc.load(Ordering::Relaxed));
        assert!(i.DAdvise.load(Ordering::Relaxed));

        assert!(!i.DUnadvise.load(Ordering::Relaxed));
        assert!(!i.EnumDAdvise.load(Ordering::Relaxed));

        d.DUnadvise(0).ok()?;
        _ = d.EnumDAdvise();

        assert!(i.DUnadvise.load(Ordering::Relaxed));
        assert!(i.EnumDAdvise.load(Ordering::Relaxed));

        Ok(())
    }
}
