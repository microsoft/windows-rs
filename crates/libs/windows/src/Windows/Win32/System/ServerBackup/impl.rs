pub trait IWsbApplicationAsyncImpl: Sized {
    fn QueryStatus(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
}
impl IWsbApplicationAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWsbApplicationAsyncVtbl {
        unsafe extern "system" fn QueryStatus<Impl: IWsbApplicationAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *phrresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IWsbApplicationAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryStatus: QueryStatus::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWsbApplicationBackupSupportImpl: Sized {
    fn CheckConsistency(&mut self, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, cvolumes: u32, rgwszsourcevolumepath: *const super::super::Foundation::PWSTR, rgwszsnapshotvolumepath: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<IWsbApplicationAsync>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWsbApplicationBackupSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationBackupSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWsbApplicationBackupSupportVtbl {
        unsafe extern "system" fn CheckConsistency<Impl: IWsbApplicationBackupSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, cvolumes: u32, rgwszsourcevolumepath: *const super::super::Foundation::PWSTR, rgwszsnapshotvolumepath: *const super::super::Foundation::PWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckConsistency(::core::mem::transmute_copy(&wszwritermetadata), ::core::mem::transmute_copy(&wszcomponentname), ::core::mem::transmute_copy(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&cvolumes), ::core::mem::transmute_copy(&rgwszsourcevolumepath), ::core::mem::transmute_copy(&rgwszsnapshotvolumepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppasync = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CheckConsistency: CheckConsistency::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationBackupSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWsbApplicationRestoreSupportImpl: Sized {
    fn PreRestore(&mut self, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn PostRestore(&mut self, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn OrderComponents(&mut self, ccomponents: u32, rgcomponentname: *const super::super::Foundation::PWSTR, rgcomponentlogicalpaths: *const super::super::Foundation::PWSTR, prgcomponentname: *mut *mut super::super::Foundation::PWSTR, prgcomponentlogicalpath: *mut *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn IsRollForwardSupported(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWsbApplicationRestoreSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWsbApplicationRestoreSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWsbApplicationRestoreSupportVtbl {
        unsafe extern "system" fn PreRestore<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreRestore(::core::mem::transmute_copy(&wszwritermetadata), ::core::mem::transmute_copy(&wszcomponentname), ::core::mem::transmute_copy(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn PostRestore<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostRestore(::core::mem::transmute_copy(&wszwritermetadata), ::core::mem::transmute_copy(&wszcomponentname), ::core::mem::transmute_copy(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn OrderComponents<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccomponents: u32, rgcomponentname: *const super::super::Foundation::PWSTR, rgcomponentlogicalpaths: *const super::super::Foundation::PWSTR, prgcomponentname: *mut *mut super::super::Foundation::PWSTR, prgcomponentlogicalpath: *mut *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OrderComponents(::core::mem::transmute_copy(&ccomponents), ::core::mem::transmute_copy(&rgcomponentname), ::core::mem::transmute_copy(&rgcomponentlogicalpaths), ::core::mem::transmute_copy(&prgcomponentname), ::core::mem::transmute_copy(&prgcomponentlogicalpath)).into()
        }
        unsafe extern "system" fn IsRollForwardSupported<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrollforwardsupported: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRollForwardSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pbrollforwardsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PreRestore: PreRestore::<Impl, IMPL_OFFSET>,
            PostRestore: PostRestore::<Impl, IMPL_OFFSET>,
            OrderComponents: OrderComponents::<Impl, IMPL_OFFSET>,
            IsRollForwardSupported: IsRollForwardSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationRestoreSupport as ::windows::core::Interface>::IID
    }
}
