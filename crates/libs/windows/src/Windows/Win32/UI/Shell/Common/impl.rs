pub trait IObjectArrayImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IObjectArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectArrayVtbl {
        unsafe extern "system" fn GetCount<Impl: IObjectArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcobjects = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IObjectArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCount: GetCount::<Impl, IMPL_OFFSET>, GetAt: GetAt::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectArray as ::windows::core::Interface>::IID
    }
}
pub trait IObjectCollectionImpl: Sized + IObjectArrayImpl {
    fn AddObject(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn AddFromArray(&mut self, poasource: ::core::option::Option<IObjectArray>) -> ::windows::core::Result<()>;
    fn RemoveObjectAt(&mut self, uiindex: u32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
impl IObjectCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectCollectionVtbl {
        unsafe extern "system" fn AddObject<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddObject(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn AddFromArray<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFromArray(::core::mem::transmute(&poasource)).into()
        }
        unsafe extern "system" fn RemoveObjectAt<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveObjectAt(::core::mem::transmute_copy(&uiindex)).into()
        }
        unsafe extern "system" fn Clear<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IObjectArrayVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
