#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IInkWorkspaceHostedAppManager_Impl: Sized {
    fn SetThumbnailAsync(&mut self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.IInkWorkspaceHostedAppManager";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IInkWorkspaceHostedAppManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWorkspaceHostedAppManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkWorkspaceHostedAppManager_Vtbl {
        unsafe extern "system" fn SetThumbnailAsync<Impl: IInkWorkspaceHostedAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetThumbnailAsync(&*(&bitmap as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkWorkspaceHostedAppManager, BASE_OFFSET>(),
            SetThumbnailAsync: SetThumbnailAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWorkspaceHostedAppManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkWorkspaceHostedAppManagerStatics_Impl: Sized {
    fn GetForCurrentApp(&mut self) -> ::windows::core::Result<InkWorkspaceHostedAppManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkWorkspaceHostedAppManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.IInkWorkspaceHostedAppManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInkWorkspaceHostedAppManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWorkspaceHostedAppManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkWorkspaceHostedAppManagerStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentApp<Impl: IInkWorkspaceHostedAppManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInkWorkspaceHostedAppManagerStatics, BASE_OFFSET>(),
            GetForCurrentApp: GetForCurrentApp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWorkspaceHostedAppManagerStatics as ::windows::core::Interface>::IID
    }
}
