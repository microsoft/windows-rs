#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDynamicPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RemoteHost(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExternalPort(&self) -> ::windows::core::Result<i32>;
    fn Protocol(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InternalPort(&self) -> ::windows::core::Result<i32>;
    fn InternalClient(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LeaseDuration(&self) -> ::windows::core::Result<i32>;
    fn RenewLease(&self, lleasedurationdesired: i32) -> ::windows::core::Result<i32>;
    fn EditInternalClient(&self, bstrinternalclient: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enable(&self, vb: i16) -> ::windows::core::Result<()>;
    fn EditDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EditInternalPort(&self, linternalport: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDynamicPortMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>() -> IDynamicPortMapping_Vtbl {
        unsafe extern "system" fn ExternalIPAddress<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExternalIPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteHost<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteHost() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InternalClient() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseDuration<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LeaseDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewLease<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RenewLease(::core::mem::transmute_copy(&lleasedurationdesired)) {
                ::core::result::Result::Ok(ok__) => {
                    *pleasedurationreturned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EditInternalClient(::core::mem::transmute(&bstrinternalclient)).into()
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&vb)).into()
        }
        unsafe extern "system" fn EditDescription<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EditDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn EditInternalPort<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EditInternalPort(::core::mem::transmute_copy(&linternalport)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Identity, Impl, OFFSET>,
            RemoteHost: RemoteHost::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            InternalClient: InternalClient::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            LeaseDuration: LeaseDuration::<Identity, Impl, OFFSET>,
            RenewLease: RenewLease::<Identity, Impl, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            EditDescription: EditDescription::<Identity, Impl, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDynamicPortMapping as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDynamicPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, bstrremotehost: &super::super::Foundation::BSTR, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR) -> ::windows::core::Result<IDynamicPortMapping>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Remove(&self, bstrremotehost: &super::super::Foundation::BSTR, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Add(&self, bstrremotehost: &super::super::Foundation::BSTR, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR, linternalport: i32, bstrinternalclient: &super::super::Foundation::BSTR, benabled: i16, bstrdescription: &super::super::Foundation::BSTR, lleaseduration: i32) -> ::windows::core::Result<IDynamicPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDynamicPortMappingCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>() -> IDynamicPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdpm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lleaseduration: i32, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Add(::core::mem::transmute(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&linternalport), ::core::mem::transmute(&bstrinternalclient), ::core::mem::transmute_copy(&benabled), ::core::mem::transmute(&bstrdescription), ::core::mem::transmute_copy(&lleaseduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdpm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDynamicPortMappingCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetConnection_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetConnection>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetConnection>;
}
impl IEnumNetConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetConnection_Impl, const OFFSET: isize>() -> IEnumNetConnection_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingEveryConnection_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingEveryConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingEveryConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>() -> IEnumNetSharingEveryConnection_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingEveryConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPortMapping_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPortMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>() -> IEnumNetSharingPortMapping_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPortMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPrivateConnection_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPrivateConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPrivateConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>() -> IEnumNetSharingPrivateConnection_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPrivateConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPublicConnection_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetSharingPublicConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPublicConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>() -> IEnumNetSharingPublicConnection_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPublicConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INATEventManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetExternalIPAddressCallback(&self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetNumberOfEntriesCallback(&self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INATEventManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATEventManager_Impl, const OFFSET: isize>() -> INATEventManager_Vtbl {
        unsafe extern "system" fn SetExternalIPAddressCallback<Identity: ::windows::core::IUnknownImpl, Impl: INATEventManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExternalIPAddressCallback(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn SetNumberOfEntriesCallback<Identity: ::windows::core::IUnknownImpl, Impl: INATEventManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumberOfEntriesCallback(::core::mem::transmute(&punk)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetExternalIPAddressCallback: SetExternalIPAddressCallback::<Identity, Impl, OFFSET>,
            SetNumberOfEntriesCallback: SetNumberOfEntriesCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATEventManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INATExternalIPAddressCallback_Impl: Sized {
    fn NewExternalIPAddress(&self, bstrnewexternalipaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INATExternalIPAddressCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATExternalIPAddressCallback_Impl, const OFFSET: isize>() -> INATExternalIPAddressCallback_Vtbl {
        unsafe extern "system" fn NewExternalIPAddress<Identity: ::windows::core::IUnknownImpl, Impl: INATExternalIPAddressCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewexternalipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NewExternalIPAddress(::core::mem::transmute(&bstrnewexternalipaddress)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), NewExternalIPAddress: NewExternalIPAddress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATExternalIPAddressCallback as ::windows::core::Interface>::IID
    }
}
pub trait INATNumberOfEntriesCallback_Impl: Sized {
    fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> ::windows::core::Result<()>;
}
impl INATNumberOfEntriesCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATNumberOfEntriesCallback_Impl, const OFFSET: isize>() -> INATNumberOfEntriesCallback_Vtbl {
        unsafe extern "system" fn NewNumberOfEntries<Identity: ::windows::core::IUnknownImpl, Impl: INATNumberOfEntriesCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewnumberofentries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NewNumberOfEntries(::core::mem::transmute_copy(&lnewnumberofentries)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), NewNumberOfEntries: NewNumberOfEntries::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATNumberOfEntriesCallback as ::windows::core::Interface>::IID
    }
}
pub trait INetConnection_Impl: Sized {
    fn Connect(&self) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Duplicate(&self, pszwduplicatename: &::windows::core::PCWSTR) -> ::windows::core::Result<INetConnection>;
    fn GetProperties(&self) -> ::windows::core::Result<*mut NETCON_PROPERTIES>;
    fn GetUiObjectClassId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Rename(&self, pszwnewname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl INetConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const OFFSET: isize>() -> INetConnection_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Duplicate<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwduplicatename: ::windows::core::PCWSTR, ppcon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Duplicate(::core::mem::transmute(&pszwduplicatename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiObjectClassId<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUiObjectClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwnewname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rename(::core::mem::transmute(&pszwnewname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Duplicate: Duplicate::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetUiObjectClassId: GetUiObjectClassId::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetConnectionConnectUi_Impl: Sized {
    fn SetConnection(&self, pcon: &::core::option::Option<INetConnection>) -> ::windows::core::Result<()>;
    fn Connect(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::Result<()>;
    fn Disconnect(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetConnectionConnectUi_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>() -> INetConnectionConnectUi_Vtbl {
        unsafe extern "system" fn SetConnection<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnection(::core::mem::transmute(&pcon)).into()
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetConnection: SetConnection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionConnectUi as ::windows::core::Interface>::IID
    }
}
pub trait INetConnectionManager_Impl: Sized {
    fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> ::windows::core::Result<IEnumNetConnection>;
}
impl INetConnectionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionManager_Impl, const OFFSET: isize>() -> INetConnectionManager_Vtbl {
        unsafe extern "system" fn EnumConnections<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumConnections(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), EnumConnections: EnumConnections::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetConnectionProps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Guid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Status(&self) -> ::windows::core::Result<NETCON_STATUS>;
    fn MediaType(&self) -> ::windows::core::Result<NETCON_MEDIATYPE>;
    fn Characteristics(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetConnectionProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionProps_Impl, const OFFSET: isize>() -> INetConnectionProps_Vtbl {
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Characteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Guid: Guid::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Characteristics: Characteristics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionProps as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwAuthorizedApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProcessImageFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProcessImageFileName(&self, imagefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()>;
    fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwAuthorizedApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>() -> INetFwAuthorizedApplication_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn ProcessImageFileName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProcessImageFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *imagefilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessImageFileName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProcessImageFileName(::core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn IpVersion<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IpVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *ipversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIpVersion(::core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            ProcessImageFileName: ProcessImageFileName::<Identity, Impl, OFFSET>,
            SetProcessImageFileName: SetProcessImageFileName::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplication as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwAuthorizedApplications_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, app: &::core::option::Option<INetFwAuthorizedApplication>) -> ::windows::core::Result<()>;
    fn Remove(&self, imagefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Item(&self, imagefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<INetFwAuthorizedApplication>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwAuthorizedApplications_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>() -> INetFwAuthorizedApplications_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, app: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&app)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, app: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute(&imagefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *app = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplications as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwIcmpSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AllowOutboundDestinationUnreachable(&self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundDestinationUnreachable(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowRedirect(&self) -> ::windows::core::Result<i16>;
    fn SetAllowRedirect(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowInboundEchoRequest(&self) -> ::windows::core::Result<i16>;
    fn SetAllowInboundEchoRequest(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowOutboundTimeExceeded(&self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundTimeExceeded(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowOutboundParameterProblem(&self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundParameterProblem(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowOutboundSourceQuench(&self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundSourceQuench(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowInboundRouterRequest(&self) -> ::windows::core::Result<i16>;
    fn SetAllowInboundRouterRequest(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowInboundTimestampRequest(&self) -> ::windows::core::Result<i16>;
    fn SetAllowInboundTimestampRequest(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowInboundMaskRequest(&self) -> ::windows::core::Result<i16>;
    fn SetAllowInboundMaskRequest(&self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowOutboundPacketTooBig(&self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundPacketTooBig(&self, allow: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwIcmpSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>() -> INetFwIcmpSettings_Vtbl {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowOutboundDestinationUnreachable() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowOutboundDestinationUnreachable(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowRedirect<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowRedirect() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowRedirect<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowRedirect(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowInboundEchoRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowInboundEchoRequest(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowOutboundTimeExceeded() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowOutboundTimeExceeded(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowOutboundParameterProblem() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowOutboundParameterProblem(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowOutboundSourceQuench() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowOutboundSourceQuench(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowInboundRouterRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowInboundRouterRequest(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowInboundTimestampRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowInboundTimestampRequest(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowInboundMaskRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowInboundMaskRequest(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowOutboundPacketTooBig() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowOutboundPacketTooBig(::core::mem::transmute_copy(&allow)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AllowOutboundDestinationUnreachable: AllowOutboundDestinationUnreachable::<Identity, Impl, OFFSET>,
            SetAllowOutboundDestinationUnreachable: SetAllowOutboundDestinationUnreachable::<Identity, Impl, OFFSET>,
            AllowRedirect: AllowRedirect::<Identity, Impl, OFFSET>,
            SetAllowRedirect: SetAllowRedirect::<Identity, Impl, OFFSET>,
            AllowInboundEchoRequest: AllowInboundEchoRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundEchoRequest: SetAllowInboundEchoRequest::<Identity, Impl, OFFSET>,
            AllowOutboundTimeExceeded: AllowOutboundTimeExceeded::<Identity, Impl, OFFSET>,
            SetAllowOutboundTimeExceeded: SetAllowOutboundTimeExceeded::<Identity, Impl, OFFSET>,
            AllowOutboundParameterProblem: AllowOutboundParameterProblem::<Identity, Impl, OFFSET>,
            SetAllowOutboundParameterProblem: SetAllowOutboundParameterProblem::<Identity, Impl, OFFSET>,
            AllowOutboundSourceQuench: AllowOutboundSourceQuench::<Identity, Impl, OFFSET>,
            SetAllowOutboundSourceQuench: SetAllowOutboundSourceQuench::<Identity, Impl, OFFSET>,
            AllowInboundRouterRequest: AllowInboundRouterRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundRouterRequest: SetAllowInboundRouterRequest::<Identity, Impl, OFFSET>,
            AllowInboundTimestampRequest: AllowInboundTimestampRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundTimestampRequest: SetAllowInboundTimestampRequest::<Identity, Impl, OFFSET>,
            AllowInboundMaskRequest: AllowInboundMaskRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundMaskRequest: SetAllowInboundMaskRequest::<Identity, Impl, OFFSET>,
            AllowOutboundPacketTooBig: AllowOutboundPacketTooBig::<Identity, Impl, OFFSET>,
            SetAllowOutboundPacketTooBig: SetAllowOutboundPacketTooBig::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwIcmpSettings as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwMgr_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LocalPolicy(&self) -> ::windows::core::Result<INetFwPolicy>;
    fn CurrentProfileType(&self) -> ::windows::core::Result<NET_FW_PROFILE_TYPE>;
    fn RestoreDefaults(&self) -> ::windows::core::Result<()>;
    fn IsPortAllowed(&self, imagefilename: &super::super::Foundation::BSTR, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: &super::super::Foundation::BSTR, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsIcmpTypeAllowed(&self, ipversion: NET_FW_IP_VERSION, localaddress: &super::super::Foundation::BSTR, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgr_Impl, const OFFSET: isize>() -> INetFwMgr_Vtbl {
        unsafe extern "system" fn LocalPolicy<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *localpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfileType<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentProfileType() {
                ::core::result::Result::Ok(ok__) => {
                    *profiletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDefaults<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreDefaults().into()
        }
        unsafe extern "system" fn IsPortAllowed<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPortAllowed(::core::mem::transmute(&imagefilename), ::core::mem::transmute_copy(&ipversion), ::core::mem::transmute_copy(&portnumber), ::core::mem::transmute(&localaddress), ::core::mem::transmute_copy(&ipprotocol), ::core::mem::transmute_copy(&allowed), ::core::mem::transmute_copy(&restricted)).into()
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsIcmpTypeAllowed(::core::mem::transmute_copy(&ipversion), ::core::mem::transmute(&localaddress), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&allowed), ::core::mem::transmute_copy(&restricted)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LocalPolicy: LocalPolicy::<Identity, Impl, OFFSET>,
            CurrentProfileType: CurrentProfileType::<Identity, Impl, OFFSET>,
            RestoreDefaults: RestoreDefaults::<Identity, Impl, OFFSET>,
            IsPortAllowed: IsPortAllowed::<Identity, Impl, OFFSET>,
            IsIcmpTypeAllowed: IsIcmpTypeAllowed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwMgr as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwOpenPort_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()>;
    fn Protocol(&self) -> ::windows::core::Result<NET_FW_IP_PROTOCOL>;
    fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<()>;
    fn Port(&self) -> ::windows::core::Result<i32>;
    fn SetPort(&self, portnumber: i32) -> ::windows::core::Result<()>;
    fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn BuiltIn(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwOpenPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>() -> INetFwOpenPort_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn IpVersion<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IpVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *ipversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIpVersion(::core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *ipprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProtocol(::core::mem::transmute_copy(&ipprotocol)).into()
        }
        unsafe extern "system" fn Port<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Port() {
                ::core::result::Result::Ok(ok__) => {
                    *portnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPort(::core::mem::transmute_copy(&portnumber)).into()
        }
        unsafe extern "system" fn Scope<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn BuiltIn<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, builtin: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BuiltIn() {
                ::core::result::Result::Ok(ok__) => {
                    *builtin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            SetProtocol: SetProtocol::<Identity, Impl, OFFSET>,
            Port: Port::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            BuiltIn: BuiltIn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwOpenPort as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwOpenPorts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, port: &::core::option::Option<INetFwOpenPort>) -> ::windows::core::Result<()>;
    fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<()>;
    fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<INetFwOpenPort>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwOpenPorts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>() -> INetFwOpenPorts_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, port: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&port)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&portnumber), ::core::mem::transmute_copy(&ipprotocol)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&portnumber), ::core::mem::transmute_copy(&ipprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *openport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwOpenPorts as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwPolicy_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentProfile(&self) -> ::windows::core::Result<INetFwProfile>;
    fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> ::windows::core::Result<INetFwProfile>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy_Impl, const OFFSET: isize>() -> INetFwPolicy_Vtbl {
        unsafe extern "system" fn CurrentProfile<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *profile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileByType<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProfileByType(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *profile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentProfile: CurrentProfile::<Identity, Impl, OFFSET>,
            GetProfileByType: GetProfileByType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwPolicy as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwPolicy2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentProfileTypes(&self) -> ::windows::core::Result<i32>;
    fn FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16>;
    fn SetFirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::Result<()>;
    fn ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16>;
    fn SetBlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::Result<()>;
    fn NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16>;
    fn SetNotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::Result<()>;
    fn Rules(&self) -> ::windows::core::Result<INetFwRules>;
    fn ServiceRestriction(&self) -> ::windows::core::Result<INetFwServiceRestriction>;
    fn EnableRuleGroup(&self, profiletypesbitmask: i32, group: &super::super::Foundation::BSTR, enable: i16) -> ::windows::core::Result<()>;
    fn IsRuleGroupEnabled(&self, profiletypesbitmask: i32, group: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn RestoreLocalFirewallDefaults(&self) -> ::windows::core::Result<()>;
    fn DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<NET_FW_ACTION>;
    fn SetDefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::Result<()>;
    fn DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<NET_FW_ACTION>;
    fn SetDefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::Result<()>;
    fn IsRuleGroupCurrentlyEnabled(&self, group: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn LocalPolicyModifyState(&self) -> ::windows::core::Result<NET_FW_MODIFY_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwPolicy2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>() -> INetFwPolicy2_Vtbl {
        unsafe extern "system" fn CurrentProfileTypes<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentProfileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *profiletypesbitmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirewallEnabled(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFirewallEnabled(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn ExcludedInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExcludedInterfaces(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *interfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludedInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExcludedInterfaces(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute(&interfaces)).into()
        }
        unsafe extern "system" fn BlockAllInboundTraffic<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BlockAllInboundTraffic(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *block = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAllInboundTraffic<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlockAllInboundTraffic(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&block)).into()
        }
        unsafe extern "system" fn NotificationsDisabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NotificationsDisabled(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *disabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotificationsDisabled(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnicastResponsesToMulticastBroadcastDisabled(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *disabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnicastResponsesToMulticastBroadcastDisabled(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn Rules<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rules() {
                ::core::result::Result::Ok(ok__) => {
                    *rules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceRestriction<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicerestriction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceRestriction() {
                ::core::result::Result::Ok(ok__) => {
                    *servicerestriction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableRuleGroup<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableRuleGroup(::core::mem::transmute_copy(&profiletypesbitmask), ::core::mem::transmute(&group), ::core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRuleGroupEnabled(::core::mem::transmute_copy(&profiletypesbitmask), ::core::mem::transmute(&group)) {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreLocalFirewallDefaults().into()
        }
        unsafe extern "system" fn DefaultInboundAction<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultInboundAction(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInboundAction<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultInboundAction(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn DefaultOutboundAction<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultOutboundAction(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultOutboundAction<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultOutboundAction(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn IsRuleGroupCurrentlyEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRuleGroupCurrentlyEnabled(::core::mem::transmute(&group)) {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPolicyModifyState<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalPolicyModifyState() {
                ::core::result::Result::Ok(ok__) => {
                    *modifystate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentProfileTypes: CurrentProfileTypes::<Identity, Impl, OFFSET>,
            FirewallEnabled: FirewallEnabled::<Identity, Impl, OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Identity, Impl, OFFSET>,
            ExcludedInterfaces: ExcludedInterfaces::<Identity, Impl, OFFSET>,
            SetExcludedInterfaces: SetExcludedInterfaces::<Identity, Impl, OFFSET>,
            BlockAllInboundTraffic: BlockAllInboundTraffic::<Identity, Impl, OFFSET>,
            SetBlockAllInboundTraffic: SetBlockAllInboundTraffic::<Identity, Impl, OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Identity, Impl, OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Identity, Impl, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
            ServiceRestriction: ServiceRestriction::<Identity, Impl, OFFSET>,
            EnableRuleGroup: EnableRuleGroup::<Identity, Impl, OFFSET>,
            IsRuleGroupEnabled: IsRuleGroupEnabled::<Identity, Impl, OFFSET>,
            RestoreLocalFirewallDefaults: RestoreLocalFirewallDefaults::<Identity, Impl, OFFSET>,
            DefaultInboundAction: DefaultInboundAction::<Identity, Impl, OFFSET>,
            SetDefaultInboundAction: SetDefaultInboundAction::<Identity, Impl, OFFSET>,
            DefaultOutboundAction: DefaultOutboundAction::<Identity, Impl, OFFSET>,
            SetDefaultOutboundAction: SetDefaultOutboundAction::<Identity, Impl, OFFSET>,
            IsRuleGroupCurrentlyEnabled: IsRuleGroupCurrentlyEnabled::<Identity, Impl, OFFSET>,
            LocalPolicyModifyState: LocalPolicyModifyState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwPolicy2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProduct_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RuleCategories(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetRuleCategories(&self, rulecategories: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&self, displayname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathToSignedProductExe(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProduct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProduct_Impl, const OFFSET: isize>() -> INetFwProduct_Vtbl {
        unsafe extern "system" fn RuleCategories<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulecategories: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RuleCategories() {
                ::core::result::Result::Ok(ok__) => {
                    *rulecategories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleCategories<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulecategories: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRuleCategories(::core::mem::transmute(&rulecategories)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *displayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute(&displayname)).into()
        }
        unsafe extern "system" fn PathToSignedProductExe<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathToSignedProductExe() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RuleCategories: RuleCategories::<Identity, Impl, OFFSET>,
            SetRuleCategories: SetRuleCategories::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            PathToSignedProductExe: PathToSignedProductExe::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProduct as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProducts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Register(&self, product: &::core::option::Option<INetFwProduct>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, index: i32) -> ::windows::core::Result<INetFwProduct>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProducts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProducts_Impl, const OFFSET: isize>() -> INetFwProducts_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, product: ::windows::core::RawPtr, registration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Register(::core::mem::transmute(&product)) {
                ::core::result::Result::Ok(ok__) => {
                    *registration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, product: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *product = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProducts as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProfile_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows::core::Result<NET_FW_PROFILE_TYPE>;
    fn FirewallEnabled(&self) -> ::windows::core::Result<i16>;
    fn SetFirewallEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn ExceptionsNotAllowed(&self) -> ::windows::core::Result<i16>;
    fn SetExceptionsNotAllowed(&self, notallowed: i16) -> ::windows::core::Result<()>;
    fn NotificationsDisabled(&self) -> ::windows::core::Result<i16>;
    fn SetNotificationsDisabled(&self, disabled: i16) -> ::windows::core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> ::windows::core::Result<i16>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: i16) -> ::windows::core::Result<()>;
    fn RemoteAdminSettings(&self) -> ::windows::core::Result<INetFwRemoteAdminSettings>;
    fn IcmpSettings(&self) -> ::windows::core::Result<INetFwIcmpSettings>;
    fn GloballyOpenPorts(&self) -> ::windows::core::Result<INetFwOpenPorts>;
    fn Services(&self) -> ::windows::core::Result<INetFwServices>;
    fn AuthorizedApplications(&self) -> ::windows::core::Result<INetFwAuthorizedApplications>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>() -> INetFwProfile_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirewallEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFirewallEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExceptionsNotAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *notallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notallowed: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExceptionsNotAllowed(::core::mem::transmute_copy(&notallowed)).into()
        }
        unsafe extern "system" fn NotificationsDisabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NotificationsDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *disabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotificationsDisabled(::core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnicastResponsesToMulticastBroadcastDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *disabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnicastResponsesToMulticastBroadcastDisabled(::core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn RemoteAdminSettings<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteadminsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteAdminSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteadminsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IcmpSettings<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmpsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IcmpSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *icmpsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GloballyOpenPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *openports = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, services: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Services() {
                ::core::result::Result::Ok(ok__) => {
                    *services = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthorizedApplications<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthorizedApplications() {
                ::core::result::Result::Ok(ok__) => {
                    *apps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            FirewallEnabled: FirewallEnabled::<Identity, Impl, OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Identity, Impl, OFFSET>,
            ExceptionsNotAllowed: ExceptionsNotAllowed::<Identity, Impl, OFFSET>,
            SetExceptionsNotAllowed: SetExceptionsNotAllowed::<Identity, Impl, OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Identity, Impl, OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Identity, Impl, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            RemoteAdminSettings: RemoteAdminSettings::<Identity, Impl, OFFSET>,
            IcmpSettings: IcmpSettings::<Identity, Impl, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, Impl, OFFSET>,
            Services: Services::<Identity, Impl, OFFSET>,
            AuthorizedApplications: AuthorizedApplications::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProfile as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRemoteAdminSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()>;
    fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRemoteAdminSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>() -> INetFwRemoteAdminSettings_Vtbl {
        unsafe extern "system" fn IpVersion<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IpVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *ipversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIpVersion(::core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRemoteAdminSettings as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, desc: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationName(&self, imagefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceName(&self, servicename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Protocol(&self) -> ::windows::core::Result<i32>;
    fn SetProtocol(&self, protocol: i32) -> ::windows::core::Result<()>;
    fn LocalPorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalPorts(&self, portnumbers: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemotePorts(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemotePorts(&self, portnumbers: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalAddresses(&self, localaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IcmpTypesAndCodes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetIcmpTypesAndCodes(&self, icmptypesandcodes: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<NET_FW_RULE_DIRECTION>;
    fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::Result<()>;
    fn Interfaces(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetInterfaces(&self, interfaces: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InterfaceTypes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInterfaceTypes(&self, interfacetypes: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn Grouping(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetGrouping(&self, context: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Profiles(&self) -> ::windows::core::Result<i32>;
    fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows::core::Result<()>;
    fn EdgeTraversal(&self) -> ::windows::core::Result<i16>;
    fn SetEdgeTraversal(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn Action(&self) -> ::windows::core::Result<NET_FW_ACTION>;
    fn SetAction(&self, action: NET_FW_ACTION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>() -> INetFwRule_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *desc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute(&desc)).into()
        }
        unsafe extern "system" fn ApplicationName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationName() {
                ::core::result::Result::Ok(ok__) => {
                    *imagefilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationName(::core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn ServiceName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *servicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceName<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceName(::core::mem::transmute(&servicename)).into()
        }
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *protocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProtocol(::core::mem::transmute_copy(&protocol)).into()
        }
        unsafe extern "system" fn LocalPorts<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *portnumbers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPorts<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalPorts(::core::mem::transmute(&portnumbers)).into()
        }
        unsafe extern "system" fn RemotePorts<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemotePorts() {
                ::core::result::Result::Ok(ok__) => {
                    *portnumbers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemotePorts<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemotePorts(::core::mem::transmute(&portnumbers)).into()
        }
        unsafe extern "system" fn LocalAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *localaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalAddresses(::core::mem::transmute(&localaddrs)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IcmpTypesAndCodes() {
                ::core::result::Result::Ok(ok__) => {
                    *icmptypesandcodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIcmpTypesAndCodes(::core::mem::transmute(&icmptypesandcodes)).into()
        }
        unsafe extern "system" fn Direction<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *dir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDirection(::core::mem::transmute_copy(&dir)).into()
        }
        unsafe extern "system" fn Interfaces<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Interfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *interfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterfaces(::core::mem::transmute(&interfaces)).into()
        }
        unsafe extern "system" fn InterfaceTypes<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacetypes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InterfaceTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *interfacetypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceTypes<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterfaceTypes(::core::mem::transmute(&interfacetypes)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Grouping<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Grouping() {
                ::core::result::Result::Ok(ok__) => {
                    *context = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGrouping<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGrouping(::core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Profiles<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Profiles() {
                ::core::result::Result::Ok(ok__) => {
                    *profiletypesbitmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfiles<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProfiles(::core::mem::transmute_copy(&profiletypesbitmask)).into()
        }
        unsafe extern "system" fn EdgeTraversal<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EdgeTraversal() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversal<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEdgeTraversal(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Action<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Action() {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAction<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAction(::core::mem::transmute_copy(&action)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationName: ApplicationName::<Identity, Impl, OFFSET>,
            SetApplicationName: SetApplicationName::<Identity, Impl, OFFSET>,
            ServiceName: ServiceName::<Identity, Impl, OFFSET>,
            SetServiceName: SetServiceName::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            SetProtocol: SetProtocol::<Identity, Impl, OFFSET>,
            LocalPorts: LocalPorts::<Identity, Impl, OFFSET>,
            SetLocalPorts: SetLocalPorts::<Identity, Impl, OFFSET>,
            RemotePorts: RemotePorts::<Identity, Impl, OFFSET>,
            SetRemotePorts: SetRemotePorts::<Identity, Impl, OFFSET>,
            LocalAddresses: LocalAddresses::<Identity, Impl, OFFSET>,
            SetLocalAddresses: SetLocalAddresses::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            IcmpTypesAndCodes: IcmpTypesAndCodes::<Identity, Impl, OFFSET>,
            SetIcmpTypesAndCodes: SetIcmpTypesAndCodes::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            SetDirection: SetDirection::<Identity, Impl, OFFSET>,
            Interfaces: Interfaces::<Identity, Impl, OFFSET>,
            SetInterfaces: SetInterfaces::<Identity, Impl, OFFSET>,
            InterfaceTypes: InterfaceTypes::<Identity, Impl, OFFSET>,
            SetInterfaceTypes: SetInterfaceTypes::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Grouping: Grouping::<Identity, Impl, OFFSET>,
            SetGrouping: SetGrouping::<Identity, Impl, OFFSET>,
            Profiles: Profiles::<Identity, Impl, OFFSET>,
            SetProfiles: SetProfiles::<Identity, Impl, OFFSET>,
            EdgeTraversal: EdgeTraversal::<Identity, Impl, OFFSET>,
            SetEdgeTraversal: SetEdgeTraversal::<Identity, Impl, OFFSET>,
            Action: Action::<Identity, Impl, OFFSET>,
            SetAction: SetAction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRule2_Impl: Sized + super::super::System::Com::IDispatch_Impl + INetFwRule_Impl {
    fn EdgeTraversalOptions(&self) -> ::windows::core::Result<i32>;
    fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRule2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule2_Impl, const OFFSET: isize>() -> INetFwRule2_Vtbl {
        unsafe extern "system" fn EdgeTraversalOptions<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EdgeTraversalOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *loptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEdgeTraversalOptions(::core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base: INetFwRule_Vtbl::new::<Identity, Impl, OFFSET>(),
            EdgeTraversalOptions: EdgeTraversalOptions::<Identity, Impl, OFFSET>,
            SetEdgeTraversalOptions: SetEdgeTraversalOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<INetFwRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRule3_Impl: Sized + super::super::System::Com::IDispatch_Impl + INetFwRule_Impl + INetFwRule2_Impl {
    fn LocalAppPackageId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalAppPackageId(&self, wszpackageid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalUserOwner(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalUserOwner(&self, wszuserowner: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalUserAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalUserAuthorizedList(&self, wszuserauthlist: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoteUserAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteUserAuthorizedList(&self, wszuserauthlist: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoteMachineAuthorizedList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteMachineAuthorizedList(&self, wszuserauthlist: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SecureFlags(&self) -> ::windows::core::Result<i32>;
    fn SetSecureFlags(&self, loptions: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRule3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>() -> INetFwRule3_Vtbl {
        unsafe extern "system" fn LocalAppPackageId<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpackageid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalAppPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    *wszpackageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAppPackageId<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalAppPackageId(::core::mem::transmute(&wszpackageid)).into()
        }
        unsafe extern "system" fn LocalUserOwner<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserowner: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalUserOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *wszuserowner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserOwner<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalUserOwner(::core::mem::transmute(&wszuserowner)).into()
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalUserAuthorizedList() {
                ::core::result::Result::Ok(ok__) => {
                    *wszuserauthlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalUserAuthorizedList(::core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteUserAuthorizedList() {
                ::core::result::Result::Ok(ok__) => {
                    *wszuserauthlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteUserAuthorizedList(::core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteMachineAuthorizedList() {
                ::core::result::Result::Ok(ok__) => {
                    *wszuserauthlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteMachineAuthorizedList(::core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn SecureFlags<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SecureFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *loptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecureFlags<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecureFlags(::core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base: INetFwRule2_Vtbl::new::<Identity, Impl, OFFSET>(),
            LocalAppPackageId: LocalAppPackageId::<Identity, Impl, OFFSET>,
            SetLocalAppPackageId: SetLocalAppPackageId::<Identity, Impl, OFFSET>,
            LocalUserOwner: LocalUserOwner::<Identity, Impl, OFFSET>,
            SetLocalUserOwner: SetLocalUserOwner::<Identity, Impl, OFFSET>,
            LocalUserAuthorizedList: LocalUserAuthorizedList::<Identity, Impl, OFFSET>,
            SetLocalUserAuthorizedList: SetLocalUserAuthorizedList::<Identity, Impl, OFFSET>,
            RemoteUserAuthorizedList: RemoteUserAuthorizedList::<Identity, Impl, OFFSET>,
            SetRemoteUserAuthorizedList: SetRemoteUserAuthorizedList::<Identity, Impl, OFFSET>,
            RemoteMachineAuthorizedList: RemoteMachineAuthorizedList::<Identity, Impl, OFFSET>,
            SetRemoteMachineAuthorizedList: SetRemoteMachineAuthorizedList::<Identity, Impl, OFFSET>,
            SecureFlags: SecureFlags::<Identity, Impl, OFFSET>,
            SetSecureFlags: SetSecureFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<INetFwRule as ::windows::core::Interface>::IID || iid == &<INetFwRule2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, rule: &::core::option::Option<INetFwRule>) -> ::windows::core::Result<()>;
    fn Remove(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Item(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<INetFwRule>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRules_Impl, const OFFSET: isize>() -> INetFwRules_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&rule)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRules as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwService_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Type(&self) -> ::windows::core::Result<NET_FW_SERVICE_TYPE>;
    fn Customized(&self) -> ::windows::core::Result<i16>;
    fn IpVersion(&self) -> ::windows::core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()>;
    fn Scope(&self) -> ::windows::core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn GloballyOpenPorts(&self) -> ::windows::core::Result<INetFwOpenPorts>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>() -> INetFwService_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Customized<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customized: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Customized() {
                ::core::result::Result::Ok(ok__) => {
                    *customized = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpVersion<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IpVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *ipversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIpVersion(::core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GloballyOpenPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *openports = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Customized: Customized::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwService as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwServiceRestriction_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RestrictService(&self, servicename: &super::super::Foundation::BSTR, appname: &super::super::Foundation::BSTR, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::Result<()>;
    fn ServiceRestricted(&self, servicename: &super::super::Foundation::BSTR, appname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Rules(&self) -> ::windows::core::Result<INetFwRules>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwServiceRestriction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>() -> INetFwServiceRestriction_Vtbl {
        unsafe extern "system" fn RestrictService<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestrictService(::core::mem::transmute(&servicename), ::core::mem::transmute(&appname), ::core::mem::transmute_copy(&restrictservice), ::core::mem::transmute_copy(&servicesidrestricted)).into()
        }
        unsafe extern "system" fn ServiceRestricted<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servicerestricted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceRestricted(::core::mem::transmute(&servicename), ::core::mem::transmute(&appname)) {
                ::core::result::Result::Ok(ok__) => {
                    *servicerestricted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rules() {
                ::core::result::Result::Ok(ok__) => {
                    *rules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RestrictService: RestrictService::<Identity, Impl, OFFSET>,
            ServiceRestricted: ServiceRestricted::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwServiceRestriction as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwServices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> ::windows::core::Result<INetFwService>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServices_Impl, const OFFSET: isize>() -> INetFwServices_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&svctype)) {
                ::core::result::Result::Ok(ok__) => {
                    *service = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwServices as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingConfiguration_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SharingEnabled(&self) -> ::windows::core::Result<i16>;
    fn SharingConnectionType(&self) -> ::windows::core::Result<SHARINGCONNECTIONTYPE>;
    fn DisableSharing(&self) -> ::windows::core::Result<()>;
    fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::Result<()>;
    fn InternetFirewallEnabled(&self) -> ::windows::core::Result<i16>;
    fn DisableInternetFirewall(&self) -> ::windows::core::Result<()>;
    fn EnableInternetFirewall(&self) -> ::windows::core::Result<()>;
    fn EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPortMappingCollection>;
    fn AddPortMapping(&self, bstrname: &super::super::Foundation::BSTR, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: &super::super::Foundation::BSTR, etargettype: ICS_TARGETTYPE) -> ::windows::core::Result<INetSharingPortMapping>;
    fn RemovePortMapping(&self, pmapping: &::core::option::Option<INetSharingPortMapping>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>() -> INetSharingConfiguration_Vtbl {
        unsafe extern "system" fn SharingEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SharingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingConnectionType<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SharingConnectionType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableSharing<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableSharing().into()
        }
        unsafe extern "system" fn EnableSharing<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableSharing(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn InternetFirewallEnabled<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InternetFirewallEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableInternetFirewall<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableInternetFirewall().into()
        }
        unsafe extern "system" fn EnableInternetFirewall<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableInternetFirewall().into()
        }
        unsafe extern "system" fn EnumPortMappings<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumPortMappings(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcoll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortMapping<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddPortMapping(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&ucipprotocol), ::core::mem::transmute_copy(&usexternalport), ::core::mem::transmute_copy(&usinternalport), ::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute(&bstrtargetnameoripaddress), ::core::mem::transmute_copy(&etargettype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmapping = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortMapping<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmapping: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePortMapping(::core::mem::transmute(&pmapping)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SharingEnabled: SharingEnabled::<Identity, Impl, OFFSET>,
            SharingConnectionType: SharingConnectionType::<Identity, Impl, OFFSET>,
            DisableSharing: DisableSharing::<Identity, Impl, OFFSET>,
            EnableSharing: EnableSharing::<Identity, Impl, OFFSET>,
            InternetFirewallEnabled: InternetFirewallEnabled::<Identity, Impl, OFFSET>,
            DisableInternetFirewall: DisableInternetFirewall::<Identity, Impl, OFFSET>,
            EnableInternetFirewall: EnableInternetFirewall::<Identity, Impl, OFFSET>,
            EnumPortMappings: EnumPortMappings::<Identity, Impl, OFFSET>,
            AddPortMapping: AddPortMapping::<Identity, Impl, OFFSET>,
            RemovePortMapping: RemovePortMapping::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingConfiguration as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingEveryConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingEveryConnectionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>() -> INetSharingEveryConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingEveryConnectionCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SharingInstalled(&self) -> ::windows::core::Result<i16>;
    fn EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPublicConnectionCollection>;
    fn EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPrivateConnectionCollection>;
    fn INetSharingConfigurationForINetConnection(&self, pnetconnection: &::core::option::Option<INetConnection>) -> ::windows::core::Result<INetSharingConfiguration>;
    fn EnumEveryConnection(&self) -> ::windows::core::Result<INetSharingEveryConnectionCollection>;
    fn NetConnectionProps(&self, pnetconnection: &::core::option::Option<INetConnection>) -> ::windows::core::Result<INetConnectionProps>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManager_Impl, const OFFSET: isize>() -> INetSharingManager_Vtbl {
        unsafe extern "system" fn SharingInstalled<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbinstalled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SharingInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbinstalled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPublicConnections<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumPublicConnections(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcoll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPrivateConnections<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumPrivateConnections(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcoll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn INetSharingConfigurationForINetConnection<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppnetsharingconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).INetSharingConfigurationForINetConnection(::core::mem::transmute(&pnetconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetsharingconfiguration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEveryConnection<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumEveryConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcoll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetConnectionProps<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NetConnectionProps(::core::mem::transmute(&pnetconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SharingInstalled: SharingInstalled::<Identity, Impl, OFFSET>,
            EnumPublicConnections: EnumPublicConnections::<Identity, Impl, OFFSET>,
            EnumPrivateConnections: EnumPrivateConnections::<Identity, Impl, OFFSET>,
            INetSharingConfigurationForINetConnection: INetSharingConfigurationForINetConnection::<Identity, Impl, OFFSET>,
            EnumEveryConnection: EnumEveryConnection::<Identity, Impl, OFFSET>,
            NetConnectionProps: NetConnectionProps::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Disable(&self) -> ::windows::core::Result<()>;
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<INetSharingPortMappingProps>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>() -> INetSharingPortMapping_Vtbl {
        unsafe extern "system" fn Disable<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disable().into()
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnspmp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnspmp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Disable: Disable::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMapping as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMappingCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: isize>() -> INetSharingPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMappingCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMappingProps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IPProtocol(&self) -> ::windows::core::Result<u8>;
    fn ExternalPort(&self) -> ::windows::core::Result<i32>;
    fn InternalPort(&self) -> ::windows::core::Result<i32>;
    fn Options(&self) -> ::windows::core::Result<i32>;
    fn TargetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TargetIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMappingProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>() -> INetSharingPortMappingProps_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPProtocol<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucipprot: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IPProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pucipprot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pusport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pusport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetName<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtargetname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetIPAddress<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TargetIPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtargetipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            IPProtocol: IPProtocol::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            TargetName: TargetName::<Identity, Impl, OFFSET>,
            TargetIPAddress: TargetIPAddress::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMappingProps as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPrivateConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPrivateConnectionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>() -> INetSharingPrivateConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPrivateConnectionCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPublicConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPublicConnectionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>() -> INetSharingPublicConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPublicConnectionCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IStaticPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExternalPort(&self) -> ::windows::core::Result<i32>;
    fn InternalPort(&self) -> ::windows::core::Result<i32>;
    fn Protocol(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InternalClient(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EditInternalClient(&self, bstrinternalclient: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enable(&self, vb: i16) -> ::windows::core::Result<()>;
    fn EditDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EditInternalPort(&self, linternalport: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IStaticPortMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>() -> IStaticPortMapping_Vtbl {
        unsafe extern "system" fn ExternalIPAddress<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExternalIPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InternalClient() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EditInternalClient(::core::mem::transmute(&bstrinternalclient)).into()
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&vb)).into()
        }
        unsafe extern "system" fn EditDescription<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EditDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn EditInternalPort<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EditInternalPort(::core::mem::transmute_copy(&linternalport)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            InternalClient: InternalClient::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            EditDescription: EditDescription::<Identity, Impl, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStaticPortMapping as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IStaticPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR) -> ::windows::core::Result<IStaticPortMapping>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Remove(&self, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Add(&self, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR, linternalport: i32, bstrinternalclient: &super::super::Foundation::BSTR, benabled: i16, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<IStaticPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IStaticPortMappingCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>() -> IStaticPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppspm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&linternalport), ::core::mem::transmute(&bstrinternalclient), ::core::mem::transmute_copy(&benabled), ::core::mem::transmute(&bstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppspm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStaticPortMappingCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPNAT_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StaticPortMappingCollection(&self) -> ::windows::core::Result<IStaticPortMappingCollection>;
    fn DynamicPortMappingCollection(&self) -> ::windows::core::Result<IDynamicPortMappingCollection>;
    fn NATEventManager(&self) -> ::windows::core::Result<INATEventManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPNAT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPNAT_Impl, const OFFSET: isize>() -> IUPnPNAT_Vtbl {
        unsafe extern "system" fn StaticPortMappingCollection<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StaticPortMappingCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppspms = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicPortMappingCollection<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdpms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DynamicPortMappingCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdpms = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NATEventManager<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NATEventManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StaticPortMappingCollection: StaticPortMappingCollection::<Identity, Impl, OFFSET>,
            DynamicPortMappingCollection: DynamicPortMappingCollection::<Identity, Impl, OFFSET>,
            NATEventManager: NATEventManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPNAT as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
