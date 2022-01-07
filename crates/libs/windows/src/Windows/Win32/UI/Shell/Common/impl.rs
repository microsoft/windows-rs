pub trait IObjectArrayImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl ::windows::core::RuntimeName for IObjectArray {
    const NAME: &'static str = "Windows.Win32.UI.Shell.Common.IObjectArray";
}
impl IObjectArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectArrayImpl, const OFFSET: isize>() -> IObjectArrayVtbl {
        unsafe extern "system" fn GetCount<Impl: IObjectArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IObjectArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(uiindex, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectArray>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectCollectionImpl, const OFFSET: isize>() -> IObjectCollectionVtbl {
        unsafe extern "system" fn AddObject<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddObject(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFromArray<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFromArray(&*(&poasource as *const <IObjectArray as ::windows::core::Abi>::Abi as *const <IObjectArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveObjectAt<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveObjectAt(uiindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectCollection>, ::windows::core::GetTrustLevel, AddObject::<Impl, OFFSET>, AddFromArray::<Impl, OFFSET>, RemoveObjectAt::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
