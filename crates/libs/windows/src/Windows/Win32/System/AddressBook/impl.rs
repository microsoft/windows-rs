pub trait IABContainerImpl: Sized + IMAPIContainerImpl + IMAPIPropImpl {
    fn CreateEntry();
    fn CopyEntries();
    fn DeleteEntries();
    fn ResolveNames();
}
impl ::windows::core::RuntimeName for IABContainer {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IABContainer";
}
impl IABContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IABContainerImpl, const OFFSET: isize>() -> IABContainerVtbl {
        unsafe extern "system" fn CreateEntry<Impl: IABContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEntry(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), ulcreateflags, ::core::mem::transmute_copy(&lppmapipropentry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyEntries<Impl: IABContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyEntries(&*(&lpentries as *const <SBinaryArray as ::windows::core::Abi>::Abi as *const <SBinaryArray as ::windows::core::DefaultType>::DefaultType), uluiparam, &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteEntries<Impl: IABContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteEntries(&*(&lpentries as *const <SBinaryArray as ::windows::core::Abi>::Abi as *const <SBinaryArray as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveNames<Impl: IABContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveNames(&*(&lpproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&lpadrlist as *const <ADRLIST as ::windows::core::Abi>::Abi as *const <ADRLIST as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lpflaglist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IABContainer>, ::windows::core::GetTrustLevel, CreateEntry::<Impl, OFFSET>, CopyEntries::<Impl, OFFSET>, DeleteEntries::<Impl, OFFSET>, ResolveNames::<Impl, OFFSET>)
    }
}
pub trait IAddrBookImpl: Sized + IMAPIPropImpl {
    fn OpenEntry();
    fn CompareEntryIDs();
    fn Advise();
    fn Unadvise();
    fn CreateOneOff();
    fn NewEntry();
    fn ResolveName();
    fn Address();
    fn Details();
    fn RecipOptions();
    fn QueryDefaultRecipOpt();
    fn GetPAB();
    fn SetPAB();
    fn GetDefaultDir();
    fn SetDefaultDir();
    fn GetSearchPath();
    fn SetSearchPath();
    fn PrepareRecips();
}
impl ::windows::core::RuntimeName for IAddrBook {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IAddrBook";
}
impl IAddrBookVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddrBookImpl, const OFFSET: isize>() -> IAddrBookVtbl {
        unsafe extern "system" fn OpenEntry<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenEntry(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, lpulobjtype, ::core::mem::transmute_copy(&lppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEntryIDs<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid1: u32, lpentryid1: *mut ENTRYID, cbentryid2: u32, lpentryid2: *mut ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEntryIDs(cbentryid1, &*(&lpentryid1 as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), cbentryid2, &*(&lpentryid2 as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), ulflags, lpulresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), uleventmask, &*(&lpadvisesink as *const <IMAPIAdviseSink as ::windows::core::Abi>::Abi as *const <IMAPIAdviseSink as ::windows::core::DefaultType>::DefaultType), lpulconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(ulconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOneOff<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszname: *mut i8, lpszadrtype: *mut i8, lpszaddress: *mut i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOneOff(lpszname, lpszadrtype, lpszaddress, ulflags, lpcbentryid, &*(&lppentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewEntry<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: u32, ulflags: u32, cbeidcontainer: u32, lpeidcontainer: *mut ENTRYID, cbeidnewentrytpl: u32, lpeidnewentrytpl: *mut ENTRYID, lpcbeidnewentry: *mut u32, lppeidnewentry: *mut *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewEntry(uluiparam, ulflags, cbeidcontainer, &*(&lpeidcontainer as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), cbeidnewentrytpl, &*(&lpeidnewentrytpl as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), lpcbeidnewentry, &*(&lppeidnewentry as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveName<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32, lpsznewentrytitle: *mut i8, lpadrlist: *mut ADRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveName(uluiparam, ulflags, lpsznewentrytitle, &*(&lpadrlist as *const <ADRLIST as ::windows::core::Abi>::Abi as *const <ADRLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpuluiparam: *mut u32, lpadrparms: *mut ADRPARM, lppadrlist: *mut *mut ADRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address(lpuluiparam, &*(&lpadrparms as *const <ADRPARM as ::windows::core::Abi>::Abi as *const <ADRPARM as ::windows::core::DefaultType>::DefaultType), &*(&lppadrlist as *const <ADRLIST as ::windows::core::Abi>::Abi as *const <ADRLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Details<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpuluiparam: *mut usize, lpfndismiss: ::windows::core::RawPtr, lpvdismisscontext: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID, lpfbuttoncallback: ::windows::core::RawPtr, lpvbuttoncontext: *mut ::core::ffi::c_void, lpszbuttontext: *mut i8, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Details(
                lpuluiparam,
                &*(&lpfndismiss as *const <LPFNDISMISS as ::windows::core::Abi>::Abi as *const <LPFNDISMISS as ::windows::core::DefaultType>::DefaultType),
                &*(&lpvdismisscontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                cbentryid,
                &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType),
                &*(&lpfbuttoncallback as *const <LPFNBUTTON as ::windows::core::Abi>::Abi as *const <LPFNBUTTON as ::windows::core::DefaultType>::DefaultType),
                &*(&lpvbuttoncontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                lpszbuttontext,
                ulflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecipOptions<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: u32, ulflags: u32, lprecip: *mut ADRENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecipOptions(uluiparam, ulflags, &*(&lprecip as *const <ADRENTRY as ::windows::core::Abi>::Abi as *const <ADRENTRY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDefaultRecipOpt<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszadrtype: *mut i8, ulflags: u32, lpcvalues: *mut u32, lppoptions: *mut *mut SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDefaultRecipOpt(lpszadrtype, ulflags, lpcvalues, &*(&lppoptions as *const <SPropValue as ::windows::core::Abi>::Abi as *const <SPropValue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPAB<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPAB(lpcbentryid, &*(&lppentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPAB<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPAB(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultDir<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultDir(lpcbentryid, &*(&lppentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultDir<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *mut ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultDir(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSearchPath<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lppsearchpath: *mut *mut SRowSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSearchPath(ulflags, &*(&lppsearchpath as *const <SRowSet as ::windows::core::Abi>::Abi as *const <SRowSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchPath<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpsearchpath: *mut SRowSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSearchPath(ulflags, &*(&lpsearchpath as *const <SRowSet as ::windows::core::Abi>::Abi as *const <SRowSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareRecips<Impl: IAddrBookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproptagarray: *mut SPropTagArray, lpreciplist: *mut ADRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareRecips(ulflags, &*(&lpproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), &*(&lpreciplist as *const <ADRLIST as ::windows::core::Abi>::Abi as *const <ADRLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAddrBook>,
            ::windows::core::GetTrustLevel,
            OpenEntry::<Impl, OFFSET>,
            CompareEntryIDs::<Impl, OFFSET>,
            Advise::<Impl, OFFSET>,
            Unadvise::<Impl, OFFSET>,
            CreateOneOff::<Impl, OFFSET>,
            NewEntry::<Impl, OFFSET>,
            ResolveName::<Impl, OFFSET>,
            Address::<Impl, OFFSET>,
            Details::<Impl, OFFSET>,
            RecipOptions::<Impl, OFFSET>,
            QueryDefaultRecipOpt::<Impl, OFFSET>,
            GetPAB::<Impl, OFFSET>,
            SetPAB::<Impl, OFFSET>,
            GetDefaultDir::<Impl, OFFSET>,
            SetDefaultDir::<Impl, OFFSET>,
            GetSearchPath::<Impl, OFFSET>,
            SetSearchPath::<Impl, OFFSET>,
            PrepareRecips::<Impl, OFFSET>,
        )
    }
}
pub trait IAttachImpl: Sized + IMAPIPropImpl {}
impl ::windows::core::RuntimeName for IAttach {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IAttach";
}
impl IAttachVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAttachImpl, const OFFSET: isize>() -> IAttachVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAttach>, ::windows::core::GetTrustLevel)
    }
}
pub trait IDistListImpl: Sized + IMAPIContainerImpl + IMAPIPropImpl {
    fn CreateEntry();
    fn CopyEntries();
    fn DeleteEntries();
    fn ResolveNames();
}
impl ::windows::core::RuntimeName for IDistList {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IDistList";
}
impl IDistListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDistListImpl, const OFFSET: isize>() -> IDistListVtbl {
        unsafe extern "system" fn CreateEntry<Impl: IDistListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulcreateflags: u32, lppmapipropentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEntry(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), ulcreateflags, ::core::mem::transmute_copy(&lppmapipropentry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyEntries<Impl: IDistListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyEntries(&*(&lpentries as *const <SBinaryArray as ::windows::core::Abi>::Abi as *const <SBinaryArray as ::windows::core::DefaultType>::DefaultType), uluiparam, &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteEntries<Impl: IDistListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpentries: *const SBinaryArray, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteEntries(&*(&lpentries as *const <SBinaryArray as ::windows::core::Abi>::Abi as *const <SBinaryArray as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveNames<Impl: IDistListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *const SPropTagArray, ulflags: u32, lpadrlist: *const ADRLIST, lpflaglist: *mut _flaglist) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveNames(&*(&lpproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&lpadrlist as *const <ADRLIST as ::windows::core::Abi>::Abi as *const <ADRLIST as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lpflaglist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDistList>, ::windows::core::GetTrustLevel, CreateEntry::<Impl, OFFSET>, CopyEntries::<Impl, OFFSET>, DeleteEntries::<Impl, OFFSET>, ResolveNames::<Impl, OFFSET>)
    }
}
pub trait IMAPIAdviseSinkImpl: Sized {
    fn OnNotify();
}
impl ::windows::core::RuntimeName for IMAPIAdviseSink {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMAPIAdviseSink";
}
impl IMAPIAdviseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIAdviseSinkImpl, const OFFSET: isize>() -> IMAPIAdviseSinkVtbl {
        unsafe extern "system" fn OnNotify<Impl: IMAPIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnotif: u32, lpnotifications: *mut NOTIFICATION) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNotify(cnotif, &*(&lpnotifications as *const <NOTIFICATION as ::windows::core::Abi>::Abi as *const <NOTIFICATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMAPIAdviseSink>, ::windows::core::GetTrustLevel, OnNotify::<Impl, OFFSET>)
    }
}
pub trait IMAPIContainerImpl: Sized + IMAPIPropImpl {
    fn GetContentsTable();
    fn GetHierarchyTable();
    fn OpenEntry();
    fn SetSearchCriteria();
    fn GetSearchCriteria();
}
impl ::windows::core::RuntimeName for IMAPIContainer {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMAPIContainer";
}
impl IMAPIContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIContainerImpl, const OFFSET: isize>() -> IMAPIContainerVtbl {
        unsafe extern "system" fn GetContentsTable<Impl: IMAPIContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentsTable(ulflags, ::core::mem::transmute_copy(&lpptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHierarchyTable<Impl: IMAPIContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHierarchyTable(ulflags, ::core::mem::transmute_copy(&lpptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenEntry<Impl: IMAPIContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenEntry(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lpulobjtype), ::core::mem::transmute_copy(&lppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchCriteria<Impl: IMAPIContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprestriction: *const SRestriction, lpcontainerlist: *const SBinaryArray, ulsearchflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSearchCriteria(&*(&lprestriction as *const <SRestriction as ::windows::core::Abi>::Abi as *const <SRestriction as ::windows::core::DefaultType>::DefaultType), &*(&lpcontainerlist as *const <SBinaryArray as ::windows::core::Abi>::Abi as *const <SBinaryArray as ::windows::core::DefaultType>::DefaultType), ulsearchflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSearchCriteria<Impl: IMAPIContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpprestriction: *mut *mut SRestriction, lppcontainerlist: *mut *mut SBinaryArray, lpulsearchstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSearchCriteria(ulflags, ::core::mem::transmute_copy(&lpprestriction), ::core::mem::transmute_copy(&lppcontainerlist), ::core::mem::transmute_copy(&lpulsearchstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMAPIContainer>, ::windows::core::GetTrustLevel, GetContentsTable::<Impl, OFFSET>, GetHierarchyTable::<Impl, OFFSET>, OpenEntry::<Impl, OFFSET>, SetSearchCriteria::<Impl, OFFSET>, GetSearchCriteria::<Impl, OFFSET>)
    }
}
pub trait IMAPIControlImpl: Sized {
    fn GetLastError();
    fn Activate();
    fn GetState();
}
impl ::windows::core::RuntimeName for IMAPIControl {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMAPIControl";
}
impl IMAPIControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIControlImpl, const OFFSET: isize>() -> IMAPIControlVtbl {
        unsafe extern "system" fn GetLastError<Impl: IMAPIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(hresult, ulflags, ::core::mem::transmute_copy(&lppmapierror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IMAPIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, uluiparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(ulflags, uluiparam) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: IMAPIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpulstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState(ulflags, lpulstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMAPIControl>, ::windows::core::GetTrustLevel, GetLastError::<Impl, OFFSET>, Activate::<Impl, OFFSET>, GetState::<Impl, OFFSET>)
    }
}
pub trait IMAPIFolderImpl: Sized + IMAPIContainerImpl + IMAPIPropImpl {
    fn CreateMessage();
    fn CopyMessages();
    fn DeleteMessages();
    fn CreateFolder();
    fn CopyFolder();
    fn DeleteFolder();
    fn SetReadFlags();
    fn GetMessageStatus();
    fn SetMessageStatus();
    fn SaveContentsSort();
    fn EmptyFolder();
}
impl ::windows::core::RuntimeName for IMAPIFolder {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMAPIFolder";
}
impl IMAPIFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIFolderImpl, const OFFSET: isize>() -> IMAPIFolderVtbl {
        unsafe extern "system" fn CreateMessage<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinterface: *mut ::windows::core::GUID, ulflags: u32, lppmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMessage(&*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lppmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyMessages<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, lpinterface: &::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyMessages(
                &*(&lpmsglist as *const <SBinaryArray as ::windows::core::Abi>::Abi as *const <SBinaryArray as ::windows::core::DefaultType>::DefaultType),
                &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&lpdestfolder as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                uluiparam,
                &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType),
                ulflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMessages<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMessages(&*(&lpmsglist as *const <SBinaryArray as ::windows::core::Abi>::Abi as *const <SBinaryArray as ::windows::core::DefaultType>::DefaultType), uluiparam, &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolder<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulfoldertype: u32, lpszfoldername: *const i8, lpszfoldercomment: *const i8, lpinterface: &::windows::core::GUID, ulflags: u32, lppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolder(ulfoldertype, lpszfoldername, lpszfoldercomment, &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lppfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFolder<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: &::windows::core::GUID, lpdestfolder: *const ::core::ffi::c_void, lpsznewfoldername: *const i8, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyFolder(
                cbentryid,
                &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType),
                &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&lpdestfolder as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                lpsznewfoldername,
                uluiparam,
                &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType),
                ulflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFolder<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteFolder(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), uluiparam, &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadFlags<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmsglist: *const SBinaryArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReadFlags(&*(&lpmsglist as *const <SBinaryArray as ::windows::core::Abi>::Abi as *const <SBinaryArray as ::windows::core::DefaultType>::DefaultType), uluiparam, &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageStatus<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32, lpulmessagestatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageStatus(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lpulmessagestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageStatus<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulnewstatus: u32, ulnewstatusmask: u32, lpuloldstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMessageStatus(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), ulnewstatus, ulnewstatusmask, ::core::mem::transmute_copy(&lpuloldstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveContentsSort<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsortcriteria: *const SSortOrderSet, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveContentsSort(&*(&lpsortcriteria as *const <SSortOrderSet as ::windows::core::Abi>::Abi as *const <SSortOrderSet as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmptyFolder<Impl: IMAPIFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmptyFolder(uluiparam, &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMAPIFolder>,
            ::windows::core::GetTrustLevel,
            CreateMessage::<Impl, OFFSET>,
            CopyMessages::<Impl, OFFSET>,
            DeleteMessages::<Impl, OFFSET>,
            CreateFolder::<Impl, OFFSET>,
            CopyFolder::<Impl, OFFSET>,
            DeleteFolder::<Impl, OFFSET>,
            SetReadFlags::<Impl, OFFSET>,
            GetMessageStatus::<Impl, OFFSET>,
            SetMessageStatus::<Impl, OFFSET>,
            SaveContentsSort::<Impl, OFFSET>,
            EmptyFolder::<Impl, OFFSET>,
        )
    }
}
pub trait IMAPIProgressImpl: Sized {
    fn Progress();
    fn GetFlags();
    fn GetMax();
    fn GetMin();
    fn SetLimits();
}
impl ::windows::core::RuntimeName for IMAPIProgress {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMAPIProgress";
}
impl IMAPIProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIProgressImpl, const OFFSET: isize>() -> IMAPIProgressVtbl {
        unsafe extern "system" fn Progress<Impl: IMAPIProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulvalue: u32, ulcount: u32, ultotal: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress(ulvalue, ulcount, ultotal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Impl: IMAPIProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags(lpulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMax<Impl: IMAPIProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMax(lpulmax) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMin<Impl: IMAPIProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulmin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMin(lpulmin) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLimits<Impl: IMAPIProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulmin: *mut u32, lpulmax: *mut u32, lpulflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLimits(lpulmin, lpulmax, lpulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMAPIProgress>, ::windows::core::GetTrustLevel, Progress::<Impl, OFFSET>, GetFlags::<Impl, OFFSET>, GetMax::<Impl, OFFSET>, GetMin::<Impl, OFFSET>, SetLimits::<Impl, OFFSET>)
    }
}
pub trait IMAPIPropImpl: Sized {
    fn GetLastError();
    fn SaveChanges();
    fn GetProps();
    fn GetPropList();
    fn OpenProperty();
    fn SetProps();
    fn DeleteProps();
    fn CopyTo();
    fn CopyProps();
    fn GetNamesFromIDs();
    fn GetIDsFromNames();
}
impl ::windows::core::RuntimeName for IMAPIProp {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMAPIProp";
}
impl IMAPIPropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIPropImpl, const OFFSET: isize>() -> IMAPIPropVtbl {
        unsafe extern "system" fn GetLastError<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(hresult, ulflags, &*(&lppmapierror as *const <MAPIERROR as ::windows::core::Abi>::Abi as *const <MAPIERROR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveChanges<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveChanges(ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProps<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32, lpcvalues: *mut u32, lppproparray: *mut *mut SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProps(&*(&lpproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), ulflags, lpcvalues, &*(&lppproparray as *const <SPropValue as ::windows::core::Abi>::Abi as *const <SPropValue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropList<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lppproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropList(ulflags, &*(&lppproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenProperty<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulproptag: u32, lpiid: *mut ::windows::core::GUID, ulinterfaceoptions: u32, ulflags: u32, lppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenProperty(ulproptag, &*(&lpiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulinterfaceoptions, ulflags, ::core::mem::transmute_copy(&lppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProps<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cvalues: u32, lpproparray: *mut SPropValue, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProps(cvalues, &*(&lpproparray as *const <SPropValue as ::windows::core::Abi>::Abi as *const <SPropValue as ::windows::core::DefaultType>::DefaultType), &*(&lppproblems as *const <SPropProblemArray as ::windows::core::Abi>::Abi as *const <SPropProblemArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProps<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteProps(&*(&lpproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), &*(&lppproblems as *const <SPropProblemArray as ::windows::core::Abi>::Abi as *const <SPropProblemArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *mut ::windows::core::GUID, lpexcludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTo(
                ciidexclude,
                &*(&rgiidexclude as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&lpexcludeprops as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType),
                uluiparam,
                &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType),
                &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&lpdestobj as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ulflags,
                &*(&lppproblems as *const <SPropProblemArray as ::windows::core::Abi>::Abi as *const <SPropProblemArray as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyProps<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpincludeprops: *mut SPropTagArray, uluiparam: usize, lpprogress: ::windows::core::RawPtr, lpinterface: *mut ::windows::core::GUID, lpdestobj: *mut ::core::ffi::c_void, ulflags: u32, lppproblems: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyProps(
                &*(&lpincludeprops as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType),
                uluiparam,
                &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType),
                &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&lpdestobj as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ulflags,
                &*(&lppproblems as *const <SPropProblemArray as ::windows::core::Abi>::Abi as *const <SPropProblemArray as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamesFromIDs<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppproptags: *mut *mut SPropTagArray, lppropsetguid: *mut ::windows::core::GUID, ulflags: u32, lpcpropnames: *mut u32, lppppropnames: *mut *mut *mut MAPINAMEID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamesFromIDs(&*(&lppproptags as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), &*(&lppropsetguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, lpcpropnames, &*(&lppppropnames as *const <MAPINAMEID as ::windows::core::Abi>::Abi as *const <MAPINAMEID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDsFromNames<Impl: IMAPIPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cpropnames: u32, lpppropnames: *mut *mut MAPINAMEID, ulflags: u32, lppproptags: *mut *mut SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIDsFromNames(cpropnames, &*(&lpppropnames as *const <MAPINAMEID as ::windows::core::Abi>::Abi as *const <MAPINAMEID as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&lppproptags as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMAPIProp>,
            ::windows::core::GetTrustLevel,
            GetLastError::<Impl, OFFSET>,
            SaveChanges::<Impl, OFFSET>,
            GetProps::<Impl, OFFSET>,
            GetPropList::<Impl, OFFSET>,
            OpenProperty::<Impl, OFFSET>,
            SetProps::<Impl, OFFSET>,
            DeleteProps::<Impl, OFFSET>,
            CopyTo::<Impl, OFFSET>,
            CopyProps::<Impl, OFFSET>,
            GetNamesFromIDs::<Impl, OFFSET>,
            GetIDsFromNames::<Impl, OFFSET>,
        )
    }
}
pub trait IMAPIStatusImpl: Sized + IMAPIPropImpl {
    fn ValidateState();
    fn SettingsDialog();
    fn ChangePassword();
    fn FlushQueues();
}
impl ::windows::core::RuntimeName for IMAPIStatus {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMAPIStatus";
}
impl IMAPIStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPIStatusImpl, const OFFSET: isize>() -> IMAPIStatusVtbl {
        unsafe extern "system" fn ValidateState<Impl: IMAPIStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateState(uluiparam, ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettingsDialog<Impl: IMAPIStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SettingsDialog(uluiparam, ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangePassword<Impl: IMAPIStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpoldpass: *const i8, lpnewpass: *const i8, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangePassword(lpoldpass, lpnewpass, ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushQueues<Impl: IMAPIStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uluiparam: usize, cbtargettransport: u32, lptargettransport: *const ENTRYID, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushQueues(uluiparam, cbtargettransport, &*(&lptargettransport as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMAPIStatus>, ::windows::core::GetTrustLevel, ValidateState::<Impl, OFFSET>, SettingsDialog::<Impl, OFFSET>, ChangePassword::<Impl, OFFSET>, FlushQueues::<Impl, OFFSET>)
    }
}
pub trait IMAPITableImpl: Sized {
    fn GetLastError();
    fn Advise();
    fn Unadvise();
    fn GetStatus();
    fn SetColumns();
    fn QueryColumns();
    fn GetRowCount();
    fn SeekRow();
    fn SeekRowApprox();
    fn QueryPosition();
    fn FindRow();
    fn Restrict();
    fn CreateBookmark();
    fn FreeBookmark();
    fn SortTable();
    fn QuerySortOrder();
    fn QueryRows();
    fn Abort();
    fn ExpandRow();
    fn CollapseRow();
    fn WaitForCompletion();
    fn GetCollapseState();
    fn SetCollapseState();
}
impl ::windows::core::RuntimeName for IMAPITable {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMAPITable";
}
impl IMAPITableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMAPITableImpl, const OFFSET: isize>() -> IMAPITableVtbl {
        unsafe extern "system" fn GetLastError<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(hresult, ulflags, &*(&lppmapierror as *const <MAPIERROR as ::windows::core::Abi>::Abi as *const <MAPIERROR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(uleventmask, &*(&lpadvisesink as *const <IMAPIAdviseSink as ::windows::core::Abi>::Abi as *const <IMAPIAdviseSink as ::windows::core::DefaultType>::DefaultType), lpulconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(ulconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpultablestatus: *mut u32, lpultabletype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(lpultablestatus, lpultabletype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColumns<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColumns(&*(&lpproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryColumns<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproptagarray: *mut *mut SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryColumns(ulflags, &*(&lpproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowCount<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRowCount(ulflags, lpulcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeekRow<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bkorigin: u32, lrowcount: i32, lplrowssought: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeekRow(bkorigin, lrowcount, lplrowssought) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeekRowApprox<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulnumerator: u32, uldenominator: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeekRowApprox(ulnumerator, uldenominator) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPosition<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulrow: *mut u32, lpulnumerator: *mut u32, lpuldenominator: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPosition(lpulrow, lpulnumerator, lpuldenominator) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindRow<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprestriction: *mut SRestriction, bkorigin: u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindRow(&*(&lprestriction as *const <SRestriction as ::windows::core::Abi>::Abi as *const <SRestriction as ::windows::core::DefaultType>::DefaultType), bkorigin, ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restrict<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprestriction: *mut SRestriction, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Restrict(&*(&lprestriction as *const <SRestriction as ::windows::core::Abi>::Abi as *const <SRestriction as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBookmark<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbkposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBookmark(lpbkposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBookmark<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bkposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeBookmark(bkposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SortTable<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsortcriteria: *mut SSortOrderSet, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SortTable(&*(&lpsortcriteria as *const <SSortOrderSet as ::windows::core::Abi>::Abi as *const <SSortOrderSet as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySortOrder<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppsortcriteria: *mut *mut SSortOrderSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySortOrder(&*(&lppsortcriteria as *const <SSortOrderSet as ::windows::core::Abi>::Abi as *const <SSortOrderSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryRows<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrowcount: i32, ulflags: u32, lpprows: *mut *mut SRowSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryRows(lrowcount, ulflags, &*(&lpprows as *const <SRowSet as ::windows::core::Abi>::Abi as *const <SRowSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandRow<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulrowcount: u32, ulflags: u32, lpprows: *mut *mut SRowSet, lpulmorerows: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandRow(cbinstancekey, pbinstancekey, ulrowcount, ulflags, &*(&lpprows as *const <SRowSet as ::windows::core::Abi>::Abi as *const <SRowSet as ::windows::core::DefaultType>::DefaultType), lpulmorerows) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollapseRow<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbinstancekey: u32, pbinstancekey: *mut u8, ulflags: u32, lpulrowcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CollapseRow(cbinstancekey, pbinstancekey, ulflags, lpulrowcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForCompletion<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ultimeout: u32, lpultablestatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForCompletion(ulflags, ultimeout, lpultablestatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollapseState<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbinstancekey: u32, lpbinstancekey: *mut u8, lpcbcollapsestate: *mut u32, lppbcollapsestate: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollapseState(ulflags, cbinstancekey, lpbinstancekey, lpcbcollapsestate, lppbcollapsestate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCollapseState<Impl: IMAPITableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbcollapsestate: u32, pbcollapsestate: *mut u8, lpbklocation: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCollapseState(ulflags, cbcollapsestate, pbcollapsestate, lpbklocation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMAPITable>,
            ::windows::core::GetTrustLevel,
            GetLastError::<Impl, OFFSET>,
            Advise::<Impl, OFFSET>,
            Unadvise::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            SetColumns::<Impl, OFFSET>,
            QueryColumns::<Impl, OFFSET>,
            GetRowCount::<Impl, OFFSET>,
            SeekRow::<Impl, OFFSET>,
            SeekRowApprox::<Impl, OFFSET>,
            QueryPosition::<Impl, OFFSET>,
            FindRow::<Impl, OFFSET>,
            Restrict::<Impl, OFFSET>,
            CreateBookmark::<Impl, OFFSET>,
            FreeBookmark::<Impl, OFFSET>,
            SortTable::<Impl, OFFSET>,
            QuerySortOrder::<Impl, OFFSET>,
            QueryRows::<Impl, OFFSET>,
            Abort::<Impl, OFFSET>,
            ExpandRow::<Impl, OFFSET>,
            CollapseRow::<Impl, OFFSET>,
            WaitForCompletion::<Impl, OFFSET>,
            GetCollapseState::<Impl, OFFSET>,
            SetCollapseState::<Impl, OFFSET>,
        )
    }
}
pub trait IMailUserImpl: Sized + IMAPIPropImpl {}
impl ::windows::core::RuntimeName for IMailUser {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMailUser";
}
impl IMailUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMailUserImpl, const OFFSET: isize>() -> IMailUserVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMailUser>, ::windows::core::GetTrustLevel)
    }
}
pub trait IMessageImpl: Sized + IMAPIPropImpl {
    fn GetAttachmentTable();
    fn OpenAttach();
    fn CreateAttach();
    fn DeleteAttach();
    fn GetRecipientTable();
    fn ModifyRecipients();
    fn SubmitMessage();
    fn SetReadFlag();
}
impl ::windows::core::RuntimeName for IMessage {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMessage";
}
impl IMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageImpl, const OFFSET: isize>() -> IMessageVtbl {
        unsafe extern "system" fn GetAttachmentTable<Impl: IMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttachmentTable(ulflags, ::core::mem::transmute_copy(&lpptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAttach<Impl: IMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulattachmentnum: u32, lpinterface: &::windows::core::GUID, ulflags: u32, lppattach: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAttach(ulattachmentnum, &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lppattach)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttach<Impl: IMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpinterface: &::windows::core::GUID, ulflags: u32, lpulattachmentnum: *mut u32, lppattach: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAttach(&*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lpulattachmentnum), ::core::mem::transmute_copy(&lppattach)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAttach<Impl: IMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulattachmentnum: u32, uluiparam: usize, lpprogress: ::windows::core::RawPtr, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAttach(ulattachmentnum, uluiparam, &*(&lpprogress as *const <IMAPIProgress as ::windows::core::Abi>::Abi as *const <IMAPIProgress as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecipientTable<Impl: IMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecipientTable(ulflags, ::core::mem::transmute_copy(&lpptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyRecipients<Impl: IMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpmods: *const ADRLIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyRecipients(ulflags, &*(&lpmods as *const <ADRLIST as ::windows::core::Abi>::Abi as *const <ADRLIST as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmitMessage<Impl: IMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmitMessage(ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadFlag<Impl: IMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReadFlag(ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMessage>,
            ::windows::core::GetTrustLevel,
            GetAttachmentTable::<Impl, OFFSET>,
            OpenAttach::<Impl, OFFSET>,
            CreateAttach::<Impl, OFFSET>,
            DeleteAttach::<Impl, OFFSET>,
            GetRecipientTable::<Impl, OFFSET>,
            ModifyRecipients::<Impl, OFFSET>,
            SubmitMessage::<Impl, OFFSET>,
            SetReadFlag::<Impl, OFFSET>,
        )
    }
}
pub trait IMsgStoreImpl: Sized + IMAPIPropImpl {
    fn Advise();
    fn Unadvise();
    fn CompareEntryIDs();
    fn OpenEntry();
    fn SetReceiveFolder();
    fn GetReceiveFolder();
    fn GetReceiveFolderTable();
    fn StoreLogoff();
    fn AbortSubmit();
    fn GetOutgoingQueue();
    fn SetLockState();
    fn FinishedMsg();
    fn NotifyNewMail();
}
impl ::windows::core::RuntimeName for IMsgStore {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IMsgStore";
}
impl IMsgStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMsgStoreImpl, const OFFSET: isize>() -> IMsgStoreVtbl {
        unsafe extern "system" fn Advise<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, uleventmask: u32, lpadvisesink: ::windows::core::RawPtr, lpulconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), uleventmask, &*(&lpadvisesink as *const <IMAPIAdviseSink as ::windows::core::Abi>::Abi as *const <IMAPIAdviseSink as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lpulconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(ulconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEntryIDs<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid1: u32, lpentryid1: *const ENTRYID, cbentryid2: u32, lpentryid2: *const ENTRYID, ulflags: u32, lpulresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEntryIDs(cbentryid1, &*(&lpentryid1 as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), cbentryid2, &*(&lpentryid2 as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lpulresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenEntry<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, lpinterface: &::windows::core::GUID, ulflags: u32, lpulobjtype: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenEntry(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lpulobjtype), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveFolder<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReceiveFolder(lpszmessageclass, ulflags, cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReceiveFolder<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszmessageclass: *const i8, ulflags: u32, lpcbentryid: *mut u32, lppentryid: *mut *mut ENTRYID, lppszexplicitclass: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReceiveFolder(lpszmessageclass, ulflags, ::core::mem::transmute_copy(&lpcbentryid), ::core::mem::transmute_copy(&lppentryid), ::core::mem::transmute_copy(&lppszexplicitclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReceiveFolderTable<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReceiveFolderTable(ulflags, ::core::mem::transmute_copy(&lpptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreLogoff<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpulflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StoreLogoff(lpulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortSubmit<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbentryid: u32, lpentryid: *const ENTRYID, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortSubmit(cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType), ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutgoingQueue<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutgoingQueue(ulflags, ::core::mem::transmute_copy(&lpptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLockState<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmessage: ::windows::core::RawPtr, ullockstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLockState(&*(&lpmessage as *const <IMessage as ::windows::core::Abi>::Abi as *const <IMessage as ::windows::core::DefaultType>::DefaultType), ullockstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishedMsg<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, cbentryid: u32, lpentryid: *const ENTRYID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishedMsg(ulflags, cbentryid, &*(&lpentryid as *const <ENTRYID as ::windows::core::Abi>::Abi as *const <ENTRYID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyNewMail<Impl: IMsgStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnotification: *const NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyNewMail(&*(&lpnotification as *const <NOTIFICATION as ::windows::core::Abi>::Abi as *const <NOTIFICATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMsgStore>,
            ::windows::core::GetTrustLevel,
            Advise::<Impl, OFFSET>,
            Unadvise::<Impl, OFFSET>,
            CompareEntryIDs::<Impl, OFFSET>,
            OpenEntry::<Impl, OFFSET>,
            SetReceiveFolder::<Impl, OFFSET>,
            GetReceiveFolder::<Impl, OFFSET>,
            GetReceiveFolderTable::<Impl, OFFSET>,
            StoreLogoff::<Impl, OFFSET>,
            AbortSubmit::<Impl, OFFSET>,
            GetOutgoingQueue::<Impl, OFFSET>,
            SetLockState::<Impl, OFFSET>,
            FinishedMsg::<Impl, OFFSET>,
            NotifyNewMail::<Impl, OFFSET>,
        )
    }
}
pub trait IProfSectImpl: Sized + IMAPIPropImpl {}
impl ::windows::core::RuntimeName for IProfSect {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IProfSect";
}
impl IProfSectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProfSectImpl, const OFFSET: isize>() -> IProfSectVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProfSect>, ::windows::core::GetTrustLevel)
    }
}
pub trait IPropDataImpl: Sized + IMAPIPropImpl {
    fn HrSetObjAccess();
    fn HrSetPropAccess();
    fn HrGetPropAccess();
    fn HrAddObjProps();
}
impl ::windows::core::RuntimeName for IPropData {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IPropData";
}
impl IPropDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropDataImpl, const OFFSET: isize>() -> IPropDataVtbl {
        unsafe extern "system" fn HrSetObjAccess<Impl: IPropDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrSetObjAccess(ulaccess) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrSetPropAccess<Impl: IPropDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpproptagarray: *mut SPropTagArray, rgulaccess: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrSetPropAccess(&*(&lpproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), rgulaccess) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrGetPropAccess<Impl: IPropDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppproptagarray: *mut *mut SPropTagArray, lprgulaccess: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrGetPropAccess(&*(&lppproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), lprgulaccess) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrAddObjProps<Impl: IPropDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lppproptagarray: *mut SPropTagArray, lprgulaccess: *mut *mut SPropProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrAddObjProps(&*(&lppproptagarray as *const <SPropTagArray as ::windows::core::Abi>::Abi as *const <SPropTagArray as ::windows::core::DefaultType>::DefaultType), &*(&lprgulaccess as *const <SPropProblemArray as ::windows::core::Abi>::Abi as *const <SPropProblemArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPropData>, ::windows::core::GetTrustLevel, HrSetObjAccess::<Impl, OFFSET>, HrSetPropAccess::<Impl, OFFSET>, HrGetPropAccess::<Impl, OFFSET>, HrAddObjProps::<Impl, OFFSET>)
    }
}
pub trait IProviderAdminImpl: Sized {
    fn GetLastError();
    fn GetProviderTable();
    fn CreateProvider();
    fn DeleteProvider();
    fn OpenProfileSection();
}
impl ::windows::core::RuntimeName for IProviderAdmin {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IProviderAdmin";
}
impl IProviderAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderAdminImpl, const OFFSET: isize>() -> IProviderAdminVtbl {
        unsafe extern "system" fn GetLastError<Impl: IProviderAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(hresult, ulflags, ::core::mem::transmute_copy(&lppmapierror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderTable<Impl: IProviderAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpptable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderTable(ulflags, ::core::mem::transmute_copy(&lpptable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProvider<Impl: IProviderAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszprovider: *const i8, cvalues: u32, lpprops: *const SPropValue, uluiparam: usize, ulflags: u32, lpuid: *mut MAPIUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProvider(lpszprovider, cvalues, &*(&lpprops as *const <SPropValue as ::windows::core::Abi>::Abi as *const <SPropValue as ::windows::core::DefaultType>::DefaultType), uluiparam, ulflags, ::core::mem::transmute_copy(&lpuid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProvider<Impl: IProviderAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpuid: *const MAPIUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteProvider(&*(&lpuid as *const <MAPIUID as ::windows::core::Abi>::Abi as *const <MAPIUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenProfileSection<Impl: IProviderAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpuid: *const MAPIUID, lpinterface: &::windows::core::GUID, ulflags: u32, lppprofsect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenProfileSection(&*(&lpuid as *const <MAPIUID as ::windows::core::Abi>::Abi as *const <MAPIUID as ::windows::core::DefaultType>::DefaultType), &*(&lpinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lppprofsect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProviderAdmin>, ::windows::core::GetTrustLevel, GetLastError::<Impl, OFFSET>, GetProviderTable::<Impl, OFFSET>, CreateProvider::<Impl, OFFSET>, DeleteProvider::<Impl, OFFSET>, OpenProfileSection::<Impl, OFFSET>)
    }
}
pub trait ITableDataImpl: Sized {
    fn HrGetView();
    fn HrModifyRow();
    fn HrDeleteRow();
    fn HrQueryRow();
    fn HrEnumRow();
    fn HrNotify();
    fn HrInsertRow();
    fn HrModifyRows();
    fn HrDeleteRows();
}
impl ::windows::core::RuntimeName for ITableData {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.ITableData";
}
impl ITableDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableDataImpl, const OFFSET: isize>() -> ITableDataVtbl {
        unsafe extern "system" fn HrGetView<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpssortorderset: *mut SSortOrderSet, lpfcallerrelease: *mut ::windows::core::RawPtr, ulcallerdata: u32, lppmapitable: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrGetView(&*(&lpssortorderset as *const <SSortOrderSet as ::windows::core::Abi>::Abi as *const <SSortOrderSet as ::windows::core::DefaultType>::DefaultType), &*(&lpfcallerrelease as *const <CALLERRELEASE as ::windows::core::Abi>::Abi as *const <CALLERRELEASE as ::windows::core::DefaultType>::DefaultType), ulcallerdata, ::core::mem::transmute_copy(&lppmapitable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrModifyRow<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut SRow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrModifyRow(&*(&param0 as *const <SRow as ::windows::core::Abi>::Abi as *const <SRow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrDeleteRow<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpspropvalue: *mut SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrDeleteRow(&*(&lpspropvalue as *const <SPropValue as ::windows::core::Abi>::Abi as *const <SPropValue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrQueryRow<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpspropvalue: *mut SPropValue, lppsrow: *mut *mut SRow, lpulirow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrQueryRow(&*(&lpspropvalue as *const <SPropValue as ::windows::core::Abi>::Abi as *const <SPropValue as ::windows::core::DefaultType>::DefaultType), &*(&lppsrow as *const <SRow as ::windows::core::Abi>::Abi as *const <SRow as ::windows::core::DefaultType>::DefaultType), lpulirow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrEnumRow<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulrownumber: u32, lppsrow: *mut *mut SRow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrEnumRow(ulrownumber, &*(&lppsrow as *const <SRow as ::windows::core::Abi>::Abi as *const <SRow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrNotify<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, cvalues: u32, lpspropvalue: *mut SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrNotify(ulflags, cvalues, &*(&lpspropvalue as *const <SPropValue as ::windows::core::Abi>::Abi as *const <SPropValue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrInsertRow<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulirow: u32, lpsrow: *mut SRow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrInsertRow(ulirow, &*(&lpsrow as *const <SRow as ::windows::core::Abi>::Abi as *const <SRow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrModifyRows<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpsrowset: *mut SRowSet) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrModifyRows(ulflags, &*(&lpsrowset as *const <SRowSet as ::windows::core::Abi>::Abi as *const <SRowSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrDeleteRows<Impl: ITableDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprowsettodelete: *mut SRowSet, crowsdeleted: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrDeleteRows(ulflags, &*(&lprowsettodelete as *const <SRowSet as ::windows::core::Abi>::Abi as *const <SRowSet as ::windows::core::DefaultType>::DefaultType), crowsdeleted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITableData>,
            ::windows::core::GetTrustLevel,
            HrGetView::<Impl, OFFSET>,
            HrModifyRow::<Impl, OFFSET>,
            HrDeleteRow::<Impl, OFFSET>,
            HrQueryRow::<Impl, OFFSET>,
            HrEnumRow::<Impl, OFFSET>,
            HrNotify::<Impl, OFFSET>,
            HrInsertRow::<Impl, OFFSET>,
            HrModifyRows::<Impl, OFFSET>,
            HrDeleteRows::<Impl, OFFSET>,
        )
    }
}
pub trait IWABExtInitImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IWABExtInit {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IWABExtInit";
}
impl IWABExtInitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWABExtInitImpl, const OFFSET: isize>() -> IWABExtInitVtbl {
        unsafe extern "system" fn Initialize<Impl: IWABExtInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwabextdisplay: *mut WABEXTDISPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&lpwabextdisplay as *const <WABEXTDISPLAY as ::windows::core::Abi>::Abi as *const <WABEXTDISPLAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWABExtInit>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>)
    }
}
pub trait IWABOBJECT_Impl: Sized {
    fn QueryInterface();
    fn AddRef();
    fn Release();
    fn GetLastError();
    fn AllocateBuffer();
    fn AllocateMore();
    fn FreeBuffer();
    fn Backup();
    fn Import();
    fn Find();
    fn VCardDisplay();
    fn LDAPUrl();
    fn VCardCreate();
    fn VCardRetrieve();
    fn GetMe();
    fn SetMe();
}
impl ::windows::core::RuntimeName for IWABOBJECT_ {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IWABOBJECT_";
}
impl IWABOBJECT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWABOBJECT_Impl, const OFFSET: isize>() -> IWABOBJECT_Vtbl {
        unsafe extern "system" fn QueryInterface<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInterface(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppvobj as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRef<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRef() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Release<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Release() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastError<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(hresult, ulflags, &*(&lppmapierror as *const <MAPIERROR as ::windows::core::Abi>::Abi as *const <MAPIERROR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateBuffer<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateBuffer(cbsize, ::core::mem::transmute_copy(&lppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateMore<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateMore(cbsize, &*(&lpobject as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeBuffer(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Backup<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Backup(&*(&lpfilename as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Import<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwip: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Import(&*(&lpwip as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Find<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Find(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardDisplay<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VCardDisplay(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&lpszfilename as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LDAPUrl<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LDAPUrl(
                &*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                ulflags,
                &*(&lpszurl as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&lppmailuser),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardCreate<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VCardCreate(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&lpszvcard as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&lpmailuser as *const <IMailUser as ::windows::core::Abi>::Abi as *const <IMailUser as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardRetrieve<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VCardRetrieve(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&lpszvcard as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lppmailuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMe<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMe(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lpdwaction), ::core::mem::transmute_copy(&lpsbeid), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMe<Impl: IWABOBJECT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMe(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&sbeid as *const <SBinary as ::windows::core::Abi>::Abi as *const <SBinary as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWABOBJECT_>,
            ::windows::core::GetTrustLevel,
            QueryInterface::<Impl, OFFSET>,
            AddRef::<Impl, OFFSET>,
            Release::<Impl, OFFSET>,
            GetLastError::<Impl, OFFSET>,
            AllocateBuffer::<Impl, OFFSET>,
            AllocateMore::<Impl, OFFSET>,
            FreeBuffer::<Impl, OFFSET>,
            Backup::<Impl, OFFSET>,
            Import::<Impl, OFFSET>,
            Find::<Impl, OFFSET>,
            VCardDisplay::<Impl, OFFSET>,
            LDAPUrl::<Impl, OFFSET>,
            VCardCreate::<Impl, OFFSET>,
            VCardRetrieve::<Impl, OFFSET>,
            GetMe::<Impl, OFFSET>,
            SetMe::<Impl, OFFSET>,
        )
    }
}
pub trait IWABObjectImpl: Sized {
    fn GetLastError();
    fn AllocateBuffer();
    fn AllocateMore();
    fn FreeBuffer();
    fn Backup();
    fn Import();
    fn Find();
    fn VCardDisplay();
    fn LDAPUrl();
    fn VCardCreate();
    fn VCardRetrieve();
    fn GetMe();
    fn SetMe();
}
impl ::windows::core::RuntimeName for IWABObject {
    const NAME: &'static str = "Windows.Win32.System.AddressBook.IWABObject";
}
impl IWABObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWABObjectImpl, const OFFSET: isize>() -> IWABObjectVtbl {
        unsafe extern "system" fn GetLastError<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(hresult, ulflags, &*(&lppmapierror as *const <MAPIERROR as ::windows::core::Abi>::Abi as *const <MAPIERROR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateBuffer<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateBuffer(cbsize, ::core::mem::transmute_copy(&lppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateMore<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateMore(cbsize, &*(&lpobject as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeBuffer(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Backup<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Backup(&*(&lpfilename as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Import<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpwip: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Import(&*(&lpwip as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Find<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Find(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardDisplay<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VCardDisplay(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&lpszfilename as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LDAPUrl<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LDAPUrl(
                &*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                ulflags,
                &*(&lpszurl as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&lppmailuser),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardCreate<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VCardCreate(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&lpszvcard as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&lpmailuser as *const <IMailUser as ::windows::core::Abi>::Abi as *const <IMailUser as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VCardRetrieve<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lppmailuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VCardRetrieve(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&lpszvcard as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&lppmailuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMe<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMe(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lpdwaction), ::core::mem::transmute_copy(&lpsbeid), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMe<Impl: IWABObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiab: ::windows::core::RawPtr, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMe(&*(&lpiab as *const <IAddrBook as ::windows::core::Abi>::Abi as *const <IAddrBook as ::windows::core::DefaultType>::DefaultType), ulflags, &*(&sbeid as *const <SBinary as ::windows::core::Abi>::Abi as *const <SBinary as ::windows::core::DefaultType>::DefaultType), &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWABObject>,
            ::windows::core::GetTrustLevel,
            GetLastError::<Impl, OFFSET>,
            AllocateBuffer::<Impl, OFFSET>,
            AllocateMore::<Impl, OFFSET>,
            FreeBuffer::<Impl, OFFSET>,
            Backup::<Impl, OFFSET>,
            Import::<Impl, OFFSET>,
            Find::<Impl, OFFSET>,
            VCardDisplay::<Impl, OFFSET>,
            LDAPUrl::<Impl, OFFSET>,
            VCardCreate::<Impl, OFFSET>,
            VCardRetrieve::<Impl, OFFSET>,
            GetMe::<Impl, OFFSET>,
            SetMe::<Impl, OFFSET>,
        )
    }
}
