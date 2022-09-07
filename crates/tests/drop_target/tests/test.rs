use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*, Win32::System::Ole::*, Win32::System::SystemServices::*};

#[implement(IDataObject)]
struct DataObject();

impl IDataObject_Impl for DataObject {
    fn GetData(&self, _: *const FORMATETC) -> Result<STGMEDIUM> {
        todo!()
    }
    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<()> {
        todo!()
    }
    fn QueryGetData(&self, _: *const FORMATETC) -> HRESULT {
        todo!()
    }
    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC, _: *mut FORMATETC) -> HRESULT {
        todo!()
    }
    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<()> {
        todo!()
    }
    fn EnumFormatEtc(&self, _: u32) -> Result<IEnumFORMATETC> {
        todo!()
    }
    fn DAdvise(&self, format: *const FORMATETC, value: u32, sink: &Option<IAdviseSink>) -> Result<u32> {
        assert!(!format.is_null());
        assert_eq!(value, 789);
        assert!(sink.is_none());
        Ok(123)
    }
    fn DUnadvise(&self, _: u32) -> Result<()> {
        todo!()
    }
    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA> {
        todo!()
    }
}

#[implement(IDropTarget)]
struct DropTarget();

impl IDropTarget_Impl for DropTarget {
    fn DragEnter(&self, object: &Option<IDataObject>, state: MODIFIERKEYS_FLAGS, point: &POINTL, effect: *mut u32) -> Result<()> {
        unsafe {
            assert_eq!(object.as_ref().unwrap().DAdvise(&FORMATETC::default(), 789, None)?, 123);
            assert_eq!(state, MK_MBUTTON);
            assert_eq!(*effect, 741);
            *effect = 147;
            assert_eq!(*point, POINTL { x: 10, y: 20 });
            Ok(())
        }
    }
    fn DragOver(&self, _: MODIFIERKEYS_FLAGS, _: &POINTL, _: *mut u32) -> Result<()> {
        todo!()
    }
    fn DragLeave(&self) -> Result<()> {
        Ok(())
    }
    fn Drop(&self, _: &Option<IDataObject>, _: MODIFIERKEYS_FLAGS, _: &POINTL, _: *mut u32) -> Result<()> {
        todo!()
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let object: IDataObject = DataObject().into();

        let target: IDropTarget = DropTarget().into();
        target.DragLeave()?;

        let mut effect = 741;
        target.DragEnter(&object, MK_MBUTTON, POINTL { x: 10, y: 20 }, &mut effect)?;
        assert_eq!(effect, 147);

        Ok(())
    }
}
