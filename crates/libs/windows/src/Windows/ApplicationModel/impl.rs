#[cfg(feature = "Foundation")]
pub trait IEnteredBackgroundEventArgs_Impl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IEnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IEnteredBackgroundEventArgs";
}
#[cfg(feature = "Foundation")]
impl IEnteredBackgroundEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnteredBackgroundEventArgs_Impl, const OFFSET: isize>() -> IEnteredBackgroundEventArgs_Vtbl {
        unsafe extern "system" fn GetDeferral<Identity: ::windows::core::IUnknownImpl, Impl: IEnteredBackgroundEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEnteredBackgroundEventArgs, OFFSET>(),
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnteredBackgroundEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ILeavingBackgroundEventArgs_Impl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ILeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ILeavingBackgroundEventArgs";
}
#[cfg(feature = "Foundation")]
impl ILeavingBackgroundEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILeavingBackgroundEventArgs_Impl, const OFFSET: isize>() -> ILeavingBackgroundEventArgs_Vtbl {
        unsafe extern "system" fn GetDeferral<Identity: ::windows::core::IUnknownImpl, Impl: ILeavingBackgroundEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILeavingBackgroundEventArgs, OFFSET>(),
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILeavingBackgroundEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait ISuspendingDeferral_Impl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISuspendingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingDeferral";
}
impl ISuspendingDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingDeferral_Impl, const OFFSET: isize>() -> ISuspendingDeferral_Vtbl {
        unsafe extern "system" fn Complete<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISuspendingDeferral, OFFSET>(), Complete: Complete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingDeferral as ::windows::core::Interface>::IID
    }
}
pub trait ISuspendingEventArgs_Impl: Sized {
    fn SuspendingOperation(&self) -> ::windows::core::Result<SuspendingOperation>;
}
impl ::windows::core::RuntimeName for ISuspendingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingEventArgs";
}
impl ISuspendingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingEventArgs_Impl, const OFFSET: isize>() -> ISuspendingEventArgs_Vtbl {
        unsafe extern "system" fn SuspendingOperation<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SuspendingOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISuspendingEventArgs, OFFSET>(),
            SuspendingOperation: SuspendingOperation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISuspendingOperation_Impl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<SuspendingDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISuspendingOperation {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingOperation";
}
#[cfg(feature = "Foundation")]
impl ISuspendingOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingOperation_Impl, const OFFSET: isize>() -> ISuspendingOperation_Vtbl {
        unsafe extern "system" fn GetDeferral<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISuspendingOperation, OFFSET>(),
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
            Deadline: Deadline::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingOperation as ::windows::core::Interface>::IID
    }
}
