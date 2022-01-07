#[cfg(feature = "implement_exclusive")]
pub trait IInkWorkspaceHostedAppManagerImpl: Sized {
    fn SetThumbnailAsync(&self, bitmap: &::core::option::Option<super::super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.IInkWorkspaceHostedAppManager";
}
#[cfg(feature = "implement_exclusive")]
impl IInkWorkspaceHostedAppManagerVtbl {
    pub const fn new<Impl: IInkWorkspaceHostedAppManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkWorkspaceHostedAppManagerVtbl {
        unsafe extern "system" fn SetThumbnailAsync<Impl: IInkWorkspaceHostedAppManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetThumbnailAsync(&*(&bitmap as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkWorkspaceHostedAppManager>, base.5, SetThumbnailAsync::<Impl, OFFSET>)
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
    pub const fn new<Impl: IInkWorkspaceHostedAppManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkWorkspaceHostedAppManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentApp<Impl: IInkWorkspaceHostedAppManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkWorkspaceHostedAppManagerStatics>, base.5, GetForCurrentApp::<Impl, OFFSET>)
    }
}
