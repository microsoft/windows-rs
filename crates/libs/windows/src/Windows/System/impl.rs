#[cfg(feature = "UI_ViewManagement")]
pub trait ILauncherViewOptions_Impl: Sized {
    fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_ViewManagement")]
impl ::windows::core::RuntimeName for ILauncherViewOptions {
    const NAME: &'static str = "Windows.System.ILauncherViewOptions";
}
#[cfg(feature = "UI_ViewManagement")]
impl ILauncherViewOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILauncherViewOptions_Impl, const OFFSET: isize>() -> ILauncherViewOptions_Vtbl {
        unsafe extern "system" fn DesiredRemainingView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILauncherViewOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredRemainingView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRemainingView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILauncherViewOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredRemainingView(value).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherViewOptions, OFFSET>(),
            DesiredRemainingView: DesiredRemainingView::<Identity, Impl, OFFSET>,
            SetDesiredRemainingView: SetDesiredRemainingView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherViewOptions as ::windows::core::Interface>::IID
    }
}
