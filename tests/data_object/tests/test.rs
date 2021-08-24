use test_data_object::*;
use windows::*;
use Windows::Win32::Foundation::*;
use Windows::Win32::System::Com::*;

#[implement(Windows::Win32::System::Com::IDataObject)]
struct Test();

#[allow(non_snake_case)]
impl Test {
    fn GetData(&self, _: *const FORMATETC) -> Result<STGMEDIUM> {
        panic!()
    }

    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<()> {
        panic!()
    }

    fn QueryGetData(&self, _: *const FORMATETC) -> Result<()> {
        panic!()
    }

    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC) -> Result<FORMATETC> {
        panic!()
    }

    fn SetData<'a>(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<()> {
        panic!()
    }

    fn EnumFormatEtc(&self, _: u32) -> Result<IEnumFORMATETC> {
        panic!()
    }

    fn DAdvise<'a>(&self, _: *const FORMATETC, _: u32, _: &Option<IAdviseSink>) -> Result<u32> {
        panic!()
    }

    fn DUnadvise(&self, _: u32) -> Result<()> {
        panic!()
    }

    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA> {
        panic!()
    }
}
