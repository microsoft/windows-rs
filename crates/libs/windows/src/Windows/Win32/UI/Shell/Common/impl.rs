pub trait IObjectArray_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IObjectArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectArray_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectArray_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IObjectArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcobjects = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IObjectArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCount: GetCount::<Impl, IMPL_OFFSET>, GetAt: GetAt::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectArray as ::windows::core::Interface>::IID
    }
}
pub trait IObjectCollection_Impl: Sized + IObjectArray_Impl {
    fn AddObject(&mut self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn AddFromArray(&mut self, poasource: &::core::option::Option<IObjectArray>) -> ::windows::core::Result<()>;
    fn RemoveObjectAt(&mut self, uiindex: u32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
impl IObjectCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectCollection_Vtbl {
        unsafe extern "system" fn AddObject<Impl: IObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddObject(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn AddFromArray<Impl: IObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFromArray(::core::mem::transmute(&poasource)).into()
        }
        unsafe extern "system" fn RemoveObjectAt<Impl: IObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveObjectAt(::core::mem::transmute_copy(&uiindex)).into()
        }
        unsafe extern "system" fn Clear<Impl: IObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IObjectArray_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddObject: AddObject::<Impl, IMPL_OFFSET>,
            AddFromArray: AddFromArray::<Impl, IMPL_OFFSET>,
            RemoveObjectAt: RemoveObjectAt::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectCollection as ::windows::core::Interface>::IID
    }
}
