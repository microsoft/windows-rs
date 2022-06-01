pub trait ISceSvcAttachmentData_Impl: Sized {
    fn GetData(&self, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: &::core::option::Option<ISceSvcAttachmentPersistInfo>, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CloseHandle(&self, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISceSvcAttachmentData {}
impl ISceSvcAttachmentData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: isize>() -> ISceSvcAttachmentData_Vtbl {
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(::core::mem::transmute_copy(&scesvchandle), ::core::mem::transmute_copy(&scetype), ::core::mem::transmute_copy(&ppvdata), ::core::mem::transmute_copy(&psceenumhandle)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: *mut ::core::ffi::c_void, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&lpservicename), ::core::mem::transmute_copy(&lptemplatename), ::core::mem::transmute(&lpscesvcpersistinfo), ::core::mem::transmute_copy(&pscesvchandle)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeBuffer(::core::mem::transmute_copy(&pvdata)).into()
        }
        unsafe extern "system" fn CloseHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseHandle(::core::mem::transmute_copy(&scesvchandle)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetData: GetData::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            CloseHandle: CloseHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceSvcAttachmentData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISceSvcAttachmentPersistInfo_Impl: Sized {
    fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsDirty(&self, lptemplatename: *mut i8) -> ::windows::core::HRESULT;
    fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISceSvcAttachmentPersistInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ISceSvcAttachmentPersistInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: isize>() -> ISceSvcAttachmentPersistInfo_Vtbl {
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute_copy(&lptemplatename), ::core::mem::transmute_copy(&scesvchandle), ::core::mem::transmute_copy(&ppvdata), ::core::mem::transmute_copy(&pboverwriteall)).into()
        }
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirty(::core::mem::transmute_copy(&lptemplatename))
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISceSvcAttachmentPersistInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeBuffer(::core::mem::transmute_copy(&pvdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Save: Save::<Identity, Impl, OFFSET>,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceSvcAttachmentPersistInfo as ::windows::core::Interface>::IID
    }
}
