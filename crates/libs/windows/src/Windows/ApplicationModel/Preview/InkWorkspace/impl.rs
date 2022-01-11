#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IInkWorkspaceHostedAppManagerImpl: Sized {
    fn SetThumbnailAsync(&self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.IInkWorkspaceHostedAppManager";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IInkWorkspaceHostedAppManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWorkspaceHostedAppManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkWorkspaceHostedAppManagerVtbl {
        unsafe extern "system" fn SetThumbnailAsync<Impl: IInkWorkspaceHostedAppManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkWorkspaceHostedAppManager>, ::windows::core::GetTrustLevel, SetThumbnailAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWorkspaceHostedAppManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkWorkspaceHostedAppManagerStaticsImpl: Sized {
    fn GetForCurrentApp(&self) -> ::windows::core::Result<InkWorkspaceHostedAppManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkWorkspaceHostedAppManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.IInkWorkspaceHostedAppManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInkWorkspaceHostedAppManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkWorkspaceHostedAppManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkWorkspaceHostedAppManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentApp<Impl: IInkWorkspaceHostedAppManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkWorkspaceHostedAppManagerStatics>, ::windows::core::GetTrustLevel, GetForCurrentApp::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkWorkspaceHostedAppManagerStatics as ::windows::core::Interface>::IID
    }
}
