#![cfg(windows)]
use windows::{Win32::*, core::*};

#[implement(IDataObject)]
struct DataObject();

impl IDataObject_Impl for DataObject_Impl {
    fn GetData(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> HRESULT {
        unimplemented!()
    }
    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> HRESULT {
        unimplemented!()
    }
    fn QueryGetData(&self, _: *const FORMATETC) -> HRESULT {
        unimplemented!()
    }
    fn GetCanonicalFormatEtc(&self, _: *const FORMATETC, _: *mut FORMATETC) -> HRESULT {
        unimplemented!()
    }
    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> HRESULT {
        unimplemented!()
    }
    fn EnumFormatEtc(&self, _: u32) -> Result<IEnumFORMATETC> {
        unimplemented!()
    }
    fn DAdvise(
        &self,
        format: *const FORMATETC,
        value: u32,
        sink: Ref<IAdviseSink>,
        connection: *mut u32,
    ) -> HRESULT {
        assert!(!format.is_null());
        assert_eq!(value, 789);
        assert!(sink.is_null());
        unsafe { *connection = 123 };
        S_OK
    }
    fn DUnadvise(&self, _: u32) -> HRESULT {
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
    ) -> HRESULT {
        unsafe {
            let mut connection = 0;
            let status = object
                .unwrap()
                .DAdvise(&FORMATETC::default(), 789, None, &mut connection);
            if status.is_err() {
                return status;
            }
            assert_eq!(connection, 123);
            assert_eq!(state, MK_MBUTTON as u32);
            assert_eq!(*effect, DROPEFFECT_LINK as u32);
            *effect = DROPEFFECT_MOVE as u32;
            assert_eq!(*point, POINTL { x: 10, y: 20 });
            S_OK
        }
    }
    fn DragOver(&self, _: u32, _: &POINTL, _: *mut u32) -> HRESULT {
        unimplemented!()
    }
    fn DragLeave(&self) -> HRESULT {
        S_OK
    }
    fn Drop(&self, _: Ref<IDataObject>, _: u32, _: &POINTL, _: *mut u32) -> HRESULT {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let object: IDataObject = DataObject().into();

        let target: IDropTarget = DropTarget().into();
        target.DragLeave().ok()?;

        let mut effect = DROPEFFECT_LINK as u32;
        target
            .DragEnter(
                &object,
                MK_MBUTTON as u32,
                POINTL { x: 10, y: 20 },
                &mut effect,
            )
            .ok()?;
        assert_eq!(effect, DROPEFFECT_MOVE as u32);

        Ok(())
    }
}
