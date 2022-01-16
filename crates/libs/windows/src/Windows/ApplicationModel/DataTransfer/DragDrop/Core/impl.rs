#[cfg(feature = "Foundation")]
pub trait ICoreDropOperationTarget_Impl: Sized {
    fn EnterAsync(&mut self, draginfo: &::core::option::Option<CoreDragInfo>, draguioverride: &::core::option::Option<CoreDragUIOverride>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn OverAsync(&mut self, draginfo: &::core::option::Option<CoreDragInfo>, draguioverride: &::core::option::Option<CoreDragUIOverride>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn LeaveAsync(&mut self, draginfo: &::core::option::Option<CoreDragInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn DropAsync(&mut self, draginfo: &::core::option::Option<CoreDragInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICoreDropOperationTarget {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTarget";
}
#[cfg(feature = "Foundation")]
impl ICoreDropOperationTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDropOperationTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreDropOperationTarget_Vtbl {
        unsafe extern "system" fn EnterAsync<Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows::core::RawPtr, draguioverride: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterAsync(&*(&draginfo as *const <CoreDragInfo as ::windows::core::Abi>::Abi as *const <CoreDragInfo as ::windows::core::DefaultType>::DefaultType), &*(&draguioverride as *const <CoreDragUIOverride as ::windows::core::Abi>::Abi as *const <CoreDragUIOverride as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverAsync<Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows::core::RawPtr, draguioverride: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverAsync(&*(&draginfo as *const <CoreDragInfo as ::windows::core::Abi>::Abi as *const <CoreDragInfo as ::windows::core::DefaultType>::DefaultType), &*(&draguioverride as *const <CoreDragUIOverride as ::windows::core::Abi>::Abi as *const <CoreDragUIOverride as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaveAsync<Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeaveAsync(&*(&draginfo as *const <CoreDragInfo as ::windows::core::Abi>::Abi as *const <CoreDragInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropAsync<Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropAsync(&*(&draginfo as *const <CoreDragInfo as ::windows::core::Abi>::Abi as *const <CoreDragInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreDropOperationTarget, BASE_OFFSET>(),
            EnterAsync: EnterAsync::<Impl, IMPL_OFFSET>,
            OverAsync: OverAsync::<Impl, IMPL_OFFSET>,
            LeaveAsync: LeaveAsync::<Impl, IMPL_OFFSET>,
            DropAsync: DropAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreDropOperationTarget as ::windows::core::Interface>::IID
    }
}
