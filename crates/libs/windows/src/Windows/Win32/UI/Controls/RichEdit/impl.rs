#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IRichEditOle_Impl: Sized {
    fn GetClientSite(&self) -> ::windows::core::Result<super::super::super::System::Ole::IOleClientSite>;
    fn GetObjectCount(&self) -> i32;
    fn GetLinkCount(&self) -> i32;
    fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::core::Result<()>;
    fn InsertObject(&self, lpreobject: *mut REOBJECT) -> ::windows::core::Result<()>;
    fn ConvertObject(&self, iob: i32, rclsidnew: *const ::windows::core::GUID, lpstrusertypenew: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn ActivateAs(&self, rclsid: *const ::windows::core::GUID, rclsidas: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetHostNames(&self, lpstrcontainerapp: &::windows::core::PCSTR, lpstrcontainerobj: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn SetLinkAvailable(&self, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> ::windows::core::Result<()>;
    fn HandsOffStorage(&self, iob: i32) -> ::windows::core::Result<()>;
    fn SaveCompleted(&self, iob: i32, lpstg: &::core::option::Option<super::super::super::System::Com::StructuredStorage::IStorage>) -> ::windows::core::Result<()>;
    fn InPlaceDeactivate(&self) -> ::windows::core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn ImportDataObject(&self, lpdataobj: &::core::option::Option<super::super::super::System::Com::IDataObject>, cf: u16, hmetapict: isize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IRichEditOle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>() -> IRichEditOle_Vtbl {
        unsafe extern "system" fn GetClientSite<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpolesite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClientSite() {
                ::core::result::Result::Ok(ok__) => {
                    *lplpolesite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectCount<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectCount()
        }
        unsafe extern "system" fn GetLinkCount<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLinkCount()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&iob), ::core::mem::transmute_copy(&lpreobject), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn InsertObject<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpreobject: *mut REOBJECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertObject(::core::mem::transmute_copy(&lpreobject)).into()
        }
        unsafe extern "system" fn ConvertObject<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, rclsidnew: *const ::windows::core::GUID, lpstrusertypenew: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConvertObject(::core::mem::transmute_copy(&iob), ::core::mem::transmute_copy(&rclsidnew), ::core::mem::transmute(&lpstrusertypenew)).into()
        }
        unsafe extern "system" fn ActivateAs<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rclsidas: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ActivateAs(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rclsidas)).into()
        }
        unsafe extern "system" fn SetHostNames<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpstrcontainerapp: ::windows::core::PCSTR, lpstrcontainerobj: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHostNames(::core::mem::transmute(&lpstrcontainerapp), ::core::mem::transmute(&lpstrcontainerobj)).into()
        }
        unsafe extern "system" fn SetLinkAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLinkAvailable(::core::mem::transmute_copy(&iob), ::core::mem::transmute_copy(&favailable)).into()
        }
        unsafe extern "system" fn SetDvaspect<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, dvaspect: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDvaspect(::core::mem::transmute_copy(&iob), ::core::mem::transmute_copy(&dvaspect)).into()
        }
        unsafe extern "system" fn HandsOffStorage<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandsOffStorage(::core::mem::transmute_copy(&iob)).into()
        }
        unsafe extern "system" fn SaveCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, lpstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveCompleted(::core::mem::transmute_copy(&iob), ::core::mem::transmute(&lpstg)).into()
        }
        unsafe extern "system" fn InPlaceDeactivate<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InPlaceDeactivate().into()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContextSensitiveHelp(::core::mem::transmute_copy(&fentermode)).into()
        }
        unsafe extern "system" fn GetClipboardData<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClipboardData(::core::mem::transmute_copy(&lpchrg), ::core::mem::transmute_copy(&reco), ::core::mem::transmute_copy(&lplpdataobj)).into()
        }
        unsafe extern "system" fn ImportDataObject<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, cf: u16, hmetapict: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ImportDataObject(::core::mem::transmute(&lpdataobj), ::core::mem::transmute_copy(&cf), ::core::mem::transmute_copy(&hmetapict)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClientSite: GetClientSite::<Identity, Impl, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, Impl, OFFSET>,
            GetLinkCount: GetLinkCount::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            InsertObject: InsertObject::<Identity, Impl, OFFSET>,
            ConvertObject: ConvertObject::<Identity, Impl, OFFSET>,
            ActivateAs: ActivateAs::<Identity, Impl, OFFSET>,
            SetHostNames: SetHostNames::<Identity, Impl, OFFSET>,
            SetLinkAvailable: SetLinkAvailable::<Identity, Impl, OFFSET>,
            SetDvaspect: SetDvaspect::<Identity, Impl, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            InPlaceDeactivate: InPlaceDeactivate::<Identity, Impl, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, Impl, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, Impl, OFFSET>,
            ImportDataObject: ImportDataObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditOle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IRichEditOleCallback_Impl: Sized {
    fn GetNewStorage(&self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::IStorage>;
    fn GetInPlaceContext(&self, lplpframe: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceFrame>, lplpdoc: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceUIWindow>, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::core::Result<()>;
    fn ShowContainerUI(&self, fshow: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn QueryInsertObject(&self, lpclsid: *mut ::windows::core::GUID, lpstg: &::core::option::Option<super::super::super::System::Com::StructuredStorage::IStorage>, cp: i32) -> ::windows::core::Result<()>;
    fn DeleteObject(&self, lpoleobj: &::core::option::Option<super::super::super::System::Ole::IOleObject>) -> ::windows::core::Result<()>;
    fn QueryAcceptData(&self, lpdataobj: &::core::option::Option<super::super::super::System::Com::IDataObject>, lpcfformat: *mut u16, reco: u32, freally: super::super::super::Foundation::BOOL, hmetapict: isize) -> ::windows::core::Result<()>;
    fn ContextSensitiveHelp(&self, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn GetDragDropEffect(&self, fdrag: super::super::super::Foundation::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::core::Result<()>;
    fn GetContextMenu(&self, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: &::core::option::Option<super::super::super::System::Ole::IOleObject>, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IRichEditOleCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>() -> IRichEditOleCallback_Vtbl {
        unsafe extern "system" fn GetNewStorage<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNewStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *lplpstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInPlaceContext<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpframe: *mut ::windows::core::RawPtr, lplpdoc: *mut ::windows::core::RawPtr, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInPlaceContext(::core::mem::transmute_copy(&lplpframe), ::core::mem::transmute_copy(&lplpdoc), ::core::mem::transmute_copy(&lpframeinfo)).into()
        }
        unsafe extern "system" fn ShowContainerUI<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowContainerUI(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn QueryInsertObject<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID, lpstg: ::windows::core::RawPtr, cp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryInsertObject(::core::mem::transmute_copy(&lpclsid), ::core::mem::transmute(&lpstg), ::core::mem::transmute_copy(&cp)).into()
        }
        unsafe extern "system" fn DeleteObject<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpoleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteObject(::core::mem::transmute(&lpoleobj)).into()
        }
        unsafe extern "system" fn QueryAcceptData<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, lpcfformat: *mut u16, reco: u32, freally: super::super::super::Foundation::BOOL, hmetapict: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryAcceptData(::core::mem::transmute(&lpdataobj), ::core::mem::transmute_copy(&lpcfformat), ::core::mem::transmute_copy(&reco), ::core::mem::transmute_copy(&freally), ::core::mem::transmute_copy(&hmetapict)).into()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ContextSensitiveHelp(::core::mem::transmute_copy(&fentermode)).into()
        }
        unsafe extern "system" fn GetClipboardData<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClipboardData(::core::mem::transmute_copy(&lpchrg), ::core::mem::transmute_copy(&reco), ::core::mem::transmute_copy(&lplpdataobj)).into()
        }
        unsafe extern "system" fn GetDragDropEffect<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdrag: super::super::super::Foundation::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDragDropEffect(::core::mem::transmute_copy(&fdrag), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn GetContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: ::windows::core::RawPtr, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContextMenu(::core::mem::transmute_copy(&seltype), ::core::mem::transmute(&lpoleobj), ::core::mem::transmute_copy(&lpchrg), ::core::mem::transmute_copy(&lphmenu)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetNewStorage: GetNewStorage::<Identity, Impl, OFFSET>,
            GetInPlaceContext: GetInPlaceContext::<Identity, Impl, OFFSET>,
            ShowContainerUI: ShowContainerUI::<Identity, Impl, OFFSET>,
            QueryInsertObject: QueryInsertObject::<Identity, Impl, OFFSET>,
            DeleteObject: DeleteObject::<Identity, Impl, OFFSET>,
            QueryAcceptData: QueryAcceptData::<Identity, Impl, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, Impl, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, Impl, OFFSET>,
            GetDragDropEffect: GetDragDropEffect::<Identity, Impl, OFFSET>,
            GetContextMenu: GetContextMenu::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditOleCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRicheditUiaOverrides_Impl: Sized {
    fn GetPropertyOverrideValue(&self, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRicheditUiaOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditUiaOverrides_Impl, const OFFSET: isize>() -> IRicheditUiaOverrides_Vtbl {
        unsafe extern "system" fn GetPropertyOverrideValue<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditUiaOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyOverrideValue(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pretvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetPropertyOverrideValue: GetPropertyOverrideValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRicheditUiaOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDisplays_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDisplays_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDisplays_Impl, const OFFSET: isize>() -> ITextDisplays_Vtbl {
        Self { base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDisplays as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetSelection(&self) -> ::windows::core::Result<ITextSelection>;
    fn GetStoryCount(&self) -> ::windows::core::Result<i32>;
    fn GetStoryRanges(&self) -> ::windows::core::Result<ITextStoryRanges>;
    fn GetSaved(&self) -> ::windows::core::Result<i32>;
    fn SetSaved(&self, value: tomConstants) -> ::windows::core::Result<()>;
    fn GetDefaultTabStop(&self) -> ::windows::core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()>;
    fn New(&self) -> ::windows::core::Result<()>;
    fn Open(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()>;
    fn Save(&self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()>;
    fn Freeze(&self) -> ::windows::core::Result<i32>;
    fn Unfreeze(&self) -> ::windows::core::Result<i32>;
    fn BeginEditCollection(&self) -> ::windows::core::Result<()>;
    fn EndEditCollection(&self) -> ::windows::core::Result<()>;
    fn Undo(&self, count: i32) -> ::windows::core::Result<i32>;
    fn Redo(&self, count: i32) -> ::windows::core::Result<i32>;
    fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange>;
    fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows::core::Result<ITextRange>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>() -> ITextDocument_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStoryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryRanges<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStoryRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSaved<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSaved() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaved<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSaved(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDefaultTabStop<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultTabStop() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultTabStop(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn New<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).New().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&codepage)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Save(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&codepage)).into()
        }
        unsafe extern "system" fn Freeze<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Freeze() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unfreeze<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Unfreeze() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEditCollection<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginEditCollection().into()
        }
        unsafe extern "system" fn EndEditCollection<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndEditCollection().into()
        }
        unsafe extern "system" fn Undo<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Undo(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Redo<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Redo(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Range<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Range(::core::mem::transmute_copy(&cpactive), ::core::mem::transmute_copy(&cpanchor)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetStoryCount: GetStoryCount::<Identity, Impl, OFFSET>,
            GetStoryRanges: GetStoryRanges::<Identity, Impl, OFFSET>,
            GetSaved: GetSaved::<Identity, Impl, OFFSET>,
            SetSaved: SetSaved::<Identity, Impl, OFFSET>,
            GetDefaultTabStop: GetDefaultTabStop::<Identity, Impl, OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Identity, Impl, OFFSET>,
            New: New::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Freeze: Freeze::<Identity, Impl, OFFSET>,
            Unfreeze: Unfreeze::<Identity, Impl, OFFSET>,
            BeginEditCollection: BeginEditCollection::<Identity, Impl, OFFSET>,
            EndEditCollection: EndEditCollection::<Identity, Impl, OFFSET>,
            Undo: Undo::<Identity, Impl, OFFSET>,
            Redo: Redo::<Identity, Impl, OFFSET>,
            Range: Range::<Identity, Impl, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocument2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ITextDocument_Impl {
    fn GetCaretType(&self) -> ::windows::core::Result<i32>;
    fn SetCaretType(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetDisplays(&self) -> ::windows::core::Result<ITextDisplays>;
    fn GetDocumentFont(&self) -> ::windows::core::Result<ITextFont2>;
    fn SetDocumentFont(&self, pfont: &::core::option::Option<ITextFont2>) -> ::windows::core::Result<()>;
    fn GetDocumentPara(&self) -> ::windows::core::Result<ITextPara2>;
    fn SetDocumentPara(&self, ppara: &::core::option::Option<ITextPara2>) -> ::windows::core::Result<()>;
    fn GetEastAsianFlags(&self) -> ::windows::core::Result<tomConstants>;
    fn GetGenerator(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetIMEInProgress(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetNotificationMode(&self) -> ::windows::core::Result<i32>;
    fn SetNotificationMode(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetSelection2(&self) -> ::windows::core::Result<ITextSelection2>;
    fn GetStoryRanges2(&self) -> ::windows::core::Result<ITextStoryRanges2>;
    fn GetTypographyOptions(&self) -> ::windows::core::Result<i32>;
    fn GetVersion(&self) -> ::windows::core::Result<i32>;
    fn GetWindow(&self) -> ::windows::core::Result<i64>;
    fn AttachMsgFilter(&self, pfilter: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> ::windows::core::Result<()>;
    fn GetCallManager(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetClientRect(&self, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()>;
    fn GetEffectColor(&self, index: i32) -> ::windows::core::Result<i32>;
    fn GetImmContext(&self) -> ::windows::core::Result<i64>;
    fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32>;
    fn GetStrings(&self) -> ::windows::core::Result<ITextStrings>;
    fn Notify(&self, notify: i32) -> ::windows::core::Result<()>;
    fn Range2(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange2>;
    fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> ::windows::core::Result<ITextRange2>;
    fn ReleaseCallManager(&self, pvoid: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ReleaseImmContext(&self, context: i64) -> ::windows::core::Result<()>;
    fn SetEffectColor(&self, index: i32, value: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
    fn SetTypographyOptions(&self, options: i32, mask: i32) -> ::windows::core::Result<()>;
    fn SysBeep(&self) -> ::windows::core::Result<()>;
    fn Update(&self, value: i32) -> ::windows::core::Result<()>;
    fn UpdateWindow(&self) -> ::windows::core::Result<()>;
    fn GetMathProperties(&self) -> ::windows::core::Result<i32>;
    fn SetMathProperties(&self, options: i32, mask: i32) -> ::windows::core::Result<()>;
    fn GetActiveStory(&self) -> ::windows::core::Result<ITextStory>;
    fn SetActiveStory(&self, pstory: &::core::option::Option<ITextStory>) -> ::windows::core::Result<()>;
    fn GetMainStory(&self) -> ::windows::core::Result<ITextStory>;
    fn GetNewStory(&self) -> ::windows::core::Result<ITextStory>;
    fn GetStory(&self, index: i32) -> ::windows::core::Result<ITextStory>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>() -> ITextDocument2_Vtbl {
        unsafe extern "system" fn GetCaretType<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCaretType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCaretType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDisplays<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplays: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDisplays() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisplays = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentFont<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDocumentFont() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentFont<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDocumentFont(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn GetDocumentPara<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDocumentPara() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentPara<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDocumentPara(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn GetEastAsianFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEastAsianFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerator<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGenerator() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIMEInProgress<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIMEInProgress(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNotificationMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNotificationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotificationMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSelection2<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelection2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryRanges2<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStoryRanges2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypographyOptions<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTypographyOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *poptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachMsgFilter<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachMsgFilter(::core::mem::transmute(&pfilter)).into()
        }
        unsafe extern "system" fn CheckTextLimit<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckTextLimit(::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn GetCallManager<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCallManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvoid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientRect<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClientRect(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)).into()
        }
        unsafe extern "system" fn GetEffectColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEffectColor(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmContext<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetImmContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredFont<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPreferredFont(::core::mem::transmute_copy(&cp), ::core::mem::transmute_copy(&charrep), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&curcharrep), ::core::mem::transmute_copy(&curfontsize), ::core::mem::transmute_copy(&pbstr), ::core::mem::transmute_copy(&ppitchandfamily), ::core::mem::transmute_copy(&pnewfontsize)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrings<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStrings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&notify)).into()
        }
        unsafe extern "system" fn Range2<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Range2(::core::mem::transmute_copy(&cpactive), ::core::mem::transmute_copy(&cpanchor)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint2<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromPoint2(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseCallManager(::core::mem::transmute(&pvoid)).into()
        }
        unsafe extern "system" fn ReleaseImmContext<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseImmContext(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn SetEffectColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEffectColor(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetTypographyOptions<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTypographyOptions(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SysBeep<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SysBeep().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UpdateWindow<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateWindow().into()
        }
        unsafe extern "system" fn GetMathProperties<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMathProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *poptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathProperties<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMathProperties(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetActiveStory<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActiveStory() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveStory<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveStory(::core::mem::transmute(&pstory)).into()
        }
        unsafe extern "system" fn GetMainStory<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMainStory() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNewStory<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNewStory() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStory<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStory(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITextDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCaretType: GetCaretType::<Identity, Impl, OFFSET>,
            SetCaretType: SetCaretType::<Identity, Impl, OFFSET>,
            GetDisplays: GetDisplays::<Identity, Impl, OFFSET>,
            GetDocumentFont: GetDocumentFont::<Identity, Impl, OFFSET>,
            SetDocumentFont: SetDocumentFont::<Identity, Impl, OFFSET>,
            GetDocumentPara: GetDocumentPara::<Identity, Impl, OFFSET>,
            SetDocumentPara: SetDocumentPara::<Identity, Impl, OFFSET>,
            GetEastAsianFlags: GetEastAsianFlags::<Identity, Impl, OFFSET>,
            GetGenerator: GetGenerator::<Identity, Impl, OFFSET>,
            SetIMEInProgress: SetIMEInProgress::<Identity, Impl, OFFSET>,
            GetNotificationMode: GetNotificationMode::<Identity, Impl, OFFSET>,
            SetNotificationMode: SetNotificationMode::<Identity, Impl, OFFSET>,
            GetSelection2: GetSelection2::<Identity, Impl, OFFSET>,
            GetStoryRanges2: GetStoryRanges2::<Identity, Impl, OFFSET>,
            GetTypographyOptions: GetTypographyOptions::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            AttachMsgFilter: AttachMsgFilter::<Identity, Impl, OFFSET>,
            CheckTextLimit: CheckTextLimit::<Identity, Impl, OFFSET>,
            GetCallManager: GetCallManager::<Identity, Impl, OFFSET>,
            GetClientRect: GetClientRect::<Identity, Impl, OFFSET>,
            GetEffectColor: GetEffectColor::<Identity, Impl, OFFSET>,
            GetImmContext: GetImmContext::<Identity, Impl, OFFSET>,
            GetPreferredFont: GetPreferredFont::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetStrings: GetStrings::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            Range2: Range2::<Identity, Impl, OFFSET>,
            RangeFromPoint2: RangeFromPoint2::<Identity, Impl, OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Identity, Impl, OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Identity, Impl, OFFSET>,
            SetEffectColor: SetEffectColor::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetTypographyOptions: SetTypographyOptions::<Identity, Impl, OFFSET>,
            SysBeep: SysBeep::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            UpdateWindow: UpdateWindow::<Identity, Impl, OFFSET>,
            GetMathProperties: GetMathProperties::<Identity, Impl, OFFSET>,
            SetMathProperties: SetMathProperties::<Identity, Impl, OFFSET>,
            GetActiveStory: GetActiveStory::<Identity, Impl, OFFSET>,
            SetActiveStory: SetActiveStory::<Identity, Impl, OFFSET>,
            GetMainStory: GetMainStory::<Identity, Impl, OFFSET>,
            GetNewStory: GetNewStory::<Identity, Impl, OFFSET>,
            GetStory: GetStory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITextDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocument2Old_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ITextDocument_Impl {
    fn AttachMsgFilter(&self, pfilter: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetEffectColor(&self, index: i32, cr: u32) -> ::windows::core::Result<()>;
    fn GetEffectColor(&self, index: i32) -> ::windows::core::Result<u32>;
    fn GetCaretType(&self) -> ::windows::core::Result<i32>;
    fn SetCaretType(&self, carettype: i32) -> ::windows::core::Result<()>;
    fn GetImmContext(&self) -> ::windows::core::Result<i64>;
    fn ReleaseImmContext(&self, context: i64) -> ::windows::core::Result<()>;
    fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::Result<()>;
    fn GetNotificationMode(&self) -> ::windows::core::Result<i32>;
    fn SetNotificationMode(&self, mode: i32) -> ::windows::core::Result<()>;
    fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()>;
    fn GetSelection2(&self) -> ::windows::core::Result<ITextSelection>;
    fn GetWindow(&self) -> ::windows::core::Result<i32>;
    fn GetFEFlags(&self) -> ::windows::core::Result<i32>;
    fn UpdateWindow(&self) -> ::windows::core::Result<()>;
    fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> ::windows::core::Result<()>;
    fn IMEInProgress(&self, value: i32) -> ::windows::core::Result<()>;
    fn SysBeep(&self) -> ::windows::core::Result<()>;
    fn Update(&self, mode: i32) -> ::windows::core::Result<()>;
    fn Notify(&self, notify: i32) -> ::windows::core::Result<()>;
    fn GetDocumentFont(&self) -> ::windows::core::Result<ITextFont>;
    fn GetDocumentPara(&self) -> ::windows::core::Result<ITextPara>;
    fn GetCallManager(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ReleaseCallManager(&self, pvoid: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocument2Old_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>() -> ITextDocument2Old_Vtbl {
        unsafe extern "system" fn AttachMsgFilter<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachMsgFilter(::core::mem::transmute(&pfilter)).into()
        }
        unsafe extern "system" fn SetEffectColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, cr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEffectColor(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&cr)).into()
        }
        unsafe extern "system" fn GetEffectColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pcr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEffectColor(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretType<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcarettype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCaretType() {
                ::core::result::Result::Ok(ok__) => {
                    *pcarettype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, carettype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCaretType(::core::mem::transmute_copy(&carettype)).into()
        }
        unsafe extern "system" fn GetImmContext<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetImmContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseImmContext(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetPreferredFont<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPreferredFont(::core::mem::transmute_copy(&cp), ::core::mem::transmute_copy(&charrep), ::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&charrepcur), ::core::mem::transmute_copy(&curfontsize), ::core::mem::transmute_copy(&pbstr), ::core::mem::transmute_copy(&ppitchandfamily), ::core::mem::transmute_copy(&pnewfontsize)).into()
        }
        unsafe extern "system" fn GetNotificationMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNotificationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotificationMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn GetClientRect<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClientRect(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)).into()
        }
        unsafe extern "system" fn GetSelection2<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelection2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFEFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFEFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateWindow<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateWindow().into()
        }
        unsafe extern "system" fn CheckTextLimit<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckTextLimit(::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn IMEInProgress<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IMEInProgress(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SysBeep<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SysBeep().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&notify)).into()
        }
        unsafe extern "system" fn GetDocumentFont<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitextfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDocumentFont() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitextfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentPara<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitextpara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDocumentPara() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitextpara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallManager<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCallManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvoid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Old_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseCallManager(::core::mem::transmute(&pvoid)).into()
        }
        Self {
            base: ITextDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            AttachMsgFilter: AttachMsgFilter::<Identity, Impl, OFFSET>,
            SetEffectColor: SetEffectColor::<Identity, Impl, OFFSET>,
            GetEffectColor: GetEffectColor::<Identity, Impl, OFFSET>,
            GetCaretType: GetCaretType::<Identity, Impl, OFFSET>,
            SetCaretType: SetCaretType::<Identity, Impl, OFFSET>,
            GetImmContext: GetImmContext::<Identity, Impl, OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Identity, Impl, OFFSET>,
            GetPreferredFont: GetPreferredFont::<Identity, Impl, OFFSET>,
            GetNotificationMode: GetNotificationMode::<Identity, Impl, OFFSET>,
            SetNotificationMode: SetNotificationMode::<Identity, Impl, OFFSET>,
            GetClientRect: GetClientRect::<Identity, Impl, OFFSET>,
            GetSelection2: GetSelection2::<Identity, Impl, OFFSET>,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            GetFEFlags: GetFEFlags::<Identity, Impl, OFFSET>,
            UpdateWindow: UpdateWindow::<Identity, Impl, OFFSET>,
            CheckTextLimit: CheckTextLimit::<Identity, Impl, OFFSET>,
            IMEInProgress: IMEInProgress::<Identity, Impl, OFFSET>,
            SysBeep: SysBeep::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            GetDocumentFont: GetDocumentFont::<Identity, Impl, OFFSET>,
            GetDocumentPara: GetDocumentPara::<Identity, Impl, OFFSET>,
            GetCallManager: GetCallManager::<Identity, Impl, OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument2Old as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITextDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextFont_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetDuplicate(&self) -> ::windows::core::Result<ITextFont>;
    fn SetDuplicate(&self, pfont: &::core::option::Option<ITextFont>) -> ::windows::core::Result<()>;
    fn CanChange(&self) -> ::windows::core::Result<i32>;
    fn IsEqual(&self, pfont: &::core::option::Option<ITextFont>) -> ::windows::core::Result<i32>;
    fn Reset(&self, value: tomConstants) -> ::windows::core::Result<()>;
    fn GetStyle(&self) -> ::windows::core::Result<i32>;
    fn SetStyle(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetAllCaps(&self) -> ::windows::core::Result<i32>;
    fn SetAllCaps(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetAnimation(&self) -> ::windows::core::Result<i32>;
    fn SetAnimation(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetBackColor(&self) -> ::windows::core::Result<i32>;
    fn SetBackColor(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetBold(&self) -> ::windows::core::Result<i32>;
    fn SetBold(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetEmboss(&self) -> ::windows::core::Result<i32>;
    fn SetEmboss(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetForeColor(&self) -> ::windows::core::Result<i32>;
    fn SetForeColor(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetHidden(&self) -> ::windows::core::Result<i32>;
    fn SetHidden(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetEngrave(&self) -> ::windows::core::Result<i32>;
    fn SetEngrave(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetItalic(&self) -> ::windows::core::Result<i32>;
    fn SetItalic(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetKerning(&self) -> ::windows::core::Result<f32>;
    fn SetKerning(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetLanguageID(&self) -> ::windows::core::Result<i32>;
    fn SetLanguageID(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetName(&self, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetOutline(&self) -> ::windows::core::Result<i32>;
    fn SetOutline(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetPosition(&self) -> ::windows::core::Result<f32>;
    fn SetPosition(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetProtected(&self) -> ::windows::core::Result<i32>;
    fn SetProtected(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetShadow(&self) -> ::windows::core::Result<i32>;
    fn SetShadow(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetSize(&self) -> ::windows::core::Result<f32>;
    fn SetSize(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetSmallCaps(&self) -> ::windows::core::Result<i32>;
    fn SetSmallCaps(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetSpacing(&self) -> ::windows::core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetStrikeThrough(&self) -> ::windows::core::Result<i32>;
    fn SetStrikeThrough(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetSubscript(&self) -> ::windows::core::Result<i32>;
    fn SetSubscript(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetSuperscript(&self) -> ::windows::core::Result<i32>;
    fn SetSuperscript(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetUnderline(&self) -> ::windows::core::Result<i32>;
    fn SetUnderline(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetWeight(&self) -> ::windows::core::Result<i32>;
    fn SetWeight(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextFont_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>() -> ITextFont_Vtbl {
        unsafe extern "system" fn GetDuplicate<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuplicate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuplicate(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn CanChange<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&pfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStyle(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAllCaps<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllCaps<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllCaps(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAnimation<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimation<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAnimation(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBackColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackColor(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBold<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBold() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBold(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEmboss<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEmboss() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmboss<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEmboss(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetForeColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetForeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForeColor(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHidden<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHidden(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEngrave<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEngrave() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEngrave<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEngrave(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetItalic<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItalic() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetItalic(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKerning<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKerning() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKerning(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLanguageID<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLanguageID() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageID<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLanguageID(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn GetOutline<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutline() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutline<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetProtected<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtected<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProtected(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetShadow<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetShadow() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShadow<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShadow(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSmallCaps<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSmallCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallCaps<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSmallCaps(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpacing<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpacing<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSpacing(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStrikeThrough<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStrikeThrough() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikeThrough<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStrikeThrough(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSubscript<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubscript() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubscript(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSuperscript<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSuperscript() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSuperscript(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnderline<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUnderline() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnderline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetWeight<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWeight(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDuplicate: GetDuplicate::<Identity, Impl, OFFSET>,
            SetDuplicate: SetDuplicate::<Identity, Impl, OFFSET>,
            CanChange: CanChange::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetStyle: GetStyle::<Identity, Impl, OFFSET>,
            SetStyle: SetStyle::<Identity, Impl, OFFSET>,
            GetAllCaps: GetAllCaps::<Identity, Impl, OFFSET>,
            SetAllCaps: SetAllCaps::<Identity, Impl, OFFSET>,
            GetAnimation: GetAnimation::<Identity, Impl, OFFSET>,
            SetAnimation: SetAnimation::<Identity, Impl, OFFSET>,
            GetBackColor: GetBackColor::<Identity, Impl, OFFSET>,
            SetBackColor: SetBackColor::<Identity, Impl, OFFSET>,
            GetBold: GetBold::<Identity, Impl, OFFSET>,
            SetBold: SetBold::<Identity, Impl, OFFSET>,
            GetEmboss: GetEmboss::<Identity, Impl, OFFSET>,
            SetEmboss: SetEmboss::<Identity, Impl, OFFSET>,
            GetForeColor: GetForeColor::<Identity, Impl, OFFSET>,
            SetForeColor: SetForeColor::<Identity, Impl, OFFSET>,
            GetHidden: GetHidden::<Identity, Impl, OFFSET>,
            SetHidden: SetHidden::<Identity, Impl, OFFSET>,
            GetEngrave: GetEngrave::<Identity, Impl, OFFSET>,
            SetEngrave: SetEngrave::<Identity, Impl, OFFSET>,
            GetItalic: GetItalic::<Identity, Impl, OFFSET>,
            SetItalic: SetItalic::<Identity, Impl, OFFSET>,
            GetKerning: GetKerning::<Identity, Impl, OFFSET>,
            SetKerning: SetKerning::<Identity, Impl, OFFSET>,
            GetLanguageID: GetLanguageID::<Identity, Impl, OFFSET>,
            SetLanguageID: SetLanguageID::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetOutline: GetOutline::<Identity, Impl, OFFSET>,
            SetOutline: SetOutline::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            GetProtected: GetProtected::<Identity, Impl, OFFSET>,
            SetProtected: SetProtected::<Identity, Impl, OFFSET>,
            GetShadow: GetShadow::<Identity, Impl, OFFSET>,
            SetShadow: SetShadow::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            GetSmallCaps: GetSmallCaps::<Identity, Impl, OFFSET>,
            SetSmallCaps: SetSmallCaps::<Identity, Impl, OFFSET>,
            GetSpacing: GetSpacing::<Identity, Impl, OFFSET>,
            SetSpacing: SetSpacing::<Identity, Impl, OFFSET>,
            GetStrikeThrough: GetStrikeThrough::<Identity, Impl, OFFSET>,
            SetStrikeThrough: SetStrikeThrough::<Identity, Impl, OFFSET>,
            GetSubscript: GetSubscript::<Identity, Impl, OFFSET>,
            SetSubscript: SetSubscript::<Identity, Impl, OFFSET>,
            GetSuperscript: GetSuperscript::<Identity, Impl, OFFSET>,
            SetSuperscript: SetSuperscript::<Identity, Impl, OFFSET>,
            GetUnderline: GetUnderline::<Identity, Impl, OFFSET>,
            SetUnderline: SetUnderline::<Identity, Impl, OFFSET>,
            GetWeight: GetWeight::<Identity, Impl, OFFSET>,
            SetWeight: SetWeight::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextFont as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextFont2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ITextFont_Impl {
    fn GetCount(&self) -> ::windows::core::Result<i32>;
    fn GetAutoLigatures(&self) -> ::windows::core::Result<i32>;
    fn SetAutoLigatures(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetAutospaceAlpha(&self) -> ::windows::core::Result<i32>;
    fn SetAutospaceAlpha(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetAutospaceNumeric(&self) -> ::windows::core::Result<i32>;
    fn SetAutospaceNumeric(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetAutospaceParens(&self) -> ::windows::core::Result<i32>;
    fn SetAutospaceParens(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCharRep(&self) -> ::windows::core::Result<i32>;
    fn SetCharRep(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCompressionMode(&self) -> ::windows::core::Result<i32>;
    fn SetCompressionMode(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCookie(&self) -> ::windows::core::Result<i32>;
    fn SetCookie(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetDoubleStrike(&self) -> ::windows::core::Result<i32>;
    fn SetDoubleStrike(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetDuplicate2(&self) -> ::windows::core::Result<ITextFont2>;
    fn SetDuplicate2(&self, pfont: &::core::option::Option<ITextFont2>) -> ::windows::core::Result<()>;
    fn GetLinkType(&self) -> ::windows::core::Result<i32>;
    fn GetMathZone(&self) -> ::windows::core::Result<i32>;
    fn SetMathZone(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetModWidthPairs(&self) -> ::windows::core::Result<i32>;
    fn SetModWidthPairs(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetModWidthSpace(&self) -> ::windows::core::Result<i32>;
    fn SetModWidthSpace(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetOldNumbers(&self) -> ::windows::core::Result<i32>;
    fn SetOldNumbers(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetOverlapping(&self) -> ::windows::core::Result<i32>;
    fn SetOverlapping(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetPositionSubSuper(&self) -> ::windows::core::Result<i32>;
    fn SetPositionSubSuper(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetScaling(&self) -> ::windows::core::Result<i32>;
    fn SetScaling(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetSpaceExtension(&self) -> ::windows::core::Result<f32>;
    fn SetSpaceExtension(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetUnderlinePositionMode(&self) -> ::windows::core::Result<i32>;
    fn SetUnderlinePositionMode(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()>;
    fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32>;
    fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::core::Result<()>;
    fn IsEqual2(&self, pfont: &::core::option::Option<ITextFont2>) -> ::windows::core::Result<i32>;
    fn SetEffects(&self, value: i32, mask: i32) -> ::windows::core::Result<()>;
    fn SetEffects2(&self, value: i32, mask: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextFont2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>() -> ITextFont2_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoLigatures<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAutoLigatures() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoLigatures<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoLigatures(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceAlpha<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAutospaceAlpha() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceAlpha<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutospaceAlpha(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceNumeric<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAutospaceNumeric() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceNumeric<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutospaceNumeric(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceParens<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAutospaceParens() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceParens<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutospaceParens(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCharRep<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCharRep() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharRep<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCharRep(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCompressionMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCompressionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompressionMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompressionMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCookie<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCookie<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCookie(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDoubleStrike<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDoubleStrike() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoubleStrike<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDoubleStrike(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuplicate2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate2<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuplicate2(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn GetLinkType<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLinkType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMathZone<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMathZone() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathZone<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMathZone(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetModWidthPairs<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetModWidthPairs() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModWidthPairs<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetModWidthPairs(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetModWidthSpace<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetModWidthSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModWidthSpace<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetModWidthSpace(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetOldNumbers<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOldNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOldNumbers<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOldNumbers(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetOverlapping<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOverlapping() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlapping<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOverlapping(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPositionSubSuper<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPositionSubSuper() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionSubSuper<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPositionSubSuper(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetScaling<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetScaling() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaling<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaling(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpaceExtension<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSpaceExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceExtension<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSpaceExtension(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnderlinePositionMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUnderlinePositionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderlinePositionMode<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnderlinePositionMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEffects<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffects(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetEffects2<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffects2(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyInfo(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn IsEqual2<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual2(::core::mem::transmute(&pfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *pb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEffects(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetEffects2<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEffects2(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ITextFont_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAutoLigatures: GetAutoLigatures::<Identity, Impl, OFFSET>,
            SetAutoLigatures: SetAutoLigatures::<Identity, Impl, OFFSET>,
            GetAutospaceAlpha: GetAutospaceAlpha::<Identity, Impl, OFFSET>,
            SetAutospaceAlpha: SetAutospaceAlpha::<Identity, Impl, OFFSET>,
            GetAutospaceNumeric: GetAutospaceNumeric::<Identity, Impl, OFFSET>,
            SetAutospaceNumeric: SetAutospaceNumeric::<Identity, Impl, OFFSET>,
            GetAutospaceParens: GetAutospaceParens::<Identity, Impl, OFFSET>,
            SetAutospaceParens: SetAutospaceParens::<Identity, Impl, OFFSET>,
            GetCharRep: GetCharRep::<Identity, Impl, OFFSET>,
            SetCharRep: SetCharRep::<Identity, Impl, OFFSET>,
            GetCompressionMode: GetCompressionMode::<Identity, Impl, OFFSET>,
            SetCompressionMode: SetCompressionMode::<Identity, Impl, OFFSET>,
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            SetCookie: SetCookie::<Identity, Impl, OFFSET>,
            GetDoubleStrike: GetDoubleStrike::<Identity, Impl, OFFSET>,
            SetDoubleStrike: SetDoubleStrike::<Identity, Impl, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, Impl, OFFSET>,
            SetDuplicate2: SetDuplicate2::<Identity, Impl, OFFSET>,
            GetLinkType: GetLinkType::<Identity, Impl, OFFSET>,
            GetMathZone: GetMathZone::<Identity, Impl, OFFSET>,
            SetMathZone: SetMathZone::<Identity, Impl, OFFSET>,
            GetModWidthPairs: GetModWidthPairs::<Identity, Impl, OFFSET>,
            SetModWidthPairs: SetModWidthPairs::<Identity, Impl, OFFSET>,
            GetModWidthSpace: GetModWidthSpace::<Identity, Impl, OFFSET>,
            SetModWidthSpace: SetModWidthSpace::<Identity, Impl, OFFSET>,
            GetOldNumbers: GetOldNumbers::<Identity, Impl, OFFSET>,
            SetOldNumbers: SetOldNumbers::<Identity, Impl, OFFSET>,
            GetOverlapping: GetOverlapping::<Identity, Impl, OFFSET>,
            SetOverlapping: SetOverlapping::<Identity, Impl, OFFSET>,
            GetPositionSubSuper: GetPositionSubSuper::<Identity, Impl, OFFSET>,
            SetPositionSubSuper: SetPositionSubSuper::<Identity, Impl, OFFSET>,
            GetScaling: GetScaling::<Identity, Impl, OFFSET>,
            SetScaling: SetScaling::<Identity, Impl, OFFSET>,
            GetSpaceExtension: GetSpaceExtension::<Identity, Impl, OFFSET>,
            SetSpaceExtension: SetSpaceExtension::<Identity, Impl, OFFSET>,
            GetUnderlinePositionMode: GetUnderlinePositionMode::<Identity, Impl, OFFSET>,
            SetUnderlinePositionMode: SetUnderlinePositionMode::<Identity, Impl, OFFSET>,
            GetEffects: GetEffects::<Identity, Impl, OFFSET>,
            GetEffects2: GetEffects2::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            IsEqual2: IsEqual2::<Identity, Impl, OFFSET>,
            SetEffects: SetEffects::<Identity, Impl, OFFSET>,
            SetEffects2: SetEffects2::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextFont2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITextFont as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHost_Impl: Sized {
    fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC;
    fn TxReleaseDC(&self, hdc: super::super::super::Graphics::Gdi::HDC) -> i32;
    fn TxShowScrollBar(&self, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL;
    fn TxSetScrollRange(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxSetScrollPos(&self, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxInvalidateRect(&self, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL);
    fn TxViewChange(&self, fupdate: super::super::super::Foundation::BOOL);
    fn TxCreateCaret(&self, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL;
    fn TxShowCaret(&self, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL;
    fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL;
    fn TxKillTimer(&self, idtimer: u32);
    fn TxScrollWindowEx(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD);
    fn TxSetCapture(&self, fcapture: super::super::super::Foundation::BOOL);
    fn TxSetFocus(&self);
    fn TxSetCursor(&self, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL);
    fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    fn TxActivate(&self, ploldstate: *mut i32) -> ::windows::core::Result<()>;
    fn TxDeactivate(&self, lnewstate: i32) -> ::windows::core::Result<()>;
    fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> ::windows::core::Result<()>;
    fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> ::windows::core::Result<()>;
    fn TxGetSysColor(&self, nindex: i32) -> u32;
    fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::Result<()>;
    fn TxGetMaxLength(&self, plength: *mut u32) -> ::windows::core::Result<()>;
    fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> ::windows::core::Result<()>;
    fn TxGetPasswordChar(&self) -> ::windows::core::Result<i8>;
    fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> ::windows::core::Result<()>;
    fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> ::windows::core::Result<()>;
    fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> ::windows::core::Result<()>;
    fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::Result<()>;
    fn TxNotify(&self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC;
    fn TxImmReleaseContext(&self, himc: super::super::super::Globalization::HIMC);
    fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>() -> ITextHost_Vtbl {
        unsafe extern "system" fn TxGetDC<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetDC()
        }
        unsafe extern "system" fn TxReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxReleaseDC(::core::mem::transmute_copy(&hdc))
        }
        unsafe extern "system" fn TxShowScrollBar<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxShowScrollBar(::core::mem::transmute_copy(&fnbar), ::core::mem::transmute_copy(&fshow))
        }
        unsafe extern "system" fn TxEnableScrollBar<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxEnableScrollBar(::core::mem::transmute_copy(&fusbflags), ::core::mem::transmute_copy(&fuarrowflags))
        }
        unsafe extern "system" fn TxSetScrollRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetScrollRange(::core::mem::transmute_copy(&fnbar), ::core::mem::transmute_copy(&nminpos), ::core::mem::transmute_copy(&nmaxpos), ::core::mem::transmute_copy(&fredraw))
        }
        unsafe extern "system" fn TxSetScrollPos<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetScrollPos(::core::mem::transmute_copy(&fnbar), ::core::mem::transmute_copy(&npos), ::core::mem::transmute_copy(&fredraw))
        }
        unsafe extern "system" fn TxInvalidateRect<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxInvalidateRect(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&fmode))
        }
        unsafe extern "system" fn TxViewChange<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fupdate: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxViewChange(::core::mem::transmute_copy(&fupdate))
        }
        unsafe extern "system" fn TxCreateCaret<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxCreateCaret(::core::mem::transmute_copy(&hbmp), ::core::mem::transmute_copy(&xwidth), ::core::mem::transmute_copy(&yheight))
        }
        unsafe extern "system" fn TxShowCaret<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxShowCaret(::core::mem::transmute_copy(&fshow))
        }
        unsafe extern "system" fn TxSetCaretPos<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetCaretPos(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y))
        }
        unsafe extern "system" fn TxSetTimer<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetTimer(::core::mem::transmute_copy(&idtimer), ::core::mem::transmute_copy(&utimeout))
        }
        unsafe extern "system" fn TxKillTimer<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idtimer: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxKillTimer(::core::mem::transmute_copy(&idtimer))
        }
        unsafe extern "system" fn TxScrollWindowEx<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxScrollWindowEx(::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&lprcscroll), ::core::mem::transmute_copy(&lprcclip), ::core::mem::transmute_copy(&hrgnupdate), ::core::mem::transmute_copy(&lprcupdate), ::core::mem::transmute_copy(&fuscroll))
        }
        unsafe extern "system" fn TxSetCapture<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetCapture(::core::mem::transmute_copy(&fcapture))
        }
        unsafe extern "system" fn TxSetFocus<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetFocus()
        }
        unsafe extern "system" fn TxSetCursor<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetCursor(::core::mem::transmute_copy(&hcur), ::core::mem::transmute_copy(&ftext))
        }
        unsafe extern "system" fn TxScreenToClient<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxScreenToClient(::core::mem::transmute_copy(&lppt))
        }
        unsafe extern "system" fn TxClientToScreen<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxClientToScreen(::core::mem::transmute_copy(&lppt))
        }
        unsafe extern "system" fn TxActivate<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploldstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxActivate(::core::mem::transmute_copy(&ploldstate)).into()
        }
        unsafe extern "system" fn TxDeactivate<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewstate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxDeactivate(::core::mem::transmute_copy(&lnewstate)).into()
        }
        unsafe extern "system" fn TxGetClientRect<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetClientRect(::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxGetViewInset<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetViewInset(::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxGetCharFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetCharFormat(::core::mem::transmute_copy(&ppcf)).into()
        }
        unsafe extern "system" fn TxGetParaFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppf: *const *const PARAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetParaFormat(::core::mem::transmute_copy(&pppf)).into()
        }
        unsafe extern "system" fn TxGetSysColor<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetSysColor(::core::mem::transmute_copy(&nindex))
        }
        unsafe extern "system" fn TxGetBackStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetBackStyle(::core::mem::transmute_copy(&pstyle)).into()
        }
        unsafe extern "system" fn TxGetMaxLength<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetMaxLength(::core::mem::transmute_copy(&plength)).into()
        }
        unsafe extern "system" fn TxGetScrollBars<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscrollbar: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetScrollBars(::core::mem::transmute_copy(&pdwscrollbar)).into()
        }
        unsafe extern "system" fn TxGetPasswordChar<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pch: *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TxGetPasswordChar() {
                ::core::result::Result::Ok(ok__) => {
                    *pch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetAcceleratorPos<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetAcceleratorPos(::core::mem::transmute_copy(&pcp)).into()
        }
        unsafe extern "system" fn TxGetExtent<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetExtent(::core::mem::transmute_copy(&lpextent)).into()
        }
        unsafe extern "system" fn OnTxCharFormatChange<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: *const CHARFORMATW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTxCharFormatChange(::core::mem::transmute_copy(&pcf)).into()
        }
        unsafe extern "system" fn OnTxParaFormatChange<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppf: *const PARAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTxParaFormatChange(::core::mem::transmute_copy(&ppf)).into()
        }
        unsafe extern "system" fn TxGetPropertyBits<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetPropertyBits(::core::mem::transmute_copy(&dwmask), ::core::mem::transmute_copy(&pdwbits)).into()
        }
        unsafe extern "system" fn TxNotify<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxNotify(::core::mem::transmute_copy(&inotify), ::core::mem::transmute_copy(&pv)).into()
        }
        unsafe extern "system" fn TxImmGetContext<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Globalization::HIMC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxImmGetContext()
        }
        unsafe extern "system" fn TxImmReleaseContext<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxImmReleaseContext(::core::mem::transmute_copy(&himc))
        }
        unsafe extern "system" fn TxGetSelectionBarWidth<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lselbarwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetSelectionBarWidth(::core::mem::transmute_copy(&lselbarwidth)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TxGetDC: TxGetDC::<Identity, Impl, OFFSET>,
            TxReleaseDC: TxReleaseDC::<Identity, Impl, OFFSET>,
            TxShowScrollBar: TxShowScrollBar::<Identity, Impl, OFFSET>,
            TxEnableScrollBar: TxEnableScrollBar::<Identity, Impl, OFFSET>,
            TxSetScrollRange: TxSetScrollRange::<Identity, Impl, OFFSET>,
            TxSetScrollPos: TxSetScrollPos::<Identity, Impl, OFFSET>,
            TxInvalidateRect: TxInvalidateRect::<Identity, Impl, OFFSET>,
            TxViewChange: TxViewChange::<Identity, Impl, OFFSET>,
            TxCreateCaret: TxCreateCaret::<Identity, Impl, OFFSET>,
            TxShowCaret: TxShowCaret::<Identity, Impl, OFFSET>,
            TxSetCaretPos: TxSetCaretPos::<Identity, Impl, OFFSET>,
            TxSetTimer: TxSetTimer::<Identity, Impl, OFFSET>,
            TxKillTimer: TxKillTimer::<Identity, Impl, OFFSET>,
            TxScrollWindowEx: TxScrollWindowEx::<Identity, Impl, OFFSET>,
            TxSetCapture: TxSetCapture::<Identity, Impl, OFFSET>,
            TxSetFocus: TxSetFocus::<Identity, Impl, OFFSET>,
            TxSetCursor: TxSetCursor::<Identity, Impl, OFFSET>,
            TxScreenToClient: TxScreenToClient::<Identity, Impl, OFFSET>,
            TxClientToScreen: TxClientToScreen::<Identity, Impl, OFFSET>,
            TxActivate: TxActivate::<Identity, Impl, OFFSET>,
            TxDeactivate: TxDeactivate::<Identity, Impl, OFFSET>,
            TxGetClientRect: TxGetClientRect::<Identity, Impl, OFFSET>,
            TxGetViewInset: TxGetViewInset::<Identity, Impl, OFFSET>,
            TxGetCharFormat: TxGetCharFormat::<Identity, Impl, OFFSET>,
            TxGetParaFormat: TxGetParaFormat::<Identity, Impl, OFFSET>,
            TxGetSysColor: TxGetSysColor::<Identity, Impl, OFFSET>,
            TxGetBackStyle: TxGetBackStyle::<Identity, Impl, OFFSET>,
            TxGetMaxLength: TxGetMaxLength::<Identity, Impl, OFFSET>,
            TxGetScrollBars: TxGetScrollBars::<Identity, Impl, OFFSET>,
            TxGetPasswordChar: TxGetPasswordChar::<Identity, Impl, OFFSET>,
            TxGetAcceleratorPos: TxGetAcceleratorPos::<Identity, Impl, OFFSET>,
            TxGetExtent: TxGetExtent::<Identity, Impl, OFFSET>,
            OnTxCharFormatChange: OnTxCharFormatChange::<Identity, Impl, OFFSET>,
            OnTxParaFormatChange: OnTxParaFormatChange::<Identity, Impl, OFFSET>,
            TxGetPropertyBits: TxGetPropertyBits::<Identity, Impl, OFFSET>,
            TxNotify: TxNotify::<Identity, Impl, OFFSET>,
            TxImmGetContext: TxImmGetContext::<Identity, Impl, OFFSET>,
            TxImmReleaseContext: TxImmReleaseContext::<Identity, Impl, OFFSET>,
            TxGetSelectionBarWidth: TxGetSelectionBarWidth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHost2_Impl: Sized + ITextHost_Impl {
    fn TxIsDoubleClickPending(&self) -> super::super::super::Foundation::BOOL;
    fn TxGetWindow(&self, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn TxSetForegroundWindow(&self) -> ::windows::core::Result<()>;
    fn TxGetPalette(&self) -> super::super::super::Graphics::Gdi::HPALETTE;
    fn TxGetEastAsianFlags(&self, pflags: *mut i32) -> ::windows::core::Result<()>;
    fn TxSetCursor2(&self, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR;
    fn TxFreeTextServicesNotification(&self);
    fn TxGetEditStyle(&self, dwitem: u32, pdwdata: *mut u32) -> ::windows::core::Result<()>;
    fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::core::Result<()>;
    fn TxShowDropCaret(&self, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn TxDestroyCaret(&self) -> ::windows::core::Result<()>;
    fn TxGetHorzExtent(&self, plhorzextent: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHost2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>() -> ITextHost2_Vtbl {
        unsafe extern "system" fn TxIsDoubleClickPending<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxIsDoubleClickPending()
        }
        unsafe extern "system" fn TxGetWindow<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetWindow(::core::mem::transmute_copy(&phwnd)).into()
        }
        unsafe extern "system" fn TxSetForegroundWindow<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetForegroundWindow().into()
        }
        unsafe extern "system" fn TxGetPalette<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetPalette()
        }
        unsafe extern "system" fn TxGetEastAsianFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetEastAsianFlags(::core::mem::transmute_copy(&pflags)).into()
        }
        unsafe extern "system" fn TxSetCursor2<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetCursor2(::core::mem::transmute_copy(&hcur), ::core::mem::transmute_copy(&btext))
        }
        unsafe extern "system" fn TxFreeTextServicesNotification<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxFreeTextServicesNotification()
        }
        unsafe extern "system" fn TxGetEditStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetEditStyle(::core::mem::transmute_copy(&dwitem), ::core::mem::transmute_copy(&pdwdata)).into()
        }
        unsafe extern "system" fn TxGetWindowStyles<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetWindowStyles(::core::mem::transmute_copy(&pdwstyle), ::core::mem::transmute_copy(&pdwexstyle)).into()
        }
        unsafe extern "system" fn TxShowDropCaret<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxShowDropCaret(::core::mem::transmute_copy(&fshow), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxDestroyCaret<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxDestroyCaret().into()
        }
        unsafe extern "system" fn TxGetHorzExtent<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhorzextent: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetHorzExtent(::core::mem::transmute_copy(&plhorzextent)).into()
        }
        Self {
            base: ITextHost_Vtbl::new::<Identity, Impl, OFFSET>(),
            TxIsDoubleClickPending: TxIsDoubleClickPending::<Identity, Impl, OFFSET>,
            TxGetWindow: TxGetWindow::<Identity, Impl, OFFSET>,
            TxSetForegroundWindow: TxSetForegroundWindow::<Identity, Impl, OFFSET>,
            TxGetPalette: TxGetPalette::<Identity, Impl, OFFSET>,
            TxGetEastAsianFlags: TxGetEastAsianFlags::<Identity, Impl, OFFSET>,
            TxSetCursor2: TxSetCursor2::<Identity, Impl, OFFSET>,
            TxFreeTextServicesNotification: TxFreeTextServicesNotification::<Identity, Impl, OFFSET>,
            TxGetEditStyle: TxGetEditStyle::<Identity, Impl, OFFSET>,
            TxGetWindowStyles: TxGetWindowStyles::<Identity, Impl, OFFSET>,
            TxShowDropCaret: TxShowDropCaret::<Identity, Impl, OFFSET>,
            TxDestroyCaret: TxDestroyCaret::<Identity, Impl, OFFSET>,
            TxGetHorzExtent: TxGetHorzExtent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextHost2 as ::windows::core::Interface>::IID || iid == &<ITextHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextPara_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetDuplicate(&self) -> ::windows::core::Result<ITextPara>;
    fn SetDuplicate(&self, ppara: &::core::option::Option<ITextPara>) -> ::windows::core::Result<()>;
    fn CanChange(&self) -> ::windows::core::Result<i32>;
    fn IsEqual(&self, ppara: &::core::option::Option<ITextPara>) -> ::windows::core::Result<i32>;
    fn Reset(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetStyle(&self) -> ::windows::core::Result<i32>;
    fn SetStyle(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetAlignment(&self) -> ::windows::core::Result<i32>;
    fn SetAlignment(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetHyphenation(&self) -> ::windows::core::Result<tomConstants>;
    fn SetHyphenation(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetFirstLineIndent(&self) -> ::windows::core::Result<f32>;
    fn GetKeepTogether(&self) -> ::windows::core::Result<tomConstants>;
    fn SetKeepTogether(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetKeepWithNext(&self) -> ::windows::core::Result<tomConstants>;
    fn SetKeepWithNext(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetLeftIndent(&self) -> ::windows::core::Result<f32>;
    fn GetLineSpacing(&self) -> ::windows::core::Result<f32>;
    fn GetLineSpacingRule(&self) -> ::windows::core::Result<i32>;
    fn GetListAlignment(&self) -> ::windows::core::Result<i32>;
    fn SetListAlignment(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetListLevelIndex(&self) -> ::windows::core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetListStart(&self) -> ::windows::core::Result<i32>;
    fn SetListStart(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetListTab(&self) -> ::windows::core::Result<f32>;
    fn SetListTab(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetListType(&self) -> ::windows::core::Result<i32>;
    fn SetListType(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetNoLineNumber(&self) -> ::windows::core::Result<i32>;
    fn SetNoLineNumber(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetPageBreakBefore(&self) -> ::windows::core::Result<i32>;
    fn SetPageBreakBefore(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetRightIndent(&self) -> ::windows::core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> ::windows::core::Result<()>;
    fn SetIndents(&self, first: f32, left: f32, right: f32) -> ::windows::core::Result<()>;
    fn SetLineSpacing(&self, rule: i32, spacing: f32) -> ::windows::core::Result<()>;
    fn GetSpaceAfter(&self) -> ::windows::core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetSpaceBefore(&self) -> ::windows::core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetWidowControl(&self) -> ::windows::core::Result<i32>;
    fn SetWidowControl(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetTabCount(&self) -> ::windows::core::Result<i32>;
    fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::Result<()>;
    fn ClearAllTabs(&self) -> ::windows::core::Result<()>;
    fn DeleteTab(&self, tbpos: f32) -> ::windows::core::Result<()>;
    fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextPara_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>() -> ITextPara_Vtbl {
        unsafe extern "system" fn GetDuplicate<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuplicate() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuplicate(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn CanChange<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&ppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStyle(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHyphenation<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHyphenation() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyphenation<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHyphenation(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFirstLineIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFirstLineIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeepTogether<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKeepTogether() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeepTogether(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepWithNext<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKeepWithNext() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeepWithNext(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLeftIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLeftIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacing<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacingRule<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLineSpacingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetListAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListLevelIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetListLevelIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListLevelIndex(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetListStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListStart(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetListTab() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListTab(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListType<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetListType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListType<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetListType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNoLineNumber<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNoLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNoLineNumber(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPageBreakBefore<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPageBreakBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPageBreakBefore(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetRightIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRightIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRightIndent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetIndents<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: f32, left: f32, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndents(::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: i32, spacing: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLineSpacing(::core::mem::transmute_copy(&rule), ::core::mem::transmute_copy(&spacing)).into()
        }
        unsafe extern "system" fn GetSpaceAfter<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSpaceAfter() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSpaceAfter(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpaceBefore<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSpaceBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSpaceBefore(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetWidowControl<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWidowControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidowControl<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWidowControl(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTabCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTabCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTab(::core::mem::transmute_copy(&tbpos), ::core::mem::transmute_copy(&tbalign), ::core::mem::transmute_copy(&tbleader)).into()
        }
        unsafe extern "system" fn ClearAllTabs<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearAllTabs().into()
        }
        unsafe extern "system" fn DeleteTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tbpos: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTab(::core::mem::transmute_copy(&tbpos)).into()
        }
        unsafe extern "system" fn GetTab<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTab(::core::mem::transmute_copy(&itab), ::core::mem::transmute_copy(&ptbpos), ::core::mem::transmute_copy(&ptbalign), ::core::mem::transmute_copy(&ptbleader)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDuplicate: GetDuplicate::<Identity, Impl, OFFSET>,
            SetDuplicate: SetDuplicate::<Identity, Impl, OFFSET>,
            CanChange: CanChange::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetStyle: GetStyle::<Identity, Impl, OFFSET>,
            SetStyle: SetStyle::<Identity, Impl, OFFSET>,
            GetAlignment: GetAlignment::<Identity, Impl, OFFSET>,
            SetAlignment: SetAlignment::<Identity, Impl, OFFSET>,
            GetHyphenation: GetHyphenation::<Identity, Impl, OFFSET>,
            SetHyphenation: SetHyphenation::<Identity, Impl, OFFSET>,
            GetFirstLineIndent: GetFirstLineIndent::<Identity, Impl, OFFSET>,
            GetKeepTogether: GetKeepTogether::<Identity, Impl, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, Impl, OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Identity, Impl, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, Impl, OFFSET>,
            GetLeftIndent: GetLeftIndent::<Identity, Impl, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, Impl, OFFSET>,
            GetLineSpacingRule: GetLineSpacingRule::<Identity, Impl, OFFSET>,
            GetListAlignment: GetListAlignment::<Identity, Impl, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, Impl, OFFSET>,
            GetListLevelIndex: GetListLevelIndex::<Identity, Impl, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, Impl, OFFSET>,
            GetListStart: GetListStart::<Identity, Impl, OFFSET>,
            SetListStart: SetListStart::<Identity, Impl, OFFSET>,
            GetListTab: GetListTab::<Identity, Impl, OFFSET>,
            SetListTab: SetListTab::<Identity, Impl, OFFSET>,
            GetListType: GetListType::<Identity, Impl, OFFSET>,
            SetListType: SetListType::<Identity, Impl, OFFSET>,
            GetNoLineNumber: GetNoLineNumber::<Identity, Impl, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, Impl, OFFSET>,
            GetPageBreakBefore: GetPageBreakBefore::<Identity, Impl, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, Impl, OFFSET>,
            GetRightIndent: GetRightIndent::<Identity, Impl, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, Impl, OFFSET>,
            SetIndents: SetIndents::<Identity, Impl, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, Impl, OFFSET>,
            GetSpaceAfter: GetSpaceAfter::<Identity, Impl, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, Impl, OFFSET>,
            GetSpaceBefore: GetSpaceBefore::<Identity, Impl, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, Impl, OFFSET>,
            GetWidowControl: GetWidowControl::<Identity, Impl, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, Impl, OFFSET>,
            GetTabCount: GetTabCount::<Identity, Impl, OFFSET>,
            AddTab: AddTab::<Identity, Impl, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, Impl, OFFSET>,
            DeleteTab: DeleteTab::<Identity, Impl, OFFSET>,
            GetTab: GetTab::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPara as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextPara2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ITextPara_Impl {
    fn GetBorders(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetDuplicate2(&self) -> ::windows::core::Result<ITextPara2>;
    fn SetDuplicate2(&self, ppara: &::core::option::Option<ITextPara2>) -> ::windows::core::Result<()>;
    fn GetFontAlignment(&self) -> ::windows::core::Result<i32>;
    fn SetFontAlignment(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetHangingPunctuation(&self) -> ::windows::core::Result<i32>;
    fn SetHangingPunctuation(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetSnapToGrid(&self) -> ::windows::core::Result<i32>;
    fn SetSnapToGrid(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetTrimPunctuationAtStart(&self) -> ::windows::core::Result<i32>;
    fn SetTrimPunctuationAtStart(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32>;
    fn IsEqual2(&self, ppara: &::core::option::Option<ITextPara2>) -> ::windows::core::Result<i32>;
    fn SetEffects(&self, value: i32, mask: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextPara2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>() -> ITextPara2_Vtbl {
        unsafe extern "system" fn GetBorders<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppborders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBorders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppborders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuplicate2() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate2<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuplicate2(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn GetFontAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFontAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFontAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHangingPunctuation<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHangingPunctuation() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHangingPunctuation<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHangingPunctuation(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSnapToGrid<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSnapToGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapToGrid<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSnapToGrid(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTrimPunctuationAtStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTrimPunctuationAtStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimPunctuationAtStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrimPunctuationAtStart(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEffects<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffects(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual2<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual2(::core::mem::transmute(&ppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *pb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEffects(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ITextPara_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBorders: GetBorders::<Identity, Impl, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, Impl, OFFSET>,
            SetDuplicate2: SetDuplicate2::<Identity, Impl, OFFSET>,
            GetFontAlignment: GetFontAlignment::<Identity, Impl, OFFSET>,
            SetFontAlignment: SetFontAlignment::<Identity, Impl, OFFSET>,
            GetHangingPunctuation: GetHangingPunctuation::<Identity, Impl, OFFSET>,
            SetHangingPunctuation: SetHangingPunctuation::<Identity, Impl, OFFSET>,
            GetSnapToGrid: GetSnapToGrid::<Identity, Impl, OFFSET>,
            SetSnapToGrid: SetSnapToGrid::<Identity, Impl, OFFSET>,
            GetTrimPunctuationAtStart: GetTrimPunctuationAtStart::<Identity, Impl, OFFSET>,
            SetTrimPunctuationAtStart: SetTrimPunctuationAtStart::<Identity, Impl, OFFSET>,
            GetEffects: GetEffects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            IsEqual2: IsEqual2::<Identity, Impl, OFFSET>,
            SetEffects: SetEffects::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPara2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITextPara as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRange_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetText(&self, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetChar(&self) -> ::windows::core::Result<i32>;
    fn SetChar(&self, char: i32) -> ::windows::core::Result<()>;
    fn GetDuplicate(&self) -> ::windows::core::Result<ITextRange>;
    fn GetFormattedText(&self) -> ::windows::core::Result<ITextRange>;
    fn SetFormattedText(&self, prange: &::core::option::Option<ITextRange>) -> ::windows::core::Result<()>;
    fn GetStart(&self) -> ::windows::core::Result<i32>;
    fn SetStart(&self, cpfirst: i32) -> ::windows::core::Result<()>;
    fn GetEnd(&self) -> ::windows::core::Result<i32>;
    fn SetEnd(&self, cplim: i32) -> ::windows::core::Result<()>;
    fn GetFont(&self) -> ::windows::core::Result<ITextFont>;
    fn SetFont(&self, pfont: &::core::option::Option<ITextFont>) -> ::windows::core::Result<()>;
    fn GetPara(&self) -> ::windows::core::Result<ITextPara>;
    fn SetPara(&self, ppara: &::core::option::Option<ITextPara>) -> ::windows::core::Result<()>;
    fn GetStoryLength(&self) -> ::windows::core::Result<i32>;
    fn GetStoryType(&self) -> ::windows::core::Result<i32>;
    fn Collapse(&self, bstart: i32) -> ::windows::core::Result<()>;
    fn Expand(&self, unit: i32) -> ::windows::core::Result<i32>;
    fn GetIndex(&self, unit: i32) -> ::windows::core::Result<i32>;
    fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()>;
    fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()>;
    fn InRange(&self, prange: &::core::option::Option<ITextRange>) -> ::windows::core::Result<i32>;
    fn InStory(&self, prange: &::core::option::Option<ITextRange>) -> ::windows::core::Result<i32>;
    fn IsEqual(&self, prange: &::core::option::Option<ITextRange>) -> ::windows::core::Result<i32>;
    fn Select(&self) -> ::windows::core::Result<()>;
    fn StartOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn EndOf(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn Move(&self, unit: i32, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStart(&self, unit: i32, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEnd(&self, unit: i32, count: i32) -> ::windows::core::Result<i32>;
    fn MoveWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStartWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndWhile(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStartUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndUntil(&self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn FindText(&self, bstr: &super::super::super::Foundation::BSTR, count: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn FindTextStart(&self, bstr: &super::super::super::Foundation::BSTR, count: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn FindTextEnd(&self, bstr: &super::super::super::Foundation::BSTR, count: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn Delete(&self, unit: i32, count: i32) -> ::windows::core::Result<i32>;
    fn Cut(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Copy(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Paste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()>;
    fn CanPaste(&self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32>;
    fn CanEdit(&self) -> ::windows::core::Result<i32>;
    fn ChangeCase(&self, r#type: i32) -> ::windows::core::Result<()>;
    fn GetPoint(&self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()>;
    fn SetPoint(&self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetEmbeddedObject(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>() -> ITextRange_Vtbl {
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetText(::core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn GetChar<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChar() {
                ::core::result::Result::Ok(ok__) => {
                    *pchar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChar<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, char: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChar(::core::mem::transmute_copy(&char)).into()
        }
        unsafe extern "system" fn GetDuplicate<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuplicate() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormattedText() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormattedText(::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn GetStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pcpfirst = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStart(::core::mem::transmute_copy(&cpfirst)).into()
        }
        unsafe extern "system" fn GetEnd<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pcplim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnd<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnd(::core::mem::transmute_copy(&cplim)).into()
        }
        unsafe extern "system" fn GetFont<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFont() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFont<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFont(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn GetPara<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPara() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPara<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPara(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn GetStoryLength<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryType<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStoryType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Collapse(::core::mem::transmute_copy(&bstart)).into()
        }
        unsafe extern "system" fn Expand<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Expand(::core::mem::transmute_copy(&unit)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndex(::core::mem::transmute_copy(&unit)) {
                ::core::result::Result::Ok(ok__) => {
                    *pindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndex(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&extend)).into()
        }
        unsafe extern "system" fn SetRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRange(::core::mem::transmute_copy(&cpanchor), ::core::mem::transmute_copy(&cpactive)).into()
        }
        unsafe extern "system" fn InRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InRange(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InStory<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InStory(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn StartOf<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartOf(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOf<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndOf(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Move(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveStart(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEnd<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveEnd(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveWhile<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveWhile(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStartWhile<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveStartWhile(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndWhile<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveEndWhile(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUntil<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveUntil(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStartUntil<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveStartUntil(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndUntil<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveEndUntil(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindText(::core::mem::transmute(&bstr), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *plength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTextStart<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindTextStart(::core::mem::transmute(&bstr), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *plength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTextEnd<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindTextEnd(::core::mem::transmute(&bstr), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *plength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Delete(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cut<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cut() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Paste(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanPaste(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanEdit<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCase<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeCase(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn GetPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPoint(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&px), ::core::mem::transmute_copy(&py)).into()
        }
        unsafe extern "system" fn SetPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&extend)).into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollIntoView(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEmbeddedObject<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEmbeddedObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetChar: GetChar::<Identity, Impl, OFFSET>,
            SetChar: SetChar::<Identity, Impl, OFFSET>,
            GetDuplicate: GetDuplicate::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, Impl, OFFSET>,
            GetStart: GetStart::<Identity, Impl, OFFSET>,
            SetStart: SetStart::<Identity, Impl, OFFSET>,
            GetEnd: GetEnd::<Identity, Impl, OFFSET>,
            SetEnd: SetEnd::<Identity, Impl, OFFSET>,
            GetFont: GetFont::<Identity, Impl, OFFSET>,
            SetFont: SetFont::<Identity, Impl, OFFSET>,
            GetPara: GetPara::<Identity, Impl, OFFSET>,
            SetPara: SetPara::<Identity, Impl, OFFSET>,
            GetStoryLength: GetStoryLength::<Identity, Impl, OFFSET>,
            GetStoryType: GetStoryType::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            Expand: Expand::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            SetIndex: SetIndex::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
            InRange: InRange::<Identity, Impl, OFFSET>,
            InStory: InStory::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            StartOf: StartOf::<Identity, Impl, OFFSET>,
            EndOf: EndOf::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveStart: MoveStart::<Identity, Impl, OFFSET>,
            MoveEnd: MoveEnd::<Identity, Impl, OFFSET>,
            MoveWhile: MoveWhile::<Identity, Impl, OFFSET>,
            MoveStartWhile: MoveStartWhile::<Identity, Impl, OFFSET>,
            MoveEndWhile: MoveEndWhile::<Identity, Impl, OFFSET>,
            MoveUntil: MoveUntil::<Identity, Impl, OFFSET>,
            MoveStartUntil: MoveStartUntil::<Identity, Impl, OFFSET>,
            MoveEndUntil: MoveEndUntil::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            FindTextStart: FindTextStart::<Identity, Impl, OFFSET>,
            FindTextEnd: FindTextEnd::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Cut: Cut::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Paste: Paste::<Identity, Impl, OFFSET>,
            CanPaste: CanPaste::<Identity, Impl, OFFSET>,
            CanEdit: CanEdit::<Identity, Impl, OFFSET>,
            ChangeCase: ChangeCase::<Identity, Impl, OFFSET>,
            GetPoint: GetPoint::<Identity, Impl, OFFSET>,
            SetPoint: SetPoint::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetEmbeddedObject: GetEmbeddedObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRange as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRange2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ITextRange_Impl + ITextSelection_Impl {
    fn GetCch(&self) -> ::windows::core::Result<i32>;
    fn GetCells(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetColumn(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetCount(&self) -> ::windows::core::Result<i32>;
    fn GetDuplicate2(&self) -> ::windows::core::Result<ITextRange2>;
    fn GetFont2(&self) -> ::windows::core::Result<ITextFont2>;
    fn SetFont2(&self, pfont: &::core::option::Option<ITextFont2>) -> ::windows::core::Result<()>;
    fn GetFormattedText2(&self) -> ::windows::core::Result<ITextRange2>;
    fn SetFormattedText2(&self, prange: &::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn GetGravity(&self) -> ::windows::core::Result<i32>;
    fn SetGravity(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetPara2(&self) -> ::windows::core::Result<ITextPara2>;
    fn SetPara2(&self, ppara: &::core::option::Option<ITextPara2>) -> ::windows::core::Result<()>;
    fn GetRow(&self) -> ::windows::core::Result<ITextRow>;
    fn GetStartPara(&self) -> ::windows::core::Result<i32>;
    fn GetTable(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetURL(&self, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::Result<()>;
    fn BuildUpMath(&self, flags: i32) -> ::windows::core::Result<()>;
    fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> ::windows::core::Result<()>;
    fn Find(&self, prange: &::core::option::Option<ITextRange2>, count: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn GetChar2(&self, pchar: *mut i32, offset: i32) -> ::windows::core::Result<()>;
    fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::Result<()>;
    fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32>;
    fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::Result<()>;
    fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::Result<()>;
    fn GetText2(&self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn HexToUnicode(&self) -> ::windows::core::Result<()>;
    fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::Result<()>;
    fn Linearize(&self, flags: i32) -> ::windows::core::Result<()>;
    fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()>;
    fn SetDropCap(&self, cline: i32, position: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
    fn SetText2(&self, flags: i32, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UnicodeToHex(&self) -> ::windows::core::Result<()>;
    fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::Result<()>;
    fn GetMathFunctionType(&self, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: &super::super::super::Foundation::BSTR, pstream: &::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRange2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>() -> ITextRange2_Vtbl {
        unsafe extern "system" fn GetCch<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCch() {
                ::core::result::Result::Ok(ok__) => {
                    *pcch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCells<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcells: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCells() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcells = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolumn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuplicate2() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFont2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFont2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFont2(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn GetFormattedText2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormattedText2() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormattedText2(::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn GetGravity<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGravity() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGravity(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPara2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPara2() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPara2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPara2(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn GetRow<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRow() {
                ::core::result::Result::Ok(ok__) => {
                    *pprow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStartPara<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStartPara() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTable<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTable() {
                ::core::result::Result::Ok(ok__) => {
                    *pptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURL<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetURL() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetURL(::core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn AddSubrange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddSubrange(::core::mem::transmute_copy(&cp1), ::core::mem::transmute_copy(&cp2), ::core::mem::transmute_copy(&activate)).into()
        }
        unsafe extern "system" fn BuildUpMath<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BuildUpMath(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn DeleteSubrange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpfirst: i32, cplim: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteSubrange(::core::mem::transmute_copy(&cpfirst), ::core::mem::transmute_copy(&cplim)).into()
        }
        unsafe extern "system" fn Find<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, count: i32, flags: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Find(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChar2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchar: *mut i32, offset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChar2(::core::mem::transmute_copy(&pchar), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GetDropCap<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDropCap(::core::mem::transmute_copy(&pcline), ::core::mem::transmute_copy(&pposition)).into()
        }
        unsafe extern "system" fn GetInlineObject<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInlineObject(::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&palign), ::core::mem::transmute_copy(&pchar), ::core::mem::transmute_copy(&pchar1), ::core::mem::transmute_copy(&pchar2), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ptexstyle), ::core::mem::transmute_copy(&pccol), ::core::mem::transmute_copy(&plevel)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRect<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRect(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom), ::core::mem::transmute_copy(&phit)).into()
        }
        unsafe extern "system" fn GetSubrange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSubrange(::core::mem::transmute_copy(&isubrange), ::core::mem::transmute_copy(&pcpfirst), ::core::mem::transmute_copy(&pcplim)).into()
        }
        unsafe extern "system" fn GetText2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetText2(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HexToUnicode<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HexToUnicode().into()
        }
        unsafe extern "system" fn InsertTable<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertTable(::core::mem::transmute_copy(&ccol), ::core::mem::transmute_copy(&crow), ::core::mem::transmute_copy(&autofit)).into()
        }
        unsafe extern "system" fn Linearize<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Linearize(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetActiveSubrange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveSubrange(::core::mem::transmute_copy(&cpanchor), ::core::mem::transmute_copy(&cpactive)).into()
        }
        unsafe extern "system" fn SetDropCap<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cline: i32, position: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDropCap(::core::mem::transmute_copy(&cline), ::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetText2<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetText2(::core::mem::transmute_copy(&flags), ::core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn UnicodeToHex<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnicodeToHex().into()
        }
        unsafe extern "system" fn SetInlineObject<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInlineObject(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&align), ::core::mem::transmute_copy(&char), ::core::mem::transmute_copy(&char1), ::core::mem::transmute_copy(&char2), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&texstyle), ::core::mem::transmute_copy(&ccol)).into()
        }
        unsafe extern "system" fn GetMathFunctionType<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMathFunctionType(::core::mem::transmute(&bstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImage<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertImage(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&ascent), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&bstralttext), ::core::mem::transmute(&pstream)).into()
        }
        Self {
            base: ITextSelection_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCch: GetCch::<Identity, Impl, OFFSET>,
            GetCells: GetCells::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetDuplicate2: GetDuplicate2::<Identity, Impl, OFFSET>,
            GetFont2: GetFont2::<Identity, Impl, OFFSET>,
            SetFont2: SetFont2::<Identity, Impl, OFFSET>,
            GetFormattedText2: GetFormattedText2::<Identity, Impl, OFFSET>,
            SetFormattedText2: SetFormattedText2::<Identity, Impl, OFFSET>,
            GetGravity: GetGravity::<Identity, Impl, OFFSET>,
            SetGravity: SetGravity::<Identity, Impl, OFFSET>,
            GetPara2: GetPara2::<Identity, Impl, OFFSET>,
            SetPara2: SetPara2::<Identity, Impl, OFFSET>,
            GetRow: GetRow::<Identity, Impl, OFFSET>,
            GetStartPara: GetStartPara::<Identity, Impl, OFFSET>,
            GetTable: GetTable::<Identity, Impl, OFFSET>,
            GetURL: GetURL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            AddSubrange: AddSubrange::<Identity, Impl, OFFSET>,
            BuildUpMath: BuildUpMath::<Identity, Impl, OFFSET>,
            DeleteSubrange: DeleteSubrange::<Identity, Impl, OFFSET>,
            Find: Find::<Identity, Impl, OFFSET>,
            GetChar2: GetChar2::<Identity, Impl, OFFSET>,
            GetDropCap: GetDropCap::<Identity, Impl, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetSubrange: GetSubrange::<Identity, Impl, OFFSET>,
            GetText2: GetText2::<Identity, Impl, OFFSET>,
            HexToUnicode: HexToUnicode::<Identity, Impl, OFFSET>,
            InsertTable: InsertTable::<Identity, Impl, OFFSET>,
            Linearize: Linearize::<Identity, Impl, OFFSET>,
            SetActiveSubrange: SetActiveSubrange::<Identity, Impl, OFFSET>,
            SetDropCap: SetDropCap::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetText2: SetText2::<Identity, Impl, OFFSET>,
            UnicodeToHex: UnicodeToHex::<Identity, Impl, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, Impl, OFFSET>,
            GetMathFunctionType: GetMathFunctionType::<Identity, Impl, OFFSET>,
            InsertImage: InsertImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRange2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITextRange as ::windows::core::Interface>::IID || iid == &<ITextSelection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRow_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetAlignment(&self) -> ::windows::core::Result<i32>;
    fn SetAlignment(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellCount(&self) -> ::windows::core::Result<i32>;
    fn SetCellCount(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellCountCache(&self) -> ::windows::core::Result<i32>;
    fn SetCellCountCache(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellIndex(&self) -> ::windows::core::Result<i32>;
    fn SetCellIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellMargin(&self) -> ::windows::core::Result<i32>;
    fn SetCellMargin(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetHeight(&self) -> ::windows::core::Result<i32>;
    fn SetHeight(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetIndent(&self) -> ::windows::core::Result<i32>;
    fn SetIndent(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetKeepTogether(&self) -> ::windows::core::Result<i32>;
    fn SetKeepTogether(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetKeepWithNext(&self) -> ::windows::core::Result<i32>;
    fn SetKeepWithNext(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetNestLevel(&self) -> ::windows::core::Result<i32>;
    fn GetRTL(&self) -> ::windows::core::Result<i32>;
    fn SetRTL(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellAlignment(&self) -> ::windows::core::Result<i32>;
    fn SetCellAlignment(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellColorBack(&self) -> ::windows::core::Result<i32>;
    fn SetCellColorBack(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellColorFore(&self) -> ::windows::core::Result<i32>;
    fn SetCellColorFore(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellMergeFlags(&self) -> ::windows::core::Result<i32>;
    fn SetCellMergeFlags(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellShading(&self) -> ::windows::core::Result<i32>;
    fn SetCellShading(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellVerticalText(&self) -> ::windows::core::Result<i32>;
    fn SetCellVerticalText(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellWidth(&self) -> ::windows::core::Result<i32>;
    fn SetCellWidth(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::core::Result<()>;
    fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::core::Result<()>;
    fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::core::Result<()>;
    fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::core::Result<()>;
    fn Apply(&self, crow: i32, flags: tomConstants) -> ::windows::core::Result<()>;
    fn CanChange(&self) -> ::windows::core::Result<i32>;
    fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32>;
    fn Insert(&self, crow: i32) -> ::windows::core::Result<()>;
    fn IsEqual(&self, prow: &::core::option::Option<ITextRow>) -> ::windows::core::Result<i32>;
    fn Reset(&self, value: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>() -> ITextRow_Vtbl {
        unsafe extern "system" fn GetAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellCount(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellCountCache<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellCountCache() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellCountCache<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellCountCache(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellIndex(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellMargin<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellMargin<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellMargin(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHeight<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHeight(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndent<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepTogether<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKeepTogether() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeepTogether(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepWithNext<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKeepWithNext() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeepWithNext(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNestLevel<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNestLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRTL<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRTL<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRTL(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellAlignment<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellColorBack<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellColorBack() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellColorBack<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellColorBack(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellColorFore<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellColorFore() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellColorFore<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellColorFore(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellMergeFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellMergeFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellMergeFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellMergeFlags(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellShading<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellShading() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellShading<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellShading(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellVerticalText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellVerticalText() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellVerticalText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellVerticalText(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellWidth<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCellWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellWidth<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellWidth(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellBorderColors<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCellBorderColors(::core::mem::transmute_copy(&pcrleft), ::core::mem::transmute_copy(&pcrtop), ::core::mem::transmute_copy(&pcrright), ::core::mem::transmute_copy(&pcrbottom)).into()
        }
        unsafe extern "system" fn GetCellBorderWidths<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCellBorderWidths(::core::mem::transmute_copy(&pduleft), ::core::mem::transmute_copy(&pdutop), ::core::mem::transmute_copy(&pduright), ::core::mem::transmute_copy(&pdubottom)).into()
        }
        unsafe extern "system" fn SetCellBorderColors<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellBorderColors(::core::mem::transmute_copy(&crleft), ::core::mem::transmute_copy(&crtop), ::core::mem::transmute_copy(&crright), ::core::mem::transmute_copy(&crbottom)).into()
        }
        unsafe extern "system" fn SetCellBorderWidths<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCellBorderWidths(::core::mem::transmute_copy(&duleft), ::core::mem::transmute_copy(&dutop), ::core::mem::transmute_copy(&duright), ::core::mem::transmute_copy(&dubottom)).into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crow: i32, flags: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Apply(::core::mem::transmute_copy(&crow), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CanChange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Insert(::core::mem::transmute_copy(&crow)).into()
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prow: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&prow)) {
                ::core::result::Result::Ok(ok__) => {
                    *pb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextRow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAlignment: GetAlignment::<Identity, Impl, OFFSET>,
            SetAlignment: SetAlignment::<Identity, Impl, OFFSET>,
            GetCellCount: GetCellCount::<Identity, Impl, OFFSET>,
            SetCellCount: SetCellCount::<Identity, Impl, OFFSET>,
            GetCellCountCache: GetCellCountCache::<Identity, Impl, OFFSET>,
            SetCellCountCache: SetCellCountCache::<Identity, Impl, OFFSET>,
            GetCellIndex: GetCellIndex::<Identity, Impl, OFFSET>,
            SetCellIndex: SetCellIndex::<Identity, Impl, OFFSET>,
            GetCellMargin: GetCellMargin::<Identity, Impl, OFFSET>,
            SetCellMargin: SetCellMargin::<Identity, Impl, OFFSET>,
            GetHeight: GetHeight::<Identity, Impl, OFFSET>,
            SetHeight: SetHeight::<Identity, Impl, OFFSET>,
            GetIndent: GetIndent::<Identity, Impl, OFFSET>,
            SetIndent: SetIndent::<Identity, Impl, OFFSET>,
            GetKeepTogether: GetKeepTogether::<Identity, Impl, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, Impl, OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Identity, Impl, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, Impl, OFFSET>,
            GetNestLevel: GetNestLevel::<Identity, Impl, OFFSET>,
            GetRTL: GetRTL::<Identity, Impl, OFFSET>,
            SetRTL: SetRTL::<Identity, Impl, OFFSET>,
            GetCellAlignment: GetCellAlignment::<Identity, Impl, OFFSET>,
            SetCellAlignment: SetCellAlignment::<Identity, Impl, OFFSET>,
            GetCellColorBack: GetCellColorBack::<Identity, Impl, OFFSET>,
            SetCellColorBack: SetCellColorBack::<Identity, Impl, OFFSET>,
            GetCellColorFore: GetCellColorFore::<Identity, Impl, OFFSET>,
            SetCellColorFore: SetCellColorFore::<Identity, Impl, OFFSET>,
            GetCellMergeFlags: GetCellMergeFlags::<Identity, Impl, OFFSET>,
            SetCellMergeFlags: SetCellMergeFlags::<Identity, Impl, OFFSET>,
            GetCellShading: GetCellShading::<Identity, Impl, OFFSET>,
            SetCellShading: SetCellShading::<Identity, Impl, OFFSET>,
            GetCellVerticalText: GetCellVerticalText::<Identity, Impl, OFFSET>,
            SetCellVerticalText: SetCellVerticalText::<Identity, Impl, OFFSET>,
            GetCellWidth: GetCellWidth::<Identity, Impl, OFFSET>,
            SetCellWidth: SetCellWidth::<Identity, Impl, OFFSET>,
            GetCellBorderColors: GetCellBorderColors::<Identity, Impl, OFFSET>,
            GetCellBorderWidths: GetCellBorderWidths::<Identity, Impl, OFFSET>,
            SetCellBorderColors: SetCellBorderColors::<Identity, Impl, OFFSET>,
            SetCellBorderWidths: SetCellBorderWidths::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            CanChange: CanChange::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRow as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextSelection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ITextRange_Impl {
    fn GetFlags(&self) -> ::windows::core::Result<i32>;
    fn SetFlags(&self, flags: i32) -> ::windows::core::Result<()>;
    fn GetType(&self) -> ::windows::core::Result<i32>;
    fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows::core::Result<i32>;
    fn EndKey(&self, unit: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn TypeText(&self, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextSelection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>() -> ITextSelection_Vtbl {
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveLeft<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveLeft(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveRight<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveRight(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUp<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveUp(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveDown<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveDown(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeKey<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HomeKey(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndKey<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndKey(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeText<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TypeText(::core::mem::transmute(&bstr)).into()
        }
        Self {
            base: ITextRange_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            MoveLeft: MoveLeft::<Identity, Impl, OFFSET>,
            MoveRight: MoveRight::<Identity, Impl, OFFSET>,
            MoveUp: MoveUp::<Identity, Impl, OFFSET>,
            MoveDown: MoveDown::<Identity, Impl, OFFSET>,
            HomeKey: HomeKey::<Identity, Impl, OFFSET>,
            EndKey: EndKey::<Identity, Impl, OFFSET>,
            TypeText: TypeText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextSelection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITextRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextSelection2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ITextRange_Impl + ITextSelection_Impl + ITextRange2_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextSelection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection2_Impl, const OFFSET: isize>() -> ITextSelection2_Vtbl {
        Self { base: ITextRange2_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextSelection2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITextRange as ::windows::core::Interface>::IID || iid == &<ITextSelection as ::windows::core::Interface>::IID || iid == &<ITextRange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServices_Impl: Sized {
    fn TxSendMessage(&self, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn TxDraw(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::Result<()>;
    fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTxSetCursor(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn TxQueryHitPoint(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::Result<()>;
    fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnTxInPlaceDeactivate(&self) -> ::windows::core::Result<()>;
    fn OnTxUIActivate(&self) -> ::windows::core::Result<()>;
    fn OnTxUIDeactivate(&self) -> ::windows::core::Result<()>;
    fn TxGetText(&self, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TxSetText(&self, psztext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn TxGetCurTargetX(&self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn TxGetBaseLinePos(&self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn TxGetNaturalSize(&self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::Result<()>;
    fn TxGetDropTarget(&self) -> ::windows::core::Result<super::super::super::System::Ole::IDropTarget>;
    fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> ::windows::core::Result<()>;
    fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>() -> ITextServices_Vtbl {
        unsafe extern "system" fn TxSendMessage<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSendMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plresult)).into()
        }
        unsafe extern "system" fn TxDraw<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .TxDraw(
                    ::core::mem::transmute_copy(&dwdrawaspect),
                    ::core::mem::transmute_copy(&lindex),
                    ::core::mem::transmute_copy(&pvaspect),
                    ::core::mem::transmute_copy(&ptd),
                    ::core::mem::transmute_copy(&hdcdraw),
                    ::core::mem::transmute_copy(&hictargetdev),
                    ::core::mem::transmute_copy(&lprcbounds),
                    ::core::mem::transmute_copy(&lprcwbounds),
                    ::core::mem::transmute_copy(&lprcupdate),
                    ::core::mem::transmute_copy(&pfncontinue),
                    ::core::mem::transmute_copy(&dwcontinue),
                    ::core::mem::transmute_copy(&lviewid),
                )
                .into()
        }
        unsafe extern "system" fn TxGetHScroll<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetHScroll(::core::mem::transmute_copy(&plmin), ::core::mem::transmute_copy(&plmax), ::core::mem::transmute_copy(&plpos), ::core::mem::transmute_copy(&plpage), ::core::mem::transmute_copy(&pfenabled)).into()
        }
        unsafe extern "system" fn TxGetVScroll<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetVScroll(::core::mem::transmute_copy(&plmin), ::core::mem::transmute_copy(&plmax), ::core::mem::transmute_copy(&plpos), ::core::mem::transmute_copy(&plpage), ::core::mem::transmute_copy(&pfenabled)).into()
        }
        unsafe extern "system" fn OnTxSetCursor<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTxSetCursor(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&lprcclient), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn TxQueryHitPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxQueryHitPoint(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&lprcclient), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&phitresult)).into()
        }
        unsafe extern "system" fn OnTxInPlaceActivate<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTxInPlaceActivate(::core::mem::transmute_copy(&prcclient)).into()
        }
        unsafe extern "system" fn OnTxInPlaceDeactivate<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTxInPlaceDeactivate().into()
        }
        unsafe extern "system" fn OnTxUIActivate<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTxUIActivate().into()
        }
        unsafe extern "system" fn OnTxUIDeactivate<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTxUIDeactivate().into()
        }
        unsafe extern "system" fn TxGetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetText(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        unsafe extern "system" fn TxSetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxSetText(::core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn TxGetCurTargetX<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetCurTargetX(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn TxGetBaseLinePos<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetBaseLinePos(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn TxGetNaturalSize<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetNaturalSize(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&psizelextent), ::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn TxGetDropTarget<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TxGetDropTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdroptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxPropertyBitsChange<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32, dwbits: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTxPropertyBitsChange(::core::mem::transmute_copy(&dwmask), ::core::mem::transmute_copy(&dwbits)).into()
        }
        unsafe extern "system" fn TxGetCachedSize<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetCachedSize(::core::mem::transmute_copy(&pdwwidth), ::core::mem::transmute_copy(&pdwheight)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TxSendMessage: TxSendMessage::<Identity, Impl, OFFSET>,
            TxDraw: TxDraw::<Identity, Impl, OFFSET>,
            TxGetHScroll: TxGetHScroll::<Identity, Impl, OFFSET>,
            TxGetVScroll: TxGetVScroll::<Identity, Impl, OFFSET>,
            OnTxSetCursor: OnTxSetCursor::<Identity, Impl, OFFSET>,
            TxQueryHitPoint: TxQueryHitPoint::<Identity, Impl, OFFSET>,
            OnTxInPlaceActivate: OnTxInPlaceActivate::<Identity, Impl, OFFSET>,
            OnTxInPlaceDeactivate: OnTxInPlaceDeactivate::<Identity, Impl, OFFSET>,
            OnTxUIActivate: OnTxUIActivate::<Identity, Impl, OFFSET>,
            OnTxUIDeactivate: OnTxUIDeactivate::<Identity, Impl, OFFSET>,
            TxGetText: TxGetText::<Identity, Impl, OFFSET>,
            TxSetText: TxSetText::<Identity, Impl, OFFSET>,
            TxGetCurTargetX: TxGetCurTargetX::<Identity, Impl, OFFSET>,
            TxGetBaseLinePos: TxGetBaseLinePos::<Identity, Impl, OFFSET>,
            TxGetNaturalSize: TxGetNaturalSize::<Identity, Impl, OFFSET>,
            TxGetDropTarget: TxGetDropTarget::<Identity, Impl, OFFSET>,
            OnTxPropertyBitsChange: OnTxPropertyBitsChange::<Identity, Impl, OFFSET>,
            TxGetCachedSize: TxGetCachedSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServices2_Impl: Sized + ITextServices_Impl {
    fn TxGetNaturalSize2(&self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::core::Result<()>;
    fn TxDrawD2D(&self, prendertarget: &::core::option::Option<super::super::super::Graphics::Direct2D::ID2D1RenderTarget>, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServices2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices2_Impl, const OFFSET: isize>() -> ITextServices2_Vtbl {
        unsafe extern "system" fn TxGetNaturalSize2<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxGetNaturalSize2(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&psizelextent), ::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight), ::core::mem::transmute_copy(&pascent)).into()
        }
        unsafe extern "system" fn TxDrawD2D<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TxDrawD2D(::core::mem::transmute(&prendertarget), ::core::mem::transmute_copy(&lprcbounds), ::core::mem::transmute_copy(&lprcupdate), ::core::mem::transmute_copy(&lviewid)).into()
        }
        Self {
            base: ITextServices_Vtbl::new::<Identity, Impl, OFFSET>(),
            TxGetNaturalSize2: TxGetNaturalSize2::<Identity, Impl, OFFSET>,
            TxDrawD2D: TxDrawD2D::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextServices2 as ::windows::core::Interface>::IID || iid == &<ITextServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITextStory_Impl: Sized {
    fn GetActive(&self) -> ::windows::core::Result<i32>;
    fn SetActive(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetDisplay(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetIndex(&self) -> ::windows::core::Result<i32>;
    fn GetType(&self) -> ::windows::core::Result<i32>;
    fn SetType(&self, value: i32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, r#type: i32) -> ::windows::core::Result<i32>;
    fn GetRange(&self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange2>;
    fn GetText(&self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetFormattedText(&self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetProperty(&self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
    fn SetText(&self, flags: i32, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITextStory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>() -> ITextStory_Vtbl {
        unsafe extern "system" fn GetActive<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActive() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActive<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActive(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDisplay<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplay: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRange(::core::mem::transmute_copy(&cpactive), ::core::mem::transmute_copy(&cpanchor)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormattedText(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextStory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&flags), ::core::mem::transmute(&bstr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetActive: GetActive::<Identity, Impl, OFFSET>,
            SetActive: SetActive::<Identity, Impl, OFFSET>,
            GetDisplay: GetDisplay::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoryRanges_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, index: i32) -> ::windows::core::Result<ITextRange>;
    fn GetCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoryRanges_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges_Impl, const OFFSET: isize>() -> ITextStoryRanges_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoryRanges as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoryRanges2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ITextStoryRanges_Impl {
    fn Item2(&self, index: i32) -> ::windows::core::Result<ITextRange2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoryRanges2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges2_Impl, const OFFSET: isize>() -> ITextStoryRanges2_Vtbl {
        unsafe extern "system" fn Item2<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item2(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITextStoryRanges_Vtbl::new::<Identity, Impl, OFFSET>(), Item2: Item2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoryRanges2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITextStoryRanges as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStrings_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<ITextRange2>;
    fn GetCount(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Append(&self, prange: &::core::option::Option<ITextRange2>, istring: i32) -> ::windows::core::Result<()>;
    fn Cat2(&self, istring: i32) -> ::windows::core::Result<()>;
    fn CatTop2(&self, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteRange(&self, prange: &::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn EncodeFunction(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: &::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn GetCch(&self, istring: i32) -> ::windows::core::Result<i32>;
    fn InsertNullStr(&self, istring: i32) -> ::windows::core::Result<()>;
    fn MoveBoundary(&self, istring: i32, cch: i32) -> ::windows::core::Result<()>;
    fn PrefixTop(&self, bstr: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Remove(&self, istring: i32, cstring: i32) -> ::windows::core::Result<()>;
    fn SetFormattedText(&self, pranged: &::core::option::Option<ITextRange2>, pranges: &::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn SetOpCp(&self, istring: i32, cp: i32) -> ::windows::core::Result<()>;
    fn SuffixTop(&self, bstr: &super::super::super::Foundation::BSTR, prange: &::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn Swap(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStrings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>() -> ITextStrings_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Append(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn Cat2<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cat2(::core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn CatTop2<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CatTop2(::core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn DeleteRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRange(::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn EncodeFunction<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EncodeFunction(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&align), ::core::mem::transmute_copy(&char), ::core::mem::transmute_copy(&char1), ::core::mem::transmute_copy(&char2), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&texstyle), ::core::mem::transmute_copy(&ccol), ::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn GetCch<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCch(::core::mem::transmute_copy(&istring)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertNullStr<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertNullStr(::core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn MoveBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveBoundary(::core::mem::transmute_copy(&istring), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn PrefixTop<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrefixTop(::core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cstring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&istring), ::core::mem::transmute_copy(&cstring)).into()
        }
        unsafe extern "system" fn SetFormattedText<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pranged: ::windows::core::RawPtr, pranges: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormattedText(::core::mem::transmute(&pranged), ::core::mem::transmute(&pranges)).into()
        }
        unsafe extern "system" fn SetOpCp<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOpCp(::core::mem::transmute_copy(&istring), ::core::mem::transmute_copy(&cp)).into()
        }
        unsafe extern "system" fn SuffixTop<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SuffixTop(::core::mem::transmute(&bstr), ::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn Swap<Identity: ::windows::core::IUnknownImpl, Impl: ITextStrings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Swap().into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Cat2: Cat2::<Identity, Impl, OFFSET>,
            CatTop2: CatTop2::<Identity, Impl, OFFSET>,
            DeleteRange: DeleteRange::<Identity, Impl, OFFSET>,
            EncodeFunction: EncodeFunction::<Identity, Impl, OFFSET>,
            GetCch: GetCch::<Identity, Impl, OFFSET>,
            InsertNullStr: InsertNullStr::<Identity, Impl, OFFSET>,
            MoveBoundary: MoveBoundary::<Identity, Impl, OFFSET>,
            PrefixTop: PrefixTop::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, Impl, OFFSET>,
            SetOpCp: SetOpCp::<Identity, Impl, OFFSET>,
            SuffixTop: SuffixTop::<Identity, Impl, OFFSET>,
            Swap: Swap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStrings as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
