#![allow(non_snake_case)]

use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*};

#[implement(IDataObject)]
#[derive(Default)]
struct Test {
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

impl IDataObject_Impl for Test {
    fn GetData(&self, _: *const FORMATETC) -> Result<STGMEDIUM> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.GetData = true;
            Ok(STGMEDIUM::default())
        }
    }

    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<()> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.GetDataHere = true;
            Ok(())
        }
    }

    fn QueryGetData(&self, _: *const FORMATETC) -> Result<()> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.QueryGetData = true;
            Ok(())
        }
    }

    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC) -> Result<FORMATETC> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.GetCanonicalFormatEtc = true;
            Ok(FORMATETC::default())
        }
    }

    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<()> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.SetData = true;
            Ok(())
        }
    }

    fn EnumFormatEtc(&self, _: u32) -> Result<IEnumFORMATETC> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.EnumFormatEtc = true;
            Err(Error::OK)
        }
    }

    fn DAdvise(&self, _: *const FORMATETC, _: u32, _: &Option<IAdviseSink>) -> Result<u32> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.DAdvise = true;
            Ok(0)
        }
    }

    fn DUnadvise(&self, _: u32) -> Result<()> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.DUnadvise = true;
            Ok(())
        }
    }

    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA> {
        unsafe {
            let writer: &mut Self = &mut *(self as *const _ as *mut Self);
            writer.EnumDAdvise = true;
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

        let i = Test::to_impl(&d);
        assert!(i.GetData);
        assert!(i.GetDataHere);
        assert!(i.QueryGetData);
        assert!(i.GetCanonicalFormatEtc);
        assert!(i.SetData);
        assert!(i.EnumFormatEtc);
        assert!(i.DAdvise);

        assert!(!i.DUnadvise);
        assert!(!i.EnumDAdvise);

        d.DUnadvise(0)?;
        let _ = d.EnumDAdvise();

        assert!(i.DUnadvise);
        assert!(i.EnumDAdvise);

        Ok(())
    }
}
