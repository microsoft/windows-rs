pub trait ISceSvcAttachmentDataImpl: Sized {
    fn GetData();
    fn Initialize();
    fn FreeBuffer();
    fn CloseHandle();
}
impl ISceSvcAttachmentDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceSvcAttachmentDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceSvcAttachmentDataVtbl {
        unsafe extern "system" fn GetData<Impl: ISceSvcAttachmentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: ISceSvcAttachmentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: ::windows::core::RawPtr, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeBuffer<Impl: ISceSvcAttachmentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseHandle<Impl: ISceSvcAttachmentDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetData::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, FreeBuffer::<Impl, IMPL_OFFSET>, CloseHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceSvcAttachmentData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISceSvcAttachmentPersistInfoImpl: Sized {
    fn Save();
    fn IsDirty();
    fn FreeBuffer();
}
#[cfg(feature = "Win32_Foundation")]
impl ISceSvcAttachmentPersistInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceSvcAttachmentPersistInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceSvcAttachmentPersistInfoVtbl {
        unsafe extern "system" fn Save<Impl: ISceSvcAttachmentPersistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDirty<Impl: ISceSvcAttachmentPersistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeBuffer<Impl: ISceSvcAttachmentPersistInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Save::<Impl, IMPL_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, FreeBuffer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceSvcAttachmentPersistInfo as ::windows::core::Interface>::IID
    }
}
