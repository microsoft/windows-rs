#![allow(non_snake_case)]

use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*};

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

impl IDataObject_Impl for Test {
    fn GetData(&self, _: *const FORMATETC) -> Result<STGMEDIUM> {
        unsafe {
            (*self.0.get()).GetData = true;
            Ok(STGMEDIUM::default())
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

    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC) -> Result<FORMATETC> {
        unsafe {
            (*self.0.get()).GetCanonicalFormatEtc = true;
            Ok(FORMATETC::default())
        }
    }

    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<()> {
        unsafe {
            (*self.0.get()).SetData = true;
            Ok(())
        }
    }

    fn EnumFormatEtc(&self, _: u32) -> Result<IEnumFORMATETC> {
        unsafe {
            (*self.0.get()).EnumFormatEtc = true;
            Err(Error::OK)
        }
    }

    fn DAdvise(&self, _: *const FORMATETC, _: u32, _: &Option<IAdviseSink>) -> Result<u32> {
        unsafe {
            (*self.0.get()).DAdvise = true;
            Ok(0)
        }
    }

    fn DUnadvise(&self, _: u32) -> Result<()> {
        unsafe {
            (*self.0.get()).DUnadvise = true;
            Ok(())
        }
    }

    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA> {
        unsafe {
            (*self.0.get()).EnumDAdvise = true;
            Err(Error::OK)
        }
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let d: IDataObject = Test::default().into();
        d.GetData(core::ptr::null_mut())?;
        d.GetDataHere(core::ptr::null_mut(), core::ptr::null_mut())?;
        d.QueryGetData(core::ptr::null_mut())?;
        d.GetCanonicalFormatEtc(core::ptr::null_mut())?;
        d.SetData(core::ptr::null_mut(), core::ptr::null_mut(), false)?;
        let _ = d.EnumFormatEtc(0);
        d.DAdvise(core::ptr::null_mut(), 0, None)?;

        let i = Test::to_impl(&d).0.get();
        assert!((*i).GetData);
        assert!((*i).GetDataHere);
        assert!((*i).QueryGetData);
        assert!((*i).GetCanonicalFormatEtc);
        assert!((*i).SetData);
        assert!((*i).EnumFormatEtc);
        assert!((*i).DAdvise);

        assert!(!(*i).DUnadvise);
        assert!(!(*i).EnumDAdvise);

        d.DUnadvise(0)?;
        let _ = d.EnumDAdvise();

        assert!((*i).DUnadvise);
        assert!((*i).EnumDAdvise);

        Ok(())
    }
}
