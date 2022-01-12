pub trait IObjectArrayImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl IObjectArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectArrayVtbl {
        unsafe extern "system" fn GetCount<Impl: IObjectArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IObjectArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCount: GetCount::<Impl, IMPL_OFFSET>, GetAt: GetAt::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectArray as ::windows::core::Interface>::IID
    }
}
pub trait IObjectCollectionImpl: Sized + IObjectArrayImpl {
    fn AddObject();
    fn AddFromArray();
    fn RemoveObjectAt();
    fn Clear();
}
impl IObjectCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectCollectionVtbl {
        unsafe extern "system" fn AddObject<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFromArray<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveObjectAt<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
