pub trait IUIApplication_Impl: Sized {
    fn OnViewChanged(&mut self, viewid: u32, typeid: UI_VIEWTYPE, view: &::core::option::Option<::windows::core::IUnknown>, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::Result<()>;
    fn OnCreateUICommand(&mut self, commandid: u32, typeid: UI_COMMANDTYPE) -> ::windows::core::Result<IUICommandHandler>;
    fn OnDestroyUICommand(&mut self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: &::core::option::Option<IUICommandHandler>) -> ::windows::core::Result<()>;
}
impl IUIApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIApplication_Impl, const OFFSET: isize>() -> IUIApplication_Vtbl {
        unsafe extern "system" fn OnViewChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut ::core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnViewChanged(::core::mem::transmute_copy(&viewid), ::core::mem::transmute_copy(&typeid), ::core::mem::transmute(&view), ::core::mem::transmute_copy(&verb), ::core::mem::transmute_copy(&ureasoncode)).into()
        }
        unsafe extern "system" fn OnCreateUICommand<Identity: ::windows::core::IUnknownImpl, Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OnCreateUICommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&typeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *commandhandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDestroyUICommand<Identity: ::windows::core::IUnknownImpl, Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDestroyUICommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&typeid), ::core::mem::transmute(&commandhandler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnViewChanged: OnViewChanged::<Identity, Impl, OFFSET>,
            OnCreateUICommand: OnCreateUICommand::<Identity, Impl, OFFSET>,
            OnDestroyUICommand: OnDestroyUICommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIApplication as ::windows::core::Interface>::IID
    }
}
pub trait IUICollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetItem(&mut self, index: u32) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, item: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Insert(&mut self, index: u32, item: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn Replace(&mut self, indexreplaced: u32, itemreplacewith: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
impl IUICollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const OFFSET: isize>() -> IUICollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn Insert<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Insert(::core::mem::transmute_copy(&index), ::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Replace<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Replace(::core::mem::transmute_copy(&indexreplaced), ::core::mem::transmute(&itemreplacewith)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            Replace: Replace::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUICollection as ::windows::core::Interface>::IID
    }
}
pub trait IUICollectionChangedEvent_Impl: Sized {
    fn OnChanged(&mut self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: &::core::option::Option<::windows::core::IUnknown>, newindex: u32, newitem: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IUICollectionChangedEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICollectionChangedEvent_Impl, const OFFSET: isize>() -> IUICollectionChangedEvent_Vtbl {
        unsafe extern "system" fn OnChanged<Identity: ::windows::core::IUnknownImpl, Impl: IUICollectionChangedEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut ::core::ffi::c_void, newindex: u32, newitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChanged(::core::mem::transmute_copy(&action), ::core::mem::transmute_copy(&oldindex), ::core::mem::transmute(&olditem), ::core::mem::transmute_copy(&newindex), ::core::mem::transmute(&newitem)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnChanged: OnChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUICollectionChangedEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUICommandHandler_Impl: Sized {
    fn Execute(&mut self, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: &::core::option::Option<IUISimplePropertySet>) -> ::windows::core::Result<()>;
    fn UpdateProperty(&mut self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUICommandHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICommandHandler_Impl, const OFFSET: isize>() -> IUICommandHandler_Vtbl {
        unsafe extern "system" fn Execute<Identity: ::windows::core::IUnknownImpl, Impl: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Execute(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&verb), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&currentvalue), ::core::mem::transmute(&commandexecutionproperties)).into()
        }
        unsafe extern "system" fn UpdateProperty<Identity: ::windows::core::IUnknownImpl, Impl: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, newvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UpdateProperty(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&currentvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *newvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Execute: Execute::<Identity, Impl, OFFSET>,
            UpdateProperty: UpdateProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUICommandHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIContextualUI_Impl: Sized {
    fn ShowAtLocation(&mut self, x: i32, y: i32) -> ::windows::core::Result<()>;
}
impl IUIContextualUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIContextualUI_Impl, const OFFSET: isize>() -> IUIContextualUI_Vtbl {
        unsafe extern "system" fn ShowAtLocation<Identity: ::windows::core::IUnknownImpl, Impl: IUIContextualUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowAtLocation(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ShowAtLocation: ShowAtLocation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIContextualUI as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIEventLogger_Impl: Sized {
    fn OnUIEvent(&mut self, peventparams: *const UI_EVENTPARAMS);
}
#[cfg(feature = "Win32_Foundation")]
impl IUIEventLogger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventLogger_Impl, const OFFSET: isize>() -> IUIEventLogger_Vtbl {
        unsafe extern "system" fn OnUIEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventparams: *const UI_EVENTPARAMS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnUIEvent(::core::mem::transmute_copy(&peventparams))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnUIEvent: OnUIEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIEventLogger as ::windows::core::Interface>::IID
    }
}
pub trait IUIEventingManager_Impl: Sized {
    fn SetEventLogger(&mut self, eventlogger: &::core::option::Option<IUIEventLogger>) -> ::windows::core::Result<()>;
}
impl IUIEventingManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventingManager_Impl, const OFFSET: isize>() -> IUIEventingManager_Vtbl {
        unsafe extern "system" fn SetEventLogger<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventlogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventLogger(::core::mem::transmute(&eventlogger)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetEventLogger: SetEventLogger::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIEventingManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUIFramework_Impl: Sized {
    fn Initialize(&mut self, framewnd: super::super::Foundation::HWND, application: &::core::option::Option<IUIApplication>) -> ::windows::core::Result<()>;
    fn Destroy(&mut self) -> ::windows::core::Result<()>;
    fn LoadUI(&mut self, instance: super::super::Foundation::HINSTANCE, resourcename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetView(&mut self, viewid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetUICommandProperty(&mut self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetUICommandProperty(&mut self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn InvalidateUICommand(&mut self, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn FlushPendingInvalidations(&mut self) -> ::windows::core::Result<()>;
    fn SetModes(&mut self, imodes: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUIFramework_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>() -> IUIFramework_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&framewnd), ::core::mem::transmute(&application)).into()
        }
        unsafe extern "system" fn Destroy<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn LoadUI<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadUI(::core::mem::transmute_copy(&instance), ::core::mem::transmute_copy(&resourcename)).into()
        }
        unsafe extern "system" fn GetView<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetView(::core::mem::transmute_copy(&viewid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetUICommandProperty<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUICommandProperty(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUICommandProperty<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUICommandProperty(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InvalidateUICommand<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvalidateUICommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn FlushPendingInvalidations<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlushPendingInvalidations().into()
        }
        unsafe extern "system" fn SetModes<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imodes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetModes(::core::mem::transmute_copy(&imodes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            LoadUI: LoadUI::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
            GetUICommandProperty: GetUICommandProperty::<Identity, Impl, OFFSET>,
            SetUICommandProperty: SetUICommandProperty::<Identity, Impl, OFFSET>,
            InvalidateUICommand: InvalidateUICommand::<Identity, Impl, OFFSET>,
            FlushPendingInvalidations: FlushPendingInvalidations::<Identity, Impl, OFFSET>,
            SetModes: SetModes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIFramework as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImage_Impl: Sized {
    fn GetBitmap(&mut self) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIImage_Impl, const OFFSET: isize>() -> IUIImage_Vtbl {
        unsafe extern "system" fn GetBitmap<Identity: ::windows::core::IUnknownImpl, Impl: IUIImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *bitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetBitmap: GetBitmap::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIImage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImageFromBitmap_Impl: Sized {
    fn CreateImage(&mut self, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP) -> ::windows::core::Result<IUIImage>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImageFromBitmap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIImageFromBitmap_Impl, const OFFSET: isize>() -> IUIImageFromBitmap_Vtbl {
        unsafe extern "system" fn CreateImage<Identity: ::windows::core::IUnknownImpl, Impl: IUIImageFromBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateImage(::core::mem::transmute_copy(&bitmap), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *image = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateImage: CreateImage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIImageFromBitmap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIRibbon_Impl: Sized {
    fn GetHeight(&mut self) -> ::windows::core::Result<u32>;
    fn LoadSettingsFromStream(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn SaveSettingsToStream(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIRibbon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIRibbon_Impl, const OFFSET: isize>() -> IUIRibbon_Vtbl {
        unsafe extern "system" fn GetHeight<Identity: ::windows::core::IUnknownImpl, Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *cy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSettingsFromStream<Identity: ::windows::core::IUnknownImpl, Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadSettingsFromStream(::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn SaveSettingsToStream<Identity: ::windows::core::IUnknownImpl, Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveSettingsToStream(::core::mem::transmute(&pstream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetHeight: GetHeight::<Identity, Impl, OFFSET>,
            LoadSettingsFromStream: LoadSettingsFromStream::<Identity, Impl, OFFSET>,
            SaveSettingsToStream: SaveSettingsToStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIRibbon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUISimplePropertySet_Impl: Sized {
    fn GetValue(&mut self, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUISimplePropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISimplePropertySet_Impl, const OFFSET: isize>() -> IUISimplePropertySet_Vtbl {
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IUISimplePropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISimplePropertySet as ::windows::core::Interface>::IID
    }
}
