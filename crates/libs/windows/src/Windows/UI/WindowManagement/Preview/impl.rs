#[cfg(feature = "implement_exclusive")]
pub trait IWindowManagementPreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.IWindowManagementPreview";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowManagementPreviewVtbl {
    pub const fn new<Impl: IWindowManagementPreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowManagementPreviewVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowManagementPreview>, base.5)
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
    pub const fn new<Impl: IWindowManagementPreviewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowManagementPreviewStaticsVtbl {
        unsafe extern "system" fn SetPreferredMinSize<Impl: IWindowManagementPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreferredMinSize(&*(&window as *const <super::AppWindow as ::windows::core::Abi>::Abi as *const <super::AppWindow as ::windows::core::DefaultType>::DefaultType), &*(&preferredframeminsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowManagementPreviewStatics>, base.5, SetPreferredMinSize::<Impl, OFFSET>)
    }
}
