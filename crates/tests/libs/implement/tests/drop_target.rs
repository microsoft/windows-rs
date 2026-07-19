#![cfg(windows)]
use windows::{Win32::*, core::*};

#[implement(IDataObject)]
struct DataObject();

impl IDataObject_Impl for DataObject_Impl {
    fn GetData(&self, _: *const FORMATETC) -> Result<STGMEDIUM> {
        unimplemented!()
    }
    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<()> {
        unimplemented!()
    }
    fn QueryGetData(&self, _: *const FORMATETC) -> Result<()> {
        unimplemented!()
    }
    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC, _: *mut FORMATETC) -> Result<()> {
        unimplemented!()
    }
    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<()> {
        unimplemented!()
    }
    fn EnumFormatEtc(&self, _: u32) -> Result<IEnumFORMATETC> {
        unimplemented!()
    }
    fn DAdvise(&self, format: *const FORMATETC, value: u32, sink: Ref<IAdviseSink>) -> Result<u32> {
        assert!(!format.is_null());
        assert_eq!(value, 789);
        assert!(sink.is_null());
        Ok(123)
    }
    fn DUnadvise(&self, _: u32) -> Result<()> {
        unimplemented!()
    }
    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA> {
        unimplemented!()
    }
}

#[implement(IDropTarget)]
struct DropTarget();

impl IDropTarget_Impl for DropTarget_Impl {
    fn DragEnter(
        &self,
        object: Ref<IDataObject>,
        state: u32,
        point: &POINTL,
        effect: *mut u32,
    ) -> Result<()> {
        unsafe {
            assert_eq!(
                object.unwrap().DAdvise(&FORMATETC::default(), 789, None)?,
                123
            );
            assert_eq!(state, MK_MBUTTON);
            assert_eq!(*effect, DROPEFFECT_LINK);
            *effect = DROPEFFECT_MOVE;
            assert_eq!(*point, POINTL { x: 10, y: 20 });
            Ok(())
        }
    }
    fn DragOver(&self, _: u32, _: &POINTL, _: *mut u32) -> Result<()> {
        unimplemented!()
    }
    fn DragLeave(&self) -> Result<()> {
        Ok(())
    }
    fn Drop(&self, _: Ref<IDataObject>, _: u32, _: &POINTL, _: *mut u32) -> Result<()> {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let object: IDataObject = DataObject().into();

        let target: IDropTarget = DropTarget().into();
        target.DragLeave().ok()?;

        let mut effect = DROPEFFECT_LINK;
        target
            .DragEnter(&object, MK_MBUTTON, POINTL { x: 10, y: 20 }, &mut effect)
            .ok()?;
        assert_eq!(effect, DROPEFFECT_MOVE);

        Ok(())
    }
}
