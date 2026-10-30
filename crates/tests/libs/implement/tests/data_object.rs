#![cfg(windows)]
#![expect(non_snake_case)]

use windows::{Win32::*, core::*};

#[derive(Default)]
struct TestData {
    GetData: bool,
    GetDataHere: bool,
    QueryGetData: bool,
    GetCanonicalFormatEtc: bool,
    SetData: bool,
    EnumFormatEtc: bool,
    DAdvise: bool,
    DUnadvise: bool,
    EnumDAdvise: bool,
}

#[implement(IDataObject)]
#[derive(Default)]
struct Test(std::cell::UnsafeCell<TestData>);

impl IDataObject_Impl for Test_Impl {
    fn GetData(&self, _: *const FORMATETC, medium: *mut STGMEDIUM) -> Result<()> {
        unsafe {
            (*self.0.get()).GetData = true;
            medium.write(STGMEDIUM::default());
            Ok(())
        }
    }

    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<()> {
        unsafe {
            (*self.0.get()).GetDataHere = true;
            Ok(())
        }
    }

    fn QueryGetData(&self, _: *const FORMATETC) -> Result<()> {
        unsafe {
            (*self.0.get()).QueryGetData = true;
            Ok(())
        }
    }

    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC, _: *mut FORMATETC) -> Result<()> {
        unsafe {
            (*self.0.get()).GetCanonicalFormatEtc = true;
            Ok(())
        }
    }

    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<()> {
        unsafe {
            (*self.0.get()).SetData = true;
            Ok(())
        }
    }

    fn EnumFormatEtc(&self, _: u32, _: OutRef<IEnumFORMATETC>) -> Result<()> {
        unsafe {
            (*self.0.get()).EnumFormatEtc = true;
            Err(Error::empty())
        }
    }

    fn DAdvise(
        &self,
        _: *const FORMATETC,
        _: u32,
        _: Ref<IAdviseSink>,
        connection: *mut u32,
    ) -> Result<()> {
        unsafe {
            (*self.0.get()).DAdvise = true;
            *connection = 0;
            Ok(())
        }
    }

    fn DUnadvise(&self, _: u32) -> Result<()> {
        unsafe {
            (*self.0.get()).DUnadvise = true;
            Ok(())
        }
    }

    fn EnumDAdvise(&self, _: OutRef<IEnumSTATDATA>) -> Result<()> {
        unsafe {
            (*self.0.get()).EnumDAdvise = true;
            Err(Error::empty())
        }
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
        let mut r = None;
        assert!(d.EnumFormatEtc(0, &mut r) == S_OK);
        assert!(r.is_none());

        let mut connection = 0;
        d.DAdvise(&Default::default(), 0, None, &mut connection)
            .ok()?;

        let i = d.as_impl().0.get();
        assert!((*i).GetData);
        assert!((*i).GetDataHere);
        assert!((*i).QueryGetData);
        assert!((*i).GetCanonicalFormatEtc);
        assert!((*i).SetData);
        assert!((*i).EnumFormatEtc);
        assert!((*i).DAdvise);

        assert!(!(*i).DUnadvise);
        assert!(!(*i).EnumDAdvise);

        d.DUnadvise(0).ok()?;
        _ = d.EnumDAdvise(&mut None);

        assert!((*i).DUnadvise);
        assert!((*i).EnumDAdvise);

        Ok(())
    }
}
