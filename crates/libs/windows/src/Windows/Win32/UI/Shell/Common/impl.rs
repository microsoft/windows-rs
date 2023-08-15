#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`, `\"implement\"`*"]
pub trait IObjectArray_Impl: Sized {
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn GetAt(&self, uiindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IObjectArray {}
impl IObjectArray_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectArray_Impl, const OFFSET: isize>() -> IObjectArray_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcobjects, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAt(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectArray as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_Shell_Common\"`, `\"implement\"`*"]
pub trait IObjectCollection_Impl: Sized + IObjectArray_Impl {
    fn AddObject(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn AddFromArray(&self, poasource: ::core::option::Option<&IObjectArray>) -> ::windows_core::Result<()>;
    fn RemoveObjectAt(&self, uiindex: u32) -> ::windows_core::Result<()>;
    fn Clear(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IObjectCollection {}
impl IObjectCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: isize>() -> IObjectCollection_Vtbl {
        unsafe extern "system" fn AddObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddObject(::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn AddFromArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poasource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFromArray(::windows_core::from_raw_borrowed(&poasource)).into()
        }
        unsafe extern "system" fn RemoveObjectAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveObjectAt(::core::mem::transmute_copy(&uiindex)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: IObjectArray_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddObject: AddObject::<Identity, Impl, OFFSET>,
            AddFromArray: AddFromArray::<Identity, Impl, OFFSET>,
            RemoveObjectAt: RemoveObjectAt::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectCollection as ::windows_core::ComInterface>::IID || iid == &<IObjectArray as ::windows_core::ComInterface>::IID
    }
}
