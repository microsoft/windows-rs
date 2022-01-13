#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IRichEditOleImpl: Sized {
    fn GetClientSite(&mut self) -> ::windows::core::Result<super::super::super::System::Ole::IOleClientSite>;
    fn GetObjectCount(&mut self) -> i32;
    fn GetLinkCount(&mut self) -> i32;
    fn GetObject(&mut self, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::core::Result<()>;
    fn InsertObject(&mut self, lpreobject: *mut REOBJECT) -> ::windows::core::Result<()>;
    fn ConvertObject(&mut self, iob: i32, rclsidnew: *const ::windows::core::GUID, lpstrusertypenew: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn ActivateAs(&mut self, rclsid: *const ::windows::core::GUID, rclsidas: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetHostNames(&mut self, lpstrcontainerapp: super::super::super::Foundation::PSTR, lpstrcontainerobj: super::super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn SetLinkAvailable(&mut self, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetDvaspect(&mut self, iob: i32, dvaspect: u32) -> ::windows::core::Result<()>;
    fn HandsOffStorage(&mut self, iob: i32) -> ::windows::core::Result<()>;
    fn SaveCompleted(&mut self, iob: i32, lpstg: ::core::option::Option<super::super::super::System::Com::StructuredStorage::IStorage>) -> ::windows::core::Result<()>;
    fn InPlaceDeactivate(&mut self) -> ::windows::core::Result<()>;
    fn ContextSensitiveHelp(&mut self, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetClipboardData(&mut self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn ImportDataObject(&mut self, lpdataobj: ::core::option::Option<super::super::super::System::Com::IDataObject>, cf: u16, hmetapict: isize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IRichEditOleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditOleVtbl {
        unsafe extern "system" fn GetClientSite<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpolesite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientSite() {
                ::core::result::Result::Ok(ok__) => {
                    *lplpolesite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectCount<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectCount()
        }
        unsafe extern "system" fn GetLinkCount<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLinkCount()
        }
        unsafe extern "system" fn GetObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&iob), ::core::mem::transmute_copy(&lpreobject), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn InsertObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpreobject: *mut REOBJECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertObject(::core::mem::transmute_copy(&lpreobject)).into()
        }
        unsafe extern "system" fn ConvertObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, rclsidnew: *const ::windows::core::GUID, lpstrusertypenew: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConvertObject(::core::mem::transmute_copy(&iob), ::core::mem::transmute_copy(&rclsidnew), ::core::mem::transmute_copy(&lpstrusertypenew)).into()
        }
        unsafe extern "system" fn ActivateAs<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rclsidas: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateAs(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rclsidas)).into()
        }
        unsafe extern "system" fn SetHostNames<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpstrcontainerapp: super::super::super::Foundation::PSTR, lpstrcontainerobj: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHostNames(::core::mem::transmute_copy(&lpstrcontainerapp), ::core::mem::transmute_copy(&lpstrcontainerobj)).into()
        }
        unsafe extern "system" fn SetLinkAvailable<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLinkAvailable(::core::mem::transmute_copy(&iob), ::core::mem::transmute_copy(&favailable)).into()
        }
        unsafe extern "system" fn SetDvaspect<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, dvaspect: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDvaspect(::core::mem::transmute_copy(&iob), ::core::mem::transmute_copy(&dvaspect)).into()
        }
        unsafe extern "system" fn HandsOffStorage<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandsOffStorage(::core::mem::transmute_copy(&iob)).into()
        }
        unsafe extern "system" fn SaveCompleted<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, lpstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveCompleted(::core::mem::transmute_copy(&iob), ::core::mem::transmute(&lpstg)).into()
        }
        unsafe extern "system" fn InPlaceDeactivate<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InPlaceDeactivate().into()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContextSensitiveHelp(::core::mem::transmute_copy(&fentermode)).into()
        }
        unsafe extern "system" fn GetClipboardData<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClipboardData(::core::mem::transmute_copy(&lpchrg), ::core::mem::transmute_copy(&reco), ::core::mem::transmute_copy(&lplpdataobj)).into()
        }
        unsafe extern "system" fn ImportDataObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, cf: u16, hmetapict: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportDataObject(::core::mem::transmute(&lpdataobj), ::core::mem::transmute_copy(&cf), ::core::mem::transmute_copy(&hmetapict)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClientSite: GetClientSite::<Impl, IMPL_OFFSET>,
            GetObjectCount: GetObjectCount::<Impl, IMPL_OFFSET>,
            GetLinkCount: GetLinkCount::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            InsertObject: InsertObject::<Impl, IMPL_OFFSET>,
            ConvertObject: ConvertObject::<Impl, IMPL_OFFSET>,
            ActivateAs: ActivateAs::<Impl, IMPL_OFFSET>,
            SetHostNames: SetHostNames::<Impl, IMPL_OFFSET>,
            SetLinkAvailable: SetLinkAvailable::<Impl, IMPL_OFFSET>,
            SetDvaspect: SetDvaspect::<Impl, IMPL_OFFSET>,
            HandsOffStorage: HandsOffStorage::<Impl, IMPL_OFFSET>,
            SaveCompleted: SaveCompleted::<Impl, IMPL_OFFSET>,
            InPlaceDeactivate: InPlaceDeactivate::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
            GetClipboardData: GetClipboardData::<Impl, IMPL_OFFSET>,
            ImportDataObject: ImportDataObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditOle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IRichEditOleCallbackImpl: Sized {
    fn GetNewStorage(&mut self) -> ::windows::core::Result<super::super::super::System::Com::StructuredStorage::IStorage>;
    fn GetInPlaceContext(&mut self, lplpframe: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceFrame>, lplpdoc: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceUIWindow>, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::core::Result<()>;
    fn ShowContainerUI(&mut self, fshow: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn QueryInsertObject(&mut self, lpclsid: *mut ::windows::core::GUID, lpstg: ::core::option::Option<super::super::super::System::Com::StructuredStorage::IStorage>, cp: i32) -> ::windows::core::Result<()>;
    fn DeleteObject(&mut self, lpoleobj: ::core::option::Option<super::super::super::System::Ole::IOleObject>) -> ::windows::core::Result<()>;
    fn QueryAcceptData(&mut self, lpdataobj: ::core::option::Option<super::super::super::System::Com::IDataObject>, lpcfformat: *mut u16, reco: u32, freally: super::super::super::Foundation::BOOL, hmetapict: isize) -> ::windows::core::Result<()>;
    fn ContextSensitiveHelp(&mut self, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetClipboardData(&mut self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::core::Result<()>;
    fn GetDragDropEffect(&mut self, fdrag: super::super::super::Foundation::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::core::Result<()>;
    fn GetContextMenu(&mut self, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: ::core::option::Option<super::super::super::System::Ole::IOleObject>, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IRichEditOleCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditOleCallbackVtbl {
        unsafe extern "system" fn GetNewStorage<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNewStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *lplpstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInPlaceContext<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpframe: *mut ::windows::core::RawPtr, lplpdoc: *mut ::windows::core::RawPtr, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInPlaceContext(::core::mem::transmute_copy(&lplpframe), ::core::mem::transmute_copy(&lplpdoc), ::core::mem::transmute_copy(&lpframeinfo)).into()
        }
        unsafe extern "system" fn ShowContainerUI<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContainerUI(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn QueryInsertObject<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID, lpstg: ::windows::core::RawPtr, cp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInsertObject(::core::mem::transmute_copy(&lpclsid), ::core::mem::transmute(&lpstg), ::core::mem::transmute_copy(&cp)).into()
        }
        unsafe extern "system" fn DeleteObject<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpoleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteObject(::core::mem::transmute(&lpoleobj)).into()
        }
        unsafe extern "system" fn QueryAcceptData<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, lpcfformat: *mut u16, reco: u32, freally: super::super::super::Foundation::BOOL, hmetapict: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryAcceptData(::core::mem::transmute(&lpdataobj), ::core::mem::transmute_copy(&lpcfformat), ::core::mem::transmute_copy(&reco), ::core::mem::transmute_copy(&freally), ::core::mem::transmute_copy(&hmetapict)).into()
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContextSensitiveHelp(::core::mem::transmute_copy(&fentermode)).into()
        }
        unsafe extern "system" fn GetClipboardData<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClipboardData(::core::mem::transmute_copy(&lpchrg), ::core::mem::transmute_copy(&reco), ::core::mem::transmute_copy(&lplpdataobj)).into()
        }
        unsafe extern "system" fn GetDragDropEffect<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdrag: super::super::super::Foundation::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDragDropEffect(::core::mem::transmute_copy(&fdrag), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute_copy(&pdweffect)).into()
        }
        unsafe extern "system" fn GetContextMenu<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: ::windows::core::RawPtr, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContextMenu(::core::mem::transmute_copy(&seltype), ::core::mem::transmute(&lpoleobj), ::core::mem::transmute_copy(&lpchrg), ::core::mem::transmute_copy(&lphmenu)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetNewStorage: GetNewStorage::<Impl, IMPL_OFFSET>,
            GetInPlaceContext: GetInPlaceContext::<Impl, IMPL_OFFSET>,
            ShowContainerUI: ShowContainerUI::<Impl, IMPL_OFFSET>,
            QueryInsertObject: QueryInsertObject::<Impl, IMPL_OFFSET>,
            DeleteObject: DeleteObject::<Impl, IMPL_OFFSET>,
            QueryAcceptData: QueryAcceptData::<Impl, IMPL_OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Impl, IMPL_OFFSET>,
            GetClipboardData: GetClipboardData::<Impl, IMPL_OFFSET>,
            GetDragDropEffect: GetDragDropEffect::<Impl, IMPL_OFFSET>,
            GetContextMenu: GetContextMenu::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditOleCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRicheditUiaOverridesImpl: Sized {
    fn GetPropertyOverrideValue(&mut self, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRicheditUiaOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditUiaOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRicheditUiaOverridesVtbl {
        unsafe extern "system" fn GetPropertyOverrideValue<Impl: IRicheditUiaOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyOverrideValue(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pretvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetPropertyOverrideValue: GetPropertyOverrideValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRicheditUiaOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDisplaysImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDisplaysVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDisplaysImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDisplaysVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDisplays as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocumentImpl: Sized + IDispatchImpl {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn GetSelection(&mut self) -> ::windows::core::Result<ITextSelection>;
    fn GetStoryCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetStoryRanges(&mut self) -> ::windows::core::Result<ITextStoryRanges>;
    fn GetSaved(&mut self) -> ::windows::core::Result<i32>;
    fn SetSaved(&mut self, value: tomConstants) -> ::windows::core::Result<()>;
    fn GetDefaultTabStop(&mut self) -> ::windows::core::Result<f32>;
    fn SetDefaultTabStop(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn New(&mut self) -> ::windows::core::Result<()>;
    fn Open(&mut self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()>;
    fn Save(&mut self, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::Result<()>;
    fn Freeze(&mut self) -> ::windows::core::Result<i32>;
    fn Unfreeze(&mut self) -> ::windows::core::Result<i32>;
    fn BeginEditCollection(&mut self) -> ::windows::core::Result<()>;
    fn EndEditCollection(&mut self) -> ::windows::core::Result<()>;
    fn Undo(&mut self, count: i32) -> ::windows::core::Result<i32>;
    fn Redo(&mut self, count: i32) -> ::windows::core::Result<i32>;
    fn Range(&mut self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange>;
    fn RangeFromPoint(&mut self, x: i32, y: i32) -> ::windows::core::Result<ITextRange>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocumentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocumentVtbl {
        unsafe extern "system" fn GetName<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryCount<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryRanges<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSaved<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSaved() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaved<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSaved(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDefaultTabStop<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultTabStop() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultTabStop(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn New<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).New().into()
        }
        unsafe extern "system" fn Open<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&codepage)).into()
        }
        unsafe extern "system" fn Save<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&codepage)).into()
        }
        unsafe extern "system" fn Freeze<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Freeze() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unfreeze<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unfreeze() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEditCollection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEditCollection().into()
        }
        unsafe extern "system" fn EndEditCollection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEditCollection().into()
        }
        unsafe extern "system" fn Undo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Undo(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Redo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Redo(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Range<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range(::core::mem::transmute_copy(&cpactive), ::core::mem::transmute_copy(&cpanchor)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            GetStoryCount: GetStoryCount::<Impl, IMPL_OFFSET>,
            GetStoryRanges: GetStoryRanges::<Impl, IMPL_OFFSET>,
            GetSaved: GetSaved::<Impl, IMPL_OFFSET>,
            SetSaved: SetSaved::<Impl, IMPL_OFFSET>,
            GetDefaultTabStop: GetDefaultTabStop::<Impl, IMPL_OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Impl, IMPL_OFFSET>,
            New: New::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            Freeze: Freeze::<Impl, IMPL_OFFSET>,
            Unfreeze: Unfreeze::<Impl, IMPL_OFFSET>,
            BeginEditCollection: BeginEditCollection::<Impl, IMPL_OFFSET>,
            EndEditCollection: EndEditCollection::<Impl, IMPL_OFFSET>,
            Undo: Undo::<Impl, IMPL_OFFSET>,
            Redo: Redo::<Impl, IMPL_OFFSET>,
            Range: Range::<Impl, IMPL_OFFSET>,
            RangeFromPoint: RangeFromPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocument2Impl: Sized + IDispatchImpl + ITextDocumentImpl {
    fn GetCaretType(&mut self) -> ::windows::core::Result<i32>;
    fn SetCaretType(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetDisplays(&mut self) -> ::windows::core::Result<ITextDisplays>;
    fn GetDocumentFont(&mut self) -> ::windows::core::Result<ITextFont2>;
    fn SetDocumentFont(&mut self, pfont: ::core::option::Option<ITextFont2>) -> ::windows::core::Result<()>;
    fn GetDocumentPara(&mut self) -> ::windows::core::Result<ITextPara2>;
    fn SetDocumentPara(&mut self, ppara: ::core::option::Option<ITextPara2>) -> ::windows::core::Result<()>;
    fn GetEastAsianFlags(&mut self) -> ::windows::core::Result<tomConstants>;
    fn GetGenerator(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetIMEInProgress(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetNotificationMode(&mut self) -> ::windows::core::Result<i32>;
    fn SetNotificationMode(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetSelection2(&mut self) -> ::windows::core::Result<ITextSelection2>;
    fn GetStoryRanges2(&mut self) -> ::windows::core::Result<ITextStoryRanges2>;
    fn GetTypographyOptions(&mut self) -> ::windows::core::Result<i32>;
    fn GetVersion(&mut self) -> ::windows::core::Result<i32>;
    fn GetWindow(&mut self) -> ::windows::core::Result<i64>;
    fn AttachMsgFilter(&mut self, pfilter: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CheckTextLimit(&mut self, cch: i32, pcch: *const i32) -> ::windows::core::Result<()>;
    fn GetCallManager(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetClientRect(&mut self, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()>;
    fn GetEffectColor(&mut self, index: i32) -> ::windows::core::Result<i32>;
    fn GetImmContext(&mut self) -> ::windows::core::Result<i64>;
    fn GetPreferredFont(&mut self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, r#type: i32) -> ::windows::core::Result<i32>;
    fn GetStrings(&mut self) -> ::windows::core::Result<ITextStrings>;
    fn Notify(&mut self, notify: i32) -> ::windows::core::Result<()>;
    fn Range2(&mut self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange2>;
    fn RangeFromPoint2(&mut self, x: i32, y: i32, r#type: i32) -> ::windows::core::Result<ITextRange2>;
    fn ReleaseCallManager(&mut self, pvoid: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ReleaseImmContext(&mut self, context: i64) -> ::windows::core::Result<()>;
    fn SetEffectColor(&mut self, index: i32, value: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
    fn SetTypographyOptions(&mut self, options: i32, mask: i32) -> ::windows::core::Result<()>;
    fn SysBeep(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn UpdateWindow(&mut self) -> ::windows::core::Result<()>;
    fn GetMathProperties(&mut self) -> ::windows::core::Result<i32>;
    fn SetMathProperties(&mut self, options: i32, mask: i32) -> ::windows::core::Result<()>;
    fn GetActiveStory(&mut self) -> ::windows::core::Result<ITextStory>;
    fn SetActiveStory(&mut self, pstory: ::core::option::Option<ITextStory>) -> ::windows::core::Result<()>;
    fn GetMainStory(&mut self) -> ::windows::core::Result<ITextStory>;
    fn GetNewStory(&mut self) -> ::windows::core::Result<ITextStory>;
    fn GetStory(&mut self, index: i32) -> ::windows::core::Result<ITextStory>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocument2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocument2Vtbl {
        unsafe extern "system" fn GetCaretType<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaretType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaretType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDisplays<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplays: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplays() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisplays = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentFont() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentFont(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn GetDocumentPara<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentPara() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentPara<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentPara(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn GetEastAsianFlags<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEastAsianFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerator<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGenerator() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIMEInProgress<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIMEInProgress(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNotificationMode<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotificationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMode<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSelection2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryRanges2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryRanges2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypographyOptions<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypographyOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *poptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachMsgFilter<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachMsgFilter(::core::mem::transmute(&pfilter)).into()
        }
        unsafe extern "system" fn CheckTextLimit<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckTextLimit(::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn GetCallManager<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCallManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvoid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientRect<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClientRect(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)).into()
        }
        unsafe extern "system" fn GetEffectColor<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectColor(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmContext<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImmContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPreferredFont(::core::mem::transmute_copy(&cp), ::core::mem::transmute_copy(&charrep), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&curcharrep), ::core::mem::transmute_copy(&curfontsize), ::core::mem::transmute_copy(&pbstr), ::core::mem::transmute_copy(&ppitchandfamily), ::core::mem::transmute_copy(&pnewfontsize)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrings<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&notify)).into()
        }
        unsafe extern "system" fn Range2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range2(::core::mem::transmute_copy(&cpactive), ::core::mem::transmute_copy(&cpanchor)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint2(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseCallManager(::core::mem::transmute(&pvoid)).into()
        }
        unsafe extern "system" fn ReleaseImmContext<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseImmContext(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn SetEffectColor<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffectColor(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetTypographyOptions<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypographyOptions(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SysBeep<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SysBeep().into()
        }
        unsafe extern "system" fn Update<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UpdateWindow<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateWindow().into()
        }
        unsafe extern "system" fn GetMathProperties<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMathProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *poptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathProperties<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMathProperties(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetActiveStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveStory() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveStory(::core::mem::transmute(&pstory)).into()
        }
        unsafe extern "system" fn GetMainStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainStory() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNewStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNewStory() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStory(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITextDocumentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCaretType: GetCaretType::<Impl, IMPL_OFFSET>,
            SetCaretType: SetCaretType::<Impl, IMPL_OFFSET>,
            GetDisplays: GetDisplays::<Impl, IMPL_OFFSET>,
            GetDocumentFont: GetDocumentFont::<Impl, IMPL_OFFSET>,
            SetDocumentFont: SetDocumentFont::<Impl, IMPL_OFFSET>,
            GetDocumentPara: GetDocumentPara::<Impl, IMPL_OFFSET>,
            SetDocumentPara: SetDocumentPara::<Impl, IMPL_OFFSET>,
            GetEastAsianFlags: GetEastAsianFlags::<Impl, IMPL_OFFSET>,
            GetGenerator: GetGenerator::<Impl, IMPL_OFFSET>,
            SetIMEInProgress: SetIMEInProgress::<Impl, IMPL_OFFSET>,
            GetNotificationMode: GetNotificationMode::<Impl, IMPL_OFFSET>,
            SetNotificationMode: SetNotificationMode::<Impl, IMPL_OFFSET>,
            GetSelection2: GetSelection2::<Impl, IMPL_OFFSET>,
            GetStoryRanges2: GetStoryRanges2::<Impl, IMPL_OFFSET>,
            GetTypographyOptions: GetTypographyOptions::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            GetWindow: GetWindow::<Impl, IMPL_OFFSET>,
            AttachMsgFilter: AttachMsgFilter::<Impl, IMPL_OFFSET>,
            CheckTextLimit: CheckTextLimit::<Impl, IMPL_OFFSET>,
            GetCallManager: GetCallManager::<Impl, IMPL_OFFSET>,
            GetClientRect: GetClientRect::<Impl, IMPL_OFFSET>,
            GetEffectColor: GetEffectColor::<Impl, IMPL_OFFSET>,
            GetImmContext: GetImmContext::<Impl, IMPL_OFFSET>,
            GetPreferredFont: GetPreferredFont::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetStrings: GetStrings::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            Range2: Range2::<Impl, IMPL_OFFSET>,
            RangeFromPoint2: RangeFromPoint2::<Impl, IMPL_OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Impl, IMPL_OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Impl, IMPL_OFFSET>,
            SetEffectColor: SetEffectColor::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            SetTypographyOptions: SetTypographyOptions::<Impl, IMPL_OFFSET>,
            SysBeep: SysBeep::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            UpdateWindow: UpdateWindow::<Impl, IMPL_OFFSET>,
            GetMathProperties: GetMathProperties::<Impl, IMPL_OFFSET>,
            SetMathProperties: SetMathProperties::<Impl, IMPL_OFFSET>,
            GetActiveStory: GetActiveStory::<Impl, IMPL_OFFSET>,
            SetActiveStory: SetActiveStory::<Impl, IMPL_OFFSET>,
            GetMainStory: GetMainStory::<Impl, IMPL_OFFSET>,
            GetNewStory: GetNewStory::<Impl, IMPL_OFFSET>,
            GetStory: GetStory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextDocument2OldImpl: Sized + IDispatchImpl + ITextDocumentImpl {
    fn AttachMsgFilter(&mut self, pfilter: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetEffectColor(&mut self, index: i32, cr: u32) -> ::windows::core::Result<()>;
    fn GetEffectColor(&mut self, index: i32) -> ::windows::core::Result<u32>;
    fn GetCaretType(&mut self) -> ::windows::core::Result<i32>;
    fn SetCaretType(&mut self, carettype: i32) -> ::windows::core::Result<()>;
    fn GetImmContext(&mut self) -> ::windows::core::Result<i64>;
    fn ReleaseImmContext(&mut self, context: i64) -> ::windows::core::Result<()>;
    fn GetPreferredFont(&mut self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::Result<()>;
    fn GetNotificationMode(&mut self) -> ::windows::core::Result<i32>;
    fn SetNotificationMode(&mut self, mode: i32) -> ::windows::core::Result<()>;
    fn GetClientRect(&mut self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()>;
    fn GetSelection2(&mut self) -> ::windows::core::Result<ITextSelection>;
    fn GetWindow(&mut self) -> ::windows::core::Result<i32>;
    fn GetFEFlags(&mut self) -> ::windows::core::Result<i32>;
    fn UpdateWindow(&mut self) -> ::windows::core::Result<()>;
    fn CheckTextLimit(&mut self, cch: i32, pcch: *const i32) -> ::windows::core::Result<()>;
    fn IMEInProgress(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SysBeep(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self, mode: i32) -> ::windows::core::Result<()>;
    fn Notify(&mut self, notify: i32) -> ::windows::core::Result<()>;
    fn GetDocumentFont(&mut self) -> ::windows::core::Result<ITextFont>;
    fn GetDocumentPara(&mut self) -> ::windows::core::Result<ITextPara>;
    fn GetCallManager(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ReleaseCallManager(&mut self, pvoid: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextDocument2OldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2OldImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextDocument2OldVtbl {
        unsafe extern "system" fn AttachMsgFilter<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachMsgFilter(::core::mem::transmute(&pfilter)).into()
        }
        unsafe extern "system" fn SetEffectColor<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, cr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffectColor(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&cr)).into()
        }
        unsafe extern "system" fn GetEffectColor<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pcr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectColor(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretType<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcarettype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaretType() {
                ::core::result::Result::Ok(ok__) => {
                    *pcarettype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, carettype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaretType(::core::mem::transmute_copy(&carettype)).into()
        }
        unsafe extern "system" fn GetImmContext<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImmContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseImmContext(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetPreferredFont<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPreferredFont(::core::mem::transmute_copy(&cp), ::core::mem::transmute_copy(&charrep), ::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&charrepcur), ::core::mem::transmute_copy(&curfontsize), ::core::mem::transmute_copy(&pbstr), ::core::mem::transmute_copy(&ppitchandfamily), ::core::mem::transmute_copy(&pnewfontsize)).into()
        }
        unsafe extern "system" fn GetNotificationMode<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotificationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMode<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn GetClientRect<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClientRect(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)).into()
        }
        unsafe extern "system" fn GetSelection2<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFEFlags<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFEFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateWindow<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateWindow().into()
        }
        unsafe extern "system" fn CheckTextLimit<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckTextLimit(::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pcch)).into()
        }
        unsafe extern "system" fn IMEInProgress<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IMEInProgress(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SysBeep<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SysBeep().into()
        }
        unsafe extern "system" fn Update<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Notify<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&notify)).into()
        }
        unsafe extern "system" fn GetDocumentFont<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitextfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentFont() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitextfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentPara<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitextpara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentPara() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitextpara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallManager<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCallManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppvoid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseCallManager(::core::mem::transmute(&pvoid)).into()
        }
        Self {
            base: ITextDocumentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AttachMsgFilter: AttachMsgFilter::<Impl, IMPL_OFFSET>,
            SetEffectColor: SetEffectColor::<Impl, IMPL_OFFSET>,
            GetEffectColor: GetEffectColor::<Impl, IMPL_OFFSET>,
            GetCaretType: GetCaretType::<Impl, IMPL_OFFSET>,
            SetCaretType: SetCaretType::<Impl, IMPL_OFFSET>,
            GetImmContext: GetImmContext::<Impl, IMPL_OFFSET>,
            ReleaseImmContext: ReleaseImmContext::<Impl, IMPL_OFFSET>,
            GetPreferredFont: GetPreferredFont::<Impl, IMPL_OFFSET>,
            GetNotificationMode: GetNotificationMode::<Impl, IMPL_OFFSET>,
            SetNotificationMode: SetNotificationMode::<Impl, IMPL_OFFSET>,
            GetClientRect: GetClientRect::<Impl, IMPL_OFFSET>,
            GetSelection2: GetSelection2::<Impl, IMPL_OFFSET>,
            GetWindow: GetWindow::<Impl, IMPL_OFFSET>,
            GetFEFlags: GetFEFlags::<Impl, IMPL_OFFSET>,
            UpdateWindow: UpdateWindow::<Impl, IMPL_OFFSET>,
            CheckTextLimit: CheckTextLimit::<Impl, IMPL_OFFSET>,
            IMEInProgress: IMEInProgress::<Impl, IMPL_OFFSET>,
            SysBeep: SysBeep::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
            GetDocumentFont: GetDocumentFont::<Impl, IMPL_OFFSET>,
            GetDocumentPara: GetDocumentPara::<Impl, IMPL_OFFSET>,
            GetCallManager: GetCallManager::<Impl, IMPL_OFFSET>,
            ReleaseCallManager: ReleaseCallManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextDocument2Old as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextFontImpl: Sized + IDispatchImpl {
    fn GetDuplicate(&mut self) -> ::windows::core::Result<ITextFont>;
    fn SetDuplicate(&mut self, pfont: ::core::option::Option<ITextFont>) -> ::windows::core::Result<()>;
    fn CanChange(&mut self) -> ::windows::core::Result<i32>;
    fn IsEqual(&mut self, pfont: ::core::option::Option<ITextFont>) -> ::windows::core::Result<i32>;
    fn Reset(&mut self, value: tomConstants) -> ::windows::core::Result<()>;
    fn GetStyle(&mut self) -> ::windows::core::Result<i32>;
    fn SetStyle(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetAllCaps(&mut self) -> ::windows::core::Result<i32>;
    fn SetAllCaps(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetAnimation(&mut self) -> ::windows::core::Result<i32>;
    fn SetAnimation(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetBackColor(&mut self) -> ::windows::core::Result<i32>;
    fn SetBackColor(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetBold(&mut self) -> ::windows::core::Result<i32>;
    fn SetBold(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetEmboss(&mut self) -> ::windows::core::Result<i32>;
    fn SetEmboss(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetForeColor(&mut self) -> ::windows::core::Result<i32>;
    fn SetForeColor(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetHidden(&mut self) -> ::windows::core::Result<i32>;
    fn SetHidden(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetEngrave(&mut self) -> ::windows::core::Result<i32>;
    fn SetEngrave(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetItalic(&mut self) -> ::windows::core::Result<i32>;
    fn SetItalic(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetKerning(&mut self) -> ::windows::core::Result<f32>;
    fn SetKerning(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetLanguageID(&mut self) -> ::windows::core::Result<i32>;
    fn SetLanguageID(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetOutline(&mut self) -> ::windows::core::Result<i32>;
    fn SetOutline(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetPosition(&mut self) -> ::windows::core::Result<f32>;
    fn SetPosition(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetProtected(&mut self) -> ::windows::core::Result<i32>;
    fn SetProtected(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetShadow(&mut self) -> ::windows::core::Result<i32>;
    fn SetShadow(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetSize(&mut self) -> ::windows::core::Result<f32>;
    fn SetSize(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetSmallCaps(&mut self) -> ::windows::core::Result<i32>;
    fn SetSmallCaps(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetSpacing(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpacing(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetStrikeThrough(&mut self) -> ::windows::core::Result<i32>;
    fn SetStrikeThrough(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetSubscript(&mut self) -> ::windows::core::Result<i32>;
    fn SetSubscript(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetSuperscript(&mut self) -> ::windows::core::Result<i32>;
    fn SetSuperscript(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetUnderline(&mut self) -> ::windows::core::Result<i32>;
    fn SetUnderline(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetWeight(&mut self) -> ::windows::core::Result<i32>;
    fn SetWeight(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextFontVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextFontImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextFontVtbl {
        unsafe extern "system" fn GetDuplicate<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuplicate(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn CanChange<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&pfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStyle<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyle(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAllCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllCaps(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAnimation<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnimation() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimation<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnimation(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBackColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackColor(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBold<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBold() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBold(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEmboss<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmboss() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmboss<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmboss(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetForeColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForeColor(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHidden<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHidden(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEngrave<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEngrave() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEngrave<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEngrave(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetItalic<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItalic() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItalic(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKerning<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKerning() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKerning(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLanguageID<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageID() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageID<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguageID(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetName<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn GetOutline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutline() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPosition<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetProtected<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtected() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtected<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtected(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetShadow<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShadow() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShadow<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShadow(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSize<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSmallCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSmallCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmallCaps(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpacing<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpacing<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpacing(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStrikeThrough<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrikeThrough() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikeThrough<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrikeThrough(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSubscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubscript() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubscript(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSuperscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSuperscript() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuperscript(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnderline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderline() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnderline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetWeight<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeight(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDuplicate: GetDuplicate::<Impl, IMPL_OFFSET>,
            SetDuplicate: SetDuplicate::<Impl, IMPL_OFFSET>,
            CanChange: CanChange::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            GetStyle: GetStyle::<Impl, IMPL_OFFSET>,
            SetStyle: SetStyle::<Impl, IMPL_OFFSET>,
            GetAllCaps: GetAllCaps::<Impl, IMPL_OFFSET>,
            SetAllCaps: SetAllCaps::<Impl, IMPL_OFFSET>,
            GetAnimation: GetAnimation::<Impl, IMPL_OFFSET>,
            SetAnimation: SetAnimation::<Impl, IMPL_OFFSET>,
            GetBackColor: GetBackColor::<Impl, IMPL_OFFSET>,
            SetBackColor: SetBackColor::<Impl, IMPL_OFFSET>,
            GetBold: GetBold::<Impl, IMPL_OFFSET>,
            SetBold: SetBold::<Impl, IMPL_OFFSET>,
            GetEmboss: GetEmboss::<Impl, IMPL_OFFSET>,
            SetEmboss: SetEmboss::<Impl, IMPL_OFFSET>,
            GetForeColor: GetForeColor::<Impl, IMPL_OFFSET>,
            SetForeColor: SetForeColor::<Impl, IMPL_OFFSET>,
            GetHidden: GetHidden::<Impl, IMPL_OFFSET>,
            SetHidden: SetHidden::<Impl, IMPL_OFFSET>,
            GetEngrave: GetEngrave::<Impl, IMPL_OFFSET>,
            SetEngrave: SetEngrave::<Impl, IMPL_OFFSET>,
            GetItalic: GetItalic::<Impl, IMPL_OFFSET>,
            SetItalic: SetItalic::<Impl, IMPL_OFFSET>,
            GetKerning: GetKerning::<Impl, IMPL_OFFSET>,
            SetKerning: SetKerning::<Impl, IMPL_OFFSET>,
            GetLanguageID: GetLanguageID::<Impl, IMPL_OFFSET>,
            SetLanguageID: SetLanguageID::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            GetOutline: GetOutline::<Impl, IMPL_OFFSET>,
            SetOutline: SetOutline::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            GetProtected: GetProtected::<Impl, IMPL_OFFSET>,
            SetProtected: SetProtected::<Impl, IMPL_OFFSET>,
            GetShadow: GetShadow::<Impl, IMPL_OFFSET>,
            SetShadow: SetShadow::<Impl, IMPL_OFFSET>,
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            GetSmallCaps: GetSmallCaps::<Impl, IMPL_OFFSET>,
            SetSmallCaps: SetSmallCaps::<Impl, IMPL_OFFSET>,
            GetSpacing: GetSpacing::<Impl, IMPL_OFFSET>,
            SetSpacing: SetSpacing::<Impl, IMPL_OFFSET>,
            GetStrikeThrough: GetStrikeThrough::<Impl, IMPL_OFFSET>,
            SetStrikeThrough: SetStrikeThrough::<Impl, IMPL_OFFSET>,
            GetSubscript: GetSubscript::<Impl, IMPL_OFFSET>,
            SetSubscript: SetSubscript::<Impl, IMPL_OFFSET>,
            GetSuperscript: GetSuperscript::<Impl, IMPL_OFFSET>,
            SetSuperscript: SetSuperscript::<Impl, IMPL_OFFSET>,
            GetUnderline: GetUnderline::<Impl, IMPL_OFFSET>,
            SetUnderline: SetUnderline::<Impl, IMPL_OFFSET>,
            GetWeight: GetWeight::<Impl, IMPL_OFFSET>,
            SetWeight: SetWeight::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextFont as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextFont2Impl: Sized + IDispatchImpl + ITextFontImpl {
    fn GetCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetAutoLigatures(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutoLigatures(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetAutospaceAlpha(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutospaceAlpha(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetAutospaceNumeric(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutospaceNumeric(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetAutospaceParens(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutospaceParens(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCharRep(&mut self) -> ::windows::core::Result<i32>;
    fn SetCharRep(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCompressionMode(&mut self) -> ::windows::core::Result<i32>;
    fn SetCompressionMode(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCookie(&mut self) -> ::windows::core::Result<i32>;
    fn SetCookie(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetDoubleStrike(&mut self) -> ::windows::core::Result<i32>;
    fn SetDoubleStrike(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetDuplicate2(&mut self) -> ::windows::core::Result<ITextFont2>;
    fn SetDuplicate2(&mut self, pfont: ::core::option::Option<ITextFont2>) -> ::windows::core::Result<()>;
    fn GetLinkType(&mut self) -> ::windows::core::Result<i32>;
    fn GetMathZone(&mut self) -> ::windows::core::Result<i32>;
    fn SetMathZone(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetModWidthPairs(&mut self) -> ::windows::core::Result<i32>;
    fn SetModWidthPairs(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetModWidthSpace(&mut self) -> ::windows::core::Result<i32>;
    fn SetModWidthSpace(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetOldNumbers(&mut self) -> ::windows::core::Result<i32>;
    fn SetOldNumbers(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetOverlapping(&mut self) -> ::windows::core::Result<i32>;
    fn SetOverlapping(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetPositionSubSuper(&mut self) -> ::windows::core::Result<i32>;
    fn SetPositionSubSuper(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetScaling(&mut self) -> ::windows::core::Result<i32>;
    fn SetScaling(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetSpaceExtension(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpaceExtension(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetUnderlinePositionMode(&mut self) -> ::windows::core::Result<i32>;
    fn SetUnderlinePositionMode(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetEffects(&mut self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()>;
    fn GetEffects2(&mut self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, r#type: i32) -> ::windows::core::Result<i32>;
    fn GetPropertyInfo(&mut self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::core::Result<()>;
    fn IsEqual2(&mut self, pfont: ::core::option::Option<ITextFont2>) -> ::windows::core::Result<i32>;
    fn SetEffects(&mut self, value: i32, mask: i32) -> ::windows::core::Result<()>;
    fn SetEffects2(&mut self, value: i32, mask: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextFont2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextFont2Vtbl {
        unsafe extern "system" fn GetCount<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoLigatures<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoLigatures() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoLigatures<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoLigatures(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceAlpha<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutospaceAlpha() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceAlpha<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutospaceAlpha(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceNumeric<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutospaceNumeric() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceNumeric<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutospaceNumeric(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAutospaceParens<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutospaceParens() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceParens<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutospaceParens(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCharRep<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharRep() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharRep<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharRep(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCompressionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompressionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompressionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompressionMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCookie<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCookie() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCookie<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCookie(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDoubleStrike<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDoubleStrike() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoubleStrike<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoubleStrike(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuplicate2(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn GetLinkType<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMathZone<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMathZone() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathZone<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMathZone(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetModWidthPairs<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModWidthPairs() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModWidthPairs<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModWidthPairs(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetModWidthSpace<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModWidthSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModWidthSpace<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModWidthSpace(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetOldNumbers<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOldNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOldNumbers<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOldNumbers(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetOverlapping<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverlapping() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlapping<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverlapping(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPositionSubSuper<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPositionSubSuper() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionSubSuper<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPositionSubSuper(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetScaling<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScaling() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaling<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaling(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpaceExtension<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpaceExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceExtension<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpaceExtension(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnderlinePositionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderlinePositionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderlinePositionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnderlinePositionMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEffects<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffects(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetEffects2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffects2(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyInfo(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn IsEqual2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual2(::core::mem::transmute(&pfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *pb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffects(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetEffects2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffects2(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ITextFontVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAutoLigatures: GetAutoLigatures::<Impl, IMPL_OFFSET>,
            SetAutoLigatures: SetAutoLigatures::<Impl, IMPL_OFFSET>,
            GetAutospaceAlpha: GetAutospaceAlpha::<Impl, IMPL_OFFSET>,
            SetAutospaceAlpha: SetAutospaceAlpha::<Impl, IMPL_OFFSET>,
            GetAutospaceNumeric: GetAutospaceNumeric::<Impl, IMPL_OFFSET>,
            SetAutospaceNumeric: SetAutospaceNumeric::<Impl, IMPL_OFFSET>,
            GetAutospaceParens: GetAutospaceParens::<Impl, IMPL_OFFSET>,
            SetAutospaceParens: SetAutospaceParens::<Impl, IMPL_OFFSET>,
            GetCharRep: GetCharRep::<Impl, IMPL_OFFSET>,
            SetCharRep: SetCharRep::<Impl, IMPL_OFFSET>,
            GetCompressionMode: GetCompressionMode::<Impl, IMPL_OFFSET>,
            SetCompressionMode: SetCompressionMode::<Impl, IMPL_OFFSET>,
            GetCookie: GetCookie::<Impl, IMPL_OFFSET>,
            SetCookie: SetCookie::<Impl, IMPL_OFFSET>,
            GetDoubleStrike: GetDoubleStrike::<Impl, IMPL_OFFSET>,
            SetDoubleStrike: SetDoubleStrike::<Impl, IMPL_OFFSET>,
            GetDuplicate2: GetDuplicate2::<Impl, IMPL_OFFSET>,
            SetDuplicate2: SetDuplicate2::<Impl, IMPL_OFFSET>,
            GetLinkType: GetLinkType::<Impl, IMPL_OFFSET>,
            GetMathZone: GetMathZone::<Impl, IMPL_OFFSET>,
            SetMathZone: SetMathZone::<Impl, IMPL_OFFSET>,
            GetModWidthPairs: GetModWidthPairs::<Impl, IMPL_OFFSET>,
            SetModWidthPairs: SetModWidthPairs::<Impl, IMPL_OFFSET>,
            GetModWidthSpace: GetModWidthSpace::<Impl, IMPL_OFFSET>,
            SetModWidthSpace: SetModWidthSpace::<Impl, IMPL_OFFSET>,
            GetOldNumbers: GetOldNumbers::<Impl, IMPL_OFFSET>,
            SetOldNumbers: SetOldNumbers::<Impl, IMPL_OFFSET>,
            GetOverlapping: GetOverlapping::<Impl, IMPL_OFFSET>,
            SetOverlapping: SetOverlapping::<Impl, IMPL_OFFSET>,
            GetPositionSubSuper: GetPositionSubSuper::<Impl, IMPL_OFFSET>,
            SetPositionSubSuper: SetPositionSubSuper::<Impl, IMPL_OFFSET>,
            GetScaling: GetScaling::<Impl, IMPL_OFFSET>,
            SetScaling: SetScaling::<Impl, IMPL_OFFSET>,
            GetSpaceExtension: GetSpaceExtension::<Impl, IMPL_OFFSET>,
            SetSpaceExtension: SetSpaceExtension::<Impl, IMPL_OFFSET>,
            GetUnderlinePositionMode: GetUnderlinePositionMode::<Impl, IMPL_OFFSET>,
            SetUnderlinePositionMode: SetUnderlinePositionMode::<Impl, IMPL_OFFSET>,
            GetEffects: GetEffects::<Impl, IMPL_OFFSET>,
            GetEffects2: GetEffects2::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Impl, IMPL_OFFSET>,
            IsEqual2: IsEqual2::<Impl, IMPL_OFFSET>,
            SetEffects: SetEffects::<Impl, IMPL_OFFSET>,
            SetEffects2: SetEffects2::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextFont2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHostImpl: Sized {
    fn TxGetDC(&mut self) -> super::super::super::Graphics::Gdi::HDC;
    fn TxReleaseDC(&mut self, hdc: super::super::super::Graphics::Gdi::HDC) -> i32;
    fn TxShowScrollBar(&mut self, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxEnableScrollBar(&mut self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL;
    fn TxSetScrollRange(&mut self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxSetScrollPos(&mut self, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxInvalidateRect(&mut self, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL);
    fn TxViewChange(&mut self, fupdate: super::super::super::Foundation::BOOL);
    fn TxCreateCaret(&mut self, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL;
    fn TxShowCaret(&mut self, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    fn TxSetCaretPos(&mut self, x: i32, y: i32) -> super::super::super::Foundation::BOOL;
    fn TxSetTimer(&mut self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL;
    fn TxKillTimer(&mut self, idtimer: u32);
    fn TxScrollWindowEx(&mut self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD);
    fn TxSetCapture(&mut self, fcapture: super::super::super::Foundation::BOOL);
    fn TxSetFocus(&mut self);
    fn TxSetCursor(&mut self, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL);
    fn TxScreenToClient(&mut self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    fn TxClientToScreen(&mut self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    fn TxActivate(&mut self, ploldstate: *mut i32) -> ::windows::core::Result<()>;
    fn TxDeactivate(&mut self, lnewstate: i32) -> ::windows::core::Result<()>;
    fn TxGetClientRect(&mut self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn TxGetViewInset(&mut self, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn TxGetCharFormat(&mut self, ppcf: *const *const CHARFORMATW) -> ::windows::core::Result<()>;
    fn TxGetParaFormat(&mut self, pppf: *const *const PARAFORMAT) -> ::windows::core::Result<()>;
    fn TxGetSysColor(&mut self, nindex: i32) -> u32;
    fn TxGetBackStyle(&mut self, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::Result<()>;
    fn TxGetMaxLength(&mut self, plength: *mut u32) -> ::windows::core::Result<()>;
    fn TxGetScrollBars(&mut self, pdwscrollbar: *mut u32) -> ::windows::core::Result<()>;
    fn TxGetPasswordChar(&mut self) -> ::windows::core::Result<i8>;
    fn TxGetAcceleratorPos(&mut self, pcp: *mut i32) -> ::windows::core::Result<()>;
    fn TxGetExtent(&mut self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::Result<()>;
    fn OnTxCharFormatChange(&mut self, pcf: *const CHARFORMATW) -> ::windows::core::Result<()>;
    fn OnTxParaFormatChange(&mut self, ppf: *const PARAFORMAT) -> ::windows::core::Result<()>;
    fn TxGetPropertyBits(&mut self, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::Result<()>;
    fn TxNotify(&mut self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn TxImmGetContext(&mut self) -> super::super::super::Globalization::HIMC;
    fn TxImmReleaseContext(&mut self, himc: super::super::super::Globalization::HIMC);
    fn TxGetSelectionBarWidth(&mut self, lselbarwidth: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextHostVtbl {
        unsafe extern "system" fn TxGetDC<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetDC()
        }
        unsafe extern "system" fn TxReleaseDC<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxReleaseDC(::core::mem::transmute_copy(&hdc))
        }
        unsafe extern "system" fn TxShowScrollBar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxShowScrollBar(::core::mem::transmute_copy(&fnbar), ::core::mem::transmute_copy(&fshow))
        }
        unsafe extern "system" fn TxEnableScrollBar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxEnableScrollBar(::core::mem::transmute_copy(&fusbflags), ::core::mem::transmute_copy(&fuarrowflags))
        }
        unsafe extern "system" fn TxSetScrollRange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetScrollRange(::core::mem::transmute_copy(&fnbar), ::core::mem::transmute_copy(&nminpos), ::core::mem::transmute_copy(&nmaxpos), ::core::mem::transmute_copy(&fredraw))
        }
        unsafe extern "system" fn TxSetScrollPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetScrollPos(::core::mem::transmute_copy(&fnbar), ::core::mem::transmute_copy(&npos), ::core::mem::transmute_copy(&fredraw))
        }
        unsafe extern "system" fn TxInvalidateRect<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxInvalidateRect(::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&fmode))
        }
        unsafe extern "system" fn TxViewChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fupdate: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxViewChange(::core::mem::transmute_copy(&fupdate))
        }
        unsafe extern "system" fn TxCreateCaret<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxCreateCaret(::core::mem::transmute_copy(&hbmp), ::core::mem::transmute_copy(&xwidth), ::core::mem::transmute_copy(&yheight))
        }
        unsafe extern "system" fn TxShowCaret<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxShowCaret(::core::mem::transmute_copy(&fshow))
        }
        unsafe extern "system" fn TxSetCaretPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetCaretPos(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y))
        }
        unsafe extern "system" fn TxSetTimer<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetTimer(::core::mem::transmute_copy(&idtimer), ::core::mem::transmute_copy(&utimeout))
        }
        unsafe extern "system" fn TxKillTimer<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idtimer: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxKillTimer(::core::mem::transmute_copy(&idtimer))
        }
        unsafe extern "system" fn TxScrollWindowEx<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxScrollWindowEx(::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&lprcscroll), ::core::mem::transmute_copy(&lprcclip), ::core::mem::transmute_copy(&hrgnupdate), ::core::mem::transmute_copy(&lprcupdate), ::core::mem::transmute_copy(&fuscroll))
        }
        unsafe extern "system" fn TxSetCapture<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetCapture(::core::mem::transmute_copy(&fcapture))
        }
        unsafe extern "system" fn TxSetFocus<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetFocus()
        }
        unsafe extern "system" fn TxSetCursor<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetCursor(::core::mem::transmute_copy(&hcur), ::core::mem::transmute_copy(&ftext))
        }
        unsafe extern "system" fn TxScreenToClient<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxScreenToClient(::core::mem::transmute_copy(&lppt))
        }
        unsafe extern "system" fn TxClientToScreen<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxClientToScreen(::core::mem::transmute_copy(&lppt))
        }
        unsafe extern "system" fn TxActivate<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploldstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxActivate(::core::mem::transmute_copy(&ploldstate)).into()
        }
        unsafe extern "system" fn TxDeactivate<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewstate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxDeactivate(::core::mem::transmute_copy(&lnewstate)).into()
        }
        unsafe extern "system" fn TxGetClientRect<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetClientRect(::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxGetViewInset<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetViewInset(::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxGetCharFormat<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetCharFormat(::core::mem::transmute_copy(&ppcf)).into()
        }
        unsafe extern "system" fn TxGetParaFormat<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppf: *const *const PARAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetParaFormat(::core::mem::transmute_copy(&pppf)).into()
        }
        unsafe extern "system" fn TxGetSysColor<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetSysColor(::core::mem::transmute_copy(&nindex))
        }
        unsafe extern "system" fn TxGetBackStyle<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetBackStyle(::core::mem::transmute_copy(&pstyle)).into()
        }
        unsafe extern "system" fn TxGetMaxLength<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetMaxLength(::core::mem::transmute_copy(&plength)).into()
        }
        unsafe extern "system" fn TxGetScrollBars<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscrollbar: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetScrollBars(::core::mem::transmute_copy(&pdwscrollbar)).into()
        }
        unsafe extern "system" fn TxGetPasswordChar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pch: *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetPasswordChar() {
                ::core::result::Result::Ok(ok__) => {
                    *pch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetAcceleratorPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetAcceleratorPos(::core::mem::transmute_copy(&pcp)).into()
        }
        unsafe extern "system" fn TxGetExtent<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetExtent(::core::mem::transmute_copy(&lpextent)).into()
        }
        unsafe extern "system" fn OnTxCharFormatChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: *const CHARFORMATW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTxCharFormatChange(::core::mem::transmute_copy(&pcf)).into()
        }
        unsafe extern "system" fn OnTxParaFormatChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppf: *const PARAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTxParaFormatChange(::core::mem::transmute_copy(&ppf)).into()
        }
        unsafe extern "system" fn TxGetPropertyBits<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetPropertyBits(::core::mem::transmute_copy(&dwmask), ::core::mem::transmute_copy(&pdwbits)).into()
        }
        unsafe extern "system" fn TxNotify<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxNotify(::core::mem::transmute_copy(&inotify), ::core::mem::transmute_copy(&pv)).into()
        }
        unsafe extern "system" fn TxImmGetContext<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Globalization::HIMC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxImmGetContext()
        }
        unsafe extern "system" fn TxImmReleaseContext<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxImmReleaseContext(::core::mem::transmute_copy(&himc))
        }
        unsafe extern "system" fn TxGetSelectionBarWidth<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lselbarwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetSelectionBarWidth(::core::mem::transmute_copy(&lselbarwidth)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TxGetDC: TxGetDC::<Impl, IMPL_OFFSET>,
            TxReleaseDC: TxReleaseDC::<Impl, IMPL_OFFSET>,
            TxShowScrollBar: TxShowScrollBar::<Impl, IMPL_OFFSET>,
            TxEnableScrollBar: TxEnableScrollBar::<Impl, IMPL_OFFSET>,
            TxSetScrollRange: TxSetScrollRange::<Impl, IMPL_OFFSET>,
            TxSetScrollPos: TxSetScrollPos::<Impl, IMPL_OFFSET>,
            TxInvalidateRect: TxInvalidateRect::<Impl, IMPL_OFFSET>,
            TxViewChange: TxViewChange::<Impl, IMPL_OFFSET>,
            TxCreateCaret: TxCreateCaret::<Impl, IMPL_OFFSET>,
            TxShowCaret: TxShowCaret::<Impl, IMPL_OFFSET>,
            TxSetCaretPos: TxSetCaretPos::<Impl, IMPL_OFFSET>,
            TxSetTimer: TxSetTimer::<Impl, IMPL_OFFSET>,
            TxKillTimer: TxKillTimer::<Impl, IMPL_OFFSET>,
            TxScrollWindowEx: TxScrollWindowEx::<Impl, IMPL_OFFSET>,
            TxSetCapture: TxSetCapture::<Impl, IMPL_OFFSET>,
            TxSetFocus: TxSetFocus::<Impl, IMPL_OFFSET>,
            TxSetCursor: TxSetCursor::<Impl, IMPL_OFFSET>,
            TxScreenToClient: TxScreenToClient::<Impl, IMPL_OFFSET>,
            TxClientToScreen: TxClientToScreen::<Impl, IMPL_OFFSET>,
            TxActivate: TxActivate::<Impl, IMPL_OFFSET>,
            TxDeactivate: TxDeactivate::<Impl, IMPL_OFFSET>,
            TxGetClientRect: TxGetClientRect::<Impl, IMPL_OFFSET>,
            TxGetViewInset: TxGetViewInset::<Impl, IMPL_OFFSET>,
            TxGetCharFormat: TxGetCharFormat::<Impl, IMPL_OFFSET>,
            TxGetParaFormat: TxGetParaFormat::<Impl, IMPL_OFFSET>,
            TxGetSysColor: TxGetSysColor::<Impl, IMPL_OFFSET>,
            TxGetBackStyle: TxGetBackStyle::<Impl, IMPL_OFFSET>,
            TxGetMaxLength: TxGetMaxLength::<Impl, IMPL_OFFSET>,
            TxGetScrollBars: TxGetScrollBars::<Impl, IMPL_OFFSET>,
            TxGetPasswordChar: TxGetPasswordChar::<Impl, IMPL_OFFSET>,
            TxGetAcceleratorPos: TxGetAcceleratorPos::<Impl, IMPL_OFFSET>,
            TxGetExtent: TxGetExtent::<Impl, IMPL_OFFSET>,
            OnTxCharFormatChange: OnTxCharFormatChange::<Impl, IMPL_OFFSET>,
            OnTxParaFormatChange: OnTxParaFormatChange::<Impl, IMPL_OFFSET>,
            TxGetPropertyBits: TxGetPropertyBits::<Impl, IMPL_OFFSET>,
            TxNotify: TxNotify::<Impl, IMPL_OFFSET>,
            TxImmGetContext: TxImmGetContext::<Impl, IMPL_OFFSET>,
            TxImmReleaseContext: TxImmReleaseContext::<Impl, IMPL_OFFSET>,
            TxGetSelectionBarWidth: TxGetSelectionBarWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITextHost2Impl: Sized + ITextHostImpl {
    fn TxIsDoubleClickPending(&mut self) -> super::super::super::Foundation::BOOL;
    fn TxGetWindow(&mut self, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn TxSetForegroundWindow(&mut self) -> ::windows::core::Result<()>;
    fn TxGetPalette(&mut self) -> super::super::super::Graphics::Gdi::HPALETTE;
    fn TxGetEastAsianFlags(&mut self, pflags: *mut i32) -> ::windows::core::Result<()>;
    fn TxSetCursor2(&mut self, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR;
    fn TxFreeTextServicesNotification(&mut self);
    fn TxGetEditStyle(&mut self, dwitem: u32, pdwdata: *mut u32) -> ::windows::core::Result<()>;
    fn TxGetWindowStyles(&mut self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::core::Result<()>;
    fn TxShowDropCaret(&mut self, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn TxDestroyCaret(&mut self) -> ::windows::core::Result<()>;
    fn TxGetHorzExtent(&mut self, plhorzextent: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ITextHost2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextHost2Vtbl {
        unsafe extern "system" fn TxIsDoubleClickPending<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxIsDoubleClickPending()
        }
        unsafe extern "system" fn TxGetWindow<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetWindow(::core::mem::transmute_copy(&phwnd)).into()
        }
        unsafe extern "system" fn TxSetForegroundWindow<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetForegroundWindow().into()
        }
        unsafe extern "system" fn TxGetPalette<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetPalette()
        }
        unsafe extern "system" fn TxGetEastAsianFlags<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetEastAsianFlags(::core::mem::transmute_copy(&pflags)).into()
        }
        unsafe extern "system" fn TxSetCursor2<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetCursor2(::core::mem::transmute_copy(&hcur), ::core::mem::transmute_copy(&btext))
        }
        unsafe extern "system" fn TxFreeTextServicesNotification<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxFreeTextServicesNotification()
        }
        unsafe extern "system" fn TxGetEditStyle<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetEditStyle(::core::mem::transmute_copy(&dwitem), ::core::mem::transmute_copy(&pdwdata)).into()
        }
        unsafe extern "system" fn TxGetWindowStyles<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetWindowStyles(::core::mem::transmute_copy(&pdwstyle), ::core::mem::transmute_copy(&pdwexstyle)).into()
        }
        unsafe extern "system" fn TxShowDropCaret<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxShowDropCaret(::core::mem::transmute_copy(&fshow), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn TxDestroyCaret<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxDestroyCaret().into()
        }
        unsafe extern "system" fn TxGetHorzExtent<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhorzextent: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetHorzExtent(::core::mem::transmute_copy(&plhorzextent)).into()
        }
        Self {
            base: ITextHostVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TxIsDoubleClickPending: TxIsDoubleClickPending::<Impl, IMPL_OFFSET>,
            TxGetWindow: TxGetWindow::<Impl, IMPL_OFFSET>,
            TxSetForegroundWindow: TxSetForegroundWindow::<Impl, IMPL_OFFSET>,
            TxGetPalette: TxGetPalette::<Impl, IMPL_OFFSET>,
            TxGetEastAsianFlags: TxGetEastAsianFlags::<Impl, IMPL_OFFSET>,
            TxSetCursor2: TxSetCursor2::<Impl, IMPL_OFFSET>,
            TxFreeTextServicesNotification: TxFreeTextServicesNotification::<Impl, IMPL_OFFSET>,
            TxGetEditStyle: TxGetEditStyle::<Impl, IMPL_OFFSET>,
            TxGetWindowStyles: TxGetWindowStyles::<Impl, IMPL_OFFSET>,
            TxShowDropCaret: TxShowDropCaret::<Impl, IMPL_OFFSET>,
            TxDestroyCaret: TxDestroyCaret::<Impl, IMPL_OFFSET>,
            TxGetHorzExtent: TxGetHorzExtent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextHost2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextParaImpl: Sized + IDispatchImpl {
    fn GetDuplicate(&mut self) -> ::windows::core::Result<ITextPara>;
    fn SetDuplicate(&mut self, ppara: ::core::option::Option<ITextPara>) -> ::windows::core::Result<()>;
    fn CanChange(&mut self) -> ::windows::core::Result<i32>;
    fn IsEqual(&mut self, ppara: ::core::option::Option<ITextPara>) -> ::windows::core::Result<i32>;
    fn Reset(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetStyle(&mut self) -> ::windows::core::Result<i32>;
    fn SetStyle(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetAlignment(&mut self) -> ::windows::core::Result<i32>;
    fn SetAlignment(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetHyphenation(&mut self) -> ::windows::core::Result<tomConstants>;
    fn SetHyphenation(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetFirstLineIndent(&mut self) -> ::windows::core::Result<f32>;
    fn GetKeepTogether(&mut self) -> ::windows::core::Result<tomConstants>;
    fn SetKeepTogether(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetKeepWithNext(&mut self) -> ::windows::core::Result<tomConstants>;
    fn SetKeepWithNext(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetLeftIndent(&mut self) -> ::windows::core::Result<f32>;
    fn GetLineSpacing(&mut self) -> ::windows::core::Result<f32>;
    fn GetLineSpacingRule(&mut self) -> ::windows::core::Result<i32>;
    fn GetListAlignment(&mut self) -> ::windows::core::Result<i32>;
    fn SetListAlignment(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetListLevelIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetListLevelIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetListStart(&mut self) -> ::windows::core::Result<i32>;
    fn SetListStart(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetListTab(&mut self) -> ::windows::core::Result<f32>;
    fn SetListTab(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetListType(&mut self) -> ::windows::core::Result<i32>;
    fn SetListType(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetNoLineNumber(&mut self) -> ::windows::core::Result<i32>;
    fn SetNoLineNumber(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetPageBreakBefore(&mut self) -> ::windows::core::Result<i32>;
    fn SetPageBreakBefore(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetRightIndent(&mut self) -> ::windows::core::Result<f32>;
    fn SetRightIndent(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn SetIndents(&mut self, first: f32, left: f32, right: f32) -> ::windows::core::Result<()>;
    fn SetLineSpacing(&mut self, rule: i32, spacing: f32) -> ::windows::core::Result<()>;
    fn GetSpaceAfter(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpaceAfter(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetSpaceBefore(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpaceBefore(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn GetWidowControl(&mut self) -> ::windows::core::Result<i32>;
    fn SetWidowControl(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetTabCount(&mut self) -> ::windows::core::Result<i32>;
    fn AddTab(&mut self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::Result<()>;
    fn ClearAllTabs(&mut self) -> ::windows::core::Result<()>;
    fn DeleteTab(&mut self, tbpos: f32) -> ::windows::core::Result<()>;
    fn GetTab(&mut self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextParaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextParaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextParaVtbl {
        unsafe extern "system" fn GetDuplicate<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuplicate(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn CanChange<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&ppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStyle<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyle(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHyphenation<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHyphenation() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyphenation<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHyphenation(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFirstLineIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstLineIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeepTogether<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeepTogether() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepTogether(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepWithNext<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeepWithNext() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepWithNext(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLeftIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLeftIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacing<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacingRule<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineSpacingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListLevelIndex<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListLevelIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListLevelIndex(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListStart<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStart<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListStart(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListTab() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListTab(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetListType<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListType<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNoLineNumber<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNoLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoLineNumber(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPageBreakBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageBreakBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPageBreakBefore(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetRightIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRightIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRightIndent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetIndents<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: f32, left: f32, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndents(::core::mem::transmute_copy(&first), ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn SetLineSpacing<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: i32, spacing: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineSpacing(::core::mem::transmute_copy(&rule), ::core::mem::transmute_copy(&spacing)).into()
        }
        unsafe extern "system" fn GetSpaceAfter<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpaceAfter() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpaceAfter(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSpaceBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpaceBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpaceBefore(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetWidowControl<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWidowControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidowControl<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidowControl(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTabCount<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTabCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTab(::core::mem::transmute_copy(&tbpos), ::core::mem::transmute_copy(&tbalign), ::core::mem::transmute_copy(&tbleader)).into()
        }
        unsafe extern "system" fn ClearAllTabs<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAllTabs().into()
        }
        unsafe extern "system" fn DeleteTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tbpos: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTab(::core::mem::transmute_copy(&tbpos)).into()
        }
        unsafe extern "system" fn GetTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTab(::core::mem::transmute_copy(&itab), ::core::mem::transmute_copy(&ptbpos), ::core::mem::transmute_copy(&ptbalign), ::core::mem::transmute_copy(&ptbleader)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDuplicate: GetDuplicate::<Impl, IMPL_OFFSET>,
            SetDuplicate: SetDuplicate::<Impl, IMPL_OFFSET>,
            CanChange: CanChange::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            GetStyle: GetStyle::<Impl, IMPL_OFFSET>,
            SetStyle: SetStyle::<Impl, IMPL_OFFSET>,
            GetAlignment: GetAlignment::<Impl, IMPL_OFFSET>,
            SetAlignment: SetAlignment::<Impl, IMPL_OFFSET>,
            GetHyphenation: GetHyphenation::<Impl, IMPL_OFFSET>,
            SetHyphenation: SetHyphenation::<Impl, IMPL_OFFSET>,
            GetFirstLineIndent: GetFirstLineIndent::<Impl, IMPL_OFFSET>,
            GetKeepTogether: GetKeepTogether::<Impl, IMPL_OFFSET>,
            SetKeepTogether: SetKeepTogether::<Impl, IMPL_OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Impl, IMPL_OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Impl, IMPL_OFFSET>,
            GetLeftIndent: GetLeftIndent::<Impl, IMPL_OFFSET>,
            GetLineSpacing: GetLineSpacing::<Impl, IMPL_OFFSET>,
            GetLineSpacingRule: GetLineSpacingRule::<Impl, IMPL_OFFSET>,
            GetListAlignment: GetListAlignment::<Impl, IMPL_OFFSET>,
            SetListAlignment: SetListAlignment::<Impl, IMPL_OFFSET>,
            GetListLevelIndex: GetListLevelIndex::<Impl, IMPL_OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Impl, IMPL_OFFSET>,
            GetListStart: GetListStart::<Impl, IMPL_OFFSET>,
            SetListStart: SetListStart::<Impl, IMPL_OFFSET>,
            GetListTab: GetListTab::<Impl, IMPL_OFFSET>,
            SetListTab: SetListTab::<Impl, IMPL_OFFSET>,
            GetListType: GetListType::<Impl, IMPL_OFFSET>,
            SetListType: SetListType::<Impl, IMPL_OFFSET>,
            GetNoLineNumber: GetNoLineNumber::<Impl, IMPL_OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Impl, IMPL_OFFSET>,
            GetPageBreakBefore: GetPageBreakBefore::<Impl, IMPL_OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Impl, IMPL_OFFSET>,
            GetRightIndent: GetRightIndent::<Impl, IMPL_OFFSET>,
            SetRightIndent: SetRightIndent::<Impl, IMPL_OFFSET>,
            SetIndents: SetIndents::<Impl, IMPL_OFFSET>,
            SetLineSpacing: SetLineSpacing::<Impl, IMPL_OFFSET>,
            GetSpaceAfter: GetSpaceAfter::<Impl, IMPL_OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Impl, IMPL_OFFSET>,
            GetSpaceBefore: GetSpaceBefore::<Impl, IMPL_OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Impl, IMPL_OFFSET>,
            GetWidowControl: GetWidowControl::<Impl, IMPL_OFFSET>,
            SetWidowControl: SetWidowControl::<Impl, IMPL_OFFSET>,
            GetTabCount: GetTabCount::<Impl, IMPL_OFFSET>,
            AddTab: AddTab::<Impl, IMPL_OFFSET>,
            ClearAllTabs: ClearAllTabs::<Impl, IMPL_OFFSET>,
            DeleteTab: DeleteTab::<Impl, IMPL_OFFSET>,
            GetTab: GetTab::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPara as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextPara2Impl: Sized + IDispatchImpl + ITextParaImpl {
    fn GetBorders(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetDuplicate2(&mut self) -> ::windows::core::Result<ITextPara2>;
    fn SetDuplicate2(&mut self, ppara: ::core::option::Option<ITextPara2>) -> ::windows::core::Result<()>;
    fn GetFontAlignment(&mut self) -> ::windows::core::Result<i32>;
    fn SetFontAlignment(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetHangingPunctuation(&mut self) -> ::windows::core::Result<i32>;
    fn SetHangingPunctuation(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetSnapToGrid(&mut self) -> ::windows::core::Result<i32>;
    fn SetSnapToGrid(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetTrimPunctuationAtStart(&mut self) -> ::windows::core::Result<i32>;
    fn SetTrimPunctuationAtStart(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetEffects(&mut self, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, r#type: i32) -> ::windows::core::Result<i32>;
    fn IsEqual2(&mut self, ppara: ::core::option::Option<ITextPara2>) -> ::windows::core::Result<i32>;
    fn SetEffects(&mut self, value: i32, mask: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextPara2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPara2Vtbl {
        unsafe extern "system" fn GetBorders<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppborders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBorders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppborders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate2() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuplicate2(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn GetFontAlignment<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontAlignment<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHangingPunctuation<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHangingPunctuation() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHangingPunctuation<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHangingPunctuation(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSnapToGrid<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapToGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapToGrid<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapToGrid(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTrimPunctuationAtStart<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrimPunctuationAtStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimPunctuationAtStart<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrimPunctuationAtStart(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEffects<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffects(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual2(::core::mem::transmute(&ppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *pb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEffects(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ITextParaVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetBorders: GetBorders::<Impl, IMPL_OFFSET>,
            GetDuplicate2: GetDuplicate2::<Impl, IMPL_OFFSET>,
            SetDuplicate2: SetDuplicate2::<Impl, IMPL_OFFSET>,
            GetFontAlignment: GetFontAlignment::<Impl, IMPL_OFFSET>,
            SetFontAlignment: SetFontAlignment::<Impl, IMPL_OFFSET>,
            GetHangingPunctuation: GetHangingPunctuation::<Impl, IMPL_OFFSET>,
            SetHangingPunctuation: SetHangingPunctuation::<Impl, IMPL_OFFSET>,
            GetSnapToGrid: GetSnapToGrid::<Impl, IMPL_OFFSET>,
            SetSnapToGrid: SetSnapToGrid::<Impl, IMPL_OFFSET>,
            GetTrimPunctuationAtStart: GetTrimPunctuationAtStart::<Impl, IMPL_OFFSET>,
            SetTrimPunctuationAtStart: SetTrimPunctuationAtStart::<Impl, IMPL_OFFSET>,
            GetEffects: GetEffects::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            IsEqual2: IsEqual2::<Impl, IMPL_OFFSET>,
            SetEffects: SetEffects::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPara2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRangeImpl: Sized + IDispatchImpl {
    fn GetText(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetText(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetChar(&mut self) -> ::windows::core::Result<i32>;
    fn SetChar(&mut self, char: i32) -> ::windows::core::Result<()>;
    fn GetDuplicate(&mut self) -> ::windows::core::Result<ITextRange>;
    fn GetFormattedText(&mut self) -> ::windows::core::Result<ITextRange>;
    fn SetFormattedText(&mut self, prange: ::core::option::Option<ITextRange>) -> ::windows::core::Result<()>;
    fn GetStart(&mut self) -> ::windows::core::Result<i32>;
    fn SetStart(&mut self, cpfirst: i32) -> ::windows::core::Result<()>;
    fn GetEnd(&mut self) -> ::windows::core::Result<i32>;
    fn SetEnd(&mut self, cplim: i32) -> ::windows::core::Result<()>;
    fn GetFont(&mut self) -> ::windows::core::Result<ITextFont>;
    fn SetFont(&mut self, pfont: ::core::option::Option<ITextFont>) -> ::windows::core::Result<()>;
    fn GetPara(&mut self) -> ::windows::core::Result<ITextPara>;
    fn SetPara(&mut self, ppara: ::core::option::Option<ITextPara>) -> ::windows::core::Result<()>;
    fn GetStoryLength(&mut self) -> ::windows::core::Result<i32>;
    fn GetStoryType(&mut self) -> ::windows::core::Result<i32>;
    fn Collapse(&mut self, bstart: i32) -> ::windows::core::Result<()>;
    fn Expand(&mut self, unit: i32) -> ::windows::core::Result<i32>;
    fn GetIndex(&mut self, unit: i32) -> ::windows::core::Result<i32>;
    fn SetIndex(&mut self, unit: i32, index: i32, extend: i32) -> ::windows::core::Result<()>;
    fn SetRange(&mut self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()>;
    fn InRange(&mut self, prange: ::core::option::Option<ITextRange>) -> ::windows::core::Result<i32>;
    fn InStory(&mut self, prange: ::core::option::Option<ITextRange>) -> ::windows::core::Result<i32>;
    fn IsEqual(&mut self, prange: ::core::option::Option<ITextRange>) -> ::windows::core::Result<i32>;
    fn Select(&mut self) -> ::windows::core::Result<()>;
    fn StartOf(&mut self, unit: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn EndOf(&mut self, unit: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn Move(&mut self, unit: i32, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStart(&mut self, unit: i32, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEnd(&mut self, unit: i32, count: i32) -> ::windows::core::Result<i32>;
    fn MoveWhile(&mut self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStartWhile(&mut self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndWhile(&mut self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveUntil(&mut self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStartUntil(&mut self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndUntil(&mut self, cset: *const super::super::super::System::Com::VARIANT, count: i32) -> ::windows::core::Result<i32>;
    fn FindText(&mut self, bstr: super::super::super::Foundation::BSTR, count: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn FindTextStart(&mut self, bstr: super::super::super::Foundation::BSTR, count: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn FindTextEnd(&mut self, bstr: super::super::super::Foundation::BSTR, count: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn Delete(&mut self, unit: i32, count: i32) -> ::windows::core::Result<i32>;
    fn Cut(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Copy(&mut self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Paste(&mut self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<()>;
    fn CanPaste(&mut self, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::Result<i32>;
    fn CanEdit(&mut self) -> ::windows::core::Result<i32>;
    fn ChangeCase(&mut self, r#type: i32) -> ::windows::core::Result<()>;
    fn GetPoint(&mut self, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::Result<()>;
    fn SetPoint(&mut self, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetEmbeddedObject(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeVtbl {
        unsafe extern "system" fn GetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn GetChar<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChar() {
                ::core::result::Result::Ok(ok__) => {
                    *pchar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChar<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, char: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChar(::core::mem::transmute_copy(&char)).into()
        }
        unsafe extern "system" fn GetDuplicate<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormattedText(::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn GetStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pcpfirst = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStart(::core::mem::transmute_copy(&cpfirst)).into()
        }
        unsafe extern "system" fn GetEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pcplim = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnd(::core::mem::transmute_copy(&cplim)).into()
        }
        unsafe extern "system" fn GetFont<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFont() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFont<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFont(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn GetPara<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPara() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPara<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPara(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn GetStoryLength<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryType<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Collapse(::core::mem::transmute_copy(&bstart)).into()
        }
        unsafe extern "system" fn Expand<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expand(::core::mem::transmute_copy(&unit)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex(::core::mem::transmute_copy(&unit)) {
                ::core::result::Result::Ok(ok__) => {
                    *pindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndex(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&extend)).into()
        }
        unsafe extern "system" fn SetRange<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRange(::core::mem::transmute_copy(&cpanchor), ::core::mem::transmute_copy(&cpactive)).into()
        }
        unsafe extern "system" fn InRange<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InRange(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InStory<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InStory(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Select<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn StartOf<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOf(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOf<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndOf(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveStart(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEnd(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveWhile(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStartWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveStartWhile(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndWhile(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveUntil(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStartUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveStartUntil(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndUntil(::core::mem::transmute_copy(&cset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindText(::core::mem::transmute_copy(&bstr), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *plength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTextStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindTextStart(::core::mem::transmute_copy(&bstr), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *plength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTextEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindTextEnd(::core::mem::transmute_copy(&bstr), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *plength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cut<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cut() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Paste(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn CanPaste<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPaste(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanEdit<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanEdit() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCase<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeCase(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn GetPoint<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPoint(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&px), ::core::mem::transmute_copy(&py)).into()
        }
        unsafe extern "system" fn SetPoint<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&extend)).into()
        }
        unsafe extern "system" fn ScrollIntoView<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScrollIntoView(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetEmbeddedObject<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddedObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetText: GetText::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            GetChar: GetChar::<Impl, IMPL_OFFSET>,
            SetChar: SetChar::<Impl, IMPL_OFFSET>,
            GetDuplicate: GetDuplicate::<Impl, IMPL_OFFSET>,
            GetFormattedText: GetFormattedText::<Impl, IMPL_OFFSET>,
            SetFormattedText: SetFormattedText::<Impl, IMPL_OFFSET>,
            GetStart: GetStart::<Impl, IMPL_OFFSET>,
            SetStart: SetStart::<Impl, IMPL_OFFSET>,
            GetEnd: GetEnd::<Impl, IMPL_OFFSET>,
            SetEnd: SetEnd::<Impl, IMPL_OFFSET>,
            GetFont: GetFont::<Impl, IMPL_OFFSET>,
            SetFont: SetFont::<Impl, IMPL_OFFSET>,
            GetPara: GetPara::<Impl, IMPL_OFFSET>,
            SetPara: SetPara::<Impl, IMPL_OFFSET>,
            GetStoryLength: GetStoryLength::<Impl, IMPL_OFFSET>,
            GetStoryType: GetStoryType::<Impl, IMPL_OFFSET>,
            Collapse: Collapse::<Impl, IMPL_OFFSET>,
            Expand: Expand::<Impl, IMPL_OFFSET>,
            GetIndex: GetIndex::<Impl, IMPL_OFFSET>,
            SetIndex: SetIndex::<Impl, IMPL_OFFSET>,
            SetRange: SetRange::<Impl, IMPL_OFFSET>,
            InRange: InRange::<Impl, IMPL_OFFSET>,
            InStory: InStory::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            Select: Select::<Impl, IMPL_OFFSET>,
            StartOf: StartOf::<Impl, IMPL_OFFSET>,
            EndOf: EndOf::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            MoveStart: MoveStart::<Impl, IMPL_OFFSET>,
            MoveEnd: MoveEnd::<Impl, IMPL_OFFSET>,
            MoveWhile: MoveWhile::<Impl, IMPL_OFFSET>,
            MoveStartWhile: MoveStartWhile::<Impl, IMPL_OFFSET>,
            MoveEndWhile: MoveEndWhile::<Impl, IMPL_OFFSET>,
            MoveUntil: MoveUntil::<Impl, IMPL_OFFSET>,
            MoveStartUntil: MoveStartUntil::<Impl, IMPL_OFFSET>,
            MoveEndUntil: MoveEndUntil::<Impl, IMPL_OFFSET>,
            FindText: FindText::<Impl, IMPL_OFFSET>,
            FindTextStart: FindTextStart::<Impl, IMPL_OFFSET>,
            FindTextEnd: FindTextEnd::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Cut: Cut::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Paste: Paste::<Impl, IMPL_OFFSET>,
            CanPaste: CanPaste::<Impl, IMPL_OFFSET>,
            CanEdit: CanEdit::<Impl, IMPL_OFFSET>,
            ChangeCase: ChangeCase::<Impl, IMPL_OFFSET>,
            GetPoint: GetPoint::<Impl, IMPL_OFFSET>,
            SetPoint: SetPoint::<Impl, IMPL_OFFSET>,
            ScrollIntoView: ScrollIntoView::<Impl, IMPL_OFFSET>,
            GetEmbeddedObject: GetEmbeddedObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRange2Impl: Sized + IDispatchImpl + ITextRangeImpl + ITextSelectionImpl {
    fn GetCch(&mut self) -> ::windows::core::Result<i32>;
    fn GetCells(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetColumn(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetDuplicate2(&mut self) -> ::windows::core::Result<ITextRange2>;
    fn GetFont2(&mut self) -> ::windows::core::Result<ITextFont2>;
    fn SetFont2(&mut self, pfont: ::core::option::Option<ITextFont2>) -> ::windows::core::Result<()>;
    fn GetFormattedText2(&mut self) -> ::windows::core::Result<ITextRange2>;
    fn SetFormattedText2(&mut self, prange: ::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn GetGravity(&mut self) -> ::windows::core::Result<i32>;
    fn SetGravity(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetPara2(&mut self) -> ::windows::core::Result<ITextPara2>;
    fn SetPara2(&mut self, ppara: ::core::option::Option<ITextPara2>) -> ::windows::core::Result<()>;
    fn GetRow(&mut self) -> ::windows::core::Result<ITextRow>;
    fn GetStartPara(&mut self) -> ::windows::core::Result<i32>;
    fn GetTable(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetURL(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetURL(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddSubrange(&mut self, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::Result<()>;
    fn BuildUpMath(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn DeleteSubrange(&mut self, cpfirst: i32, cplim: i32) -> ::windows::core::Result<()>;
    fn Find(&mut self, prange: ::core::option::Option<ITextRange2>, count: i32, flags: i32) -> ::windows::core::Result<i32>;
    fn GetChar2(&mut self, pchar: *mut i32, offset: i32) -> ::windows::core::Result<()>;
    fn GetDropCap(&mut self, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::Result<()>;
    fn GetInlineObject(&mut self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, r#type: i32) -> ::windows::core::Result<i32>;
    fn GetRect(&mut self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::Result<()>;
    fn GetSubrange(&mut self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::Result<()>;
    fn GetText2(&mut self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn HexToUnicode(&mut self) -> ::windows::core::Result<()>;
    fn InsertTable(&mut self, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::Result<()>;
    fn Linearize(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn SetActiveSubrange(&mut self, cpanchor: i32, cpactive: i32) -> ::windows::core::Result<()>;
    fn SetDropCap(&mut self, cline: i32, position: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
    fn SetText2(&mut self, flags: i32, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UnicodeToHex(&mut self) -> ::windows::core::Result<()>;
    fn SetInlineObject(&mut self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::Result<()>;
    fn GetMathFunctionType(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn InsertImage(&mut self, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: super::super::super::Foundation::BSTR, pstream: ::core::option::Option<super::super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRange2Vtbl {
        unsafe extern "system" fn GetCch<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCch() {
                ::core::result::Result::Ok(ok__) => {
                    *pcch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCells<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcells: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCells() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcells = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolumn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate2() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFont2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFont2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFont2(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn GetFormattedText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText2() {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormattedText2(::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn GetGravity<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGravity() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGravity(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetPara2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPara2() {
                ::core::result::Result::Ok(ok__) => {
                    *pppara = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPara2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPara2(::core::mem::transmute(&ppara)).into()
        }
        unsafe extern "system" fn GetRow<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRow() {
                ::core::result::Result::Ok(ok__) => {
                    *pprow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStartPara<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStartPara() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTable<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTable() {
                ::core::result::Result::Ok(ok__) => {
                    *pptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURL<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetURL() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetURL(::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn AddSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSubrange(::core::mem::transmute_copy(&cp1), ::core::mem::transmute_copy(&cp2), ::core::mem::transmute_copy(&activate)).into()
        }
        unsafe extern "system" fn BuildUpMath<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BuildUpMath(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn DeleteSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpfirst: i32, cplim: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteSubrange(::core::mem::transmute_copy(&cpfirst), ::core::mem::transmute_copy(&cplim)).into()
        }
        unsafe extern "system" fn Find<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, count: i32, flags: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Find(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChar2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchar: *mut i32, offset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChar2(::core::mem::transmute_copy(&pchar), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn GetDropCap<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDropCap(::core::mem::transmute_copy(&pcline), ::core::mem::transmute_copy(&pposition)).into()
        }
        unsafe extern "system" fn GetInlineObject<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInlineObject(::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&palign), ::core::mem::transmute_copy(&pchar), ::core::mem::transmute_copy(&pchar1), ::core::mem::transmute_copy(&pchar2), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ptexstyle), ::core::mem::transmute_copy(&pccol), ::core::mem::transmute_copy(&plevel)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRect<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRect(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom), ::core::mem::transmute_copy(&phit)).into()
        }
        unsafe extern "system" fn GetSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSubrange(::core::mem::transmute_copy(&isubrange), ::core::mem::transmute_copy(&pcpfirst), ::core::mem::transmute_copy(&pcplim)).into()
        }
        unsafe extern "system" fn GetText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText2(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HexToUnicode<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HexToUnicode().into()
        }
        unsafe extern "system" fn InsertTable<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertTable(::core::mem::transmute_copy(&ccol), ::core::mem::transmute_copy(&crow), ::core::mem::transmute_copy(&autofit)).into()
        }
        unsafe extern "system" fn Linearize<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Linearize(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetActiveSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveSubrange(::core::mem::transmute_copy(&cpanchor), ::core::mem::transmute_copy(&cpactive)).into()
        }
        unsafe extern "system" fn SetDropCap<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cline: i32, position: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDropCap(::core::mem::transmute_copy(&cline), ::core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText2(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn UnicodeToHex<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnicodeToHex().into()
        }
        unsafe extern "system" fn SetInlineObject<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInlineObject(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&align), ::core::mem::transmute_copy(&char), ::core::mem::transmute_copy(&char1), ::core::mem::transmute_copy(&char2), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&texstyle), ::core::mem::transmute_copy(&ccol)).into()
        }
        unsafe extern "system" fn GetMathFunctionType<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMathFunctionType(::core::mem::transmute_copy(&bstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImage<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertImage(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&ascent), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&bstralttext), ::core::mem::transmute(&pstream)).into()
        }
        Self {
            base: ITextSelectionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCch: GetCch::<Impl, IMPL_OFFSET>,
            GetCells: GetCells::<Impl, IMPL_OFFSET>,
            GetColumn: GetColumn::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetDuplicate2: GetDuplicate2::<Impl, IMPL_OFFSET>,
            GetFont2: GetFont2::<Impl, IMPL_OFFSET>,
            SetFont2: SetFont2::<Impl, IMPL_OFFSET>,
            GetFormattedText2: GetFormattedText2::<Impl, IMPL_OFFSET>,
            SetFormattedText2: SetFormattedText2::<Impl, IMPL_OFFSET>,
            GetGravity: GetGravity::<Impl, IMPL_OFFSET>,
            SetGravity: SetGravity::<Impl, IMPL_OFFSET>,
            GetPara2: GetPara2::<Impl, IMPL_OFFSET>,
            SetPara2: SetPara2::<Impl, IMPL_OFFSET>,
            GetRow: GetRow::<Impl, IMPL_OFFSET>,
            GetStartPara: GetStartPara::<Impl, IMPL_OFFSET>,
            GetTable: GetTable::<Impl, IMPL_OFFSET>,
            GetURL: GetURL::<Impl, IMPL_OFFSET>,
            SetURL: SetURL::<Impl, IMPL_OFFSET>,
            AddSubrange: AddSubrange::<Impl, IMPL_OFFSET>,
            BuildUpMath: BuildUpMath::<Impl, IMPL_OFFSET>,
            DeleteSubrange: DeleteSubrange::<Impl, IMPL_OFFSET>,
            Find: Find::<Impl, IMPL_OFFSET>,
            GetChar2: GetChar2::<Impl, IMPL_OFFSET>,
            GetDropCap: GetDropCap::<Impl, IMPL_OFFSET>,
            GetInlineObject: GetInlineObject::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetRect: GetRect::<Impl, IMPL_OFFSET>,
            GetSubrange: GetSubrange::<Impl, IMPL_OFFSET>,
            GetText2: GetText2::<Impl, IMPL_OFFSET>,
            HexToUnicode: HexToUnicode::<Impl, IMPL_OFFSET>,
            InsertTable: InsertTable::<Impl, IMPL_OFFSET>,
            Linearize: Linearize::<Impl, IMPL_OFFSET>,
            SetActiveSubrange: SetActiveSubrange::<Impl, IMPL_OFFSET>,
            SetDropCap: SetDropCap::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            SetText2: SetText2::<Impl, IMPL_OFFSET>,
            UnicodeToHex: UnicodeToHex::<Impl, IMPL_OFFSET>,
            SetInlineObject: SetInlineObject::<Impl, IMPL_OFFSET>,
            GetMathFunctionType: GetMathFunctionType::<Impl, IMPL_OFFSET>,
            InsertImage: InsertImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRowImpl: Sized + IDispatchImpl {
    fn GetAlignment(&mut self) -> ::windows::core::Result<i32>;
    fn SetAlignment(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellCount(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellCount(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellCountCache(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellCountCache(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellMargin(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellMargin(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetHeight(&mut self) -> ::windows::core::Result<i32>;
    fn SetHeight(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetIndent(&mut self) -> ::windows::core::Result<i32>;
    fn SetIndent(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetKeepTogether(&mut self) -> ::windows::core::Result<i32>;
    fn SetKeepTogether(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetKeepWithNext(&mut self) -> ::windows::core::Result<i32>;
    fn SetKeepWithNext(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetNestLevel(&mut self) -> ::windows::core::Result<i32>;
    fn GetRTL(&mut self) -> ::windows::core::Result<i32>;
    fn SetRTL(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellAlignment(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellAlignment(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellColorBack(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellColorBack(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellColorFore(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellColorFore(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellMergeFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellMergeFlags(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellShading(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellShading(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellVerticalText(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellVerticalText(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellWidth(&mut self) -> ::windows::core::Result<i32>;
    fn SetCellWidth(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetCellBorderColors(&mut self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::core::Result<()>;
    fn GetCellBorderWidths(&mut self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::core::Result<()>;
    fn SetCellBorderColors(&mut self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::core::Result<()>;
    fn SetCellBorderWidths(&mut self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::core::Result<()>;
    fn Apply(&mut self, crow: i32, flags: tomConstants) -> ::windows::core::Result<()>;
    fn CanChange(&mut self) -> ::windows::core::Result<i32>;
    fn GetProperty(&mut self, r#type: i32) -> ::windows::core::Result<i32>;
    fn Insert(&mut self, crow: i32) -> ::windows::core::Result<()>;
    fn IsEqual(&mut self, prow: ::core::option::Option<ITextRow>) -> ::windows::core::Result<i32>;
    fn Reset(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRowVtbl {
        unsafe extern "system" fn GetAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellCount<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellCount<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellCount(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellCountCache<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellCountCache() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellCountCache<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellCountCache(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellIndex<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellIndex<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellIndex(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellMargin<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellMargin<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellMargin(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetHeight<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeight(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetIndent<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndent() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndent<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndent(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepTogether<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeepTogether() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepTogether(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeepWithNext<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeepWithNext() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepWithNext(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetNestLevel<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNestLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRTL<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRTL<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRTL(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellAlignment(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellColorBack<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellColorBack() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellColorBack<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellColorBack(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellColorFore<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellColorFore() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellColorFore<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellColorFore(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellMergeFlags<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellMergeFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellMergeFlags<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellMergeFlags(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellShading<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellShading() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellShading<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellShading(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellVerticalText<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellVerticalText() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellVerticalText<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellVerticalText(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellWidth<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellWidth<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellWidth(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCellBorderColors<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCellBorderColors(::core::mem::transmute_copy(&pcrleft), ::core::mem::transmute_copy(&pcrtop), ::core::mem::transmute_copy(&pcrright), ::core::mem::transmute_copy(&pcrbottom)).into()
        }
        unsafe extern "system" fn GetCellBorderWidths<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCellBorderWidths(::core::mem::transmute_copy(&pduleft), ::core::mem::transmute_copy(&pdutop), ::core::mem::transmute_copy(&pduright), ::core::mem::transmute_copy(&pdubottom)).into()
        }
        unsafe extern "system" fn SetCellBorderColors<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellBorderColors(::core::mem::transmute_copy(&crleft), ::core::mem::transmute_copy(&crtop), ::core::mem::transmute_copy(&crright), ::core::mem::transmute_copy(&crbottom)).into()
        }
        unsafe extern "system" fn SetCellBorderWidths<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCellBorderWidths(::core::mem::transmute_copy(&duleft), ::core::mem::transmute_copy(&dutop), ::core::mem::transmute_copy(&duright), ::core::mem::transmute_copy(&dubottom)).into()
        }
        unsafe extern "system" fn Apply<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crow: i32, flags: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Apply(::core::mem::transmute_copy(&crow), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CanChange<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Insert(::core::mem::transmute_copy(&crow)).into()
        }
        unsafe extern "system" fn IsEqual<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prow: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&prow)) {
                ::core::result::Result::Ok(ok__) => {
                    *pb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAlignment: GetAlignment::<Impl, IMPL_OFFSET>,
            SetAlignment: SetAlignment::<Impl, IMPL_OFFSET>,
            GetCellCount: GetCellCount::<Impl, IMPL_OFFSET>,
            SetCellCount: SetCellCount::<Impl, IMPL_OFFSET>,
            GetCellCountCache: GetCellCountCache::<Impl, IMPL_OFFSET>,
            SetCellCountCache: SetCellCountCache::<Impl, IMPL_OFFSET>,
            GetCellIndex: GetCellIndex::<Impl, IMPL_OFFSET>,
            SetCellIndex: SetCellIndex::<Impl, IMPL_OFFSET>,
            GetCellMargin: GetCellMargin::<Impl, IMPL_OFFSET>,
            SetCellMargin: SetCellMargin::<Impl, IMPL_OFFSET>,
            GetHeight: GetHeight::<Impl, IMPL_OFFSET>,
            SetHeight: SetHeight::<Impl, IMPL_OFFSET>,
            GetIndent: GetIndent::<Impl, IMPL_OFFSET>,
            SetIndent: SetIndent::<Impl, IMPL_OFFSET>,
            GetKeepTogether: GetKeepTogether::<Impl, IMPL_OFFSET>,
            SetKeepTogether: SetKeepTogether::<Impl, IMPL_OFFSET>,
            GetKeepWithNext: GetKeepWithNext::<Impl, IMPL_OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Impl, IMPL_OFFSET>,
            GetNestLevel: GetNestLevel::<Impl, IMPL_OFFSET>,
            GetRTL: GetRTL::<Impl, IMPL_OFFSET>,
            SetRTL: SetRTL::<Impl, IMPL_OFFSET>,
            GetCellAlignment: GetCellAlignment::<Impl, IMPL_OFFSET>,
            SetCellAlignment: SetCellAlignment::<Impl, IMPL_OFFSET>,
            GetCellColorBack: GetCellColorBack::<Impl, IMPL_OFFSET>,
            SetCellColorBack: SetCellColorBack::<Impl, IMPL_OFFSET>,
            GetCellColorFore: GetCellColorFore::<Impl, IMPL_OFFSET>,
            SetCellColorFore: SetCellColorFore::<Impl, IMPL_OFFSET>,
            GetCellMergeFlags: GetCellMergeFlags::<Impl, IMPL_OFFSET>,
            SetCellMergeFlags: SetCellMergeFlags::<Impl, IMPL_OFFSET>,
            GetCellShading: GetCellShading::<Impl, IMPL_OFFSET>,
            SetCellShading: SetCellShading::<Impl, IMPL_OFFSET>,
            GetCellVerticalText: GetCellVerticalText::<Impl, IMPL_OFFSET>,
            SetCellVerticalText: SetCellVerticalText::<Impl, IMPL_OFFSET>,
            GetCellWidth: GetCellWidth::<Impl, IMPL_OFFSET>,
            SetCellWidth: SetCellWidth::<Impl, IMPL_OFFSET>,
            GetCellBorderColors: GetCellBorderColors::<Impl, IMPL_OFFSET>,
            GetCellBorderWidths: GetCellBorderWidths::<Impl, IMPL_OFFSET>,
            SetCellBorderColors: SetCellBorderColors::<Impl, IMPL_OFFSET>,
            SetCellBorderWidths: SetCellBorderWidths::<Impl, IMPL_OFFSET>,
            Apply: Apply::<Impl, IMPL_OFFSET>,
            CanChange: CanChange::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            Insert: Insert::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextSelectionImpl: Sized + IDispatchImpl + ITextRangeImpl {
    fn GetFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetFlags(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn GetType(&mut self) -> ::windows::core::Result<i32>;
    fn MoveLeft(&mut self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn MoveRight(&mut self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn MoveUp(&mut self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn MoveDown(&mut self, unit: i32, count: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn HomeKey(&mut self, unit: tomConstants, extend: i32) -> ::windows::core::Result<i32>;
    fn EndKey(&mut self, unit: i32, extend: i32) -> ::windows::core::Result<i32>;
    fn TypeText(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextSelectionVtbl {
        unsafe extern "system" fn GetFlags<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetType<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveLeft<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveLeft(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveRight<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveRight(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUp<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveUp(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveDown<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveDown(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeKey<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeKey(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndKey<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndKey(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&extend)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdelta = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeText<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TypeText(::core::mem::transmute_copy(&bstr)).into()
        }
        Self {
            base: ITextRangeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            MoveLeft: MoveLeft::<Impl, IMPL_OFFSET>,
            MoveRight: MoveRight::<Impl, IMPL_OFFSET>,
            MoveUp: MoveUp::<Impl, IMPL_OFFSET>,
            MoveDown: MoveDown::<Impl, IMPL_OFFSET>,
            HomeKey: HomeKey::<Impl, IMPL_OFFSET>,
            EndKey: EndKey::<Impl, IMPL_OFFSET>,
            TypeText: TypeText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextSelection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextSelection2Impl: Sized + IDispatchImpl + ITextRangeImpl + ITextSelectionImpl + ITextRange2Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextSelection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextSelection2Vtbl {
        Self { base: ITextRange2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextSelection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServicesImpl: Sized {
    fn TxSendMessage(&mut self, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>;
    fn TxDraw(&mut self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::Result<()>;
    fn TxGetHScroll(&mut self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TxGetVScroll(&mut self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTxSetCursor(&mut self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn TxQueryHitPoint(&mut self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::Result<()>;
    fn OnTxInPlaceActivate(&mut self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn OnTxInPlaceDeactivate(&mut self) -> ::windows::core::Result<()>;
    fn OnTxUIActivate(&mut self) -> ::windows::core::Result<()>;
    fn OnTxUIDeactivate(&mut self) -> ::windows::core::Result<()>;
    fn TxGetText(&mut self, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TxSetText(&mut self, psztext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn TxGetCurTargetX(&mut self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn TxGetBaseLinePos(&mut self, param0: *mut i32) -> ::windows::core::Result<()>;
    fn TxGetNaturalSize(&mut self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::Result<()>;
    fn TxGetDropTarget(&mut self) -> ::windows::core::Result<super::super::super::System::Ole::IDropTarget>;
    fn OnTxPropertyBitsChange(&mut self, dwmask: u32, dwbits: u32) -> ::windows::core::Result<()>;
    fn TxGetCachedSize(&mut self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextServicesVtbl {
        unsafe extern "system" fn TxSendMessage<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSendMessage(::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plresult)).into()
        }
        unsafe extern "system" fn TxDraw<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn TxGetHScroll<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetHScroll(::core::mem::transmute_copy(&plmin), ::core::mem::transmute_copy(&plmax), ::core::mem::transmute_copy(&plpos), ::core::mem::transmute_copy(&plpage), ::core::mem::transmute_copy(&pfenabled)).into()
        }
        unsafe extern "system" fn TxGetVScroll<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetVScroll(::core::mem::transmute_copy(&plmin), ::core::mem::transmute_copy(&plmax), ::core::mem::transmute_copy(&plpos), ::core::mem::transmute_copy(&plpage), ::core::mem::transmute_copy(&pfenabled)).into()
        }
        unsafe extern "system" fn OnTxSetCursor<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTxSetCursor(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&lprcclient), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn TxQueryHitPoint<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxQueryHitPoint(::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&lprcclient), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&phitresult)).into()
        }
        unsafe extern "system" fn OnTxInPlaceActivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTxInPlaceActivate(::core::mem::transmute_copy(&prcclient)).into()
        }
        unsafe extern "system" fn OnTxInPlaceDeactivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTxInPlaceDeactivate().into()
        }
        unsafe extern "system" fn OnTxUIActivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTxUIActivate().into()
        }
        unsafe extern "system" fn OnTxUIDeactivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTxUIDeactivate().into()
        }
        unsafe extern "system" fn TxGetText<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetText(::core::mem::transmute_copy(&pbstrtext)).into()
        }
        unsafe extern "system" fn TxSetText<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetText(::core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn TxGetCurTargetX<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetCurTargetX(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn TxGetBaseLinePos<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetBaseLinePos(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn TxGetNaturalSize<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetNaturalSize(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&psizelextent), ::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn TxGetDropTarget<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetDropTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdroptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxPropertyBitsChange<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32, dwbits: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTxPropertyBitsChange(::core::mem::transmute_copy(&dwmask), ::core::mem::transmute_copy(&dwbits)).into()
        }
        unsafe extern "system" fn TxGetCachedSize<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetCachedSize(::core::mem::transmute_copy(&pdwwidth), ::core::mem::transmute_copy(&pdwheight)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TxSendMessage: TxSendMessage::<Impl, IMPL_OFFSET>,
            TxDraw: TxDraw::<Impl, IMPL_OFFSET>,
            TxGetHScroll: TxGetHScroll::<Impl, IMPL_OFFSET>,
            TxGetVScroll: TxGetVScroll::<Impl, IMPL_OFFSET>,
            OnTxSetCursor: OnTxSetCursor::<Impl, IMPL_OFFSET>,
            TxQueryHitPoint: TxQueryHitPoint::<Impl, IMPL_OFFSET>,
            OnTxInPlaceActivate: OnTxInPlaceActivate::<Impl, IMPL_OFFSET>,
            OnTxInPlaceDeactivate: OnTxInPlaceDeactivate::<Impl, IMPL_OFFSET>,
            OnTxUIActivate: OnTxUIActivate::<Impl, IMPL_OFFSET>,
            OnTxUIDeactivate: OnTxUIDeactivate::<Impl, IMPL_OFFSET>,
            TxGetText: TxGetText::<Impl, IMPL_OFFSET>,
            TxSetText: TxSetText::<Impl, IMPL_OFFSET>,
            TxGetCurTargetX: TxGetCurTargetX::<Impl, IMPL_OFFSET>,
            TxGetBaseLinePos: TxGetBaseLinePos::<Impl, IMPL_OFFSET>,
            TxGetNaturalSize: TxGetNaturalSize::<Impl, IMPL_OFFSET>,
            TxGetDropTarget: TxGetDropTarget::<Impl, IMPL_OFFSET>,
            OnTxPropertyBitsChange: OnTxPropertyBitsChange::<Impl, IMPL_OFFSET>,
            TxGetCachedSize: TxGetCachedSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextServices2Impl: Sized + ITextServicesImpl {
    fn TxGetNaturalSize2(&mut self, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::core::Result<()>;
    fn TxDrawD2D(&mut self, prendertarget: ::core::option::Option<super::super::super::Graphics::Direct2D::ID2D1RenderTarget>, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextServices2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextServices2Vtbl {
        unsafe extern "system" fn TxGetNaturalSize2<Impl: ITextServices2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxGetNaturalSize2(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&psizelextent), ::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight), ::core::mem::transmute_copy(&pascent)).into()
        }
        unsafe extern "system" fn TxDrawD2D<Impl: ITextServices2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxDrawD2D(::core::mem::transmute(&prendertarget), ::core::mem::transmute_copy(&lprcbounds), ::core::mem::transmute_copy(&lprcupdate), ::core::mem::transmute_copy(&lviewid)).into()
        }
        Self {
            base: ITextServicesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TxGetNaturalSize2: TxGetNaturalSize2::<Impl, IMPL_OFFSET>,
            TxDrawD2D: TxDrawD2D::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextServices2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITextStoryImpl: Sized {
    fn GetActive(&mut self) -> ::windows::core::Result<i32>;
    fn SetActive(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetDisplay(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetIndex(&mut self) -> ::windows::core::Result<i32>;
    fn GetType(&mut self) -> ::windows::core::Result<i32>;
    fn SetType(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, r#type: i32) -> ::windows::core::Result<i32>;
    fn GetRange(&mut self, cpactive: i32, cpanchor: i32) -> ::windows::core::Result<ITextRange2>;
    fn GetText(&mut self, flags: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetFormattedText(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, r#type: i32, value: i32) -> ::windows::core::Result<()>;
    fn SetText(&mut self, flags: i32, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITextStoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoryVtbl {
        unsafe extern "system" fn GetActive<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActive() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActive<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActive(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetDisplay<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplay: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRange(::core::mem::transmute_copy(&cpactive), ::core::mem::transmute_copy(&cpanchor)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormattedText(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&bstr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetActive: GetActive::<Impl, IMPL_OFFSET>,
            SetActive: SetActive::<Impl, IMPL_OFFSET>,
            GetDisplay: GetDisplay::<Impl, IMPL_OFFSET>,
            GetIndex: GetIndex::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetRange: GetRange::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            SetFormattedText: SetFormattedText::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoryRangesImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ITextRange>;
    fn GetCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoryRangesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRangesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoryRangesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoryRanges as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStoryRanges2Impl: Sized + IDispatchImpl + ITextStoryRangesImpl {
    fn Item2(&mut self, index: i32) -> ::windows::core::Result<ITextRange2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStoryRanges2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStoryRanges2Vtbl {
        unsafe extern "system" fn Item2<Impl: ITextStoryRanges2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item2(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITextStoryRangesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Item2: Item2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStoryRanges2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextStringsImpl: Sized + IDispatchImpl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ITextRange2>;
    fn GetCount(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Append(&mut self, prange: ::core::option::Option<ITextRange2>, istring: i32) -> ::windows::core::Result<()>;
    fn Cat2(&mut self, istring: i32) -> ::windows::core::Result<()>;
    fn CatTop2(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteRange(&mut self, prange: ::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn EncodeFunction(&mut self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: ::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn GetCch(&mut self, istring: i32) -> ::windows::core::Result<i32>;
    fn InsertNullStr(&mut self, istring: i32) -> ::windows::core::Result<()>;
    fn MoveBoundary(&mut self, istring: i32, cch: i32) -> ::windows::core::Result<()>;
    fn PrefixTop(&mut self, bstr: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Remove(&mut self, istring: i32, cstring: i32) -> ::windows::core::Result<()>;
    fn SetFormattedText(&mut self, pranged: ::core::option::Option<ITextRange2>, pranges: ::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn SetOpCp(&mut self, istring: i32, cp: i32) -> ::windows::core::Result<()>;
    fn SuffixTop(&mut self, bstr: super::super::super::Foundation::BSTR, prange: ::core::option::Option<ITextRange2>) -> ::windows::core::Result<()>;
    fn Swap(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextStringsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStringsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextStringsVtbl {
        unsafe extern "system" fn Item<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn Append<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&prange), ::core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn Cat2<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cat2(::core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn CatTop2<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CatTop2(::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn DeleteRange<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRange(::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn EncodeFunction<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncodeFunction(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&align), ::core::mem::transmute_copy(&char), ::core::mem::transmute_copy(&char1), ::core::mem::transmute_copy(&char2), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&texstyle), ::core::mem::transmute_copy(&ccol), ::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn GetCch<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCch(::core::mem::transmute_copy(&istring)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertNullStr<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertNullStr(::core::mem::transmute_copy(&istring)).into()
        }
        unsafe extern "system" fn MoveBoundary<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveBoundary(::core::mem::transmute_copy(&istring), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn PrefixTop<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrefixTop(::core::mem::transmute_copy(&bstr)).into()
        }
        unsafe extern "system" fn Remove<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cstring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&istring), ::core::mem::transmute_copy(&cstring)).into()
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pranged: ::windows::core::RawPtr, pranges: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormattedText(::core::mem::transmute(&pranged), ::core::mem::transmute(&pranges)).into()
        }
        unsafe extern "system" fn SetOpCp<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpCp(::core::mem::transmute_copy(&istring), ::core::mem::transmute_copy(&cp)).into()
        }
        unsafe extern "system" fn SuffixTop<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuffixTop(::core::mem::transmute_copy(&bstr), ::core::mem::transmute(&prange)).into()
        }
        unsafe extern "system" fn Swap<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Swap().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            Cat2: Cat2::<Impl, IMPL_OFFSET>,
            CatTop2: CatTop2::<Impl, IMPL_OFFSET>,
            DeleteRange: DeleteRange::<Impl, IMPL_OFFSET>,
            EncodeFunction: EncodeFunction::<Impl, IMPL_OFFSET>,
            GetCch: GetCch::<Impl, IMPL_OFFSET>,
            InsertNullStr: InsertNullStr::<Impl, IMPL_OFFSET>,
            MoveBoundary: MoveBoundary::<Impl, IMPL_OFFSET>,
            PrefixTop: PrefixTop::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            SetFormattedText: SetFormattedText::<Impl, IMPL_OFFSET>,
            SetOpCp: SetOpCp::<Impl, IMPL_OFFSET>,
            SuffixTop: SuffixTop::<Impl, IMPL_OFFSET>,
            Swap: Swap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextStrings as ::windows::core::Interface>::IID
    }
}
