#[cfg(feature = "implement_exclusive")]
pub trait IWindowManagementPreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.IWindowManagementPreview";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowManagementPreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowManagementPreviewImpl, const OFFSET: isize>() -> IWindowManagementPreviewVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowManagementPreview>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowManagementPreviewStaticsImpl: Sized {
    fn SetPreferredMinSize(&self, window: &::core::option::Option<super::AppWindow>, preferredframeminsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowManagementPreviewStatics {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.IWindowManagementPreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowManagementPreviewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowManagementPreviewStaticsImpl, const OFFSET: isize>() -> IWindowManagementPreviewStaticsVtbl {
        unsafe extern "system" fn SetPreferredMinSize<Impl: IWindowManagementPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredMinSize(&*(&window as *const <super::AppWindow as ::windows::core::Abi>::Abi as *const <super::AppWindow as ::windows::core::DefaultType>::DefaultType), &*(&preferredframeminsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowManagementPreviewStatics>, ::windows::core::GetTrustLevel, SetPreferredMinSize::<Impl, OFFSET>)
    }
}
