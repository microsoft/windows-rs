pub trait IObjectArrayImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl ::windows::core::RuntimeName for IObjectArray {
    const NAME: &'static str = "Windows.Win32.UI.Shell.Common.IObjectArray";
}
impl IObjectArrayVtbl {
    pub const fn new<Impl: IObjectArrayImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectArrayVtbl {
        unsafe extern "system" fn GetCount<Impl: IObjectArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IObjectArrayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAt(uiindex, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectArray>, base.5, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>)
    }
}
pub trait IObjectCollectionImpl: Sized + IObjectArrayImpl {
    fn AddObject();
    fn AddFromArray();
    fn RemoveObjectAt();
    fn Clear();
}
impl ::windows::core::RuntimeName for IObjectCollection {
    const NAME: &'static str = "Windows.Win32.UI.Shell.Common.IObjectCollection";
}
impl IObjectCollectionVtbl {
    pub const fn new<Impl: IObjectCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IObjectCollectionVtbl {
        unsafe extern "system" fn AddObject<Impl: IObjectCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddObject(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFromArray<Impl: IObjectCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddFromArray(&*(&poasource as *const <IObjectArray as ::windows::core::Abi>::Abi as *const <IObjectArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveObjectAt<Impl: IObjectCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveObjectAt(uiindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IObjectCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IObjectCollection>, base.5, AddObject::<Impl, OFFSET>, AddFromArray::<Impl, OFFSET>, RemoveObjectAt::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
