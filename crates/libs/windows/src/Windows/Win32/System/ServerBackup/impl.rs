pub trait IWsbApplicationAsync_Impl: Sized {
    fn QueryStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Abort(&self) -> ::windows::core::Result<()>;
}
impl IWsbApplicationAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationAsync_Impl, const OFFSET: isize>() -> IWsbApplicationAsync_Vtbl {
        unsafe extern "system" fn QueryStatus<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *phrresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abort().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWsbApplicationBackupSupport_Impl: Sized {
    fn CheckConsistency(&self, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, cvolumes: u32, rgwszsourcevolumepath: *const super::super::Foundation::PWSTR, rgwszsnapshotvolumepath: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<IWsbApplicationAsync>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWsbApplicationBackupSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationBackupSupport_Impl, const OFFSET: isize>() -> IWsbApplicationBackupSupport_Vtbl {
        unsafe extern "system" fn CheckConsistency<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationBackupSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, cvolumes: u32, rgwszsourcevolumepath: *const super::super::Foundation::PWSTR, rgwszsnapshotvolumepath: *const super::super::Foundation::PWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckConsistency(::core::mem::transmute_copy(&wszwritermetadata), ::core::mem::transmute_copy(&wszcomponentname), ::core::mem::transmute_copy(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&cvolumes), ::core::mem::transmute_copy(&rgwszsourcevolumepath), ::core::mem::transmute_copy(&rgwszsnapshotvolumepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CheckConsistency: CheckConsistency::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationBackupSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWsbApplicationRestoreSupport_Impl: Sized {
    fn PreRestore(&self, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn PostRestore(&self, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn OrderComponents(&self, ccomponents: u32, rgcomponentname: *const super::super::Foundation::PWSTR, rgcomponentlogicalpaths: *const super::super::Foundation::PWSTR, prgcomponentname: *mut *mut super::super::Foundation::PWSTR, prgcomponentlogicalpath: *mut *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn IsRollForwardSupported(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWsbApplicationRestoreSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>() -> IWsbApplicationRestoreSupport_Vtbl {
        unsafe extern "system" fn PreRestore<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreRestore(::core::mem::transmute_copy(&wszwritermetadata), ::core::mem::transmute_copy(&wszcomponentname), ::core::mem::transmute_copy(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn PostRestore<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PostRestore(::core::mem::transmute_copy(&wszwritermetadata), ::core::mem::transmute_copy(&wszcomponentname), ::core::mem::transmute_copy(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn OrderComponents<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccomponents: u32, rgcomponentname: *const super::super::Foundation::PWSTR, rgcomponentlogicalpaths: *const super::super::Foundation::PWSTR, prgcomponentname: *mut *mut super::super::Foundation::PWSTR, prgcomponentlogicalpath: *mut *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OrderComponents(::core::mem::transmute_copy(&ccomponents), ::core::mem::transmute_copy(&rgcomponentname), ::core::mem::transmute_copy(&rgcomponentlogicalpaths), ::core::mem::transmute_copy(&prgcomponentname), ::core::mem::transmute_copy(&prgcomponentlogicalpath)).into()
        }
        unsafe extern "system" fn IsRollForwardSupported<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrollforwardsupported: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRollForwardSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pbrollforwardsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PreRestore: PreRestore::<Identity, Impl, OFFSET>,
            PostRestore: PostRestore::<Identity, Impl, OFFSET>,
            OrderComponents: OrderComponents::<Identity, Impl, OFFSET>,
            IsRollForwardSupported: IsRollForwardSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationRestoreSupport as ::windows::core::Interface>::IID
    }
}
