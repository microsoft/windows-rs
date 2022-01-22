#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IABContainer_Impl: Sized + IMAPIProp_Impl + IMAPIContainer_Impl {
    fn CreateEntry(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> ::windows::core::Result<IMAPIProp>;
    fn CopyEntries(&mut self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
    fn DeleteEntries(&mut self, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::Result<()>;
    fn ResolveNames(&mut self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> ::windows::core::Result<_flaglist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IABContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IABContainer_Impl, const OFFSET: isize>() -> IABContainer_Vtbl {
        unsafe extern "system" fn CreateEntry<Identity: ::windows::core::IUnknownImpl, Impl: IABContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEntry(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulcreateflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppmapipropentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyEntries<Identity: ::windows::core::IUnknownImpl, Impl: IABContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyEntries(::core::mem::transmute_copy(&lpentries), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn DeleteEntries<Identity: ::windows::core::IUnknownImpl, Impl: IABContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteEntries(::core::mem::transmute_copy(&lpentries), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ResolveNames<Identity: ::windows::core::IUnknownImpl, Impl: IABContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResolveNames(::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpadrlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpflaglist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMAPIContainer_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateEntry: CreateEntry::<Identity, Impl, OFFSET>,
            CopyEntries: CopyEntries::<Identity, Impl, OFFSET>,
            DeleteEntries: DeleteEntries::<Identity, Impl, OFFSET>,
            ResolveNames: ResolveNames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IABContainer as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID || iid == &<IMAPIContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAddrBook_Impl: Sized + IMAPIProp_Impl {
    fn OpenEntry(&mut self, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn CompareEntryIDs(&mut self, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::Result<()>;
    fn Advise(&mut self, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: &::core::option::Option<IMAPIAdviseSink>, lpulconnection: *mut u32) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self, ulconnection: u32) -> ::windows::core::Result<()>;
    fn CreateOneOff(&mut self, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()>;
    fn NewEntry(&mut self, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows::core::Result<()>;
    fn ResolveName(&mut self, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows::core::Result<()>;
    fn Address(&mut self, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows::core::Result<()>;
    fn Details(&mut self, lpuluiparam: *mut usize, lpfndismiss: &LPFNDISMISS, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: &LPFNBUTTON, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows::core::Result<()>;
    fn RecipOptions(&mut self, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows::core::Result<()>;
    fn QueryDefaultRecipOpt(&mut self, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows::core::Result<()>;
    fn GetPAB(&mut self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()>;
    fn SetPAB(&mut self, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::Result<()>;
    fn GetDefaultDir(&mut self, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::Result<()>;
    fn SetDefaultDir(&mut self, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::Result<()>;
    fn GetSearchPath(&mut self, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows::core::Result<()>;
    fn SetSearchPath(&mut self, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows::core::Result<()>;
    fn PrepareRecips(&mut self, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAddrBook_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>() -> IAddrBook_Vtbl {
        unsafe extern "system" fn OpenEntry<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenEntry(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulobjtype), ::core::mem::transmute_copy(&lppunk)).into()
        }
        unsafe extern "system" fn CompareEntryIDs<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompareEntryIDs(::core::mem::transmute_copy(&cbentryid1), ::core::mem::transmute_copy(&lpentryid1), ::core::mem::transmute_copy(&cbentryid2), ::core::mem::transmute_copy(&lpentryid2), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulresult)).into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Advise(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&uleventmask), ::core::mem::transmute(&lpadvisesink), ::core::mem::transmute_copy(&lpulconnection)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&ulconnection)).into()
        }
        unsafe extern "system" fn CreateOneOff<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateOneOff(::core::mem::transmute_copy(&lpszname), ::core::mem::transmute_copy(&lpszadrtype), ::core::mem::transmute_copy(&lpszaddress), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid)).into()
        }
        unsafe extern "system" fn NewEntry<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NewEntry(::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbeidcontainer), ::core::mem::transmute_copy(&lpeidcontainer), ::core::mem::transmute_copy(&cbeidnewentrytpl), ::core::mem::transmute_copy(&lpeidnewentrytpl), ::core::mem::transmute_copy(&lpcbeidnewentry), ::core::mem::transmute_copy(&lppeidnewentry)).into()
        }
        unsafe extern "system" fn ResolveName<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveName(::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpsznewentrytitle), ::core::mem::transmute_copy(&lpadrlist)).into()
        }
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Address(::core::mem::transmute_copy(&lpuluiparam), ::core::mem::transmute_copy(&lpadrparms), ::core::mem::transmute_copy(&lppadrlist)).into()
        }
        unsafe extern "system" fn Details<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpuluiparam: *mut usize, lpfndismiss: ::windows::core::RawPtr, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: ::windows::core::RawPtr, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Details(::core::mem::transmute_copy(&lpuluiparam), ::core::mem::transmute_copy(&lpfndismiss), ::core::mem::transmute_copy(&lpvdismisscontext), ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpfbuttoncallback), ::core::mem::transmute_copy(&lpvbuttoncontext), ::core::mem::transmute_copy(&lpszbuttontext), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn RecipOptions<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecipOptions(::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lprecip)).into()
        }
        unsafe extern "system" fn QueryDefaultRecipOpt<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryDefaultRecipOpt(::core::mem::transmute_copy(&lpszadrtype), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcvalues), ::core::mem::transmute_copy(&lppoptions)).into()
        }
        unsafe extern "system" fn GetPAB<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPAB(::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid)).into()
        }
        unsafe extern "system" fn SetPAB<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPAB(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid)).into()
        }
        unsafe extern "system" fn GetDefaultDir<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDefaultDir(::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid)).into()
        }
        unsafe extern "system" fn SetDefaultDir<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultDir(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid)).into()
        }
        unsafe extern "system" fn GetSearchPath<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSearchPath(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppsearchpath)).into()
        }
        unsafe extern "system" fn SetSearchPath<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSearchPath(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpsearchpath)).into()
        }
        unsafe extern "system" fn PrepareRecips<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrepareRecips(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&lpreciplist)).into()
        }
        Self {
            base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenEntry: OpenEntry::<Identity, Impl, OFFSET>,
            CompareEntryIDs: CompareEntryIDs::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            CreateOneOff: CreateOneOff::<Identity, Impl, OFFSET>,
            NewEntry: NewEntry::<Identity, Impl, OFFSET>,
            ResolveName: ResolveName::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            Details: Details::<Identity, Impl, OFFSET>,
            RecipOptions: RecipOptions::<Identity, Impl, OFFSET>,
            QueryDefaultRecipOpt: QueryDefaultRecipOpt::<Identity, Impl, OFFSET>,
            GetPAB: GetPAB::<Identity, Impl, OFFSET>,
            SetPAB: SetPAB::<Identity, Impl, OFFSET>,
            GetDefaultDir: GetDefaultDir::<Identity, Impl, OFFSET>,
            SetDefaultDir: SetDefaultDir::<Identity, Impl, OFFSET>,
            GetSearchPath: GetSearchPath::<Identity, Impl, OFFSET>,
            SetSearchPath: SetSearchPath::<Identity, Impl, OFFSET>,
            PrepareRecips: PrepareRecips::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddrBook as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAttach_Impl: Sized + IMAPIProp_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IAttach_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAttach_Impl, const OFFSET: isize>() -> IAttach_Vtbl {
        Self { base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAttach as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDistList_Impl: Sized + IMAPIProp_Impl + IMAPIContainer_Impl {
    fn CreateEntry(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32) -> ::windows::core::Result<IMAPIProp>;
    fn CopyEntries(&mut self, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
    fn DeleteEntries(&mut self, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::Result<()>;
    fn ResolveNames(&mut self, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST) -> ::windows::core::Result<_flaglist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDistList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDistList_Impl, const OFFSET: isize>() -> IDistList_Vtbl {
        unsafe extern "system" fn CreateEntry<Identity: ::windows::core::IUnknownImpl, Impl: IDistList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEntry(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulcreateflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppmapipropentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyEntries<Identity: ::windows::core::IUnknownImpl, Impl: IDistList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyEntries(::core::mem::transmute_copy(&lpentries), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn DeleteEntries<Identity: ::windows::core::IUnknownImpl, Impl: IDistList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteEntries(::core::mem::transmute_copy(&lpentries), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ResolveNames<Identity: ::windows::core::IUnknownImpl, Impl: IDistList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResolveNames(::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpadrlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpflaglist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMAPIContainer_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateEntry: CreateEntry::<Identity, Impl, OFFSET>,
            CopyEntries: CopyEntries::<Identity, Impl, OFFSET>,
            DeleteEntries: DeleteEntries::<Identity, Impl, OFFSET>,
            ResolveNames: ResolveNames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDistList as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID || iid == &<IMAPIContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIAdviseSink_Impl: Sized {
    fn OnNotify(&mut self, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMAPIAdviseSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIAdviseSink_Impl, const OFFSET: isize>() -> IMAPIAdviseSink_Vtbl {
        unsafe extern "system" fn OnNotify<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNotify(::core::mem::transmute_copy(&cnotif), ::core::mem::transmute_copy(&lpnotifications))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMAPIAdviseSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIContainer_Impl: Sized + IMAPIProp_Impl {
    fn GetContentsTable(&mut self, ulflags: u32) -> ::windows::core::Result<IMAPITable>;
    fn GetHierarchyTable(&mut self, ulflags: u32) -> ::windows::core::Result<IMAPITable>;
    fn OpenEntry(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetSearchCriteria(&mut self, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::Result<()>;
    fn GetSearchCriteria(&mut self, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMAPIContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIContainer_Impl, const OFFSET: isize>() -> IMAPIContainer_Vtbl {
        unsafe extern "system" fn GetContentsTable<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContentsTable(::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHierarchyTable<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHierarchyTable(::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenEntry<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenEntry(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulobjtype), ::core::mem::transmute_copy(&lppunk)).into()
        }
        unsafe extern "system" fn SetSearchCriteria<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSearchCriteria(::core::mem::transmute_copy(&lprestriction), ::core::mem::transmute_copy(&lpcontainerlist), ::core::mem::transmute_copy(&ulsearchflags)).into()
        }
        unsafe extern "system" fn GetSearchCriteria<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSearchCriteria(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpprestriction), ::core::mem::transmute_copy(&lppcontainerlist), ::core::mem::transmute_copy(&lpulsearchstate)).into()
        }
        Self {
            base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetContentsTable: GetContentsTable::<Identity, Impl, OFFSET>,
            GetHierarchyTable: GetHierarchyTable::<Identity, Impl, OFFSET>,
            OpenEntry: OpenEntry::<Identity, Impl, OFFSET>,
            SetSearchCriteria: SetSearchCriteria::<Identity, Impl, OFFSET>,
            GetSearchCriteria: GetSearchCriteria::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMAPIContainer as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
pub trait IMAPIControl_Impl: Sized {
    fn GetLastError(&mut self, hresult: ::windows::core::HRESULT, ulflags: u32) -> ::windows::core::Result<*mut MAPIERROR>;
    fn Activate(&mut self, ulflags: u32, uluiparam: usize) -> ::windows::core::Result<()>;
    fn GetState(&mut self, ulflags: u32, lpulstate: *mut u32) -> ::windows::core::Result<()>;
}
impl IMAPIControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIControl_Impl, const OFFSET: isize>() -> IMAPIControl_Vtbl {
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastError(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppmapierror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, uluiparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&uluiparam)).into()
        }
        unsafe extern "system" fn GetState<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpulstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetState(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulstate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMAPIControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIFolder_Impl: Sized + IMAPIProp_Impl + IMAPIContainer_Impl {
    fn CreateMessage(&mut self, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppmessage: *mut ::core::option::Option<IMessage>) -> ::windows::core::Result<()>;
    fn CopyMessages(&mut self, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
    fn DeleteMessages(&mut self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
    fn CreateFolder(&mut self, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IMAPIFolder>;
    fn CopyFolder(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
    fn DeleteFolder(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
    fn SetReadFlags(&mut self, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
    fn GetMessageStatus(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<u32>;
    fn SetMessageStatus(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32) -> ::windows::core::Result<u32>;
    fn SaveContentsSort(&mut self, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows::core::Result<()>;
    fn EmptyFolder(&mut self, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMAPIFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>() -> IMAPIFolder_Vtbl {
        unsafe extern "system" fn CreateMessage<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateMessage(::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmessage)).into()
        }
        unsafe extern "system" fn CopyMessages<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyMessages(::core::mem::transmute_copy(&lpmsglist), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&lpdestfolder), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn DeleteMessages<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMessages(::core::mem::transmute_copy(&lpmsglist), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn CreateFolder<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFolder(::core::mem::transmute_copy(&ulfoldertype), ::core::mem::transmute_copy(&lpszfoldername), ::core::mem::transmute_copy(&lpszfoldercomment), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFolder<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyFolder(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&lpdestfolder), ::core::mem::transmute_copy(&lpsznewfoldername), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteFolder(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SetReadFlags<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReadFlags(::core::mem::transmute_copy(&lpmsglist), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetMessageStatus<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32, lpulmessagestatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMessageStatus(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpulmessagestatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageStatus<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32, lpuloldstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetMessageStatus(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulnewstatus), ::core::mem::transmute_copy(&ulnewstatusmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpuloldstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveContentsSort<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveContentsSort(::core::mem::transmute_copy(&lpsortcriteria), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn EmptyFolder<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EmptyFolder(::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base: IMAPIContainer_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateMessage: CreateMessage::<Identity, Impl, OFFSET>,
            CopyMessages: CopyMessages::<Identity, Impl, OFFSET>,
            DeleteMessages: DeleteMessages::<Identity, Impl, OFFSET>,
            CreateFolder: CreateFolder::<Identity, Impl, OFFSET>,
            CopyFolder: CopyFolder::<Identity, Impl, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, Impl, OFFSET>,
            SetReadFlags: SetReadFlags::<Identity, Impl, OFFSET>,
            GetMessageStatus: GetMessageStatus::<Identity, Impl, OFFSET>,
            SetMessageStatus: SetMessageStatus::<Identity, Impl, OFFSET>,
            SaveContentsSort: SaveContentsSort::<Identity, Impl, OFFSET>,
            EmptyFolder: EmptyFolder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMAPIFolder as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID || iid == &<IMAPIContainer as ::windows::core::Interface>::IID
    }
}
pub trait IMAPIProgress_Impl: Sized {
    fn Progress(&mut self, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self, lpulflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetMax(&mut self, lpulmax: *mut u32) -> ::windows::core::Result<()>;
    fn GetMin(&mut self, lpulmin: *mut u32) -> ::windows::core::Result<()>;
    fn SetLimits(&mut self, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows::core::Result<()>;
}
impl IMAPIProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProgress_Impl, const OFFSET: isize>() -> IMAPIProgress_Vtbl {
        unsafe extern "system" fn Progress<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Progress(::core::mem::transmute_copy(&ulvalue), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ultotal)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFlags(::core::mem::transmute_copy(&lpulflags)).into()
        }
        unsafe extern "system" fn GetMax<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMax(::core::mem::transmute_copy(&lpulmax)).into()
        }
        unsafe extern "system" fn GetMin<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulmin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMin(::core::mem::transmute_copy(&lpulmin)).into()
        }
        unsafe extern "system" fn SetLimits<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLimits(::core::mem::transmute_copy(&lpulmin), ::core::mem::transmute_copy(&lpulmax), ::core::mem::transmute_copy(&lpulflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Progress: Progress::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetMax: GetMax::<Identity, Impl, OFFSET>,
            GetMin: GetMin::<Identity, Impl, OFFSET>,
            SetLimits: SetLimits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMAPIProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIProp_Impl: Sized {
    fn GetLastError(&mut self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()>;
    fn SaveChanges(&mut self, ulflags: u32) -> ::windows::core::Result<()>;
    fn GetProps(&mut self, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::Result<()>;
    fn GetPropList(&mut self, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()>;
    fn OpenProperty(&mut self, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetProps(&mut self, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>;
    fn DeleteProps(&mut self, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>;
    fn CopyTo(&mut self, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>;
    fn CopyProps(&mut self, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>;
    fn GetNamesFromIDs(&mut self, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::Result<()>;
    fn GetIDsFromNames(&mut self, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMAPIProp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>() -> IMAPIProp_Vtbl {
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLastError(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmapierror)).into()
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveChanges(::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetProps<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProps(::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcvalues), ::core::mem::transmute_copy(&lppproparray)).into()
        }
        unsafe extern "system" fn GetPropList<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropList(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppproptagarray)).into()
        }
        unsafe extern "system" fn OpenProperty<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenProperty(::core::mem::transmute_copy(&ulproptag), ::core::mem::transmute_copy(&lpiid), ::core::mem::transmute_copy(&ulinterfaceoptions), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppunk)).into()
        }
        unsafe extern "system" fn SetProps<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProps(::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpproparray), ::core::mem::transmute_copy(&lppproblems)).into()
        }
        unsafe extern "system" fn DeleteProps<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteProps(::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&lppproblems)).into()
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyTo(::core::mem::transmute_copy(&ciidexclude), ::core::mem::transmute_copy(&rgiidexclude), ::core::mem::transmute_copy(&lpexcludeprops), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&lpdestobj), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppproblems)).into()
        }
        unsafe extern "system" fn CopyProps<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyProps(::core::mem::transmute_copy(&lpincludeprops), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&lpdestobj), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppproblems)).into()
        }
        unsafe extern "system" fn GetNamesFromIDs<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNamesFromIDs(::core::mem::transmute_copy(&lppproptags), ::core::mem::transmute_copy(&lppropsetguid), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcpropnames), ::core::mem::transmute_copy(&lppppropnames)).into()
        }
        unsafe extern "system" fn GetIDsFromNames<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIDsFromNames(::core::mem::transmute_copy(&cpropnames), ::core::mem::transmute_copy(&lpppropnames), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppproptags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            GetProps: GetProps::<Identity, Impl, OFFSET>,
            GetPropList: GetPropList::<Identity, Impl, OFFSET>,
            OpenProperty: OpenProperty::<Identity, Impl, OFFSET>,
            SetProps: SetProps::<Identity, Impl, OFFSET>,
            DeleteProps: DeleteProps::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            CopyProps: CopyProps::<Identity, Impl, OFFSET>,
            GetNamesFromIDs: GetNamesFromIDs::<Identity, Impl, OFFSET>,
            GetIDsFromNames: GetIDsFromNames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPIStatus_Impl: Sized + IMAPIProp_Impl {
    fn ValidateState(&mut self, uluiparam: usize, ulflags: u32) -> ::windows::core::Result<()>;
    fn SettingsDialog(&mut self, uluiparam: usize, ulflags: u32) -> ::windows::core::Result<()>;
    fn ChangePassword(&mut self, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows::core::Result<()>;
    fn FlushQueues(&mut self, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMAPIStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIStatus_Impl, const OFFSET: isize>() -> IMAPIStatus_Vtbl {
        unsafe extern "system" fn ValidateState<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ValidateState(::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SettingsDialog<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SettingsDialog(::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ChangePassword<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangePassword(::core::mem::transmute_copy(&lpoldpass), ::core::mem::transmute_copy(&lpnewpass), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn FlushQueues<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlushQueues(::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&cbtargettransport), ::core::mem::transmute_copy(&lptargettransport), ::core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>(),
            ValidateState: ValidateState::<Identity, Impl, OFFSET>,
            SettingsDialog: SettingsDialog::<Identity, Impl, OFFSET>,
            ChangePassword: ChangePassword::<Identity, Impl, OFFSET>,
            FlushQueues: FlushQueues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMAPIStatus as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMAPITable_Impl: Sized {
    fn GetLastError(&mut self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()>;
    fn Advise(&mut self, uleventmask: u32, lpadvisesink: &::core::option::Option<IMAPIAdviseSink>, lpulconnection: *mut u32) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self, ulconnection: u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows::core::Result<()>;
    fn SetColumns(&mut self, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows::core::Result<()>;
    fn QueryColumns(&mut self, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows::core::Result<()>;
    fn GetRowCount(&mut self, ulflags: u32, lpulcount: *mut u32) -> ::windows::core::Result<()>;
    fn SeekRow(&mut self, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows::core::Result<()>;
    fn SeekRowApprox(&mut self, ulnumerator: u32, uldenominator: u32) -> ::windows::core::Result<()>;
    fn QueryPosition(&mut self, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows::core::Result<()>;
    fn FindRow(&mut self, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows::core::Result<()>;
    fn Restrict(&mut self, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows::core::Result<()>;
    fn CreateBookmark(&mut self, lpbkposition: *mut u32) -> ::windows::core::Result<()>;
    fn FreeBookmark(&mut self, bkposition: u32) -> ::windows::core::Result<()>;
    fn SortTable(&mut self, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows::core::Result<()>;
    fn QuerySortOrder(&mut self, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows::core::Result<()>;
    fn QueryRows(&mut self, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
    fn ExpandRow(&mut self, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows::core::Result<()>;
    fn CollapseRow(&mut self, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows::core::Result<()>;
    fn WaitForCompletion(&mut self, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetCollapseState(&mut self, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetCollapseState(&mut self, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMAPITable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>() -> IMAPITable_Vtbl {
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLastError(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmapierror)).into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Advise(::core::mem::transmute_copy(&uleventmask), ::core::mem::transmute(&lpadvisesink), ::core::mem::transmute_copy(&lpulconnection)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&ulconnection)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&lpultablestatus), ::core::mem::transmute_copy(&lpultabletype)).into()
        }
        unsafe extern "system" fn SetColumns<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColumns(::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn QueryColumns<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryColumns(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpproptagarray)).into()
        }
        unsafe extern "system" fn GetRowCount<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRowCount(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulcount)).into()
        }
        unsafe extern "system" fn SeekRow<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeekRow(::core::mem::transmute_copy(&bkorigin), ::core::mem::transmute_copy(&lrowcount), ::core::mem::transmute_copy(&lplrowssought)).into()
        }
        unsafe extern "system" fn SeekRowApprox<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulnumerator: u32, uldenominator: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeekRowApprox(::core::mem::transmute_copy(&ulnumerator), ::core::mem::transmute_copy(&uldenominator)).into()
        }
        unsafe extern "system" fn QueryPosition<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryPosition(::core::mem::transmute_copy(&lpulrow), ::core::mem::transmute_copy(&lpulnumerator), ::core::mem::transmute_copy(&lpuldenominator)).into()
        }
        unsafe extern "system" fn FindRow<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindRow(::core::mem::transmute_copy(&lprestriction), ::core::mem::transmute_copy(&bkorigin), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn Restrict<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Restrict(::core::mem::transmute_copy(&lprestriction), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn CreateBookmark<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbkposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateBookmark(::core::mem::transmute_copy(&lpbkposition)).into()
        }
        unsafe extern "system" fn FreeBookmark<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bkposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreeBookmark(::core::mem::transmute_copy(&bkposition)).into()
        }
        unsafe extern "system" fn SortTable<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SortTable(::core::mem::transmute_copy(&lpsortcriteria), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn QuerySortOrder<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QuerySortOrder(::core::mem::transmute_copy(&lppsortcriteria)).into()
        }
        unsafe extern "system" fn QueryRows<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryRows(::core::mem::transmute_copy(&lrowcount), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpprows)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn ExpandRow<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExpandRow(::core::mem::transmute_copy(&cbinstancekey), ::core::mem::transmute_copy(&pbinstancekey), ::core::mem::transmute_copy(&ulrowcount), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpprows), ::core::mem::transmute_copy(&lpulmorerows)).into()
        }
        unsafe extern "system" fn CollapseRow<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CollapseRow(::core::mem::transmute_copy(&cbinstancekey), ::core::mem::transmute_copy(&pbinstancekey), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulrowcount)).into()
        }
        unsafe extern "system" fn WaitForCompletion<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForCompletion(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ultimeout), ::core::mem::transmute_copy(&lpultablestatus)).into()
        }
        unsafe extern "system" fn GetCollapseState<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCollapseState(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbinstancekey), ::core::mem::transmute_copy(&lpbinstancekey), ::core::mem::transmute_copy(&lpcbcollapsestate), ::core::mem::transmute_copy(&lppbcollapsestate)).into()
        }
        unsafe extern "system" fn SetCollapseState<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCollapseState(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbcollapsestate), ::core::mem::transmute_copy(&pbcollapsestate), ::core::mem::transmute_copy(&lpbklocation)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetColumns: SetColumns::<Identity, Impl, OFFSET>,
            QueryColumns: QueryColumns::<Identity, Impl, OFFSET>,
            GetRowCount: GetRowCount::<Identity, Impl, OFFSET>,
            SeekRow: SeekRow::<Identity, Impl, OFFSET>,
            SeekRowApprox: SeekRowApprox::<Identity, Impl, OFFSET>,
            QueryPosition: QueryPosition::<Identity, Impl, OFFSET>,
            FindRow: FindRow::<Identity, Impl, OFFSET>,
            Restrict: Restrict::<Identity, Impl, OFFSET>,
            CreateBookmark: CreateBookmark::<Identity, Impl, OFFSET>,
            FreeBookmark: FreeBookmark::<Identity, Impl, OFFSET>,
            SortTable: SortTable::<Identity, Impl, OFFSET>,
            QuerySortOrder: QuerySortOrder::<Identity, Impl, OFFSET>,
            QueryRows: QueryRows::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            ExpandRow: ExpandRow::<Identity, Impl, OFFSET>,
            CollapseRow: CollapseRow::<Identity, Impl, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, Impl, OFFSET>,
            GetCollapseState: GetCollapseState::<Identity, Impl, OFFSET>,
            SetCollapseState: SetCollapseState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMAPITable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMailUser_Impl: Sized + IMAPIProp_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMailUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMailUser_Impl, const OFFSET: isize>() -> IMailUser_Vtbl {
        Self { base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMailUser as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMessage_Impl: Sized + IMAPIProp_Impl {
    fn GetAttachmentTable(&mut self, ulflags: u32) -> ::windows::core::Result<IMAPITable>;
    fn OpenAttach(&mut self, ulattachmentnum: u32, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IAttach>;
    fn CreateAttach(&mut self, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut ::core::option::Option<IAttach>) -> ::windows::core::Result<()>;
    fn DeleteAttach(&mut self, ulattachmentnum: u32, uluiparam: usize, lpprogress: &::core::option::Option<IMAPIProgress>, ulflags: u32) -> ::windows::core::Result<()>;
    fn GetRecipientTable(&mut self, ulflags: u32) -> ::windows::core::Result<IMAPITable>;
    fn ModifyRecipients(&mut self, ulflags: u32, lpmods: *const ADRLIST) -> ::windows::core::Result<()>;
    fn SubmitMessage(&mut self, ulflags: u32) -> ::windows::core::Result<()>;
    fn SetReadFlag(&mut self, ulflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>() -> IMessage_Vtbl {
        unsafe extern "system" fn GetAttachmentTable<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttachmentTable(::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAttach<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulattachmentnum: u32, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppattach: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenAttach(::core::mem::transmute_copy(&ulattachmentnum), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppattach = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttach<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateAttach(::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulattachmentnum), ::core::mem::transmute_copy(&lppattach)).into()
        }
        unsafe extern "system" fn DeleteAttach<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulattachmentnum: u32, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAttach(::core::mem::transmute_copy(&ulattachmentnum), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute(&lpprogress), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetRecipientTable<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecipientTable(::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyRecipients<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpmods: *const ADRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModifyRecipients(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpmods)).into()
        }
        unsafe extern "system" fn SubmitMessage<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubmitMessage(::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SetReadFlag<Identity: ::windows::core::IUnknownImpl, Impl: IMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReadFlag(::core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAttachmentTable: GetAttachmentTable::<Identity, Impl, OFFSET>,
            OpenAttach: OpenAttach::<Identity, Impl, OFFSET>,
            CreateAttach: CreateAttach::<Identity, Impl, OFFSET>,
            DeleteAttach: DeleteAttach::<Identity, Impl, OFFSET>,
            GetRecipientTable: GetRecipientTable::<Identity, Impl, OFFSET>,
            ModifyRecipients: ModifyRecipients::<Identity, Impl, OFFSET>,
            SubmitMessage: SubmitMessage::<Identity, Impl, OFFSET>,
            SetReadFlag: SetReadFlag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessage as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMsgStore_Impl: Sized + IMAPIProp_Impl {
    fn Advise(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: &::core::option::Option<IMAPIAdviseSink>) -> ::windows::core::Result<u32>;
    fn Unadvise(&mut self, ulconnection: u32) -> ::windows::core::Result<()>;
    fn CompareEntryIDs(&mut self, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<u32>;
    fn OpenEntry(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetReceiveFolder(&mut self, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::Result<()>;
    fn GetReceiveFolder(&mut self, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows::core::Result<()>;
    fn GetReceiveFolderTable(&mut self, ulflags: u32) -> ::windows::core::Result<IMAPITable>;
    fn StoreLogoff(&mut self, lpulflags: *mut u32) -> ::windows::core::Result<()>;
    fn AbortSubmit(&mut self, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::Result<()>;
    fn GetOutgoingQueue(&mut self, ulflags: u32) -> ::windows::core::Result<IMAPITable>;
    fn SetLockState(&mut self, lpmessage: &::core::option::Option<IMessage>, ullockstate: u32) -> ::windows::core::Result<()>;
    fn FinishedMsg(&mut self, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::Result<()>;
    fn NotifyNewMail(&mut self, lpnotification: *const NOTIFICATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMsgStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>() -> IMsgStore_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Advise(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&uleventmask), ::core::mem::transmute(&lpadvisesink)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpulconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&ulconnection)).into()
        }
        unsafe extern "system" fn CompareEntryIDs<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompareEntryIDs(::core::mem::transmute_copy(&cbentryid1), ::core::mem::transmute_copy(&lpentryid1), ::core::mem::transmute_copy(&cbentryid2), ::core::mem::transmute_copy(&lpentryid2), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpulresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenEntry<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenEntry(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpulobjtype), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn SetReceiveFolder<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReceiveFolder(::core::mem::transmute_copy(&lpszmessageclass), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid)).into()
        }
        unsafe extern "system" fn GetReceiveFolder<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetReceiveFolder(::core::mem::transmute_copy(&lpszmessageclass), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid), ::core::mem::transmute_copy(&lppszexplicitclass)).into()
        }
        unsafe extern "system" fn GetReceiveFolderTable<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReceiveFolderTable(::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreLogoff<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StoreLogoff(::core::mem::transmute_copy(&lpulflags)).into()
        }
        unsafe extern "system" fn AbortSubmit<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbortSubmit(::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetOutgoingQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutgoingQueue(::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockState<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmessage: ::windows::core::RawPtr, ullockstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLockState(::core::mem::transmute(&lpmessage), ::core::mem::transmute_copy(&ullockstate)).into()
        }
        unsafe extern "system" fn FinishedMsg<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishedMsg(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cbentryid), ::core::mem::transmute_copy(&lpentryid)).into()
        }
        unsafe extern "system" fn NotifyNewMail<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnotification: *const NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyNewMail(::core::mem::transmute_copy(&lpnotification)).into()
        }
        Self {
            base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            CompareEntryIDs: CompareEntryIDs::<Identity, Impl, OFFSET>,
            OpenEntry: OpenEntry::<Identity, Impl, OFFSET>,
            SetReceiveFolder: SetReceiveFolder::<Identity, Impl, OFFSET>,
            GetReceiveFolder: GetReceiveFolder::<Identity, Impl, OFFSET>,
            GetReceiveFolderTable: GetReceiveFolderTable::<Identity, Impl, OFFSET>,
            StoreLogoff: StoreLogoff::<Identity, Impl, OFFSET>,
            AbortSubmit: AbortSubmit::<Identity, Impl, OFFSET>,
            GetOutgoingQueue: GetOutgoingQueue::<Identity, Impl, OFFSET>,
            SetLockState: SetLockState::<Identity, Impl, OFFSET>,
            FinishedMsg: FinishedMsg::<Identity, Impl, OFFSET>,
            NotifyNewMail: NotifyNewMail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMsgStore as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IProfSect_Impl: Sized + IMAPIProp_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IProfSect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProfSect_Impl, const OFFSET: isize>() -> IProfSect_Vtbl {
        Self { base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProfSect as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPropData_Impl: Sized + IMAPIProp_Impl {
    fn HrSetObjAccess(&mut self, ulaccess: u32) -> ::windows::core::Result<()>;
    fn HrSetPropAccess(&mut self, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows::core::Result<()>;
    fn HrGetPropAccess(&mut self, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows::core::Result<()>;
    fn HrAddObjProps(&mut self, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPropData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropData_Impl, const OFFSET: isize>() -> IPropData_Vtbl {
        unsafe extern "system" fn HrSetObjAccess<Identity: ::windows::core::IUnknownImpl, Impl: IPropData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrSetObjAccess(::core::mem::transmute_copy(&ulaccess)).into()
        }
        unsafe extern "system" fn HrSetPropAccess<Identity: ::windows::core::IUnknownImpl, Impl: IPropData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrSetPropAccess(::core::mem::transmute_copy(&lpproptagarray), ::core::mem::transmute_copy(&rgulaccess)).into()
        }
        unsafe extern "system" fn HrGetPropAccess<Identity: ::windows::core::IUnknownImpl, Impl: IPropData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrGetPropAccess(::core::mem::transmute_copy(&lppproptagarray), ::core::mem::transmute_copy(&lprgulaccess)).into()
        }
        unsafe extern "system" fn HrAddObjProps<Identity: ::windows::core::IUnknownImpl, Impl: IPropData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrAddObjProps(::core::mem::transmute_copy(&lppproptagarray), ::core::mem::transmute_copy(&lprgulaccess)).into()
        }
        Self {
            base: IMAPIProp_Vtbl::new::<Identity, Impl, OFFSET>(),
            HrSetObjAccess: HrSetObjAccess::<Identity, Impl, OFFSET>,
            HrSetPropAccess: HrSetPropAccess::<Identity, Impl, OFFSET>,
            HrGetPropAccess: HrGetPropAccess::<Identity, Impl, OFFSET>,
            HrAddObjProps: HrAddObjProps::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropData as ::windows::core::Interface>::IID || iid == &<IMAPIProp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IProviderAdmin_Impl: Sized {
    fn GetLastError(&mut self, hresult: ::windows::core::HRESULT, ulflags: u32) -> ::windows::core::Result<*mut MAPIERROR>;
    fn GetProviderTable(&mut self, ulflags: u32) -> ::windows::core::Result<IMAPITable>;
    fn CreateProvider(&mut self, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32) -> ::windows::core::Result<MAPIUID>;
    fn DeleteProvider(&mut self, lpuid: *const MAPIUID) -> ::windows::core::Result<()>;
    fn OpenProfileSection(&mut self, lpuid: *const MAPIUID, lpinterface: *const ::windows::core::GUID, ulflags: u32) -> ::windows::core::Result<IProfSect>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IProviderAdmin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderAdmin_Impl, const OFFSET: isize>() -> IProviderAdmin_Vtbl {
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl, Impl: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastError(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppmapierror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderTable<Identity: ::windows::core::IUnknownImpl, Impl: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderTable(::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpptable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProvider<Identity: ::windows::core::IUnknownImpl, Impl: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32, lpuid: *mut MAPIUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateProvider(::core::mem::transmute_copy(&lpszprovider), ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpprops), ::core::mem::transmute_copy(&uluiparam), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lpuid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProvider<Identity: ::windows::core::IUnknownImpl, Impl: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpuid: *const MAPIUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteProvider(::core::mem::transmute_copy(&lpuid)).into()
        }
        unsafe extern "system" fn OpenProfileSection<Identity: ::windows::core::IUnknownImpl, Impl: IProviderAdmin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpuid: *const MAPIUID, lpinterface: *const ::windows::core::GUID, ulflags: u32, lppprofsect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenProfileSection(::core::mem::transmute_copy(&lpuid), ::core::mem::transmute_copy(&lpinterface), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppprofsect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            GetProviderTable: GetProviderTable::<Identity, Impl, OFFSET>,
            CreateProvider: CreateProvider::<Identity, Impl, OFFSET>,
            DeleteProvider: DeleteProvider::<Identity, Impl, OFFSET>,
            OpenProfileSection: OpenProfileSection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderAdmin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITableData_Impl: Sized {
    fn HrGetView(&mut self, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut CALLERRELEASE, ulcallerdata: u32, lppmapitable: *mut ::core::option::Option<IMAPITable>) -> ::windows::core::Result<()>;
    fn HrModifyRow(&mut self, param0: *mut SRow) -> ::windows::core::Result<()>;
    fn HrDeleteRow(&mut self, lpspropvalue: *mut SPropValue) -> ::windows::core::Result<()>;
    fn HrQueryRow(&mut self, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows::core::Result<()>;
    fn HrEnumRow(&mut self, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows::core::Result<()>;
    fn HrNotify(&mut self, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows::core::Result<()>;
    fn HrInsertRow(&mut self, ulirow: u32, lpsrow: *mut SRow) -> ::windows::core::Result<()>;
    fn HrModifyRows(&mut self, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows::core::Result<()>;
    fn HrDeleteRows(&mut self, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITableData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>() -> ITableData_Vtbl {
        unsafe extern "system" fn HrGetView<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut ::windows::core::RawPtr, ulcallerdata: u32, lppmapitable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrGetView(::core::mem::transmute_copy(&lpssortorderset), ::core::mem::transmute_copy(&lpfcallerrelease), ::core::mem::transmute_copy(&ulcallerdata), ::core::mem::transmute_copy(&lppmapitable)).into()
        }
        unsafe extern "system" fn HrModifyRow<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut SRow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrModifyRow(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn HrDeleteRow<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpspropvalue: *mut SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrDeleteRow(::core::mem::transmute_copy(&lpspropvalue)).into()
        }
        unsafe extern "system" fn HrQueryRow<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrQueryRow(::core::mem::transmute_copy(&lpspropvalue), ::core::mem::transmute_copy(&lppsrow), ::core::mem::transmute_copy(&lpulirow)).into()
        }
        unsafe extern "system" fn HrEnumRow<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrEnumRow(::core::mem::transmute_copy(&ulrownumber), ::core::mem::transmute_copy(&lppsrow)).into()
        }
        unsafe extern "system" fn HrNotify<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrNotify(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpspropvalue)).into()
        }
        unsafe extern "system" fn HrInsertRow<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulirow: u32, lpsrow: *mut SRow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrInsertRow(::core::mem::transmute_copy(&ulirow), ::core::mem::transmute_copy(&lpsrow)).into()
        }
        unsafe extern "system" fn HrModifyRows<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrModifyRows(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpsrowset)).into()
        }
        unsafe extern "system" fn HrDeleteRows<Identity: ::windows::core::IUnknownImpl, Impl: ITableData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HrDeleteRows(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lprowsettodelete), ::core::mem::transmute_copy(&crowsdeleted)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            HrGetView: HrGetView::<Identity, Impl, OFFSET>,
            HrModifyRow: HrModifyRow::<Identity, Impl, OFFSET>,
            HrDeleteRow: HrDeleteRow::<Identity, Impl, OFFSET>,
            HrQueryRow: HrQueryRow::<Identity, Impl, OFFSET>,
            HrEnumRow: HrEnumRow::<Identity, Impl, OFFSET>,
            HrNotify: HrNotify::<Identity, Impl, OFFSET>,
            HrInsertRow: HrInsertRow::<Identity, Impl, OFFSET>,
            HrModifyRows: HrModifyRows::<Identity, Impl, OFFSET>,
            HrDeleteRows: HrDeleteRows::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITableData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWABExtInit_Impl: Sized {
    fn Initialize(&mut self, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWABExtInit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWABExtInit_Impl, const OFFSET: isize>() -> IWABExtInit_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWABExtInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&lpwabextdisplay)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWABExtInit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWABOBJECT__Impl: Sized {
    fn QueryInterface(&mut self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AddRef(&mut self) -> u32;
    fn Release(&mut self) -> u32;
    fn GetLastError(&mut self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()>;
    fn AllocateBuffer(&mut self, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateMore(&mut self, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FreeBuffer(&mut self, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Backup(&mut self, lpfilename: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn Import(&mut self, lpwip: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn Find(&mut self, lpiab: &::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn VCardDisplay(&mut self, lpiab: &::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn LDAPUrl(&mut self, lpiab: &::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR) -> ::windows::core::Result<IMailUser>;
    fn VCardCreate(&mut self, lpiab: &::core::option::Option<IAddrBook>, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: &::core::option::Option<IMailUser>) -> ::windows::core::Result<()>;
    fn VCardRetrieve(&mut self, lpiab: &::core::option::Option<IAddrBook>, ulflags: u32, lpszvcard: super::super::Foundation::PSTR) -> ::windows::core::Result<IMailUser>;
    fn GetMe(&mut self, lpiab: &::core::option::Option<IAddrBook>, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn SetMe(&mut self, lpiab: &::core::option::Option<IAddrBook>, ulflags: u32, sbeid: &SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWABOBJECT__Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>() -> IWABOBJECT__Vtbl {
        unsafe extern "system" fn QueryInterface<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryInterface(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn AddRef<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRef()
        }
        unsafe extern "system" fn Release<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Release()
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLastError(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmapierror)).into()
        }
        unsafe extern "system" fn AllocateBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateBuffer(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&lppbuffer)).into()
        }
        unsafe extern "system" fn AllocateMore<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateMore(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&lpobject), ::core::mem::transmute_copy(&lppbuffer)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreeBuffer(::core::mem::transmute_copy(&lpbuffer)).into()
        }
        unsafe extern "system" fn Backup<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&lpfilename)).into()
        }
        unsafe extern "system" fn Import<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwip: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&lpwip)).into()
        }
        unsafe extern "system" fn Find<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Find(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn VCardDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VCardDisplay(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&lpszfilename)).into()
        }
        unsafe extern "system" fn LDAPUrl<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LDAPUrl(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppmailuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardCreate<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VCardCreate(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpszvcard), ::core::mem::transmute(&lpmailuser)).into()
        }
        unsafe extern "system" fn VCardRetrieve<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VCardRetrieve(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpszvcard)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppmailuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMe<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMe(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpdwaction), ::core::mem::transmute_copy(&lpsbeid), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn SetMe<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT__Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMe(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&sbeid), ::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            QueryInterface: QueryInterface::<Identity, Impl, OFFSET>,
            AddRef: AddRef::<Identity, Impl, OFFSET>,
            Release: Release::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, Impl, OFFSET>,
            AllocateMore: AllocateMore::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            Find: Find::<Identity, Impl, OFFSET>,
            VCardDisplay: VCardDisplay::<Identity, Impl, OFFSET>,
            LDAPUrl: LDAPUrl::<Identity, Impl, OFFSET>,
            VCardCreate: VCardCreate::<Identity, Impl, OFFSET>,
            VCardRetrieve: VCardRetrieve::<Identity, Impl, OFFSET>,
            GetMe: GetMe::<Identity, Impl, OFFSET>,
            SetMe: SetMe::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWABOBJECT_ as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWABObject_Impl: Sized {
    fn GetLastError(&mut self, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::Result<()>;
    fn AllocateBuffer(&mut self, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateMore(&mut self, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FreeBuffer(&mut self, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Backup(&mut self, lpfilename: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn Import(&mut self, lpwip: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn Find(&mut self, lpiab: &::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn VCardDisplay(&mut self, lpiab: &::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn LDAPUrl(&mut self, lpiab: &::core::option::Option<IAddrBook>, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR) -> ::windows::core::Result<IMailUser>;
    fn VCardCreate(&mut self, lpiab: &::core::option::Option<IAddrBook>, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: &::core::option::Option<IMailUser>) -> ::windows::core::Result<()>;
    fn VCardRetrieve(&mut self, lpiab: &::core::option::Option<IAddrBook>, ulflags: u32, lpszvcard: super::super::Foundation::PSTR) -> ::windows::core::Result<IMailUser>;
    fn GetMe(&mut self, lpiab: &::core::option::Option<IAddrBook>, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn SetMe(&mut self, lpiab: &::core::option::Option<IAddrBook>, ulflags: u32, sbeid: &SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWABObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>() -> IWABObject_Vtbl {
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLastError(::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lppmapierror)).into()
        }
        unsafe extern "system" fn AllocateBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateBuffer(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&lppbuffer)).into()
        }
        unsafe extern "system" fn AllocateMore<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateMore(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&lpobject), ::core::mem::transmute_copy(&lppbuffer)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreeBuffer(::core::mem::transmute_copy(&lpbuffer)).into()
        }
        unsafe extern "system" fn Backup<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&lpfilename)).into()
        }
        unsafe extern "system" fn Import<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwip: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Import(::core::mem::transmute_copy(&lpwip)).into()
        }
        unsafe extern "system" fn Find<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Find(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn VCardDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VCardDisplay(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&lpszfilename)).into()
        }
        unsafe extern "system" fn LDAPUrl<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LDAPUrl(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppmailuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardCreate<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).VCardCreate(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpszvcard), ::core::mem::transmute(&lpmailuser)).into()
        }
        unsafe extern "system" fn VCardRetrieve<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VCardRetrieve(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpszvcard)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppmailuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMe<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMe(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpdwaction), ::core::mem::transmute_copy(&lpsbeid), ::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn SetMe<Identity: ::windows::core::IUnknownImpl, Impl: IWABObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMe(::core::mem::transmute(&lpiab), ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&sbeid), ::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, Impl, OFFSET>,
            AllocateMore: AllocateMore::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            Find: Find::<Identity, Impl, OFFSET>,
            VCardDisplay: VCardDisplay::<Identity, Impl, OFFSET>,
            LDAPUrl: LDAPUrl::<Identity, Impl, OFFSET>,
            VCardCreate: VCardCreate::<Identity, Impl, OFFSET>,
            VCardRetrieve: VCardRetrieve::<Identity, Impl, OFFSET>,
            GetMe: GetMe::<Identity, Impl, OFFSET>,
            SetMe: SetMe::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWABObject as ::windows::core::Interface>::IID
    }
}
