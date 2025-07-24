use windows::{
    core::*, Win32::Foundation::*, Win32::System::Com::*, Win32::System::Ole::*,
    Win32::System::SystemServices::*,
};

#[implement(IDataObject)]
struct DataObject();

impl IDataObject_Impl for DataObject_Impl {
    fn GetData(&self, _: *const FORMATETC) -> Result<STGMEDIUM, HRESULT> {
        unimplemented!()
    }
    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<(), HRESULT> {
        unimplemented!()
    }
    fn QueryGetData(&self, _: *const FORMATETC) -> HRESULT {
        unimplemented!()
    }
    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC, _: *mut FORMATETC) -> HRESULT {
        unimplemented!()
    }
    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> Result<(), HRESULT> {
        unimplemented!()
    }
    fn EnumFormatEtc(&self, _: u32) -> Result<IEnumFORMATETC, HRESULT> {
        unimplemented!()
    }
    fn DAdvise(
        &self,
        format: *const FORMATETC,
        value: u32,
        sink: Ref<IAdviseSink>,
    ) -> Result<u32, HRESULT> {
        assert!(!format.is_null());
        assert_eq!(value, 789);
        assert!(sink.is_null());
        Ok(123)
    }
    fn DUnadvise(&self, _: u32) -> Result<(), HRESULT> {
        unimplemented!()
    }
    fn EnumDAdvise(&self) -> Result<IEnumSTATDATA, HRESULT> {
        unimplemented!()
    }
}

#[implement(IDropTarget)]
struct DropTarget();

impl IDropTarget_Impl for DropTarget_Impl {
    fn DragEnter(
        &self,
        object: Ref<IDataObject>,
        state: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
    ) -> Result<(), HRESULT> {
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
    fn DragOver(
        &self,
        _: MODIFIERKEYS_FLAGS,
        _: &POINTL,
        _: *mut DROPEFFECT,
    ) -> Result<(), HRESULT> {
        unimplemented!()
    }
    fn DragLeave(&self) -> Result<(), HRESULT> {
        Ok(())
    }
    fn Drop(
        &self,
        _: Ref<IDataObject>,
        _: MODIFIERKEYS_FLAGS,
        _: &POINTL,
        _: *mut DROPEFFECT,
    ) -> Result<(), HRESULT> {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<(), HRESULT> {
    unsafe {
        let object: IDataObject = DataObject().into();

        let target: IDropTarget = DropTarget().into();
        target.DragLeave()?;

        let mut effect = DROPEFFECT_LINK;
        target.DragEnter(&object, MK_MBUTTON, POINTL { x: 10, y: 20 }, &mut effect)?;
        assert_eq!(effect, DROPEFFECT_MOVE);

        Ok(())
    }
}
