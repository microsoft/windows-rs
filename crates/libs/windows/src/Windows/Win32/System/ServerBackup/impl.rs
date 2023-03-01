#[doc = "*Required features: `\"Win32_System_ServerBackup\"`, `\"implement\"`*"]
pub trait IWsbApplicationAsync_Impl: Sized {
    fn QueryStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Abort(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWsbApplicationAsync {}
impl IWsbApplicationAsync_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationAsync_Impl, const OFFSET: isize>() -> IWsbApplicationAsync_Vtbl {
        unsafe extern "system" fn QueryStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationAsync_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationAsync as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`, `\"implement\"`*"]
pub trait IWsbApplicationBackupSupport_Impl: Sized {
    fn CheckConsistency(&self, wszwritermetadata: &::windows::core::PCWSTR, wszcomponentname: &::windows::core::PCWSTR, wszcomponentlogicalpath: &::windows::core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const ::windows::core::PCWSTR, rgwszsnapshotvolumepath: *const ::windows::core::PCWSTR) -> ::windows::core::Result<IWsbApplicationAsync>;
}
impl ::windows::core::RuntimeName for IWsbApplicationBackupSupport {}
impl IWsbApplicationBackupSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationBackupSupport_Impl, const OFFSET: isize>() -> IWsbApplicationBackupSupport_Vtbl {
        unsafe extern "system" fn CheckConsistency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationBackupSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows::core::PCWSTR, wszcomponentname: ::windows::core::PCWSTR, wszcomponentlogicalpath: ::windows::core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const ::windows::core::PCWSTR, rgwszsnapshotvolumepath: *const ::windows::core::PCWSTR, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckConsistency(::core::mem::transmute(&wszwritermetadata), ::core::mem::transmute(&wszcomponentname), ::core::mem::transmute(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&cvolumes), ::core::mem::transmute_copy(&rgwszsourcevolumepath), ::core::mem::transmute_copy(&rgwszsnapshotvolumepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasync, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CheckConsistency: CheckConsistency::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationBackupSupport as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ServerBackup\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWsbApplicationRestoreSupport_Impl: Sized {
    fn PreRestore(&self, wszwritermetadata: &::windows::core::PCWSTR, wszcomponentname: &::windows::core::PCWSTR, wszcomponentlogicalpath: &::windows::core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn PostRestore(&self, wszwritermetadata: &::windows::core::PCWSTR, wszcomponentname: &::windows::core::PCWSTR, wszcomponentlogicalpath: &::windows::core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn OrderComponents(&self, ccomponents: u32, rgcomponentname: *const ::windows::core::PCWSTR, rgcomponentlogicalpaths: *const ::windows::core::PCWSTR, prgcomponentname: *mut *mut ::windows::core::PWSTR, prgcomponentlogicalpath: *mut *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn IsRollForwardSupported(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWsbApplicationRestoreSupport {}
#[cfg(feature = "Win32_Foundation")]
impl IWsbApplicationRestoreSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>() -> IWsbApplicationRestoreSupport_Vtbl {
        unsafe extern "system" fn PreRestore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows::core::PCWSTR, wszcomponentname: ::windows::core::PCWSTR, wszcomponentlogicalpath: ::windows::core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreRestore(::core::mem::transmute(&wszwritermetadata), ::core::mem::transmute(&wszcomponentname), ::core::mem::transmute(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn PostRestore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszwritermetadata: ::windows::core::PCWSTR, wszcomponentname: ::windows::core::PCWSTR, wszcomponentlogicalpath: ::windows::core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostRestore(::core::mem::transmute(&wszwritermetadata), ::core::mem::transmute(&wszcomponentname), ::core::mem::transmute(&wszcomponentlogicalpath), ::core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn OrderComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccomponents: u32, rgcomponentname: *const ::windows::core::PCWSTR, rgcomponentlogicalpaths: *const ::windows::core::PCWSTR, prgcomponentname: *mut *mut ::windows::core::PWSTR, prgcomponentlogicalpath: *mut *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OrderComponents(::core::mem::transmute_copy(&ccomponents), ::core::mem::transmute_copy(&rgcomponentname), ::core::mem::transmute_copy(&rgcomponentlogicalpaths), ::core::mem::transmute_copy(&prgcomponentname), ::core::mem::transmute_copy(&prgcomponentlogicalpath)).into()
        }
        unsafe extern "system" fn IsRollForwardSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbrollforwardsupported: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRollForwardSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbrollforwardsupported, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreRestore: PreRestore::<Identity, Impl, OFFSET>,
            PostRestore: PostRestore::<Identity, Impl, OFFSET>,
            OrderComponents: OrderComponents::<Identity, Impl, OFFSET>,
            IsRollForwardSupported: IsRollForwardSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWsbApplicationRestoreSupport as ::windows::core::ComInterface>::IID
    }
}
