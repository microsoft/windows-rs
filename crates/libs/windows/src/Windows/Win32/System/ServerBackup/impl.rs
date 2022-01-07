pub trait IWsbApplicationAsyncImpl: Sized {
    fn QueryStatus();
    fn Abort();
}
impl ::windows::core::RuntimeName for IWsbApplicationAsync {
    const NAME: &'static str = "Windows.Win32.System.ServerBackup.IWsbApplicationAsync";
}
impl IWsbApplicationAsyncVtbl {
    pub const fn new<Impl: IWsbApplicationAsyncImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWsbApplicationAsyncVtbl {
        unsafe extern "system" fn QueryStatus<Impl: IWsbApplicationAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryStatus(::core::mem::transmute_copy(&phrresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IWsbApplicationAsyncImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWsbApplicationAsync>, base.5, QueryStatus::<Impl, OFFSET>, Abort::<Impl, OFFSET>)
    }
}
pub trait IWsbApplicationBackupSupportImpl: Sized {
    fn CheckConsistency();
}
impl ::windows::core::RuntimeName for IWsbApplicationBackupSupport {
    const NAME: &'static str = "Windows.Win32.System.ServerBackup.IWsbApplicationBackupSupport";
}
impl IWsbApplicationBackupSupportVtbl {
    pub const fn new<Impl: IWsbApplicationBackupSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWsbApplicationBackupSupportVtbl {
        unsafe extern "system" fn CheckConsistency<Impl: IWsbApplicationBackupSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, cvolumes: u32, rgwszsourcevolumepath: *const super::super::Foundation::PWSTR, rgwszsnapshotvolumepath: *const super::super::Foundation::PWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckConsistency(
                &*(&wszwritermetadata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcomponentlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cvolumes,
                &*(&rgwszsourcevolumepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&rgwszsnapshotvolumepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppasync),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWsbApplicationBackupSupport>, base.5, CheckConsistency::<Impl, OFFSET>)
    }
}
pub trait IWsbApplicationRestoreSupportImpl: Sized {
    fn PreRestore();
    fn PostRestore();
    fn OrderComponents();
    fn IsRollForwardSupported();
}
impl ::windows::core::RuntimeName for IWsbApplicationRestoreSupport {
    const NAME: &'static str = "Windows.Win32.System.ServerBackup.IWsbApplicationRestoreSupport";
}
impl IWsbApplicationRestoreSupportVtbl {
    pub const fn new<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWsbApplicationRestoreSupportVtbl {
        unsafe extern "system" fn PreRestore<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreRestore(
                &*(&wszwritermetadata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcomponentlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bnorollforward as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostRestore<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszwritermetadata: super::super::Foundation::PWSTR, wszcomponentname: super::super::Foundation::PWSTR, wszcomponentlogicalpath: super::super::Foundation::PWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PostRestore(
                &*(&wszwritermetadata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszcomponentlogicalpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bnorollforward as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrderComponents<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccomponents: u32, rgcomponentname: *const super::super::Foundation::PWSTR, rgcomponentlogicalpaths: *const super::super::Foundation::PWSTR, prgcomponentname: *mut *mut super::super::Foundation::PWSTR, prgcomponentlogicalpath: *mut *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OrderComponents(
                ccomponents,
                &*(&rgcomponentname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&rgcomponentlogicalpaths as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&prgcomponentname),
                ::core::mem::transmute_copy(&prgcomponentlogicalpath),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRollForwardSupported<Impl: IWsbApplicationRestoreSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbrollforwardsupported: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRollForwardSupported(::core::mem::transmute_copy(&pbrollforwardsupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWsbApplicationRestoreSupport>, base.5, PreRestore::<Impl, OFFSET>, PostRestore::<Impl, OFFSET>, OrderComponents::<Impl, OFFSET>, IsRollForwardSupported::<Impl, OFFSET>)
    }
}
