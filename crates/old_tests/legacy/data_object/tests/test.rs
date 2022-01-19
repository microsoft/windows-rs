use test_data_object::*;
use windows::core::*;
use Windows::Win32::Foundation::*;
use Windows::Win32::System::Com::*;

#[implement(Windows::Win32::System::Com::IDataObject)]
#[derive(Default)]
#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
impl Test {
    fn GetData(&mut self, _: *const FORMATETC) -> Result<STGMEDIUM> {
        self.GetData = true;
        Ok(STGMEDIUM { tymed: 0, Anonymous: STGMEDIUM_0 { pstg: core::ptr::null_mut() }, pUnkForRelease: None })
    }

    fn GetDataHere(&mut self, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<()> {
        self.GetDataHere = true;
        Ok(())
    }

    fn QueryGetData(&mut self, _: *const FORMATETC) -> Result<()> {
        self.QueryGetData = true;
        Ok(())
    }

    fn GetCanonicalFormatEtc(&mut self, _: *const FORMATETC) -> Result<FORMATETC> {
        self.GetCanonicalFormatEtc = true;
        Ok(FORMATETC::default())
    }

    fn SetData(&mut self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<()> {
        self.SetData = true;
        Ok(())
    }

    fn EnumFormatEtc(&mut self, _: u32) -> Result<IEnumFORMATETC> {
        self.EnumFormatEtc = true;
        Err(Error::OK)
    }

    fn DAdvise(&mut self, _: *const FORMATETC, _: u32, _: &Option<IAdviseSink>) -> Result<u32> {
        self.DAdvise = true;
        Ok(0)
    }

    fn DUnadvise(&mut self, _: u32) -> Result<()> {
        self.DUnadvise = true;
        Ok(())
    }

    fn EnumDAdvise(&mut self) -> Result<IEnumSTATDATA> {
        self.EnumDAdvise = true;
        Err(Error::OK)
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
        d.DUnadvise(0)?;
        let _ = d.EnumDAdvise();

        let i = Test::to_impl(&d);
        assert!(i.GetData);
        assert!(i.GetDataHere);
        assert!(i.QueryGetData);
        assert!(i.GetCanonicalFormatEtc);
        assert!(i.SetData);
        assert!(i.EnumFormatEtc);
        assert!(i.DAdvise);
        assert!(i.DUnadvise);
        assert!(i.EnumDAdvise);

        Ok(())
    }
}
