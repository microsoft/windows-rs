pub trait IUIApplication_Impl: Sized {
    fn OnViewChanged(&mut self, viewid: u32, typeid: UI_VIEWTYPE, view: &::core::option::Option<::windows::core::IUnknown>, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::Result<()>;
    fn OnCreateUICommand(&mut self, commandid: u32, typeid: UI_COMMANDTYPE) -> ::windows::core::Result<IUICommandHandler>;
    fn OnDestroyUICommand(&mut self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: &::core::option::Option<IUICommandHandler>) -> ::windows::core::Result<()>;
}
impl IUIApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIApplication_Vtbl {
        unsafe extern "system" fn OnViewChanged<Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut ::core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnViewChanged(::core::mem::transmute_copy(&viewid), ::core::mem::transmute_copy(&typeid), ::core::mem::transmute(&view), ::core::mem::transmute_copy(&verb), ::core::mem::transmute_copy(&ureasoncode)).into()
        }
        unsafe extern "system" fn OnCreateUICommand<Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCreateUICommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&typeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *commandhandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDestroyUICommand<Impl: IUIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDestroyUICommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&typeid), ::core::mem::transmute(&commandhandler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnViewChanged: OnViewChanged::<Impl, IMPL_OFFSET>,
            OnCreateUICommand: OnCreateUICommand::<Impl, IMPL_OFFSET>,
            OnDestroyUICommand: OnDestroyUICommand::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUICollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn Insert<Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Insert(::core::mem::transmute_copy(&index), ::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Replace<Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Replace(::core::mem::transmute_copy(&indexreplaced), ::core::mem::transmute(&itemreplacewith)).into()
        }
        unsafe extern "system" fn Clear<Impl: IUICollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetItem: GetItem::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Insert: Insert::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
            Replace: Replace::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICollectionChangedEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUICollectionChangedEvent_Vtbl {
        unsafe extern "system" fn OnChanged<Impl: IUICollectionChangedEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut ::core::ffi::c_void, newindex: u32, newitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChanged(::core::mem::transmute_copy(&action), ::core::mem::transmute_copy(&oldindex), ::core::mem::transmute(&olditem), ::core::mem::transmute_copy(&newindex), ::core::mem::transmute(&newitem)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnChanged: OnChanged::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICommandHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUICommandHandler_Vtbl {
        unsafe extern "system" fn Execute<Impl: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Execute(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&verb), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&currentvalue), ::core::mem::transmute(&commandexecutionproperties)).into()
        }
        unsafe extern "system" fn UpdateProperty<Impl: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, newvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateProperty(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&currentvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *newvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Execute: Execute::<Impl, IMPL_OFFSET>,
            UpdateProperty: UpdateProperty::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIContextualUI_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIContextualUI_Vtbl {
        unsafe extern "system" fn ShowAtLocation<Impl: IUIContextualUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowAtLocation(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ShowAtLocation: ShowAtLocation::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventLogger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIEventLogger_Vtbl {
        unsafe extern "system" fn OnUIEvent<Impl: IUIEventLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventparams: *const UI_EVENTPARAMS) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUIEvent(::core::mem::transmute_copy(&peventparams))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUIEvent: OnUIEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIEventLogger as ::windows::core::Interface>::IID
    }
}
pub trait IUIEventingManager_Impl: Sized {
    fn SetEventLogger(&mut self, eventlogger: &::core::option::Option<IUIEventLogger>) -> ::windows::core::Result<()>;
}
impl IUIEventingManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIEventingManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIEventingManager_Vtbl {
        unsafe extern "system" fn SetEventLogger<Impl: IUIEventingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventlogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventLogger(::core::mem::transmute(&eventlogger)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetEventLogger: SetEventLogger::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIFramework_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIFramework_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&framewnd), ::core::mem::transmute(&application)).into()
        }
        unsafe extern "system" fn Destroy<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Destroy().into()
        }
        unsafe extern "system" fn LoadUI<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadUI(::core::mem::transmute_copy(&instance), ::core::mem::transmute_copy(&resourcename)).into()
        }
        unsafe extern "system" fn GetView<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetView(::core::mem::transmute_copy(&viewid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetUICommandProperty<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUICommandProperty(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUICommandProperty<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUICommandProperty(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InvalidateUICommand<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateUICommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn FlushPendingInvalidations<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushPendingInvalidations().into()
        }
        unsafe extern "system" fn SetModes<Impl: IUIFramework_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imodes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModes(::core::mem::transmute_copy(&imodes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Destroy: Destroy::<Impl, IMPL_OFFSET>,
            LoadUI: LoadUI::<Impl, IMPL_OFFSET>,
            GetView: GetView::<Impl, IMPL_OFFSET>,
            GetUICommandProperty: GetUICommandProperty::<Impl, IMPL_OFFSET>,
            SetUICommandProperty: SetUICommandProperty::<Impl, IMPL_OFFSET>,
            InvalidateUICommand: InvalidateUICommand::<Impl, IMPL_OFFSET>,
            FlushPendingInvalidations: FlushPendingInvalidations::<Impl, IMPL_OFFSET>,
            SetModes: SetModes::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIImage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIImage_Vtbl {
        unsafe extern "system" fn GetBitmap<Impl: IUIImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *bitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetBitmap: GetBitmap::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIImageFromBitmap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIImageFromBitmap_Vtbl {
        unsafe extern "system" fn CreateImage<Impl: IUIImageFromBitmap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateImage(::core::mem::transmute_copy(&bitmap), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *image = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateImage: CreateImage::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIRibbon_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIRibbon_Vtbl {
        unsafe extern "system" fn GetHeight<Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *cy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSettingsFromStream<Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadSettingsFromStream(::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn SaveSettingsToStream<Impl: IUIRibbon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveSettingsToStream(::core::mem::transmute(&pstream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetHeight: GetHeight::<Impl, IMPL_OFFSET>,
            LoadSettingsFromStream: LoadSettingsFromStream::<Impl, IMPL_OFFSET>,
            SaveSettingsToStream: SaveSettingsToStream::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISimplePropertySet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISimplePropertySet_Vtbl {
        unsafe extern "system" fn GetValue<Impl: IUISimplePropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetValue: GetValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISimplePropertySet as ::windows::core::Interface>::IID
    }
}
