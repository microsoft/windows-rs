#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragDropManagerImpl: Sized {
    fn TargetRequested(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetRequested(&self, value: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AreConcurrentOperationsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDragDropManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManager";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDragDropManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDragDropManagerImpl, const OFFSET: isize>() -> ICoreDragDropManagerVtbl {
        unsafe extern "system" fn TargetRequested<Impl: ICoreDragDropManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetRequested(&*(&value as *const <super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTargetRequested<Impl: ICoreDragDropManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTargetRequested(&*(&value as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AreConcurrentOperationsEnabled<Impl: ICoreDragDropManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreConcurrentOperationsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAreConcurrentOperationsEnabled<Impl: ICoreDragDropManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreConcurrentOperationsEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreDragDropManager>, ::windows::core::GetTrustLevel, TargetRequested::<Impl, OFFSET>, RemoveTargetRequested::<Impl, OFFSET>, AreConcurrentOperationsEnabled::<Impl, OFFSET>, SetAreConcurrentOperationsEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragDropManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreDragDropManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDragDropManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragDropManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDragDropManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDragDropManagerStaticsImpl, const OFFSET: isize>() -> ICoreDragDropManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ICoreDragDropManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreDragDropManagerStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragInfoImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::DataPackageView>;
    fn Modifiers(&self) -> ::windows::core::Result<super::DragDropModifiers>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDragInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDragInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDragInfoImpl, const OFFSET: isize>() -> ICoreDragInfoVtbl {
        unsafe extern "system" fn Data<Impl: ICoreDragInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modifiers<Impl: ICoreDragInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DragDropModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: ICoreDragInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreDragInfo>, ::windows::core::GetTrustLevel, Data::<Impl, OFFSET>, Modifiers::<Impl, OFFSET>, Position::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragInfo2Impl: Sized + ICoreDragInfoImpl {
    fn AllowedOperations(&self) -> ::windows::core::Result<super::super::DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDragInfo2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDragInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDragInfo2Impl, const OFFSET: isize>() -> ICoreDragInfo2Vtbl {
        unsafe extern "system" fn AllowedOperations<Impl: ICoreDragInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreDragInfo2>, ::windows::core::GetTrustLevel, AllowedOperations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragOperationImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::DataPackage>;
    fn SetPointerId(&self, pointerid: u32) -> ::windows::core::Result<()>;
    fn SetDragUIContentFromSoftwareBitmap(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetDragUIContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn DragUIContentMode(&self) -> ::windows::core::Result<CoreDragUIContentMode>;
    fn SetDragUIContentMode(&self, value: CoreDragUIContentMode) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDragOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDragOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDragOperationImpl, const OFFSET: isize>() -> ICoreDragOperationVtbl {
        unsafe extern "system" fn Data<Impl: ICoreDragOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerId<Impl: ICoreDragOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerId(pointerid).into()
        }
        unsafe extern "system" fn SetDragUIContentFromSoftwareBitmap<Impl: ICoreDragOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragUIContentFromSoftwareBitmap(&*(&softwarebitmap as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDragUIContentFromSoftwareBitmapWithAnchorPoint<Impl: ICoreDragOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetDragUIContentFromSoftwareBitmapWithAnchorPoint(&*(&softwarebitmap as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn DragUIContentMode<Impl: ICoreDragOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreDragUIContentMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragUIContentMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragUIContentMode<Impl: ICoreDragOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreDragUIContentMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragUIContentMode(value).into()
        }
        unsafe extern "system" fn StartAsync<Impl: ICoreDragOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
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
            ::windows::core::GetRuntimeClassName::<ICoreDragOperation>,
            ::windows::core::GetTrustLevel,
            Data::<Impl, OFFSET>,
            SetPointerId::<Impl, OFFSET>,
            SetDragUIContentFromSoftwareBitmap::<Impl, OFFSET>,
            SetDragUIContentFromSoftwareBitmapWithAnchorPoint::<Impl, OFFSET>,
            DragUIContentMode::<Impl, OFFSET>,
            SetDragUIContentMode::<Impl, OFFSET>,
            StartAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragOperation2Impl: Sized + ICoreDragOperationImpl {
    fn AllowedOperations(&self) -> ::windows::core::Result<super::super::DataPackageOperation>;
    fn SetAllowedOperations(&self, value: super::super::DataPackageOperation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDragOperation2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragOperation2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDragOperation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDragOperation2Impl, const OFFSET: isize>() -> ICoreDragOperation2Vtbl {
        unsafe extern "system" fn AllowedOperations<Impl: ICoreDragOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedOperations<Impl: ICoreDragOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedOperations(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreDragOperation2>, ::windows::core::GetTrustLevel, AllowedOperations::<Impl, OFFSET>, SetAllowedOperations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDragUIOverrideImpl: Sized {
    fn SetContentFromSoftwareBitmap(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &::core::option::Option<super::super::super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsContentVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsContentVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCaptionVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCaptionVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsGlyphVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsGlyphVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDragUIOverride {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDragUIOverrideVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>() -> ICoreDragUIOverrideVtbl {
        unsafe extern "system" fn SetContentFromSoftwareBitmap<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmap(&*(&softwarebitmap as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmapWithAnchorPoint<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr, anchorpoint: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetContentFromSoftwareBitmapWithAnchorPoint(&*(&softwarebitmap as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn IsContentVisible<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsContentVisible<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsContentVisible(value).into()
        }
        unsafe extern "system" fn Caption<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Caption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaption<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaption(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsCaptionVisible<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCaptionVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCaptionVisible<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCaptionVisible(value).into()
        }
        unsafe extern "system" fn IsGlyphVisible<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGlyphVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsGlyphVisible<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsGlyphVisible(value).into()
        }
        unsafe extern "system" fn Clear<Impl: ICoreDragUIOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICoreDragUIOverride>,
            ::windows::core::GetTrustLevel,
            SetContentFromSoftwareBitmap::<Impl, OFFSET>,
            SetContentFromSoftwareBitmapWithAnchorPoint::<Impl, OFFSET>,
            IsContentVisible::<Impl, OFFSET>,
            SetIsContentVisible::<Impl, OFFSET>,
            Caption::<Impl, OFFSET>,
            SetCaption::<Impl, OFFSET>,
            IsCaptionVisible::<Impl, OFFSET>,
            SetIsCaptionVisible::<Impl, OFFSET>,
            IsGlyphVisible::<Impl, OFFSET>,
            SetIsGlyphVisible::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
        )
    }
}
pub trait ICoreDropOperationTargetImpl: Sized {
    fn EnterAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>, draguioverride: &::core::option::Option<CoreDragUIOverride>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn OverAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>, draguioverride: &::core::option::Option<CoreDragUIOverride>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn LeaveAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn DropAsync(&self, draginfo: &::core::option::Option<CoreDragInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
}
impl ::windows::core::RuntimeName for ICoreDropOperationTarget {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTarget";
}
impl ICoreDropOperationTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDropOperationTargetImpl, const OFFSET: isize>() -> ICoreDropOperationTargetVtbl {
        unsafe extern "system" fn EnterAsync<Impl: ICoreDropOperationTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows::core::RawPtr, draguioverride: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverAsync<Impl: ICoreDropOperationTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows::core::RawPtr, draguioverride: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LeaveAsync<Impl: ICoreDropOperationTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DropAsync<Impl: ICoreDropOperationTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, draginfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreDropOperationTarget>, ::windows::core::GetTrustLevel, EnterAsync::<Impl, OFFSET>, OverAsync::<Impl, OFFSET>, LeaveAsync::<Impl, OFFSET>, DropAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDropOperationTargetRequestedEventArgsImpl: Sized {
    fn SetTarget(&self, target: &::core::option::Option<ICoreDropOperationTarget>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDropOperationTargetRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTargetRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDropOperationTargetRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDropOperationTargetRequestedEventArgsImpl, const OFFSET: isize>() -> ICoreDropOperationTargetRequestedEventArgsVtbl {
        unsafe extern "system" fn SetTarget<Impl: ICoreDropOperationTargetRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&target as *const <ICoreDropOperationTarget as ::windows::core::Abi>::Abi as *const <ICoreDropOperationTarget as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreDropOperationTargetRequestedEventArgs>, ::windows::core::GetTrustLevel, SetTarget::<Impl, OFFSET>)
    }
}
