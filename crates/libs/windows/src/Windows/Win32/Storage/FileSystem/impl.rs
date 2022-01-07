#[cfg(feature = "Win32_System_Com")]
pub trait IDiskQuotaControlImpl: Sized + IConnectionPointContainerImpl {
    fn Initialize();
    fn SetQuotaState();
    fn GetQuotaState();
    fn SetQuotaLogFlags();
    fn GetQuotaLogFlags();
    fn SetDefaultQuotaThreshold();
    fn GetDefaultQuotaThreshold();
    fn GetDefaultQuotaThresholdText();
    fn SetDefaultQuotaLimit();
    fn GetDefaultQuotaLimit();
    fn GetDefaultQuotaLimitText();
    fn AddUserSid();
    fn AddUserName();
    fn DeleteUser();
    fn FindUserSid();
    fn FindUserName();
    fn CreateEnumUsers();
    fn CreateUserBatch();
    fn InvalidateSidNameCache();
    fn GiveUserNameResolutionPriority();
    fn ShutdownNameResolution();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiskQuotaControl {
    const NAME: &'static str = "Windows.Win32.Storage.FileSystem.IDiskQuotaControl";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiskQuotaControlVtbl {
    pub const fn new<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiskQuotaControlVtbl {
        unsafe extern "system" fn Initialize<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&breadwrite as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuotaState<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuotaState(dwstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaState<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaState(pdwstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuotaLogFlags<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuotaLogFlags(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaLogFlags<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaLogFlags(pdwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuotaThreshold<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llthreshold: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDefaultQuotaThreshold(llthreshold) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultQuotaThreshold<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultQuotaThreshold(pllthreshold) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultQuotaThresholdText<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultQuotaThresholdText(&*(&psztext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuotaLimit<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lllimit: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDefaultQuotaLimit(lllimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultQuotaLimit<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultQuotaLimit(plllimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultQuotaLimitText<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultQuotaLimitText(&*(&psztext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddUserSid<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddUserSid(&*(&pusersid as *const <super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType), fnameresolution, ::core::mem::transmute_copy(&ppuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddUserName<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlogonname: super::super::Foundation::PWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddUserName(&*(&pszlogonname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), fnameresolution, ::core::mem::transmute_copy(&ppuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteUser<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteUser(&*(&puser as *const <IDiskQuotaUser as ::windows::core::Abi>::Abi as *const <IDiskQuotaUser as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUserSid<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindUserSid(&*(&pusersid as *const <super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType), fnameresolution, ::core::mem::transmute_copy(&ppuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUserName<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlogonname: super::super::Foundation::PWSTR, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindUserName(&*(&pszlogonname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnumUsers<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEnumUsers(&*(&rgpusersids as *const <super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType), cpsids, fnameresolution, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUserBatch<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateUserBatch(::core::mem::transmute_copy(&ppbatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateSidNameCache<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvalidateSidNameCache() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GiveUserNameResolutionPriority<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GiveUserNameResolutionPriority(&*(&puser as *const <IDiskQuotaUser as ::windows::core::Abi>::Abi as *const <IDiskQuotaUser as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownNameResolution<Impl: IDiskQuotaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutdownNameResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDiskQuotaControl>,
            base.5,
            Initialize::<Impl, OFFSET>,
            SetQuotaState::<Impl, OFFSET>,
            GetQuotaState::<Impl, OFFSET>,
            SetQuotaLogFlags::<Impl, OFFSET>,
            GetQuotaLogFlags::<Impl, OFFSET>,
            SetDefaultQuotaThreshold::<Impl, OFFSET>,
            GetDefaultQuotaThreshold::<Impl, OFFSET>,
            GetDefaultQuotaThresholdText::<Impl, OFFSET>,
            SetDefaultQuotaLimit::<Impl, OFFSET>,
            GetDefaultQuotaLimit::<Impl, OFFSET>,
            GetDefaultQuotaLimitText::<Impl, OFFSET>,
            AddUserSid::<Impl, OFFSET>,
            AddUserName::<Impl, OFFSET>,
            DeleteUser::<Impl, OFFSET>,
            FindUserSid::<Impl, OFFSET>,
            FindUserName::<Impl, OFFSET>,
            CreateEnumUsers::<Impl, OFFSET>,
            CreateUserBatch::<Impl, OFFSET>,
            InvalidateSidNameCache::<Impl, OFFSET>,
            GiveUserNameResolutionPriority::<Impl, OFFSET>,
            ShutdownNameResolution::<Impl, OFFSET>,
        )
    }
}
pub trait IDiskQuotaEventsImpl: Sized {
    fn OnUserNameChanged();
}
impl ::windows::core::RuntimeName for IDiskQuotaEvents {
    const NAME: &'static str = "Windows.Win32.Storage.FileSystem.IDiskQuotaEvents";
}
impl IDiskQuotaEventsVtbl {
    pub const fn new<Impl: IDiskQuotaEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiskQuotaEventsVtbl {
        unsafe extern "system" fn OnUserNameChanged<Impl: IDiskQuotaEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnUserNameChanged(&*(&puser as *const <IDiskQuotaUser as ::windows::core::Abi>::Abi as *const <IDiskQuotaUser as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiskQuotaEvents>, base.5, OnUserNameChanged::<Impl, OFFSET>)
    }
}
pub trait IDiskQuotaUserImpl: Sized {
    fn GetID();
    fn GetName();
    fn GetSidLength();
    fn GetSid();
    fn GetQuotaThreshold();
    fn GetQuotaThresholdText();
    fn GetQuotaLimit();
    fn GetQuotaLimitText();
    fn GetQuotaUsed();
    fn GetQuotaUsedText();
    fn GetQuotaInformation();
    fn SetQuotaThreshold();
    fn SetQuotaLimit();
    fn Invalidate();
    fn GetAccountStatus();
}
impl ::windows::core::RuntimeName for IDiskQuotaUser {
    const NAME: &'static str = "Windows.Win32.Storage.FileSystem.IDiskQuotaUser";
}
impl IDiskQuotaUserVtbl {
    pub const fn new<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiskQuotaUserVtbl {
        unsafe extern "system" fn GetID<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetID(pulid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszaccountcontainer: super::super::Foundation::PWSTR, cchaccountcontainer: u32, pszlogonname: super::super::Foundation::PWSTR, cchlogonname: u32, pszdisplayname: super::super::Foundation::PWSTR, cchdisplayname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(
                &*(&pszaccountcontainer as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchaccountcontainer,
                &*(&pszlogonname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchlogonname,
                &*(&pszdisplayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchdisplayname,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSidLength<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSidLength(pdwlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSid<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSid(pbsidbuffer, cbsidbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaThreshold<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaThreshold(pllthreshold) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaThresholdText<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaThresholdText(&*(&psztext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaLimit<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaLimit(plllimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaLimitText<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaLimitText(&*(&psztext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaUsed<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllused: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaUsed(pllused) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaUsedText<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaUsedText(&*(&psztext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchtext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuotaInformation<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuotaInformation(&*(&pbquotainfo as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbquotainfo) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuotaThreshold<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuotaThreshold(llthreshold, &*(&fwritethrough as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuotaLimit<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuotaLimit(lllimit, &*(&fwritethrough as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invalidate<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invalidate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccountStatus<Impl: IDiskQuotaUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccountStatus(pdwstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDiskQuotaUser>,
            base.5,
            GetID::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            GetSidLength::<Impl, OFFSET>,
            GetSid::<Impl, OFFSET>,
            GetQuotaThreshold::<Impl, OFFSET>,
            GetQuotaThresholdText::<Impl, OFFSET>,
            GetQuotaLimit::<Impl, OFFSET>,
            GetQuotaLimitText::<Impl, OFFSET>,
            GetQuotaUsed::<Impl, OFFSET>,
            GetQuotaUsedText::<Impl, OFFSET>,
            GetQuotaInformation::<Impl, OFFSET>,
            SetQuotaThreshold::<Impl, OFFSET>,
            SetQuotaLimit::<Impl, OFFSET>,
            Invalidate::<Impl, OFFSET>,
            GetAccountStatus::<Impl, OFFSET>,
        )
    }
}
pub trait IDiskQuotaUserBatchImpl: Sized {
    fn Add();
    fn Remove();
    fn RemoveAll();
    fn FlushToDisk();
}
impl ::windows::core::RuntimeName for IDiskQuotaUserBatch {
    const NAME: &'static str = "Windows.Win32.Storage.FileSystem.IDiskQuotaUserBatch";
}
impl IDiskQuotaUserBatchVtbl {
    pub const fn new<Impl: IDiskQuotaUserBatchImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDiskQuotaUserBatchVtbl {
        unsafe extern "system" fn Add<Impl: IDiskQuotaUserBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&puser as *const <IDiskQuotaUser as ::windows::core::Abi>::Abi as *const <IDiskQuotaUser as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IDiskQuotaUserBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&puser as *const <IDiskQuotaUser as ::windows::core::Abi>::Abi as *const <IDiskQuotaUser as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAll<Impl: IDiskQuotaUserBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushToDisk<Impl: IDiskQuotaUserBatchImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlushToDisk() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDiskQuotaUserBatch>, base.5, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, RemoveAll::<Impl, OFFSET>, FlushToDisk::<Impl, OFFSET>)
    }
}
pub trait IEnumDiskQuotaUsersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDiskQuotaUsers {
    const NAME: &'static str = "Windows.Win32.Storage.FileSystem.IEnumDiskQuotaUsers";
}
impl IEnumDiskQuotaUsersVtbl {
    pub const fn new<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumDiskQuotaUsersVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cusers: u32, rgusers: *mut ::windows::core::RawPtr, pcusersfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(cusers, ::core::mem::transmute_copy(&rgusers), pcusersfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cusers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(cusers) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumDiskQuotaUsers>, base.5, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
