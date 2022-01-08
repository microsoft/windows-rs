pub trait IInkCommitRequestHandlerImpl: Sized {
    fn OnCommitRequested();
}
impl ::windows::core::RuntimeName for IInkCommitRequestHandler {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ink.IInkCommitRequestHandler";
}
impl IInkCommitRequestHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkCommitRequestHandlerImpl, const OFFSET: isize>() -> IInkCommitRequestHandlerVtbl {
        unsafe extern "system" fn OnCommitRequested<Impl: IInkCommitRequestHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCommitRequested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkCommitRequestHandler>, ::windows::core::GetTrustLevel, OnCommitRequested::<Impl, OFFSET>)
    }
}
pub trait IInkD2DRendererImpl: Sized {
    fn Draw();
}
impl ::windows::core::RuntimeName for IInkD2DRenderer {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ink.IInkD2DRenderer";
}
impl IInkD2DRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRendererImpl, const OFFSET: isize>() -> IInkD2DRendererVtbl {
        unsafe extern "system" fn Draw<Impl: IInkD2DRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(
                &*(&pd2d1devicecontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pinkstrokeiterable as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&fhighcontrast as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkD2DRenderer>, ::windows::core::GetTrustLevel, Draw::<Impl, OFFSET>)
    }
}
pub trait IInkD2DRenderer2Impl: Sized {
    fn Draw();
}
impl ::windows::core::RuntimeName for IInkD2DRenderer2 {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ink.IInkD2DRenderer2";
}
impl IInkD2DRenderer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkD2DRenderer2Impl, const OFFSET: isize>() -> IInkD2DRenderer2Vtbl {
        unsafe extern "system" fn Draw<Impl: IInkD2DRenderer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Draw(&*(&pd2d1devicecontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&pinkstrokeiterable as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), highcontrastadjustment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkD2DRenderer2>, ::windows::core::GetTrustLevel, Draw::<Impl, OFFSET>)
    }
}
pub trait IInkDesktopHostImpl: Sized {
    fn QueueWorkItem();
    fn CreateInkPresenter();
    fn CreateAndInitializeInkPresenter();
}
impl ::windows::core::RuntimeName for IInkDesktopHost {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ink.IInkDesktopHost";
}
impl IInkDesktopHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkDesktopHostImpl, const OFFSET: isize>() -> IInkDesktopHostVtbl {
        unsafe extern "system" fn QueueWorkItem<Impl: IInkDesktopHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueWorkItem(&*(&workitem as *const <IInkHostWorkItem as ::windows::core::Abi>::Abi as *const <IInkHostWorkItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInkPresenter<Impl: IInkDesktopHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInkPresenter(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndInitializeInkPresenter<Impl: IInkDesktopHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndInitializeInkPresenter(&*(&rootvisual as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), width, height, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkDesktopHost>, ::windows::core::GetTrustLevel, QueueWorkItem::<Impl, OFFSET>, CreateInkPresenter::<Impl, OFFSET>, CreateAndInitializeInkPresenter::<Impl, OFFSET>)
    }
}
pub trait IInkHostWorkItemImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for IInkHostWorkItem {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ink.IInkHostWorkItem";
}
impl IInkHostWorkItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkHostWorkItemImpl, const OFFSET: isize>() -> IInkHostWorkItemVtbl {
        unsafe extern "system" fn Invoke<Impl: IInkHostWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkHostWorkItem>, ::windows::core::GetTrustLevel, Invoke::<Impl, OFFSET>)
    }
}
pub trait IInkPresenterDesktopImpl: Sized {
    fn SetRootVisual();
    fn SetCommitRequestHandler();
    fn GetSize();
    fn SetSize();
    fn OnHighContrastChanged();
}
impl ::windows::core::RuntimeName for IInkPresenterDesktop {
    const NAME: &'static str = "Windows.Win32.UI.Input.Ink.IInkPresenterDesktop";
}
impl IInkPresenterDesktopVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkPresenterDesktopImpl, const OFFSET: isize>() -> IInkPresenterDesktopVtbl {
        unsafe extern "system" fn SetRootVisual<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRootVisual(&*(&rootvisual as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&device as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitRequestHandler<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCommitRequestHandler(&*(&handler as *const <IInkCommitRequestHandler as ::windows::core::Abi>::Abi as *const <IInkCommitRequestHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnHighContrastChanged<Impl: IInkPresenterDesktopImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnHighContrastChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkPresenterDesktop>, ::windows::core::GetTrustLevel, SetRootVisual::<Impl, OFFSET>, SetCommitRequestHandler::<Impl, OFFSET>, GetSize::<Impl, OFFSET>, SetSize::<Impl, OFFSET>, OnHighContrastChanged::<Impl, OFFSET>)
    }
}
