#[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IEnteredBackgroundEventArgs_Impl: Sized {
    fn GetDeferral(&self) -> ::windows_core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IEnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IEnteredBackgroundEventArgs";
}
#[cfg(feature = "Foundation")]
impl IEnteredBackgroundEventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnteredBackgroundEventArgs_Impl, const OFFSET: isize>() -> IEnteredBackgroundEventArgs_Vtbl {
        unsafe extern "system" fn GetDeferral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnteredBackgroundEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IEnteredBackgroundEventArgs, OFFSET>(),
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnteredBackgroundEventArgs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait ILeavingBackgroundEventArgs_Impl: Sized {
    fn GetDeferral(&self) -> ::windows_core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for ILeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ILeavingBackgroundEventArgs";
}
#[cfg(feature = "Foundation")]
impl ILeavingBackgroundEventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILeavingBackgroundEventArgs_Impl, const OFFSET: isize>() -> ILeavingBackgroundEventArgs_Vtbl {
        unsafe extern "system" fn GetDeferral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILeavingBackgroundEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ILeavingBackgroundEventArgs, OFFSET>(),
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILeavingBackgroundEventArgs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`, `\"implement\"`*"]
pub trait IPackageCatalogStatics2_Impl: Sized {
    fn OpenForPackage(&self, package: ::core::option::Option<&Package>) -> ::windows_core::Result<PackageCatalog>;
}
impl ::windows_core::RuntimeName for IPackageCatalogStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalogStatics2";
}
impl IPackageCatalogStatics2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPackageCatalogStatics2_Impl, const OFFSET: isize>() -> IPackageCatalogStatics2_Vtbl {
        unsafe extern "system" fn OpenForPackage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPackageCatalogStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenForPackage(::windows_core::from_raw_borrowed(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IPackageCatalogStatics2, OFFSET>(),
            OpenForPackage: OpenForPackage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPackageCatalogStatics2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`, `\"implement\"`*"]
pub trait ISuspendingDeferral_Impl: Sized {
    fn Complete(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISuspendingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingDeferral";
}
impl ISuspendingDeferral_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISuspendingDeferral_Impl, const OFFSET: isize>() -> ISuspendingDeferral_Vtbl {
        unsafe extern "system" fn Complete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISuspendingDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Complete().into()
        }
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISuspendingDeferral, OFFSET>(), Complete: Complete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISuspendingDeferral as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`, `\"implement\"`*"]
pub trait ISuspendingEventArgs_Impl: Sized {
    fn SuspendingOperation(&self) -> ::windows_core::Result<SuspendingOperation>;
}
impl ::windows_core::RuntimeName for ISuspendingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingEventArgs";
}
impl ISuspendingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISuspendingEventArgs_Impl, const OFFSET: isize>() -> ISuspendingEventArgs_Vtbl {
        unsafe extern "system" fn SuspendingOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISuspendingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SuspendingOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISuspendingEventArgs, OFFSET>(),
            SuspendingOperation: SuspendingOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISuspendingEventArgs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait ISuspendingOperation_Impl: Sized {
    fn GetDeferral(&self) -> ::windows_core::Result<SuspendingDeferral>;
    fn Deadline(&self) -> ::windows_core::Result<super::Foundation::DateTime>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for ISuspendingOperation {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingOperation";
}
#[cfg(feature = "Foundation")]
impl ISuspendingOperation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISuspendingOperation_Impl, const OFFSET: isize>() -> ISuspendingOperation_Vtbl {
        unsafe extern "system" fn GetDeferral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISuspendingOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISuspendingOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISuspendingOperation, OFFSET>(),
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
            Deadline: Deadline::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISuspendingOperation as ::windows_core::ComInterface>::IID
    }
}
