pub trait IRichEditOleImpl: Sized {
    fn GetClientSite();
    fn GetObjectCount();
    fn GetLinkCount();
    fn GetObject();
    fn InsertObject();
    fn ConvertObject();
    fn ActivateAs();
    fn SetHostNames();
    fn SetLinkAvailable();
    fn SetDvaspect();
    fn HandsOffStorage();
    fn SaveCompleted();
    fn InPlaceDeactivate();
    fn ContextSensitiveHelp();
    fn GetClipboardData();
    fn ImportDataObject();
}
impl ::windows::core::RuntimeName for IRichEditOle {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.IRichEditOle";
}
impl IRichEditOleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleImpl, const OFFSET: isize>() -> IRichEditOleVtbl {
        unsafe extern "system" fn GetClientSite<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpolesite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientSite(::core::mem::transmute_copy(&lplpolesite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectCount<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinkCount<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(iob, &*(&lpreobject as *const <REOBJECT as ::windows::core::Abi>::Abi as *const <REOBJECT as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpreobject: *mut REOBJECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertObject(&*(&lpreobject as *const <REOBJECT as ::windows::core::Abi>::Abi as *const <REOBJECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, rclsidnew: &::windows::core::GUID, lpstrusertypenew: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertObject(iob, &*(&rclsidnew as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&lpstrusertypenew as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateAs<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: &::windows::core::GUID, rclsidas: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateAs(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&rclsidas as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostNames<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpstrcontainerapp: super::super::super::Foundation::PSTR, lpstrcontainerobj: super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHostNames(&*(&lpstrcontainerapp as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&lpstrcontainerobj as *const <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkAvailable<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLinkAvailable(iob, &*(&favailable as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDvaspect<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, dvaspect: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDvaspect(iob, dvaspect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandsOffStorage<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandsOffStorage(iob) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveCompleted<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iob: i32, lpstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveCompleted(iob, &*(&lpstg as *const <super::super::super::System::Com::StructuredStorage::IStorage as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::IStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InPlaceDeactivate<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InPlaceDeactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextSensitiveHelp(&*(&fentermode as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipboardData<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipboardData(&*(&lpchrg as *const <CHARRANGE as ::windows::core::Abi>::Abi as *const <CHARRANGE as ::windows::core::DefaultType>::DefaultType), reco, ::core::mem::transmute_copy(&lplpdataobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportDataObject<Impl: IRichEditOleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, cf: u16, hmetapict: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportDataObject(&*(&lpdataobj as *const <super::super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), cf, hmetapict) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRichEditOle>,
            ::windows::core::GetTrustLevel,
            GetClientSite::<Impl, OFFSET>,
            GetObjectCount::<Impl, OFFSET>,
            GetLinkCount::<Impl, OFFSET>,
            GetObject::<Impl, OFFSET>,
            InsertObject::<Impl, OFFSET>,
            ConvertObject::<Impl, OFFSET>,
            ActivateAs::<Impl, OFFSET>,
            SetHostNames::<Impl, OFFSET>,
            SetLinkAvailable::<Impl, OFFSET>,
            SetDvaspect::<Impl, OFFSET>,
            HandsOffStorage::<Impl, OFFSET>,
            SaveCompleted::<Impl, OFFSET>,
            InPlaceDeactivate::<Impl, OFFSET>,
            ContextSensitiveHelp::<Impl, OFFSET>,
            GetClipboardData::<Impl, OFFSET>,
            ImportDataObject::<Impl, OFFSET>,
        )
    }
}
pub trait IRichEditOleCallbackImpl: Sized {
    fn GetNewStorage();
    fn GetInPlaceContext();
    fn ShowContainerUI();
    fn QueryInsertObject();
    fn DeleteObject();
    fn QueryAcceptData();
    fn ContextSensitiveHelp();
    fn GetClipboardData();
    fn GetDragDropEffect();
    fn GetContextMenu();
}
impl ::windows::core::RuntimeName for IRichEditOleCallback {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.IRichEditOleCallback";
}
impl IRichEditOleCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditOleCallbackImpl, const OFFSET: isize>() -> IRichEditOleCallbackVtbl {
        unsafe extern "system" fn GetNewStorage<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNewStorage(::core::mem::transmute_copy(&lplpstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInPlaceContext<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lplpframe: *mut ::windows::core::RawPtr, lplpdoc: *mut ::windows::core::RawPtr, lpframeinfo: *mut super::super::super::System::Ole::OIFI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInPlaceContext(::core::mem::transmute_copy(&lplpframe), ::core::mem::transmute_copy(&lplpdoc), &*(&lpframeinfo as *const <super::super::super::System::Ole::OIFI as ::windows::core::Abi>::Abi as *const <super::super::super::System::Ole::OIFI as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowContainerUI<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowContainerUI(&*(&fshow as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInsertObject<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID, lpstg: ::windows::core::RawPtr, cp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInsertObject(&*(&lpclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&lpstg as *const <super::super::super::System::Com::StructuredStorage::IStorage as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::StructuredStorage::IStorage as ::windows::core::DefaultType>::DefaultType), cp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteObject<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpoleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteObject(&*(&lpoleobj as *const <super::super::super::System::Ole::IOleObject as ::windows::core::Abi>::Abi as *const <super::super::super::System::Ole::IOleObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAcceptData<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdataobj: ::windows::core::RawPtr, lpcfformat: *mut u16, reco: u32, freally: super::super::super::Foundation::BOOL, hmetapict: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAcceptData(&*(&lpdataobj as *const <super::super::super::System::Com::IDataObject as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IDataObject as ::windows::core::DefaultType>::DefaultType), lpcfformat, reco, &*(&freally as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), hmetapict) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextSensitiveHelp<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextSensitiveHelp(&*(&fentermode as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClipboardData<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClipboardData(&*(&lpchrg as *const <CHARRANGE as ::windows::core::Abi>::Abi as *const <CHARRANGE as ::windows::core::DefaultType>::DefaultType), reco, ::core::mem::transmute_copy(&lplpdataobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDragDropEffect<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdrag: super::super::super::Foundation::BOOL, grfkeystate: u32, pdweffect: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDragDropEffect(&*(&fdrag as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), grfkeystate, pdweffect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextMenu<Impl: IRichEditOleCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: ::windows::core::RawPtr, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextMenu(
                seltype,
                &*(&lpoleobj as *const <super::super::super::System::Ole::IOleObject as ::windows::core::Abi>::Abi as *const <super::super::super::System::Ole::IOleObject as ::windows::core::DefaultType>::DefaultType),
                &*(&lpchrg as *const <CHARRANGE as ::windows::core::Abi>::Abi as *const <CHARRANGE as ::windows::core::DefaultType>::DefaultType),
                &*(&lphmenu as *const <super::super::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::super::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRichEditOleCallback>,
            ::windows::core::GetTrustLevel,
            GetNewStorage::<Impl, OFFSET>,
            GetInPlaceContext::<Impl, OFFSET>,
            ShowContainerUI::<Impl, OFFSET>,
            QueryInsertObject::<Impl, OFFSET>,
            DeleteObject::<Impl, OFFSET>,
            QueryAcceptData::<Impl, OFFSET>,
            ContextSensitiveHelp::<Impl, OFFSET>,
            GetClipboardData::<Impl, OFFSET>,
            GetDragDropEffect::<Impl, OFFSET>,
            GetContextMenu::<Impl, OFFSET>,
        )
    }
}
pub trait IRicheditUiaOverridesImpl: Sized {
    fn GetPropertyOverrideValue();
}
impl ::windows::core::RuntimeName for IRicheditUiaOverrides {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.IRicheditUiaOverrides";
}
impl IRicheditUiaOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditUiaOverridesImpl, const OFFSET: isize>() -> IRicheditUiaOverridesVtbl {
        unsafe extern "system" fn GetPropertyOverrideValue<Impl: IRicheditUiaOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, pretvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyOverrideValue(propertyid, &*(&pretvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRicheditUiaOverrides>, ::windows::core::GetTrustLevel, GetPropertyOverrideValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDisplaysImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextDisplays {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextDisplays";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextDisplaysVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDisplaysImpl, const OFFSET: isize>() -> ITextDisplaysVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextDisplays>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocumentImpl: Sized + IDispatchImpl {
    fn GetName();
    fn GetSelection();
    fn GetStoryCount();
    fn GetStoryRanges();
    fn GetSaved();
    fn SetSaved();
    fn GetDefaultTabStop();
    fn SetDefaultTabStop();
    fn New();
    fn Open();
    fn Save();
    fn Freeze();
    fn Unfreeze();
    fn BeginEditCollection();
    fn EndEditCollection();
    fn Undo();
    fn Redo();
    fn Range();
    fn RangeFromPoint();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextDocument {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextDocument";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocumentImpl, const OFFSET: isize>() -> ITextDocumentVtbl {
        unsafe extern "system" fn GetName<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(::core::mem::transmute_copy(&ppsel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryCount<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryRanges<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryRanges(::core::mem::transmute_copy(&ppstories)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSaved<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSaved(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSaved<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSaved(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultTabStop<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultTabStop(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultTabStop(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn New<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).New() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pvar as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), flags, codepage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, flags: i32, codepage: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(&*(&pvar as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), flags, codepage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Freeze<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Freeze(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unfreeze<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unfreeze(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEditCollection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginEditCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndEditCollection<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndEditCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Undo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Undo(count, ::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Redo<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Redo(count, ::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Range<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range(cpactive, cpanchor, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Impl: ITextDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint(x, y, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextDocument>,
            ::windows::core::GetTrustLevel,
            GetName::<Impl, OFFSET>,
            GetSelection::<Impl, OFFSET>,
            GetStoryCount::<Impl, OFFSET>,
            GetStoryRanges::<Impl, OFFSET>,
            GetSaved::<Impl, OFFSET>,
            SetSaved::<Impl, OFFSET>,
            GetDefaultTabStop::<Impl, OFFSET>,
            SetDefaultTabStop::<Impl, OFFSET>,
            New::<Impl, OFFSET>,
            Open::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            Freeze::<Impl, OFFSET>,
            Unfreeze::<Impl, OFFSET>,
            BeginEditCollection::<Impl, OFFSET>,
            EndEditCollection::<Impl, OFFSET>,
            Undo::<Impl, OFFSET>,
            Redo::<Impl, OFFSET>,
            Range::<Impl, OFFSET>,
            RangeFromPoint::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocument2Impl: Sized + ITextDocumentImpl + IDispatchImpl {
    fn GetCaretType();
    fn SetCaretType();
    fn GetDisplays();
    fn GetDocumentFont();
    fn SetDocumentFont();
    fn GetDocumentPara();
    fn SetDocumentPara();
    fn GetEastAsianFlags();
    fn GetGenerator();
    fn SetIMEInProgress();
    fn GetNotificationMode();
    fn SetNotificationMode();
    fn GetSelection2();
    fn GetStoryRanges2();
    fn GetTypographyOptions();
    fn GetVersion();
    fn GetWindow();
    fn AttachMsgFilter();
    fn CheckTextLimit();
    fn GetCallManager();
    fn GetClientRect();
    fn GetEffectColor();
    fn GetImmContext();
    fn GetPreferredFont();
    fn GetProperty();
    fn GetStrings();
    fn Notify();
    fn Range2();
    fn RangeFromPoint2();
    fn ReleaseCallManager();
    fn ReleaseImmContext();
    fn SetEffectColor();
    fn SetProperty();
    fn SetTypographyOptions();
    fn SysBeep();
    fn Update();
    fn UpdateWindow();
    fn GetMathProperties();
    fn SetMathProperties();
    fn GetActiveStory();
    fn SetActiveStory();
    fn GetMainStory();
    fn GetNewStory();
    fn GetStory();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextDocument2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextDocument2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2Impl, const OFFSET: isize>() -> ITextDocument2Vtbl {
        unsafe extern "system" fn GetCaretType<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaretType(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCaretType(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplays<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplays: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplays(::core::mem::transmute_copy(&ppdisplays)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentFont(::core::mem::transmute_copy(&ppfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDocumentFont(&*(&pfont as *const <ITextFont2 as ::windows::core::Abi>::Abi as *const <ITextFont2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentPara<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentPara(::core::mem::transmute_copy(&pppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentPara<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDocumentPara(&*(&ppara as *const <ITextPara2 as ::windows::core::Abi>::Abi as *const <ITextPara2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEastAsianFlags<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEastAsianFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerator<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGenerator(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIMEInProgress<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIMEInProgress(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotificationMode<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotificationMode(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMode<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotificationMode(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection2(::core::mem::transmute_copy(&ppsel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryRanges2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryRanges2(::core::mem::transmute_copy(&ppstories)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypographyOptions<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypographyOptions(::core::mem::transmute_copy(&poptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachMsgFilter<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachMsgFilter(&*(&pfilter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckTextLimit<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckTextLimit(cch, pcch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallManager<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCallManager(::core::mem::transmute_copy(&ppvoid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientRect<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientRect(r#type, ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectColor<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectColor(index, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmContext<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImmContext(::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredFont<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredFont(cp, charrep, options, curcharrep, curfontsize, ::core::mem::transmute_copy(&pbstr), ::core::mem::transmute_copy(&ppitchandfamily), ::core::mem::transmute_copy(&pnewfontsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(r#type, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrings<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrings(::core::mem::transmute_copy(&ppstrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(notify) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Range2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Range2(cpactive, cpanchor, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint2<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint2(x, y, r#type, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseCallManager(&*(&pvoid as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseImmContext(context) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffectColor<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEffectColor(index, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(r#type, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypographyOptions<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTypographyOptions(options, mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysBeep<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SysBeep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateWindow<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMathProperties<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMathProperties(::core::mem::transmute_copy(&poptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathProperties<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMathProperties(options, mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveStory(::core::mem::transmute_copy(&ppstory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveStory(&*(&pstory as *const <ITextStory as ::windows::core::Abi>::Abi as *const <ITextStory as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMainStory(::core::mem::transmute_copy(&ppstory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNewStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNewStory(::core::mem::transmute_copy(&ppstory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStory<Impl: ITextDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppstory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStory(index, ::core::mem::transmute_copy(&ppstory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextDocument2>,
            ::windows::core::GetTrustLevel,
            GetCaretType::<Impl, OFFSET>,
            SetCaretType::<Impl, OFFSET>,
            GetDisplays::<Impl, OFFSET>,
            GetDocumentFont::<Impl, OFFSET>,
            SetDocumentFont::<Impl, OFFSET>,
            GetDocumentPara::<Impl, OFFSET>,
            SetDocumentPara::<Impl, OFFSET>,
            GetEastAsianFlags::<Impl, OFFSET>,
            GetGenerator::<Impl, OFFSET>,
            SetIMEInProgress::<Impl, OFFSET>,
            GetNotificationMode::<Impl, OFFSET>,
            SetNotificationMode::<Impl, OFFSET>,
            GetSelection2::<Impl, OFFSET>,
            GetStoryRanges2::<Impl, OFFSET>,
            GetTypographyOptions::<Impl, OFFSET>,
            GetVersion::<Impl, OFFSET>,
            GetWindow::<Impl, OFFSET>,
            AttachMsgFilter::<Impl, OFFSET>,
            CheckTextLimit::<Impl, OFFSET>,
            GetCallManager::<Impl, OFFSET>,
            GetClientRect::<Impl, OFFSET>,
            GetEffectColor::<Impl, OFFSET>,
            GetImmContext::<Impl, OFFSET>,
            GetPreferredFont::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetStrings::<Impl, OFFSET>,
            Notify::<Impl, OFFSET>,
            Range2::<Impl, OFFSET>,
            RangeFromPoint2::<Impl, OFFSET>,
            ReleaseCallManager::<Impl, OFFSET>,
            ReleaseImmContext::<Impl, OFFSET>,
            SetEffectColor::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            SetTypographyOptions::<Impl, OFFSET>,
            SysBeep::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            UpdateWindow::<Impl, OFFSET>,
            GetMathProperties::<Impl, OFFSET>,
            SetMathProperties::<Impl, OFFSET>,
            GetActiveStory::<Impl, OFFSET>,
            SetActiveStory::<Impl, OFFSET>,
            GetMainStory::<Impl, OFFSET>,
            GetNewStory::<Impl, OFFSET>,
            GetStory::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocument2OldImpl: Sized + ITextDocumentImpl + IDispatchImpl {
    fn AttachMsgFilter();
    fn SetEffectColor();
    fn GetEffectColor();
    fn GetCaretType();
    fn SetCaretType();
    fn GetImmContext();
    fn ReleaseImmContext();
    fn GetPreferredFont();
    fn GetNotificationMode();
    fn SetNotificationMode();
    fn GetClientRect();
    fn GetSelection2();
    fn GetWindow();
    fn GetFEFlags();
    fn UpdateWindow();
    fn CheckTextLimit();
    fn IMEInProgress();
    fn SysBeep();
    fn Update();
    fn Notify();
    fn GetDocumentFont();
    fn GetDocumentPara();
    fn GetCallManager();
    fn ReleaseCallManager();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextDocument2Old {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextDocument2Old";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2OldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextDocument2OldImpl, const OFFSET: isize>() -> ITextDocument2OldVtbl {
        unsafe extern "system" fn AttachMsgFilter<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachMsgFilter(&*(&pfilter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffectColor<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, cr: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEffectColor(index, cr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectColor<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pcr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectColor(index, ::core::mem::transmute_copy(&pcr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretType<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcarettype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaretType(::core::mem::transmute_copy(&pcarettype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, carettype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCaretType(carettype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImmContext<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImmContext(::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseImmContext<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseImmContext(context) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredFont<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut super::super::super::Foundation::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredFont(cp, charrep, option, charrepcur, curfontsize, ::core::mem::transmute_copy(&pbstr), ::core::mem::transmute_copy(&ppitchandfamily), ::core::mem::transmute_copy(&pnewfontsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotificationMode<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotificationMode(::core::mem::transmute_copy(&pmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationMode<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotificationMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientRect<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientRect(r#type, ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection2<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection2(::core::mem::transmute_copy(&ppsel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindow<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFEFlags<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFEFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateWindow<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckTextLimit<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckTextLimit(cch, pcch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IMEInProgress<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IMEInProgress(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysBeep<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SysBeep() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notify(notify) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentFont<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitextfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentFont(::core::mem::transmute_copy(&ppitextfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentPara<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitextpara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDocumentPara(::core::mem::transmute_copy(&ppitextpara)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallManager<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCallManager(::core::mem::transmute_copy(&ppvoid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseCallManager<Impl: ITextDocument2OldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseCallManager(&*(&pvoid as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextDocument2Old>,
            ::windows::core::GetTrustLevel,
            AttachMsgFilter::<Impl, OFFSET>,
            SetEffectColor::<Impl, OFFSET>,
            GetEffectColor::<Impl, OFFSET>,
            GetCaretType::<Impl, OFFSET>,
            SetCaretType::<Impl, OFFSET>,
            GetImmContext::<Impl, OFFSET>,
            ReleaseImmContext::<Impl, OFFSET>,
            GetPreferredFont::<Impl, OFFSET>,
            GetNotificationMode::<Impl, OFFSET>,
            SetNotificationMode::<Impl, OFFSET>,
            GetClientRect::<Impl, OFFSET>,
            GetSelection2::<Impl, OFFSET>,
            GetWindow::<Impl, OFFSET>,
            GetFEFlags::<Impl, OFFSET>,
            UpdateWindow::<Impl, OFFSET>,
            CheckTextLimit::<Impl, OFFSET>,
            IMEInProgress::<Impl, OFFSET>,
            SysBeep::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            Notify::<Impl, OFFSET>,
            GetDocumentFont::<Impl, OFFSET>,
            GetDocumentPara::<Impl, OFFSET>,
            GetCallManager::<Impl, OFFSET>,
            ReleaseCallManager::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextFontImpl: Sized + IDispatchImpl {
    fn GetDuplicate();
    fn SetDuplicate();
    fn CanChange();
    fn IsEqual();
    fn Reset();
    fn GetStyle();
    fn SetStyle();
    fn GetAllCaps();
    fn SetAllCaps();
    fn GetAnimation();
    fn SetAnimation();
    fn GetBackColor();
    fn SetBackColor();
    fn GetBold();
    fn SetBold();
    fn GetEmboss();
    fn SetEmboss();
    fn GetForeColor();
    fn SetForeColor();
    fn GetHidden();
    fn SetHidden();
    fn GetEngrave();
    fn SetEngrave();
    fn GetItalic();
    fn SetItalic();
    fn GetKerning();
    fn SetKerning();
    fn GetLanguageID();
    fn SetLanguageID();
    fn GetName();
    fn SetName();
    fn GetOutline();
    fn SetOutline();
    fn GetPosition();
    fn SetPosition();
    fn GetProtected();
    fn SetProtected();
    fn GetShadow();
    fn SetShadow();
    fn GetSize();
    fn SetSize();
    fn GetSmallCaps();
    fn SetSmallCaps();
    fn GetSpacing();
    fn SetSpacing();
    fn GetStrikeThrough();
    fn SetStrikeThrough();
    fn GetSubscript();
    fn SetSubscript();
    fn GetSuperscript();
    fn SetSuperscript();
    fn GetUnderline();
    fn SetUnderline();
    fn GetWeight();
    fn SetWeight();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextFont {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextFont";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextFontVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextFontImpl, const OFFSET: isize>() -> ITextFontVtbl {
        unsafe extern "system" fn GetDuplicate<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate(::core::mem::transmute_copy(&ppfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDuplicate(&*(&pfont as *const <ITextFont as ::windows::core::Abi>::Abi as *const <ITextFont as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChange<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChange(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&pfont as *const <ITextFont as ::windows::core::Abi>::Abi as *const <ITextFont as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyle<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyle(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStyle(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCaps(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllCaps(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnimation<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnimation(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimation<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAnimation(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackColor(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackColor(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBold<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBold(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBold(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmboss<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmboss(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmboss<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEmboss(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForeColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForeColor(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetForeColor(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHidden<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHidden(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHidden(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEngrave<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEngrave(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEngrave<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEngrave(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItalic<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItalic(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetItalic(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKerning<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKerning(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKerning(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageID<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageID(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageID<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLanguageID(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutline(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutline(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPosition(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtected<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtected(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtected<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProtected(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShadow<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShadow(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShadow<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetShadow(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSize(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSmallCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSmallCaps(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallCaps<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSmallCaps(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpacing<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpacing(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpacing<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSpacing(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStrikeThrough<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStrikeThrough(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikeThrough<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStrikeThrough(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubscript(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSubscript(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSuperscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSuperscript(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSuperscript(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnderline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderline(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUnderline(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWeight<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWeight(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Impl: ITextFontImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWeight(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextFont>,
            ::windows::core::GetTrustLevel,
            GetDuplicate::<Impl, OFFSET>,
            SetDuplicate::<Impl, OFFSET>,
            CanChange::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            GetStyle::<Impl, OFFSET>,
            SetStyle::<Impl, OFFSET>,
            GetAllCaps::<Impl, OFFSET>,
            SetAllCaps::<Impl, OFFSET>,
            GetAnimation::<Impl, OFFSET>,
            SetAnimation::<Impl, OFFSET>,
            GetBackColor::<Impl, OFFSET>,
            SetBackColor::<Impl, OFFSET>,
            GetBold::<Impl, OFFSET>,
            SetBold::<Impl, OFFSET>,
            GetEmboss::<Impl, OFFSET>,
            SetEmboss::<Impl, OFFSET>,
            GetForeColor::<Impl, OFFSET>,
            SetForeColor::<Impl, OFFSET>,
            GetHidden::<Impl, OFFSET>,
            SetHidden::<Impl, OFFSET>,
            GetEngrave::<Impl, OFFSET>,
            SetEngrave::<Impl, OFFSET>,
            GetItalic::<Impl, OFFSET>,
            SetItalic::<Impl, OFFSET>,
            GetKerning::<Impl, OFFSET>,
            SetKerning::<Impl, OFFSET>,
            GetLanguageID::<Impl, OFFSET>,
            SetLanguageID::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            GetOutline::<Impl, OFFSET>,
            SetOutline::<Impl, OFFSET>,
            GetPosition::<Impl, OFFSET>,
            SetPosition::<Impl, OFFSET>,
            GetProtected::<Impl, OFFSET>,
            SetProtected::<Impl, OFFSET>,
            GetShadow::<Impl, OFFSET>,
            SetShadow::<Impl, OFFSET>,
            GetSize::<Impl, OFFSET>,
            SetSize::<Impl, OFFSET>,
            GetSmallCaps::<Impl, OFFSET>,
            SetSmallCaps::<Impl, OFFSET>,
            GetSpacing::<Impl, OFFSET>,
            SetSpacing::<Impl, OFFSET>,
            GetStrikeThrough::<Impl, OFFSET>,
            SetStrikeThrough::<Impl, OFFSET>,
            GetSubscript::<Impl, OFFSET>,
            SetSubscript::<Impl, OFFSET>,
            GetSuperscript::<Impl, OFFSET>,
            SetSuperscript::<Impl, OFFSET>,
            GetUnderline::<Impl, OFFSET>,
            SetUnderline::<Impl, OFFSET>,
            GetWeight::<Impl, OFFSET>,
            SetWeight::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextFont2Impl: Sized + ITextFontImpl + IDispatchImpl {
    fn GetCount();
    fn GetAutoLigatures();
    fn SetAutoLigatures();
    fn GetAutospaceAlpha();
    fn SetAutospaceAlpha();
    fn GetAutospaceNumeric();
    fn SetAutospaceNumeric();
    fn GetAutospaceParens();
    fn SetAutospaceParens();
    fn GetCharRep();
    fn SetCharRep();
    fn GetCompressionMode();
    fn SetCompressionMode();
    fn GetCookie();
    fn SetCookie();
    fn GetDoubleStrike();
    fn SetDoubleStrike();
    fn GetDuplicate2();
    fn SetDuplicate2();
    fn GetLinkType();
    fn GetMathZone();
    fn SetMathZone();
    fn GetModWidthPairs();
    fn SetModWidthPairs();
    fn GetModWidthSpace();
    fn SetModWidthSpace();
    fn GetOldNumbers();
    fn SetOldNumbers();
    fn GetOverlapping();
    fn SetOverlapping();
    fn GetPositionSubSuper();
    fn SetPositionSubSuper();
    fn GetScaling();
    fn SetScaling();
    fn GetSpaceExtension();
    fn SetSpaceExtension();
    fn GetUnderlinePositionMode();
    fn SetUnderlinePositionMode();
    fn GetEffects();
    fn GetEffects2();
    fn GetProperty();
    fn GetPropertyInfo();
    fn IsEqual2();
    fn SetEffects();
    fn SetEffects2();
    fn SetProperty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextFont2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextFont2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextFont2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextFont2Impl, const OFFSET: isize>() -> ITextFont2Vtbl {
        unsafe extern "system" fn GetCount<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoLigatures<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoLigatures(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoLigatures<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoLigatures(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutospaceAlpha<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutospaceAlpha(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceAlpha<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutospaceAlpha(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutospaceNumeric<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutospaceNumeric(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceNumeric<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutospaceNumeric(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutospaceParens<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutospaceParens(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutospaceParens<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutospaceParens(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharRep<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharRep(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharRep<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCharRep(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompressionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCompressionMode(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompressionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompressionMode(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCookie<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCookie(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCookie<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCookie(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDoubleStrike<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDoubleStrike(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoubleStrike<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDoubleStrike(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate2(::core::mem::transmute_copy(&ppfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDuplicate2(&*(&pfont as *const <ITextFont2 as ::windows::core::Abi>::Abi as *const <ITextFont2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLinkType<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLinkType(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMathZone<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMathZone(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMathZone<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMathZone(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModWidthPairs<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModWidthPairs(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModWidthPairs<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetModWidthPairs(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModWidthSpace<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModWidthSpace(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModWidthSpace<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetModWidthSpace(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOldNumbers<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOldNumbers(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOldNumbers<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOldNumbers(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlapping<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverlapping(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlapping<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOverlapping(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPositionSubSuper<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPositionSubSuper(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPositionSubSuper<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPositionSubSuper(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScaling<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScaling(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaling<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScaling(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpaceExtension<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpaceExtension(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceExtension<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSpaceExtension(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnderlinePositionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderlinePositionMode(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderlinePositionMode<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUnderlinePositionMode(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffects<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffects(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffects2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffects2(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(r#type, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfo<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyInfo(index, ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual2(&*(&pfont as *const <ITextFont2 as ::windows::core::Abi>::Abi as *const <ITextFont2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEffects(value, mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects2<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEffects2(value, mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ITextFont2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(r#type, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextFont2>,
            ::windows::core::GetTrustLevel,
            GetCount::<Impl, OFFSET>,
            GetAutoLigatures::<Impl, OFFSET>,
            SetAutoLigatures::<Impl, OFFSET>,
            GetAutospaceAlpha::<Impl, OFFSET>,
            SetAutospaceAlpha::<Impl, OFFSET>,
            GetAutospaceNumeric::<Impl, OFFSET>,
            SetAutospaceNumeric::<Impl, OFFSET>,
            GetAutospaceParens::<Impl, OFFSET>,
            SetAutospaceParens::<Impl, OFFSET>,
            GetCharRep::<Impl, OFFSET>,
            SetCharRep::<Impl, OFFSET>,
            GetCompressionMode::<Impl, OFFSET>,
            SetCompressionMode::<Impl, OFFSET>,
            GetCookie::<Impl, OFFSET>,
            SetCookie::<Impl, OFFSET>,
            GetDoubleStrike::<Impl, OFFSET>,
            SetDoubleStrike::<Impl, OFFSET>,
            GetDuplicate2::<Impl, OFFSET>,
            SetDuplicate2::<Impl, OFFSET>,
            GetLinkType::<Impl, OFFSET>,
            GetMathZone::<Impl, OFFSET>,
            SetMathZone::<Impl, OFFSET>,
            GetModWidthPairs::<Impl, OFFSET>,
            SetModWidthPairs::<Impl, OFFSET>,
            GetModWidthSpace::<Impl, OFFSET>,
            SetModWidthSpace::<Impl, OFFSET>,
            GetOldNumbers::<Impl, OFFSET>,
            SetOldNumbers::<Impl, OFFSET>,
            GetOverlapping::<Impl, OFFSET>,
            SetOverlapping::<Impl, OFFSET>,
            GetPositionSubSuper::<Impl, OFFSET>,
            SetPositionSubSuper::<Impl, OFFSET>,
            GetScaling::<Impl, OFFSET>,
            SetScaling::<Impl, OFFSET>,
            GetSpaceExtension::<Impl, OFFSET>,
            SetSpaceExtension::<Impl, OFFSET>,
            GetUnderlinePositionMode::<Impl, OFFSET>,
            SetUnderlinePositionMode::<Impl, OFFSET>,
            GetEffects::<Impl, OFFSET>,
            GetEffects2::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetPropertyInfo::<Impl, OFFSET>,
            IsEqual2::<Impl, OFFSET>,
            SetEffects::<Impl, OFFSET>,
            SetEffects2::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
        )
    }
}
pub trait ITextHostImpl: Sized {
    fn TxGetDC();
    fn TxReleaseDC();
    fn TxShowScrollBar();
    fn TxEnableScrollBar();
    fn TxSetScrollRange();
    fn TxSetScrollPos();
    fn TxInvalidateRect();
    fn TxViewChange();
    fn TxCreateCaret();
    fn TxShowCaret();
    fn TxSetCaretPos();
    fn TxSetTimer();
    fn TxKillTimer();
    fn TxScrollWindowEx();
    fn TxSetCapture();
    fn TxSetFocus();
    fn TxSetCursor();
    fn TxScreenToClient();
    fn TxClientToScreen();
    fn TxActivate();
    fn TxDeactivate();
    fn TxGetClientRect();
    fn TxGetViewInset();
    fn TxGetCharFormat();
    fn TxGetParaFormat();
    fn TxGetSysColor();
    fn TxGetBackStyle();
    fn TxGetMaxLength();
    fn TxGetScrollBars();
    fn TxGetPasswordChar();
    fn TxGetAcceleratorPos();
    fn TxGetExtent();
    fn OnTxCharFormatChange();
    fn OnTxParaFormatChange();
    fn TxGetPropertyBits();
    fn TxNotify();
    fn TxImmGetContext();
    fn TxImmReleaseContext();
    fn TxGetSelectionBarWidth();
}
impl ::windows::core::RuntimeName for ITextHost {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextHost";
}
impl ITextHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHostImpl, const OFFSET: isize>() -> ITextHostVtbl {
        unsafe extern "system" fn TxGetDC<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetDC() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxReleaseDC<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxReleaseDC(&*(&hdc as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxShowScrollBar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxShowScrollBar(fnbar, &*(&fshow as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxEnableScrollBar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: super::ENABLE_SCROLL_BAR_ARROWS) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxEnableScrollBar(fusbflags, fuarrowflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxSetScrollRange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxSetScrollRange(fnbar, nminpos, nmaxpos, &*(&fredraw as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxSetScrollPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxSetScrollPos(fnbar, npos, &*(&fredraw as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxInvalidateRect<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxInvalidateRect(&*(&prc as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&fmode as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TxViewChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fupdate: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxViewChange(&*(&fupdate as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TxCreateCaret<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxCreateCaret(&*(&hbmp as *const <super::super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HBITMAP as ::windows::core::DefaultType>::DefaultType), xwidth, yheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxShowCaret<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxShowCaret(&*(&fshow as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxSetCaretPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxSetCaretPos(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxSetTimer<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxSetTimer(idtimer, utimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxKillTimer<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idtimer: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxKillTimer(idtimer).into()
        }
        unsafe extern "system" fn TxScrollWindowEx<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SHOW_WINDOW_CMD) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .TxScrollWindowEx(
                    dx,
                    dy,
                    &*(&lprcscroll as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                    &*(&lprcclip as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                    &*(&hrgnupdate as *const <super::super::super::Graphics::Gdi::HRGN as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HRGN as ::windows::core::DefaultType>::DefaultType),
                    &*(&lprcupdate as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                    fuscroll,
                )
                .into()
        }
        unsafe extern "system" fn TxSetCapture<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcapture: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetCapture(&*(&fcapture as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TxSetFocus<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetFocus().into()
        }
        unsafe extern "system" fn TxSetCursor<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxSetCursor(&*(&hcur as *const <super::super::WindowsAndMessaging::HCURSOR as ::windows::core::Abi>::Abi as *const <super::super::WindowsAndMessaging::HCURSOR as ::windows::core::DefaultType>::DefaultType), &*(&ftext as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TxScreenToClient<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxScreenToClient(&*(&lppt as *const <super::super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxClientToScreen<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxClientToScreen(&*(&lppt as *const <super::super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxActivate<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploldstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxActivate(ploldstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxDeactivate<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewstate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxDeactivate(lnewstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetClientRect<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetClientRect(&*(&prc as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetViewInset<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetViewInset(&*(&prc as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetCharFormat<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetCharFormat(&*(&ppcf as *const <CHARFORMATW as ::windows::core::Abi>::Abi as *const <CHARFORMATW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetParaFormat<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppf: *const *const PARAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetParaFormat(&*(&pppf as *const <PARAFORMAT as ::windows::core::Abi>::Abi as *const <PARAFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetSysColor<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetSysColor(nindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetBackStyle<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetBackStyle(pstyle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetMaxLength<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetMaxLength(plength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetScrollBars<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscrollbar: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetScrollBars(pdwscrollbar) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetPasswordChar<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pch: *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetPasswordChar(::core::mem::transmute_copy(&pch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetAcceleratorPos<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetAcceleratorPos(pcp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetExtent<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetExtent(&*(&lpextent as *const <super::super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxCharFormatChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: *const CHARFORMATW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTxCharFormatChange(&*(&pcf as *const <CHARFORMATW as ::windows::core::Abi>::Abi as *const <CHARFORMATW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxParaFormatChange<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppf: *const PARAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTxParaFormatChange(&*(&ppf as *const <PARAFORMAT as ::windows::core::Abi>::Abi as *const <PARAFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetPropertyBits<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetPropertyBits(dwmask, pdwbits) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxNotify<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxNotify(inotify, &*(&pv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxImmGetContext<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Globalization::HIMC {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxImmGetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxImmReleaseContext<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxImmReleaseContext(&*(&himc as *const <super::super::super::Globalization::HIMC as ::windows::core::Abi>::Abi as *const <super::super::super::Globalization::HIMC as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TxGetSelectionBarWidth<Impl: ITextHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lselbarwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetSelectionBarWidth(lselbarwidth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextHost>,
            ::windows::core::GetTrustLevel,
            TxGetDC::<Impl, OFFSET>,
            TxReleaseDC::<Impl, OFFSET>,
            TxShowScrollBar::<Impl, OFFSET>,
            TxEnableScrollBar::<Impl, OFFSET>,
            TxSetScrollRange::<Impl, OFFSET>,
            TxSetScrollPos::<Impl, OFFSET>,
            TxInvalidateRect::<Impl, OFFSET>,
            TxViewChange::<Impl, OFFSET>,
            TxCreateCaret::<Impl, OFFSET>,
            TxShowCaret::<Impl, OFFSET>,
            TxSetCaretPos::<Impl, OFFSET>,
            TxSetTimer::<Impl, OFFSET>,
            TxKillTimer::<Impl, OFFSET>,
            TxScrollWindowEx::<Impl, OFFSET>,
            TxSetCapture::<Impl, OFFSET>,
            TxSetFocus::<Impl, OFFSET>,
            TxSetCursor::<Impl, OFFSET>,
            TxScreenToClient::<Impl, OFFSET>,
            TxClientToScreen::<Impl, OFFSET>,
            TxActivate::<Impl, OFFSET>,
            TxDeactivate::<Impl, OFFSET>,
            TxGetClientRect::<Impl, OFFSET>,
            TxGetViewInset::<Impl, OFFSET>,
            TxGetCharFormat::<Impl, OFFSET>,
            TxGetParaFormat::<Impl, OFFSET>,
            TxGetSysColor::<Impl, OFFSET>,
            TxGetBackStyle::<Impl, OFFSET>,
            TxGetMaxLength::<Impl, OFFSET>,
            TxGetScrollBars::<Impl, OFFSET>,
            TxGetPasswordChar::<Impl, OFFSET>,
            TxGetAcceleratorPos::<Impl, OFFSET>,
            TxGetExtent::<Impl, OFFSET>,
            OnTxCharFormatChange::<Impl, OFFSET>,
            OnTxParaFormatChange::<Impl, OFFSET>,
            TxGetPropertyBits::<Impl, OFFSET>,
            TxNotify::<Impl, OFFSET>,
            TxImmGetContext::<Impl, OFFSET>,
            TxImmReleaseContext::<Impl, OFFSET>,
            TxGetSelectionBarWidth::<Impl, OFFSET>,
        )
    }
}
pub trait ITextHost2Impl: Sized + ITextHostImpl {
    fn TxIsDoubleClickPending();
    fn TxGetWindow();
    fn TxSetForegroundWindow();
    fn TxGetPalette();
    fn TxGetEastAsianFlags();
    fn TxSetCursor2();
    fn TxFreeTextServicesNotification();
    fn TxGetEditStyle();
    fn TxGetWindowStyles();
    fn TxShowDropCaret();
    fn TxDestroyCaret();
    fn TxGetHorzExtent();
}
impl ::windows::core::RuntimeName for ITextHost2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextHost2";
}
impl ITextHost2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextHost2Impl, const OFFSET: isize>() -> ITextHost2Vtbl {
        unsafe extern "system" fn TxIsDoubleClickPending<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxIsDoubleClickPending() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetWindow<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetWindow(&*(&phwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxSetForegroundWindow<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxSetForegroundWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetPalette<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetPalette() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetEastAsianFlags<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetEastAsianFlags(pflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxSetCursor2<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxSetCursor2(&*(&hcur as *const <super::super::WindowsAndMessaging::HCURSOR as ::windows::core::Abi>::Abi as *const <super::super::WindowsAndMessaging::HCURSOR as ::windows::core::DefaultType>::DefaultType), &*(&btext as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxFreeTextServicesNotification<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TxFreeTextServicesNotification().into()
        }
        unsafe extern "system" fn TxGetEditStyle<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetEditStyle(dwitem, pdwdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetWindowStyles<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetWindowStyles(pdwstyle, pdwexstyle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxShowDropCaret<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxShowDropCaret(
                &*(&fshow as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&hdc as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&prc as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxDestroyCaret<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxDestroyCaret() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetHorzExtent<Impl: ITextHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhorzextent: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetHorzExtent(plhorzextent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextHost2>,
            ::windows::core::GetTrustLevel,
            TxIsDoubleClickPending::<Impl, OFFSET>,
            TxGetWindow::<Impl, OFFSET>,
            TxSetForegroundWindow::<Impl, OFFSET>,
            TxGetPalette::<Impl, OFFSET>,
            TxGetEastAsianFlags::<Impl, OFFSET>,
            TxSetCursor2::<Impl, OFFSET>,
            TxFreeTextServicesNotification::<Impl, OFFSET>,
            TxGetEditStyle::<Impl, OFFSET>,
            TxGetWindowStyles::<Impl, OFFSET>,
            TxShowDropCaret::<Impl, OFFSET>,
            TxDestroyCaret::<Impl, OFFSET>,
            TxGetHorzExtent::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextParaImpl: Sized + IDispatchImpl {
    fn GetDuplicate();
    fn SetDuplicate();
    fn CanChange();
    fn IsEqual();
    fn Reset();
    fn GetStyle();
    fn SetStyle();
    fn GetAlignment();
    fn SetAlignment();
    fn GetHyphenation();
    fn SetHyphenation();
    fn GetFirstLineIndent();
    fn GetKeepTogether();
    fn SetKeepTogether();
    fn GetKeepWithNext();
    fn SetKeepWithNext();
    fn GetLeftIndent();
    fn GetLineSpacing();
    fn GetLineSpacingRule();
    fn GetListAlignment();
    fn SetListAlignment();
    fn GetListLevelIndex();
    fn SetListLevelIndex();
    fn GetListStart();
    fn SetListStart();
    fn GetListTab();
    fn SetListTab();
    fn GetListType();
    fn SetListType();
    fn GetNoLineNumber();
    fn SetNoLineNumber();
    fn GetPageBreakBefore();
    fn SetPageBreakBefore();
    fn GetRightIndent();
    fn SetRightIndent();
    fn SetIndents();
    fn SetLineSpacing();
    fn GetSpaceAfter();
    fn SetSpaceAfter();
    fn GetSpaceBefore();
    fn SetSpaceBefore();
    fn GetWidowControl();
    fn SetWidowControl();
    fn GetTabCount();
    fn AddTab();
    fn ClearAllTabs();
    fn DeleteTab();
    fn GetTab();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextPara {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextPara";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextParaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextParaImpl, const OFFSET: isize>() -> ITextParaVtbl {
        unsafe extern "system" fn GetDuplicate<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate(::core::mem::transmute_copy(&pppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDuplicate(&*(&ppara as *const <ITextPara as ::windows::core::Abi>::Abi as *const <ITextPara as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChange<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChange(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&ppara as *const <ITextPara as ::windows::core::Abi>::Abi as *const <ITextPara as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStyle<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStyle(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStyle(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlignment(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlignment(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHyphenation<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHyphenation(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHyphenation<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHyphenation(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstLineIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstLineIndent(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeepTogether<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeepTogether(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKeepTogether(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeepWithNext<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeepWithNext(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKeepWithNext(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLeftIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLeftIndent(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacing<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineSpacing(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineSpacingRule<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLineSpacingRule(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListAlignment(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListAlignment<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetListAlignment(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListLevelIndex<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListLevelIndex(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetListLevelIndex(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListStart<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListStart(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStart<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetListStart(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListTab(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetListTab(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListType<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListType(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListType<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetListType(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNoLineNumber<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNoLineNumber(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNoLineNumber(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageBreakBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPageBreakBefore(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPageBreakBefore(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRightIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRightIndent(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightIndent<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRightIndent(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndents<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: f32, left: f32, right: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIndents(first, left, right) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineSpacing<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: i32, spacing: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLineSpacing(rule, spacing) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpaceAfter<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpaceAfter(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSpaceAfter(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpaceBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpaceBefore(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSpaceBefore(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWidowControl<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWidowControl(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidowControl<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWidowControl(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTabCount<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTabCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTab(tbpos, tbalign, tbleader) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearAllTabs<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearAllTabs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tbpos: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteTab(tbpos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTab<Impl: ITextParaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTab(itab, ::core::mem::transmute_copy(&ptbpos), ::core::mem::transmute_copy(&ptbalign), ::core::mem::transmute_copy(&ptbleader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextPara>,
            ::windows::core::GetTrustLevel,
            GetDuplicate::<Impl, OFFSET>,
            SetDuplicate::<Impl, OFFSET>,
            CanChange::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            GetStyle::<Impl, OFFSET>,
            SetStyle::<Impl, OFFSET>,
            GetAlignment::<Impl, OFFSET>,
            SetAlignment::<Impl, OFFSET>,
            GetHyphenation::<Impl, OFFSET>,
            SetHyphenation::<Impl, OFFSET>,
            GetFirstLineIndent::<Impl, OFFSET>,
            GetKeepTogether::<Impl, OFFSET>,
            SetKeepTogether::<Impl, OFFSET>,
            GetKeepWithNext::<Impl, OFFSET>,
            SetKeepWithNext::<Impl, OFFSET>,
            GetLeftIndent::<Impl, OFFSET>,
            GetLineSpacing::<Impl, OFFSET>,
            GetLineSpacingRule::<Impl, OFFSET>,
            GetListAlignment::<Impl, OFFSET>,
            SetListAlignment::<Impl, OFFSET>,
            GetListLevelIndex::<Impl, OFFSET>,
            SetListLevelIndex::<Impl, OFFSET>,
            GetListStart::<Impl, OFFSET>,
            SetListStart::<Impl, OFFSET>,
            GetListTab::<Impl, OFFSET>,
            SetListTab::<Impl, OFFSET>,
            GetListType::<Impl, OFFSET>,
            SetListType::<Impl, OFFSET>,
            GetNoLineNumber::<Impl, OFFSET>,
            SetNoLineNumber::<Impl, OFFSET>,
            GetPageBreakBefore::<Impl, OFFSET>,
            SetPageBreakBefore::<Impl, OFFSET>,
            GetRightIndent::<Impl, OFFSET>,
            SetRightIndent::<Impl, OFFSET>,
            SetIndents::<Impl, OFFSET>,
            SetLineSpacing::<Impl, OFFSET>,
            GetSpaceAfter::<Impl, OFFSET>,
            SetSpaceAfter::<Impl, OFFSET>,
            GetSpaceBefore::<Impl, OFFSET>,
            SetSpaceBefore::<Impl, OFFSET>,
            GetWidowControl::<Impl, OFFSET>,
            SetWidowControl::<Impl, OFFSET>,
            GetTabCount::<Impl, OFFSET>,
            AddTab::<Impl, OFFSET>,
            ClearAllTabs::<Impl, OFFSET>,
            DeleteTab::<Impl, OFFSET>,
            GetTab::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextPara2Impl: Sized + ITextParaImpl + IDispatchImpl {
    fn GetBorders();
    fn GetDuplicate2();
    fn SetDuplicate2();
    fn GetFontAlignment();
    fn SetFontAlignment();
    fn GetHangingPunctuation();
    fn SetHangingPunctuation();
    fn GetSnapToGrid();
    fn SetSnapToGrid();
    fn GetTrimPunctuationAtStart();
    fn SetTrimPunctuationAtStart();
    fn GetEffects();
    fn GetProperty();
    fn IsEqual2();
    fn SetEffects();
    fn SetProperty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextPara2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextPara2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextPara2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPara2Impl, const OFFSET: isize>() -> ITextPara2Vtbl {
        unsafe extern "system" fn GetBorders<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppborders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBorders(::core::mem::transmute_copy(&ppborders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate2(::core::mem::transmute_copy(&pppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuplicate2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDuplicate2(&*(&ppara as *const <ITextPara2 as ::windows::core::Abi>::Abi as *const <ITextPara2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFontAlignment<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFontAlignment(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontAlignment<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFontAlignment(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHangingPunctuation<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHangingPunctuation(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHangingPunctuation<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHangingPunctuation(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapToGrid<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapToGrid(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapToGrid<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSnapToGrid(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrimPunctuationAtStart<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrimPunctuationAtStart(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrimPunctuationAtStart<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrimPunctuationAtStart(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffects<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffects(::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(r#type, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual2<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual2(&*(&ppara as *const <ITextPara2 as ::windows::core::Abi>::Abi as *const <ITextPara2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEffects<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEffects(value, mask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ITextPara2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(r#type, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextPara2>,
            ::windows::core::GetTrustLevel,
            GetBorders::<Impl, OFFSET>,
            GetDuplicate2::<Impl, OFFSET>,
            SetDuplicate2::<Impl, OFFSET>,
            GetFontAlignment::<Impl, OFFSET>,
            SetFontAlignment::<Impl, OFFSET>,
            GetHangingPunctuation::<Impl, OFFSET>,
            SetHangingPunctuation::<Impl, OFFSET>,
            GetSnapToGrid::<Impl, OFFSET>,
            SetSnapToGrid::<Impl, OFFSET>,
            GetTrimPunctuationAtStart::<Impl, OFFSET>,
            SetTrimPunctuationAtStart::<Impl, OFFSET>,
            GetEffects::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            IsEqual2::<Impl, OFFSET>,
            SetEffects::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRangeImpl: Sized + IDispatchImpl {
    fn GetText();
    fn SetText();
    fn GetChar();
    fn SetChar();
    fn GetDuplicate();
    fn GetFormattedText();
    fn SetFormattedText();
    fn GetStart();
    fn SetStart();
    fn GetEnd();
    fn SetEnd();
    fn GetFont();
    fn SetFont();
    fn GetPara();
    fn SetPara();
    fn GetStoryLength();
    fn GetStoryType();
    fn Collapse();
    fn Expand();
    fn GetIndex();
    fn SetIndex();
    fn SetRange();
    fn InRange();
    fn InStory();
    fn IsEqual();
    fn Select();
    fn StartOf();
    fn EndOf();
    fn Move();
    fn MoveStart();
    fn MoveEnd();
    fn MoveWhile();
    fn MoveStartWhile();
    fn MoveEndWhile();
    fn MoveUntil();
    fn MoveStartUntil();
    fn MoveEndUntil();
    fn FindText();
    fn FindTextStart();
    fn FindTextEnd();
    fn Delete();
    fn Cut();
    fn Copy();
    fn Paste();
    fn CanPaste();
    fn CanEdit();
    fn ChangeCase();
    fn GetPoint();
    fn SetPoint();
    fn ScrollIntoView();
    fn GetEmbeddedObject();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextRange {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextRange";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeImpl, const OFFSET: isize>() -> ITextRangeVtbl {
        unsafe extern "system" fn GetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChar<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChar(::core::mem::transmute_copy(&pchar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChar<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, char: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChar(char) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate(::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText(::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormattedText(&*(&prange as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStart(::core::mem::transmute_copy(&pcpfirst)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStart(cpfirst) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnd(::core::mem::transmute_copy(&pcplim)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnd(cplim) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFont(::core::mem::transmute_copy(&ppfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFont<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFont(&*(&pfont as *const <ITextFont as ::windows::core::Abi>::Abi as *const <ITextFont as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPara<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPara(::core::mem::transmute_copy(&pppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPara<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPara(&*(&ppara as *const <ITextPara as ::windows::core::Abi>::Abi as *const <ITextPara as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryLength<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryLength(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryType<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStoryType(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collapse(bstart) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expand(unit, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex(unit, ::core::mem::transmute_copy(&pindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIndex(unit, index, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRange<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRange(cpanchor, cpactive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InRange<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InRange(&*(&prange as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InStory<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InStory(&*(&prange as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&prange as *const <ITextRange as ::windows::core::Abi>::Abi as *const <ITextRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Select<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOf<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOf(unit, extend, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOf<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndOf(unit, extend, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(unit, count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveStart(unit, count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEnd(unit, count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveWhile(&*(&cset as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStartWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveStartWhile(&*(&cset as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndWhile<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndWhile(&*(&cset as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveUntil(&*(&cset as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStartUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveStartUntil(&*(&cset as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndUntil<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Com::VARIANT, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndUntil(&*(&cset as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindText(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), count, flags, ::core::mem::transmute_copy(&plength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTextStart<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindTextStart(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), count, flags, ::core::mem::transmute_copy(&plength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTextEnd<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, count: i32, flags: i32, plength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindTextEnd(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), count, flags, ::core::mem::transmute_copy(&plength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(unit, count, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cut<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cut(::core::mem::transmute_copy(&pvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy(::core::mem::transmute_copy(&pvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Paste(&*(&pvar as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Com::VARIANT, format: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPaste(&*(&pvar as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), format, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanEdit<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanEdit(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCase<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeCase(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoint<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, px: *mut i32, py: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPoint(r#type, ::core::mem::transmute_copy(&px), ::core::mem::transmute_copy(&py)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPoint<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, extend: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPoint(x, y, r#type, extend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollIntoView<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollIntoView(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbeddedObject<Impl: ITextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddedObject(::core::mem::transmute_copy(&ppobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextRange>,
            ::windows::core::GetTrustLevel,
            GetText::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
            GetChar::<Impl, OFFSET>,
            SetChar::<Impl, OFFSET>,
            GetDuplicate::<Impl, OFFSET>,
            GetFormattedText::<Impl, OFFSET>,
            SetFormattedText::<Impl, OFFSET>,
            GetStart::<Impl, OFFSET>,
            SetStart::<Impl, OFFSET>,
            GetEnd::<Impl, OFFSET>,
            SetEnd::<Impl, OFFSET>,
            GetFont::<Impl, OFFSET>,
            SetFont::<Impl, OFFSET>,
            GetPara::<Impl, OFFSET>,
            SetPara::<Impl, OFFSET>,
            GetStoryLength::<Impl, OFFSET>,
            GetStoryType::<Impl, OFFSET>,
            Collapse::<Impl, OFFSET>,
            Expand::<Impl, OFFSET>,
            GetIndex::<Impl, OFFSET>,
            SetIndex::<Impl, OFFSET>,
            SetRange::<Impl, OFFSET>,
            InRange::<Impl, OFFSET>,
            InStory::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            Select::<Impl, OFFSET>,
            StartOf::<Impl, OFFSET>,
            EndOf::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            MoveStart::<Impl, OFFSET>,
            MoveEnd::<Impl, OFFSET>,
            MoveWhile::<Impl, OFFSET>,
            MoveStartWhile::<Impl, OFFSET>,
            MoveEndWhile::<Impl, OFFSET>,
            MoveUntil::<Impl, OFFSET>,
            MoveStartUntil::<Impl, OFFSET>,
            MoveEndUntil::<Impl, OFFSET>,
            FindText::<Impl, OFFSET>,
            FindTextStart::<Impl, OFFSET>,
            FindTextEnd::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Cut::<Impl, OFFSET>,
            Copy::<Impl, OFFSET>,
            Paste::<Impl, OFFSET>,
            CanPaste::<Impl, OFFSET>,
            CanEdit::<Impl, OFFSET>,
            ChangeCase::<Impl, OFFSET>,
            GetPoint::<Impl, OFFSET>,
            SetPoint::<Impl, OFFSET>,
            ScrollIntoView::<Impl, OFFSET>,
            GetEmbeddedObject::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRange2Impl: Sized + ITextSelectionImpl + ITextRangeImpl + IDispatchImpl {
    fn GetCch();
    fn GetCells();
    fn GetColumn();
    fn GetCount();
    fn GetDuplicate2();
    fn GetFont2();
    fn SetFont2();
    fn GetFormattedText2();
    fn SetFormattedText2();
    fn GetGravity();
    fn SetGravity();
    fn GetPara2();
    fn SetPara2();
    fn GetRow();
    fn GetStartPara();
    fn GetTable();
    fn GetURL();
    fn SetURL();
    fn AddSubrange();
    fn BuildUpMath();
    fn DeleteSubrange();
    fn Find();
    fn GetChar2();
    fn GetDropCap();
    fn GetInlineObject();
    fn GetProperty();
    fn GetRect();
    fn GetSubrange();
    fn GetText2();
    fn HexToUnicode();
    fn InsertTable();
    fn Linearize();
    fn SetActiveSubrange();
    fn SetDropCap();
    fn SetProperty();
    fn SetText2();
    fn UnicodeToHex();
    fn SetInlineObject();
    fn GetMathFunctionType();
    fn InsertImage();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextRange2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextRange2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextRange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRange2Impl, const OFFSET: isize>() -> ITextRange2Vtbl {
        unsafe extern "system" fn GetCch<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCch(::core::mem::transmute_copy(&pcch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCells<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcells: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCells(::core::mem::transmute_copy(&ppcells)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolumn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumn(::core::mem::transmute_copy(&ppcolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuplicate2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuplicate2(::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFont2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFont2(::core::mem::transmute_copy(&ppfont)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFont2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFont2(&*(&pfont as *const <ITextFont2 as ::windows::core::Abi>::Abi as *const <ITextFont2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormattedText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormattedText2(::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormattedText2(&*(&prange as *const <ITextRange2 as ::windows::core::Abi>::Abi as *const <ITextRange2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGravity<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGravity(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGravity(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPara2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppara: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPara2(::core::mem::transmute_copy(&pppara)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPara2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppara: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPara2(&*(&ppara as *const <ITextPara2 as ::windows::core::Abi>::Abi as *const <ITextPara2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRow<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRow(::core::mem::transmute_copy(&pprow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStartPara<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStartPara(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTable<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptable: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTable(::core::mem::transmute_copy(&pptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURL<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetURL(::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetURL(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSubrange(cp1, cp2, activate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildUpMath<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildUpMath(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpfirst: i32, cplim: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteSubrange(cpfirst, cplim) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Find<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, count: i32, flags: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Find(&*(&prange as *const <ITextRange2 as ::windows::core::Abi>::Abi as *const <ITextRange2 as ::windows::core::DefaultType>::DefaultType), count, flags, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChar2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchar: *mut i32, offset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChar2(::core::mem::transmute_copy(&pchar), offset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDropCap<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDropCap(::core::mem::transmute_copy(&pcline), ::core::mem::transmute_copy(&pposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInlineObject<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInlineObject(::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&palign), ::core::mem::transmute_copy(&pchar), ::core::mem::transmute_copy(&pchar1), ::core::mem::transmute_copy(&pchar2), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&ptexstyle), ::core::mem::transmute_copy(&pccol), ::core::mem::transmute_copy(&plevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(r#type, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRect<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRect(r#type, ::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom), ::core::mem::transmute_copy(&phit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubrange(isubrange, ::core::mem::transmute_copy(&pcpfirst), ::core::mem::transmute_copy(&pcplim)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText2(flags, ::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HexToUnicode<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HexToUnicode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertTable<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertTable(ccol, crow, autofit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Linearize<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Linearize(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveSubrange<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveSubrange(cpanchor, cpactive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDropCap<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cline: i32, position: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDropCap(cline, position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(r#type, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText2<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText2(flags, &*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeToHex<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicodeToHex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInlineObject<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInlineObject(r#type, align, char, char1, char2, count, texstyle, ccol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMathFunctionType<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMathFunctionType(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImage<Impl: ITextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: super::super::super::Graphics::Gdi::TEXT_ALIGN_OPTIONS, bstralttext: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertImage(width, height, ascent, r#type, &*(&bstralttext as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pstream as *const <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextRange2>,
            ::windows::core::GetTrustLevel,
            GetCch::<Impl, OFFSET>,
            GetCells::<Impl, OFFSET>,
            GetColumn::<Impl, OFFSET>,
            GetCount::<Impl, OFFSET>,
            GetDuplicate2::<Impl, OFFSET>,
            GetFont2::<Impl, OFFSET>,
            SetFont2::<Impl, OFFSET>,
            GetFormattedText2::<Impl, OFFSET>,
            SetFormattedText2::<Impl, OFFSET>,
            GetGravity::<Impl, OFFSET>,
            SetGravity::<Impl, OFFSET>,
            GetPara2::<Impl, OFFSET>,
            SetPara2::<Impl, OFFSET>,
            GetRow::<Impl, OFFSET>,
            GetStartPara::<Impl, OFFSET>,
            GetTable::<Impl, OFFSET>,
            GetURL::<Impl, OFFSET>,
            SetURL::<Impl, OFFSET>,
            AddSubrange::<Impl, OFFSET>,
            BuildUpMath::<Impl, OFFSET>,
            DeleteSubrange::<Impl, OFFSET>,
            Find::<Impl, OFFSET>,
            GetChar2::<Impl, OFFSET>,
            GetDropCap::<Impl, OFFSET>,
            GetInlineObject::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetRect::<Impl, OFFSET>,
            GetSubrange::<Impl, OFFSET>,
            GetText2::<Impl, OFFSET>,
            HexToUnicode::<Impl, OFFSET>,
            InsertTable::<Impl, OFFSET>,
            Linearize::<Impl, OFFSET>,
            SetActiveSubrange::<Impl, OFFSET>,
            SetDropCap::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            SetText2::<Impl, OFFSET>,
            UnicodeToHex::<Impl, OFFSET>,
            SetInlineObject::<Impl, OFFSET>,
            GetMathFunctionType::<Impl, OFFSET>,
            InsertImage::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRowImpl: Sized + IDispatchImpl {
    fn GetAlignment();
    fn SetAlignment();
    fn GetCellCount();
    fn SetCellCount();
    fn GetCellCountCache();
    fn SetCellCountCache();
    fn GetCellIndex();
    fn SetCellIndex();
    fn GetCellMargin();
    fn SetCellMargin();
    fn GetHeight();
    fn SetHeight();
    fn GetIndent();
    fn SetIndent();
    fn GetKeepTogether();
    fn SetKeepTogether();
    fn GetKeepWithNext();
    fn SetKeepWithNext();
    fn GetNestLevel();
    fn GetRTL();
    fn SetRTL();
    fn GetCellAlignment();
    fn SetCellAlignment();
    fn GetCellColorBack();
    fn SetCellColorBack();
    fn GetCellColorFore();
    fn SetCellColorFore();
    fn GetCellMergeFlags();
    fn SetCellMergeFlags();
    fn GetCellShading();
    fn SetCellShading();
    fn GetCellVerticalText();
    fn SetCellVerticalText();
    fn GetCellWidth();
    fn SetCellWidth();
    fn GetCellBorderColors();
    fn GetCellBorderWidths();
    fn SetCellBorderColors();
    fn SetCellBorderWidths();
    fn Apply();
    fn CanChange();
    fn GetProperty();
    fn Insert();
    fn IsEqual();
    fn Reset();
    fn SetProperty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextRow {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextRow";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextRowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRowImpl, const OFFSET: isize>() -> ITextRowVtbl {
        unsafe extern "system" fn GetAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlignment(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlignment(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellCount<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellCount(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellCount<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellCount(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellCountCache<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellCountCache(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellCountCache<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellCountCache(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellIndex<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellIndex(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellIndex<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellIndex(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellMargin<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellMargin(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellMargin<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellMargin(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHeight<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeight(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHeight(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndent<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndent(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndent<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIndent(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeepTogether<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeepTogether(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKeepTogether(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeepWithNext<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeepWithNext(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKeepWithNext(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNestLevel<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNestLevel(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRTL<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRTL(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRTL<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRTL(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellAlignment(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellAlignment<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellAlignment(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellColorBack<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellColorBack(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellColorBack<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellColorBack(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellColorFore<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellColorFore(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellColorFore<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellColorFore(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellMergeFlags<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellMergeFlags(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellMergeFlags<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellMergeFlags(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellShading<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellShading(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellShading<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellShading(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellVerticalText<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellVerticalText(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellVerticalText<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellVerticalText(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellWidth<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellWidth(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellWidth<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellWidth(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellBorderColors<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellBorderColors(::core::mem::transmute_copy(&pcrleft), ::core::mem::transmute_copy(&pcrtop), ::core::mem::transmute_copy(&pcrright), ::core::mem::transmute_copy(&pcrbottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCellBorderWidths<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellBorderWidths(::core::mem::transmute_copy(&pduleft), ::core::mem::transmute_copy(&pdutop), ::core::mem::transmute_copy(&pduright), ::core::mem::transmute_copy(&pdubottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellBorderColors<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellBorderColors(crleft, crtop, crright, crbottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCellBorderWidths<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCellBorderWidths(duleft, dutop, duright, dubottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Apply<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crow: i32, flags: tomConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Apply(crow, flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChange<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChange(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(r#type, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crow: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(crow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prow: ::windows::core::RawPtr, pb: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&prow as *const <ITextRow as ::windows::core::Abi>::Abi as *const <ITextRow as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ITextRowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(r#type, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextRow>,
            ::windows::core::GetTrustLevel,
            GetAlignment::<Impl, OFFSET>,
            SetAlignment::<Impl, OFFSET>,
            GetCellCount::<Impl, OFFSET>,
            SetCellCount::<Impl, OFFSET>,
            GetCellCountCache::<Impl, OFFSET>,
            SetCellCountCache::<Impl, OFFSET>,
            GetCellIndex::<Impl, OFFSET>,
            SetCellIndex::<Impl, OFFSET>,
            GetCellMargin::<Impl, OFFSET>,
            SetCellMargin::<Impl, OFFSET>,
            GetHeight::<Impl, OFFSET>,
            SetHeight::<Impl, OFFSET>,
            GetIndent::<Impl, OFFSET>,
            SetIndent::<Impl, OFFSET>,
            GetKeepTogether::<Impl, OFFSET>,
            SetKeepTogether::<Impl, OFFSET>,
            GetKeepWithNext::<Impl, OFFSET>,
            SetKeepWithNext::<Impl, OFFSET>,
            GetNestLevel::<Impl, OFFSET>,
            GetRTL::<Impl, OFFSET>,
            SetRTL::<Impl, OFFSET>,
            GetCellAlignment::<Impl, OFFSET>,
            SetCellAlignment::<Impl, OFFSET>,
            GetCellColorBack::<Impl, OFFSET>,
            SetCellColorBack::<Impl, OFFSET>,
            GetCellColorFore::<Impl, OFFSET>,
            SetCellColorFore::<Impl, OFFSET>,
            GetCellMergeFlags::<Impl, OFFSET>,
            SetCellMergeFlags::<Impl, OFFSET>,
            GetCellShading::<Impl, OFFSET>,
            SetCellShading::<Impl, OFFSET>,
            GetCellVerticalText::<Impl, OFFSET>,
            SetCellVerticalText::<Impl, OFFSET>,
            GetCellWidth::<Impl, OFFSET>,
            SetCellWidth::<Impl, OFFSET>,
            GetCellBorderColors::<Impl, OFFSET>,
            GetCellBorderWidths::<Impl, OFFSET>,
            SetCellBorderColors::<Impl, OFFSET>,
            SetCellBorderWidths::<Impl, OFFSET>,
            Apply::<Impl, OFFSET>,
            CanChange::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            Insert::<Impl, OFFSET>,
            IsEqual::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextSelectionImpl: Sized + ITextRangeImpl + IDispatchImpl {
    fn GetFlags();
    fn SetFlags();
    fn GetType();
    fn MoveLeft();
    fn MoveRight();
    fn MoveUp();
    fn MoveDown();
    fn HomeKey();
    fn EndKey();
    fn TypeText();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextSelection {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextSelection";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextSelectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelectionImpl, const OFFSET: isize>() -> ITextSelectionVtbl {
        unsafe extern "system" fn GetFlags<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFlags(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveLeft<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveLeft(unit, count, extend, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveRight<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveRight(unit, count, extend, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUp<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveUp(unit, count, extend, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveDown<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveDown(unit, count, extend, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeKey<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeKey(unit, extend, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndKey<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndKey(unit, extend, ::core::mem::transmute_copy(&pdelta)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeText<Impl: ITextSelectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TypeText(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextSelection>,
            ::windows::core::GetTrustLevel,
            GetFlags::<Impl, OFFSET>,
            SetFlags::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            MoveLeft::<Impl, OFFSET>,
            MoveRight::<Impl, OFFSET>,
            MoveUp::<Impl, OFFSET>,
            MoveDown::<Impl, OFFSET>,
            HomeKey::<Impl, OFFSET>,
            EndKey::<Impl, OFFSET>,
            TypeText::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextSelection2Impl: Sized + ITextRange2Impl + ITextSelectionImpl + ITextRangeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextSelection2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextSelection2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextSelection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextSelection2Impl, const OFFSET: isize>() -> ITextSelection2Vtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextSelection2>, ::windows::core::GetTrustLevel)
    }
}
pub trait ITextServicesImpl: Sized {
    fn TxSendMessage();
    fn TxDraw();
    fn TxGetHScroll();
    fn TxGetVScroll();
    fn OnTxSetCursor();
    fn TxQueryHitPoint();
    fn OnTxInPlaceActivate();
    fn OnTxInPlaceDeactivate();
    fn OnTxUIActivate();
    fn OnTxUIDeactivate();
    fn TxGetText();
    fn TxSetText();
    fn TxGetCurTargetX();
    fn TxGetBaseLinePos();
    fn TxGetNaturalSize();
    fn TxGetDropTarget();
    fn OnTxPropertyBitsChange();
    fn TxGetCachedSize();
}
impl ::windows::core::RuntimeName for ITextServices {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextServices";
}
impl ITextServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextServicesImpl, const OFFSET: isize>() -> ITextServicesVtbl {
        unsafe extern "system" fn TxSendMessage<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxSendMessage(
                msg,
                &*(&wparam as *const <super::super::super::Foundation::WPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::WPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&lparam as *const <super::super::super::Foundation::LPARAM as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LPARAM as ::windows::core::DefaultType>::DefaultType),
                &*(&plresult as *const <super::super::super::Foundation::LRESULT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::LRESULT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxDraw<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxDraw(
                dwdrawaspect,
                lindex,
                &*(&pvaspect as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&ptd as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                &*(&hdcdraw as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&hictargetdev as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcbounds as *const <super::super::super::Foundation::RECTL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECTL as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcwbounds as *const <super::super::super::Foundation::RECTL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECTL as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcupdate as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                pfncontinue,
                dwcontinue,
                lviewid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetHScroll<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetHScroll(plmin, plmax, plpos, plpage, &*(&pfenabled as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetVScroll<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetVScroll(plmin, plmax, plpos, plpage, &*(&pfenabled as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxSetCursor<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTxSetCursor(
                dwdrawaspect,
                lindex,
                &*(&pvaspect as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&ptd as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                &*(&hdcdraw as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&hictargetdev as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcclient as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                x,
                y,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxQueryHitPoint<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxQueryHitPoint(
                dwdrawaspect,
                lindex,
                &*(&pvaspect as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&ptd as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                &*(&hdcdraw as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&hictargetdev as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcclient as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                x,
                y,
                phitresult,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxInPlaceActivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTxInPlaceActivate(&*(&prcclient as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxInPlaceDeactivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTxInPlaceDeactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxUIActivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTxUIActivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxUIDeactivate<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTxUIDeactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetText<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetText(&*(&pbstrtext as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxSetText<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxSetText(&*(&psztext as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetCurTargetX<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetCurTargetX(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetBaseLinePos<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetBaseLinePos(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetNaturalSize<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetNaturalSize(
                dwaspect,
                &*(&hdcdraw as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&hictargetdev as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&ptd as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                dwmode,
                &*(&psizelextent as *const <super::super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType),
                pwidth,
                pheight,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetDropTarget<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetDropTarget(::core::mem::transmute_copy(&ppdroptarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTxPropertyBitsChange<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmask: u32, dwbits: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTxPropertyBitsChange(dwmask, dwbits) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxGetCachedSize<Impl: ITextServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetCachedSize(pdwwidth, pdwheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextServices>,
            ::windows::core::GetTrustLevel,
            TxSendMessage::<Impl, OFFSET>,
            TxDraw::<Impl, OFFSET>,
            TxGetHScroll::<Impl, OFFSET>,
            TxGetVScroll::<Impl, OFFSET>,
            OnTxSetCursor::<Impl, OFFSET>,
            TxQueryHitPoint::<Impl, OFFSET>,
            OnTxInPlaceActivate::<Impl, OFFSET>,
            OnTxInPlaceDeactivate::<Impl, OFFSET>,
            OnTxUIActivate::<Impl, OFFSET>,
            OnTxUIDeactivate::<Impl, OFFSET>,
            TxGetText::<Impl, OFFSET>,
            TxSetText::<Impl, OFFSET>,
            TxGetCurTargetX::<Impl, OFFSET>,
            TxGetBaseLinePos::<Impl, OFFSET>,
            TxGetNaturalSize::<Impl, OFFSET>,
            TxGetDropTarget::<Impl, OFFSET>,
            OnTxPropertyBitsChange::<Impl, OFFSET>,
            TxGetCachedSize::<Impl, OFFSET>,
        )
    }
}
pub trait ITextServices2Impl: Sized + ITextServicesImpl {
    fn TxGetNaturalSize2();
    fn TxDrawD2D();
}
impl ::windows::core::RuntimeName for ITextServices2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextServices2";
}
impl ITextServices2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextServices2Impl, const OFFSET: isize>() -> ITextServices2Vtbl {
        unsafe extern "system" fn TxGetNaturalSize2<Impl: ITextServices2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxGetNaturalSize2(
                dwaspect,
                &*(&hdcdraw as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&hictargetdev as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType),
                &*(&ptd as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::DVTARGETDEVICE as ::windows::core::DefaultType>::DefaultType),
                dwmode,
                &*(&psizelextent as *const <super::super::super::Foundation::SIZE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::SIZE as ::windows::core::DefaultType>::DefaultType),
                pwidth,
                pheight,
                pascent,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxDrawD2D<Impl: ITextServices2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendertarget: ::windows::core::RawPtr, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxDrawD2D(
                &*(&prendertarget as *const <super::super::super::Graphics::Direct2D::ID2D1RenderTarget as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Direct2D::ID2D1RenderTarget as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcbounds as *const <super::super::super::Foundation::RECTL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECTL as ::windows::core::DefaultType>::DefaultType),
                &*(&lprcupdate as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                lviewid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextServices2>, ::windows::core::GetTrustLevel, TxGetNaturalSize2::<Impl, OFFSET>, TxDrawD2D::<Impl, OFFSET>)
    }
}
pub trait ITextStoryImpl: Sized {
    fn GetActive();
    fn SetActive();
    fn GetDisplay();
    fn GetIndex();
    fn GetType();
    fn SetType();
    fn GetProperty();
    fn GetRange();
    fn GetText();
    fn SetFormattedText();
    fn SetProperty();
    fn SetText();
}
impl ::windows::core::RuntimeName for ITextStory {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextStory";
}
impl ITextStoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryImpl, const OFFSET: isize>() -> ITextStoryVtbl {
        unsafe extern "system" fn GetActive<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActive(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActive<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActive(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplay<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplay: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplay(::core::mem::transmute_copy(&ppdisplay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndex(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetType(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(r#type, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRange(cpactive, cpanchor, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(flags, ::core::mem::transmute_copy(&pbstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormattedText(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(r#type, value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITextStoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetText(flags, &*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextStory>,
            ::windows::core::GetTrustLevel,
            GetActive::<Impl, OFFSET>,
            SetActive::<Impl, OFFSET>,
            GetDisplay::<Impl, OFFSET>,
            GetIndex::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            SetType::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetRange::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            SetFormattedText::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            SetText::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoryRangesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn GetCount();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextStoryRanges {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextStoryRanges";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRangesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRangesImpl, const OFFSET: isize>() -> ITextStoryRangesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunkenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITextStoryRangesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextStoryRanges>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, GetCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoryRanges2Impl: Sized + ITextStoryRangesImpl + IDispatchImpl {
    fn Item2();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextStoryRanges2 {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextStoryRanges2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRanges2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStoryRanges2Impl, const OFFSET: isize>() -> ITextStoryRanges2Vtbl {
        unsafe extern "system" fn Item2<Impl: ITextStoryRanges2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item2(index, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextStoryRanges2>, ::windows::core::GetTrustLevel, Item2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStringsImpl: Sized + IDispatchImpl {
    fn Item();
    fn GetCount();
    fn Add();
    fn Append();
    fn Cat2();
    fn CatTop2();
    fn DeleteRange();
    fn EncodeFunction();
    fn GetCch();
    fn InsertNullStr();
    fn MoveBoundary();
    fn PrefixTop();
    fn Remove();
    fn SetFormattedText();
    fn SetOpCp();
    fn SuffixTop();
    fn Swap();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITextStrings {
    const NAME: &'static str = "Windows.Win32.UI.Controls.RichEdit.ITextStrings";
}
#[cfg(feature = "Win32_System_Com")]
impl ITextStringsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextStringsImpl, const OFFSET: isize>() -> ITextStringsVtbl {
        unsafe extern "system" fn Item<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pprange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Append(&*(&prange as *const <ITextRange2 as ::windows::core::Abi>::Abi as *const <ITextRange2 as ::windows::core::DefaultType>::DefaultType), istring) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cat2<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cat2(istring) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CatTop2<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CatTop2(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRange<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRange(&*(&prange as *const <ITextRange2 as ::windows::core::Abi>::Abi as *const <ITextRange2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodeFunction<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodeFunction(r#type, align, char, char1, char2, count, texstyle, ccol, &*(&prange as *const <ITextRange2 as ::windows::core::Abi>::Abi as *const <ITextRange2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCch<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, pcch: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCch(istring, ::core::mem::transmute_copy(&pcch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertNullStr<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertNullStr(istring) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveBoundary<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cch: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveBoundary(istring, cch) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefixTop<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefixTop(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cstring: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(istring, cstring) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pranged: ::windows::core::RawPtr, pranges: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormattedText(&*(&pranged as *const <ITextRange2 as ::windows::core::Abi>::Abi as *const <ITextRange2 as ::windows::core::DefaultType>::DefaultType), &*(&pranges as *const <ITextRange2 as ::windows::core::Abi>::Abi as *const <ITextRange2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpCp<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istring: i32, cp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOpCp(istring, cp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuffixTop<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstr: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuffixTop(&*(&bstr as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&prange as *const <ITextRange2 as ::windows::core::Abi>::Abi as *const <ITextRange2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Swap<Impl: ITextStringsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Swap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextStrings>,
            ::windows::core::GetTrustLevel,
            Item::<Impl, OFFSET>,
            GetCount::<Impl, OFFSET>,
            Add::<Impl, OFFSET>,
            Append::<Impl, OFFSET>,
            Cat2::<Impl, OFFSET>,
            CatTop2::<Impl, OFFSET>,
            DeleteRange::<Impl, OFFSET>,
            EncodeFunction::<Impl, OFFSET>,
            GetCch::<Impl, OFFSET>,
            InsertNullStr::<Impl, OFFSET>,
            MoveBoundary::<Impl, OFFSET>,
            PrefixTop::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
            SetFormattedText::<Impl, OFFSET>,
            SetOpCp::<Impl, OFFSET>,
            SuffixTop::<Impl, OFFSET>,
            Swap::<Impl, OFFSET>,
        )
    }
}
