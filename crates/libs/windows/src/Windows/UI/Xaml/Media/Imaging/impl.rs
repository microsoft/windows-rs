#[cfg(feature = "ApplicationModel_Background")]
pub trait IXamlRenderingBackgroundTaskOverrides_Impl: Sized {
    fn OnRun(&self, taskinstance: &::core::option::Option<super::super::super::super::ApplicationModel::Background::IBackgroundTaskInstance>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows::core::RuntimeName for IXamlRenderingBackgroundTaskOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides";
}
#[cfg(feature = "ApplicationModel_Background")]
impl IXamlRenderingBackgroundTaskOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskOverrides_Impl, const OFFSET: isize>() -> IXamlRenderingBackgroundTaskOverrides_Vtbl {
        unsafe extern "system" fn OnRun<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRenderingBackgroundTaskOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRun(::core::mem::transmute(&taskinstance)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlRenderingBackgroundTaskOverrides, OFFSET>(),
            OnRun: OnRun::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlRenderingBackgroundTaskOverrides as ::windows::core::Interface>::IID
    }
}
