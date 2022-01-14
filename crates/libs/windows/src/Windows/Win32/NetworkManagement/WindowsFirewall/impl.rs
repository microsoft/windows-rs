#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDynamicPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RemoteHost(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExternalPort(&mut self) -> ::windows::core::Result<i32>;
    fn Protocol(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InternalPort(&mut self) -> ::windows::core::Result<i32>;
    fn InternalClient(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LeaseDuration(&mut self) -> ::windows::core::Result<i32>;
    fn RenewLease(&mut self, lleasedurationdesired: i32) -> ::windows::core::Result<i32>;
    fn EditInternalClient(&mut self, bstrinternalclient: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enable(&mut self, vb: i16) -> ::windows::core::Result<()>;
    fn EditDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EditInternalPort(&mut self, linternalport: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDynamicPortMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMapping_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDynamicPortMapping_Vtbl {
        unsafe extern "system" fn ExternalIPAddress<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalIPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteHost<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteHost() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalClient() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseDuration<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeaseDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewLease<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewLease(::core::mem::transmute_copy(&lleasedurationdesired)) {
                ::core::result::Result::Ok(ok__) => {
                    *pleasedurationreturned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EditInternalClient(::core::mem::transmute_copy(&bstrinternalclient)).into()
        }
        unsafe extern "system" fn Enable<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&vb)).into()
        }
        unsafe extern "system" fn EditDescription<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EditDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn EditInternalPort<Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EditInternalPort(::core::mem::transmute_copy(&linternalport)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Impl, IMPL_OFFSET>,
            RemoteHost: RemoteHost::<Impl, IMPL_OFFSET>,
            ExternalPort: ExternalPort::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            InternalPort: InternalPort::<Impl, IMPL_OFFSET>,
            InternalClient: InternalClient::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            LeaseDuration: LeaseDuration::<Impl, IMPL_OFFSET>,
            RenewLease: RenewLease::<Impl, IMPL_OFFSET>,
            EditInternalClient: EditInternalClient::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            EditDescription: EditDescription::<Impl, IMPL_OFFSET>,
            EditInternalPort: EditInternalPort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDynamicPortMapping as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDynamicPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, bstrremotehost: &super::super::Foundation::BSTR, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR) -> ::windows::core::Result<IDynamicPortMapping>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Remove(&mut self, bstrremotehost: &super::super::Foundation::BSTR, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Add(&mut self, bstrremotehost: &super::super::Foundation::BSTR, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR, linternalport: i32, bstrinternalclient: &super::super::Foundation::BSTR, benabled: i16, bstrdescription: &super::super::Foundation::BSTR, lleaseduration: i32) -> ::windows::core::Result<IDynamicPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDynamicPortMappingCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDynamicPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute_copy(&bstrprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdpm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute_copy(&bstrprotocol)).into()
        }
        unsafe extern "system" fn Add<Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lleaseduration: i32, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&bstrremotehost), ::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&linternalport), ::core::mem::transmute_copy(&bstrinternalclient), ::core::mem::transmute_copy(&benabled), ::core::mem::transmute_copy(&bstrdescription), ::core::mem::transmute_copy(&lleaseduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdpm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDynamicPortMappingCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetConnection_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetConnection>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetConnection>;
}
impl IEnumNetConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetConnection_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingEveryConnection_Impl: Sized {
    fn Next(&mut self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetSharingEveryConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingEveryConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingEveryConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetSharingEveryConnection_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingEveryConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPortMapping_Impl: Sized {
    fn Next(&mut self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetSharingPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPortMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPortMapping_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetSharingPortMapping_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPortMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPrivateConnection_Impl: Sized {
    fn Next(&mut self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetSharingPrivateConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPrivateConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPrivateConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetSharingPrivateConnection_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPrivateConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPublicConnection_Impl: Sized {
    fn Next(&mut self, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetSharingPublicConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPublicConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPublicConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetSharingPublicConnection_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPublicConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INATEventManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetExternalIPAddressCallback(&mut self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetNumberOfEntriesCallback(&mut self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INATEventManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATEventManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INATEventManager_Vtbl {
        unsafe extern "system" fn SetExternalIPAddressCallback<Impl: INATEventManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExternalIPAddressCallback(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn SetNumberOfEntriesCallback<Impl: INATEventManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumberOfEntriesCallback(::core::mem::transmute(&punk)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetExternalIPAddressCallback: SetExternalIPAddressCallback::<Impl, IMPL_OFFSET>,
            SetNumberOfEntriesCallback: SetNumberOfEntriesCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATEventManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INATExternalIPAddressCallback_Impl: Sized {
    fn NewExternalIPAddress(&mut self, bstrnewexternalipaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INATExternalIPAddressCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATExternalIPAddressCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INATExternalIPAddressCallback_Vtbl {
        unsafe extern "system" fn NewExternalIPAddress<Impl: INATExternalIPAddressCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewexternalipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewExternalIPAddress(::core::mem::transmute_copy(&bstrnewexternalipaddress)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), NewExternalIPAddress: NewExternalIPAddress::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATExternalIPAddressCallback as ::windows::core::Interface>::IID
    }
}
pub trait INATNumberOfEntriesCallback_Impl: Sized {
    fn NewNumberOfEntries(&mut self, lnewnumberofentries: i32) -> ::windows::core::Result<()>;
}
impl INATNumberOfEntriesCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATNumberOfEntriesCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INATNumberOfEntriesCallback_Vtbl {
        unsafe extern "system" fn NewNumberOfEntries<Impl: INATNumberOfEntriesCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewnumberofentries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewNumberOfEntries(::core::mem::transmute_copy(&lnewnumberofentries)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), NewNumberOfEntries: NewNumberOfEntries::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATNumberOfEntriesCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetConnection_Impl: Sized {
    fn Connect(&mut self) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Duplicate(&mut self, pszwduplicatename: super::super::Foundation::PWSTR) -> ::windows::core::Result<INetConnection>;
    fn GetProperties(&mut self) -> ::windows::core::Result<*mut NETCON_PROPERTIES>;
    fn GetUiObjectClassId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Rename(&mut self, pszwnewname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetConnection_Vtbl {
        unsafe extern "system" fn Connect<Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Delete<Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Duplicate<Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwduplicatename: super::super::Foundation::PWSTR, ppcon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duplicate(::core::mem::transmute_copy(&pszwduplicatename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiObjectClassId<Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUiObjectClassId() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwnewname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&pszwnewname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Duplicate: Duplicate::<Impl, IMPL_OFFSET>,
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetUiObjectClassId: GetUiObjectClassId::<Impl, IMPL_OFFSET>,
            Rename: Rename::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetConnectionConnectUi_Impl: Sized {
    fn SetConnection(&mut self, pcon: &::core::option::Option<INetConnection>) -> ::windows::core::Result<()>;
    fn Connect(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetConnectionConnectUi_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionConnectUi_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetConnectionConnectUi_Vtbl {
        unsafe extern "system" fn SetConnection<Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnection(::core::mem::transmute(&pcon)).into()
        }
        unsafe extern "system" fn Connect<Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetConnection: SetConnection::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionConnectUi as ::windows::core::Interface>::IID
    }
}
pub trait INetConnectionManager_Impl: Sized {
    fn EnumConnections(&mut self, flags: NETCONMGR_ENUM_FLAGS) -> ::windows::core::Result<IEnumNetConnection>;
}
impl INetConnectionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetConnectionManager_Vtbl {
        unsafe extern "system" fn EnumConnections<Impl: INetConnectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumConnections(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), EnumConnections: EnumConnections::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetConnectionProps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Guid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DeviceName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Status(&mut self) -> ::windows::core::Result<NETCON_STATUS>;
    fn MediaType(&mut self) -> ::windows::core::Result<NETCON_MEDIATYPE>;
    fn Characteristics(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetConnectionProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionProps_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetConnectionProps_Vtbl {
        unsafe extern "system" fn Guid<Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Guid: Guid::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            DeviceName: DeviceName::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            Characteristics: Characteristics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionProps as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwAuthorizedApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProcessImageFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetProcessImageFileName(&mut self, imagefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IpVersion(&mut self) -> ::windows::core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&mut self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()>;
    fn Scope(&mut self) -> ::windows::core::Result<NET_FW_SCOPE>;
    fn SetScope(&mut self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&mut self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwAuthorizedApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwAuthorizedApplication_Vtbl {
        unsafe extern "system" fn Name<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn ProcessImageFileName<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessImageFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *imagefilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessImageFileName<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProcessImageFileName(::core::mem::transmute_copy(&imagefilename)).into()
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *ipversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpVersion(::core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            ProcessImageFileName: ProcessImageFileName::<Impl, IMPL_OFFSET>,
            SetProcessImageFileName: SetProcessImageFileName::<Impl, IMPL_OFFSET>,
            IpVersion: IpVersion::<Impl, IMPL_OFFSET>,
            SetIpVersion: SetIpVersion::<Impl, IMPL_OFFSET>,
            Scope: Scope::<Impl, IMPL_OFFSET>,
            SetScope: SetScope::<Impl, IMPL_OFFSET>,
            RemoteAddresses: RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplication as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwAuthorizedApplications_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, app: &::core::option::Option<INetFwAuthorizedApplication>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, imagefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Item(&mut self, imagefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<INetFwAuthorizedApplication>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwAuthorizedApplications_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplications_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwAuthorizedApplications_Vtbl {
        unsafe extern "system" fn Count<Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, app: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&app)).into()
        }
        unsafe extern "system" fn Remove<Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&imagefilename)).into()
        }
        unsafe extern "system" fn Item<Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, app: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&imagefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *app = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplications as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwIcmpSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AllowOutboundDestinationUnreachable(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundDestinationUnreachable(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowRedirect(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowRedirect(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowInboundEchoRequest(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowInboundEchoRequest(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowOutboundTimeExceeded(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundTimeExceeded(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowOutboundParameterProblem(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundParameterProblem(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowOutboundSourceQuench(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundSourceQuench(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowInboundRouterRequest(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowInboundRouterRequest(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowInboundTimestampRequest(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowInboundTimestampRequest(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowInboundMaskRequest(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowInboundMaskRequest(&mut self, allow: i16) -> ::windows::core::Result<()>;
    fn AllowOutboundPacketTooBig(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowOutboundPacketTooBig(&mut self, allow: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwIcmpSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwIcmpSettings_Vtbl {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundDestinationUnreachable() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOutboundDestinationUnreachable(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowRedirect<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowRedirect() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowRedirect<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowRedirect(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInboundEchoRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInboundEchoRequest(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundTimeExceeded() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOutboundTimeExceeded(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundParameterProblem() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOutboundParameterProblem(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundSourceQuench() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOutboundSourceQuench(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInboundRouterRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInboundRouterRequest(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInboundTimestampRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInboundTimestampRequest(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInboundMaskRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInboundMaskRequest(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundPacketTooBig() {
                ::core::result::Result::Ok(ok__) => {
                    *allow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOutboundPacketTooBig(::core::mem::transmute_copy(&allow)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AllowOutboundDestinationUnreachable: AllowOutboundDestinationUnreachable::<Impl, IMPL_OFFSET>,
            SetAllowOutboundDestinationUnreachable: SetAllowOutboundDestinationUnreachable::<Impl, IMPL_OFFSET>,
            AllowRedirect: AllowRedirect::<Impl, IMPL_OFFSET>,
            SetAllowRedirect: SetAllowRedirect::<Impl, IMPL_OFFSET>,
            AllowInboundEchoRequest: AllowInboundEchoRequest::<Impl, IMPL_OFFSET>,
            SetAllowInboundEchoRequest: SetAllowInboundEchoRequest::<Impl, IMPL_OFFSET>,
            AllowOutboundTimeExceeded: AllowOutboundTimeExceeded::<Impl, IMPL_OFFSET>,
            SetAllowOutboundTimeExceeded: SetAllowOutboundTimeExceeded::<Impl, IMPL_OFFSET>,
            AllowOutboundParameterProblem: AllowOutboundParameterProblem::<Impl, IMPL_OFFSET>,
            SetAllowOutboundParameterProblem: SetAllowOutboundParameterProblem::<Impl, IMPL_OFFSET>,
            AllowOutboundSourceQuench: AllowOutboundSourceQuench::<Impl, IMPL_OFFSET>,
            SetAllowOutboundSourceQuench: SetAllowOutboundSourceQuench::<Impl, IMPL_OFFSET>,
            AllowInboundRouterRequest: AllowInboundRouterRequest::<Impl, IMPL_OFFSET>,
            SetAllowInboundRouterRequest: SetAllowInboundRouterRequest::<Impl, IMPL_OFFSET>,
            AllowInboundTimestampRequest: AllowInboundTimestampRequest::<Impl, IMPL_OFFSET>,
            SetAllowInboundTimestampRequest: SetAllowInboundTimestampRequest::<Impl, IMPL_OFFSET>,
            AllowInboundMaskRequest: AllowInboundMaskRequest::<Impl, IMPL_OFFSET>,
            SetAllowInboundMaskRequest: SetAllowInboundMaskRequest::<Impl, IMPL_OFFSET>,
            AllowOutboundPacketTooBig: AllowOutboundPacketTooBig::<Impl, IMPL_OFFSET>,
            SetAllowOutboundPacketTooBig: SetAllowOutboundPacketTooBig::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwIcmpSettings as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwMgr_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LocalPolicy(&mut self) -> ::windows::core::Result<INetFwPolicy>;
    fn CurrentProfileType(&mut self) -> ::windows::core::Result<NET_FW_PROFILE_TYPE>;
    fn RestoreDefaults(&mut self) -> ::windows::core::Result<()>;
    fn IsPortAllowed(&mut self, imagefilename: &super::super::Foundation::BSTR, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: &super::super::Foundation::BSTR, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsIcmpTypeAllowed(&mut self, ipversion: NET_FW_IP_VERSION, localaddress: &super::super::Foundation::BSTR, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwMgr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwMgr_Vtbl {
        unsafe extern "system" fn LocalPolicy<Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *localpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfileType<Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProfileType() {
                ::core::result::Result::Ok(ok__) => {
                    *profiletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDefaults<Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreDefaults().into()
        }
        unsafe extern "system" fn IsPortAllowed<Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPortAllowed(::core::mem::transmute_copy(&imagefilename), ::core::mem::transmute_copy(&ipversion), ::core::mem::transmute_copy(&portnumber), ::core::mem::transmute_copy(&localaddress), ::core::mem::transmute_copy(&ipprotocol), ::core::mem::transmute_copy(&allowed), ::core::mem::transmute_copy(&restricted)).into()
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsIcmpTypeAllowed(::core::mem::transmute_copy(&ipversion), ::core::mem::transmute_copy(&localaddress), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&allowed), ::core::mem::transmute_copy(&restricted)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LocalPolicy: LocalPolicy::<Impl, IMPL_OFFSET>,
            CurrentProfileType: CurrentProfileType::<Impl, IMPL_OFFSET>,
            RestoreDefaults: RestoreDefaults::<Impl, IMPL_OFFSET>,
            IsPortAllowed: IsPortAllowed::<Impl, IMPL_OFFSET>,
            IsIcmpTypeAllowed: IsIcmpTypeAllowed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwMgr as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwOpenPort_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IpVersion(&mut self) -> ::windows::core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&mut self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()>;
    fn Protocol(&mut self) -> ::windows::core::Result<NET_FW_IP_PROTOCOL>;
    fn SetProtocol(&mut self, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<()>;
    fn Port(&mut self) -> ::windows::core::Result<i32>;
    fn SetPort(&mut self, portnumber: i32) -> ::windows::core::Result<()>;
    fn Scope(&mut self) -> ::windows::core::Result<NET_FW_SCOPE>;
    fn SetScope(&mut self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&mut self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn BuiltIn(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwOpenPort_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPort_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwOpenPort_Vtbl {
        unsafe extern "system" fn Name<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *ipversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpVersion(::core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Protocol<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *ipprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtocol(::core::mem::transmute_copy(&ipprotocol)).into()
        }
        unsafe extern "system" fn Port<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Port() {
                ::core::result::Result::Ok(ok__) => {
                    *portnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPort(::core::mem::transmute_copy(&portnumber)).into()
        }
        unsafe extern "system" fn Scope<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn BuiltIn<Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, builtin: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuiltIn() {
                ::core::result::Result::Ok(ok__) => {
                    *builtin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            IpVersion: IpVersion::<Impl, IMPL_OFFSET>,
            SetIpVersion: SetIpVersion::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            SetProtocol: SetProtocol::<Impl, IMPL_OFFSET>,
            Port: Port::<Impl, IMPL_OFFSET>,
            SetPort: SetPort::<Impl, IMPL_OFFSET>,
            Scope: Scope::<Impl, IMPL_OFFSET>,
            SetScope: SetScope::<Impl, IMPL_OFFSET>,
            RemoteAddresses: RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            BuiltIn: BuiltIn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwOpenPort as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwOpenPorts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, port: &::core::option::Option<INetFwOpenPort>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<()>;
    fn Item(&mut self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::Result<INetFwOpenPort>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwOpenPorts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPorts_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwOpenPorts_Vtbl {
        unsafe extern "system" fn Count<Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, port: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&port)).into()
        }
        unsafe extern "system" fn Remove<Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&portnumber), ::core::mem::transmute_copy(&ipprotocol)).into()
        }
        unsafe extern "system" fn Item<Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&portnumber), ::core::mem::transmute_copy(&ipprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *openport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwOpenPorts as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwPolicy_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentProfile(&mut self) -> ::windows::core::Result<INetFwProfile>;
    fn GetProfileByType(&mut self, profiletype: NET_FW_PROFILE_TYPE) -> ::windows::core::Result<INetFwProfile>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwPolicy_Vtbl {
        unsafe extern "system" fn CurrentProfile<Impl: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *profile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileByType<Impl: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfileByType(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *profile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentProfile: CurrentProfile::<Impl, IMPL_OFFSET>,
            GetProfileByType: GetProfileByType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwPolicy as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwPolicy2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentProfileTypes(&mut self) -> ::windows::core::Result<i32>;
    fn FirewallEnabled(&mut self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16>;
    fn SetFirewallEnabled(&mut self, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::Result<()>;
    fn ExcludedInterfaces(&mut self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExcludedInterfaces(&mut self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn BlockAllInboundTraffic(&mut self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16>;
    fn SetBlockAllInboundTraffic(&mut self, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::Result<()>;
    fn NotificationsDisabled(&mut self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16>;
    fn SetNotificationsDisabled(&mut self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(&mut self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<i16>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(&mut self, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::Result<()>;
    fn Rules(&mut self) -> ::windows::core::Result<INetFwRules>;
    fn ServiceRestriction(&mut self) -> ::windows::core::Result<INetFwServiceRestriction>;
    fn EnableRuleGroup(&mut self, profiletypesbitmask: i32, group: &super::super::Foundation::BSTR, enable: i16) -> ::windows::core::Result<()>;
    fn IsRuleGroupEnabled(&mut self, profiletypesbitmask: i32, group: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn RestoreLocalFirewallDefaults(&mut self) -> ::windows::core::Result<()>;
    fn DefaultInboundAction(&mut self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<NET_FW_ACTION>;
    fn SetDefaultInboundAction(&mut self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::Result<()>;
    fn DefaultOutboundAction(&mut self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows::core::Result<NET_FW_ACTION>;
    fn SetDefaultOutboundAction(&mut self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::Result<()>;
    fn IsRuleGroupCurrentlyEnabled(&mut self, group: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn LocalPolicyModifyState(&mut self) -> ::windows::core::Result<NET_FW_MODIFY_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwPolicy2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwPolicy2_Vtbl {
        unsafe extern "system" fn CurrentProfileTypes<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProfileTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *profiletypesbitmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallEnabled<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallEnabled(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFirewallEnabled(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn ExcludedInterfaces<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExcludedInterfaces(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *interfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludedInterfaces<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExcludedInterfaces(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&interfaces)).into()
        }
        unsafe extern "system" fn BlockAllInboundTraffic<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockAllInboundTraffic(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *block = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAllInboundTraffic<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlockAllInboundTraffic(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&block)).into()
        }
        unsafe extern "system" fn NotificationsDisabled<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationsDisabled(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *disabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationsDisabled(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicastResponsesToMulticastBroadcastDisabled(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *disabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnicastResponsesToMulticastBroadcastDisabled(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn Rules<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rules() {
                ::core::result::Result::Ok(ok__) => {
                    *rules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceRestriction<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicerestriction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceRestriction() {
                ::core::result::Result::Ok(ok__) => {
                    *servicerestriction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableRuleGroup<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableRuleGroup(::core::mem::transmute_copy(&profiletypesbitmask), ::core::mem::transmute_copy(&group), ::core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRuleGroupEnabled(::core::mem::transmute_copy(&profiletypesbitmask), ::core::mem::transmute_copy(&group)) {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreLocalFirewallDefaults().into()
        }
        unsafe extern "system" fn DefaultInboundAction<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInboundAction(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInboundAction<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultInboundAction(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn DefaultOutboundAction<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultOutboundAction(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultOutboundAction<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultOutboundAction(::core::mem::transmute_copy(&profiletype), ::core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn IsRuleGroupCurrentlyEnabled<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRuleGroupCurrentlyEnabled(::core::mem::transmute_copy(&group)) {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPolicyModifyState<Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPolicyModifyState() {
                ::core::result::Result::Ok(ok__) => {
                    *modifystate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentProfileTypes: CurrentProfileTypes::<Impl, IMPL_OFFSET>,
            FirewallEnabled: FirewallEnabled::<Impl, IMPL_OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Impl, IMPL_OFFSET>,
            ExcludedInterfaces: ExcludedInterfaces::<Impl, IMPL_OFFSET>,
            SetExcludedInterfaces: SetExcludedInterfaces::<Impl, IMPL_OFFSET>,
            BlockAllInboundTraffic: BlockAllInboundTraffic::<Impl, IMPL_OFFSET>,
            SetBlockAllInboundTraffic: SetBlockAllInboundTraffic::<Impl, IMPL_OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Impl, IMPL_OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Impl, IMPL_OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Impl, IMPL_OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Impl, IMPL_OFFSET>,
            Rules: Rules::<Impl, IMPL_OFFSET>,
            ServiceRestriction: ServiceRestriction::<Impl, IMPL_OFFSET>,
            EnableRuleGroup: EnableRuleGroup::<Impl, IMPL_OFFSET>,
            IsRuleGroupEnabled: IsRuleGroupEnabled::<Impl, IMPL_OFFSET>,
            RestoreLocalFirewallDefaults: RestoreLocalFirewallDefaults::<Impl, IMPL_OFFSET>,
            DefaultInboundAction: DefaultInboundAction::<Impl, IMPL_OFFSET>,
            SetDefaultInboundAction: SetDefaultInboundAction::<Impl, IMPL_OFFSET>,
            DefaultOutboundAction: DefaultOutboundAction::<Impl, IMPL_OFFSET>,
            SetDefaultOutboundAction: SetDefaultOutboundAction::<Impl, IMPL_OFFSET>,
            IsRuleGroupCurrentlyEnabled: IsRuleGroupCurrentlyEnabled::<Impl, IMPL_OFFSET>,
            LocalPolicyModifyState: LocalPolicyModifyState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwPolicy2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProduct_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RuleCategories(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetRuleCategories(&mut self, rulecategories: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, displayname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathToSignedProductExe(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProduct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProduct_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwProduct_Vtbl {
        unsafe extern "system" fn RuleCategories<Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulecategories: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuleCategories() {
                ::core::result::Result::Ok(ok__) => {
                    *rulecategories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleCategories<Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulecategories: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRuleCategories(::core::mem::transmute_copy(&rulecategories)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *displayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&displayname)).into()
        }
        unsafe extern "system" fn PathToSignedProductExe<Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathToSignedProductExe() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RuleCategories: RuleCategories::<Impl, IMPL_OFFSET>,
            SetRuleCategories: SetRuleCategories::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            PathToSignedProductExe: PathToSignedProductExe::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProduct as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProducts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Register(&mut self, product: &::core::option::Option<INetFwProduct>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<INetFwProduct>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProducts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProducts_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwProducts_Vtbl {
        unsafe extern "system" fn Count<Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, product: ::windows::core::RawPtr, registration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register(::core::mem::transmute(&product)) {
                ::core::result::Result::Ok(ok__) => {
                    *registration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, product: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *product = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Register: Register::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProducts as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProfile_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<NET_FW_PROFILE_TYPE>;
    fn FirewallEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetFirewallEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn ExceptionsNotAllowed(&mut self) -> ::windows::core::Result<i16>;
    fn SetExceptionsNotAllowed(&mut self, notallowed: i16) -> ::windows::core::Result<()>;
    fn NotificationsDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetNotificationsDisabled(&mut self, disabled: i16) -> ::windows::core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(&mut self, disabled: i16) -> ::windows::core::Result<()>;
    fn RemoteAdminSettings(&mut self) -> ::windows::core::Result<INetFwRemoteAdminSettings>;
    fn IcmpSettings(&mut self) -> ::windows::core::Result<INetFwIcmpSettings>;
    fn GloballyOpenPorts(&mut self) -> ::windows::core::Result<INetFwOpenPorts>;
    fn Services(&mut self) -> ::windows::core::Result<INetFwServices>;
    fn AuthorizedApplications(&mut self) -> ::windows::core::Result<INetFwAuthorizedApplications>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwProfile_Vtbl {
        unsafe extern "system" fn Type<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallEnabled<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFirewallEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExceptionsNotAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *notallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notallowed: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExceptionsNotAllowed(::core::mem::transmute_copy(&notallowed)).into()
        }
        unsafe extern "system" fn NotificationsDisabled<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationsDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *disabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationsDisabled(::core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicastResponsesToMulticastBroadcastDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *disabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnicastResponsesToMulticastBroadcastDisabled(::core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn RemoteAdminSettings<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteadminsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAdminSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteadminsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IcmpSettings<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmpsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IcmpSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *icmpsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GloballyOpenPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *openports = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, services: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Services() {
                ::core::result::Result::Ok(ok__) => {
                    *services = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthorizedApplications<Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorizedApplications() {
                ::core::result::Result::Ok(ok__) => {
                    *apps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            FirewallEnabled: FirewallEnabled::<Impl, IMPL_OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Impl, IMPL_OFFSET>,
            ExceptionsNotAllowed: ExceptionsNotAllowed::<Impl, IMPL_OFFSET>,
            SetExceptionsNotAllowed: SetExceptionsNotAllowed::<Impl, IMPL_OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Impl, IMPL_OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Impl, IMPL_OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Impl, IMPL_OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Impl, IMPL_OFFSET>,
            RemoteAdminSettings: RemoteAdminSettings::<Impl, IMPL_OFFSET>,
            IcmpSettings: IcmpSettings::<Impl, IMPL_OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Impl, IMPL_OFFSET>,
            Services: Services::<Impl, IMPL_OFFSET>,
            AuthorizedApplications: AuthorizedApplications::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProfile as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRemoteAdminSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IpVersion(&mut self) -> ::windows::core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&mut self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()>;
    fn Scope(&mut self) -> ::windows::core::Result<NET_FW_SCOPE>;
    fn SetScope(&mut self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&mut self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRemoteAdminSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRemoteAdminSettings_Vtbl {
        unsafe extern "system" fn IpVersion<Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *ipversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpVersion(::core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IpVersion: IpVersion::<Impl, IMPL_OFFSET>,
            SetIpVersion: SetIpVersion::<Impl, IMPL_OFFSET>,
            Scope: Scope::<Impl, IMPL_OFFSET>,
            SetScope: SetScope::<Impl, IMPL_OFFSET>,
            RemoteAddresses: RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRemoteAdminSettings as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, desc: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationName(&mut self, imagefilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceName(&mut self, servicename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Protocol(&mut self) -> ::windows::core::Result<i32>;
    fn SetProtocol(&mut self, protocol: i32) -> ::windows::core::Result<()>;
    fn LocalPorts(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalPorts(&mut self, portnumbers: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemotePorts(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemotePorts(&mut self, portnumbers: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalAddresses(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalAddresses(&mut self, localaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&mut self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IcmpTypesAndCodes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetIcmpTypesAndCodes(&mut self, icmptypesandcodes: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Direction(&mut self) -> ::windows::core::Result<NET_FW_RULE_DIRECTION>;
    fn SetDirection(&mut self, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::Result<()>;
    fn Interfaces(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetInterfaces(&mut self, interfaces: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InterfaceTypes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInterfaceTypes(&mut self, interfacetypes: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn Grouping(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetGrouping(&mut self, context: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Profiles(&mut self) -> ::windows::core::Result<i32>;
    fn SetProfiles(&mut self, profiletypesbitmask: i32) -> ::windows::core::Result<()>;
    fn EdgeTraversal(&mut self) -> ::windows::core::Result<i16>;
    fn SetEdgeTraversal(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn Action(&mut self) -> ::windows::core::Result<NET_FW_ACTION>;
    fn SetAction(&mut self, action: NET_FW_ACTION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRule_Vtbl {
        unsafe extern "system" fn Name<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Description<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *desc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&desc)).into()
        }
        unsafe extern "system" fn ApplicationName<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationName() {
                ::core::result::Result::Ok(ok__) => {
                    *imagefilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationName<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationName(::core::mem::transmute_copy(&imagefilename)).into()
        }
        unsafe extern "system" fn ServiceName<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *servicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceName<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceName(::core::mem::transmute_copy(&servicename)).into()
        }
        unsafe extern "system" fn Protocol<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *protocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtocol(::core::mem::transmute_copy(&protocol)).into()
        }
        unsafe extern "system" fn LocalPorts<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *portnumbers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPorts<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalPorts(::core::mem::transmute_copy(&portnumbers)).into()
        }
        unsafe extern "system" fn RemotePorts<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePorts() {
                ::core::result::Result::Ok(ok__) => {
                    *portnumbers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemotePorts<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemotePorts(::core::mem::transmute_copy(&portnumbers)).into()
        }
        unsafe extern "system" fn LocalAddresses<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *localaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddresses<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalAddresses(::core::mem::transmute_copy(&localaddrs)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)).into()
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IcmpTypesAndCodes() {
                ::core::result::Result::Ok(ok__) => {
                    *icmptypesandcodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIcmpTypesAndCodes(::core::mem::transmute_copy(&icmptypesandcodes)).into()
        }
        unsafe extern "system" fn Direction<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *dir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirection(::core::mem::transmute_copy(&dir)).into()
        }
        unsafe extern "system" fn Interfaces<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *interfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaces<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterfaces(::core::mem::transmute_copy(&interfaces)).into()
        }
        unsafe extern "system" fn InterfaceTypes<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacetypes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *interfacetypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceTypes<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterfaceTypes(::core::mem::transmute_copy(&interfacetypes)).into()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Grouping<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Grouping() {
                ::core::result::Result::Ok(ok__) => {
                    *context = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGrouping<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGrouping(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn Profiles<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profiles() {
                ::core::result::Result::Ok(ok__) => {
                    *profiletypesbitmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfiles<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfiles(::core::mem::transmute_copy(&profiletypesbitmask)).into()
        }
        unsafe extern "system" fn EdgeTraversal<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EdgeTraversal() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversal<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEdgeTraversal(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Action<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Action() {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAction<Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAction(::core::mem::transmute_copy(&action)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationName: ApplicationName::<Impl, IMPL_OFFSET>,
            SetApplicationName: SetApplicationName::<Impl, IMPL_OFFSET>,
            ServiceName: ServiceName::<Impl, IMPL_OFFSET>,
            SetServiceName: SetServiceName::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            SetProtocol: SetProtocol::<Impl, IMPL_OFFSET>,
            LocalPorts: LocalPorts::<Impl, IMPL_OFFSET>,
            SetLocalPorts: SetLocalPorts::<Impl, IMPL_OFFSET>,
            RemotePorts: RemotePorts::<Impl, IMPL_OFFSET>,
            SetRemotePorts: SetRemotePorts::<Impl, IMPL_OFFSET>,
            LocalAddresses: LocalAddresses::<Impl, IMPL_OFFSET>,
            SetLocalAddresses: SetLocalAddresses::<Impl, IMPL_OFFSET>,
            RemoteAddresses: RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            IcmpTypesAndCodes: IcmpTypesAndCodes::<Impl, IMPL_OFFSET>,
            SetIcmpTypesAndCodes: SetIcmpTypesAndCodes::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            SetDirection: SetDirection::<Impl, IMPL_OFFSET>,
            Interfaces: Interfaces::<Impl, IMPL_OFFSET>,
            SetInterfaces: SetInterfaces::<Impl, IMPL_OFFSET>,
            InterfaceTypes: InterfaceTypes::<Impl, IMPL_OFFSET>,
            SetInterfaceTypes: SetInterfaceTypes::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Grouping: Grouping::<Impl, IMPL_OFFSET>,
            SetGrouping: SetGrouping::<Impl, IMPL_OFFSET>,
            Profiles: Profiles::<Impl, IMPL_OFFSET>,
            SetProfiles: SetProfiles::<Impl, IMPL_OFFSET>,
            EdgeTraversal: EdgeTraversal::<Impl, IMPL_OFFSET>,
            SetEdgeTraversal: SetEdgeTraversal::<Impl, IMPL_OFFSET>,
            Action: Action::<Impl, IMPL_OFFSET>,
            SetAction: SetAction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRule2_Impl: Sized + super::super::System::Com::IDispatch_Impl + INetFwRule_Impl {
    fn EdgeTraversalOptions(&mut self) -> ::windows::core::Result<i32>;
    fn SetEdgeTraversalOptions(&mut self, loptions: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRule2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRule2_Vtbl {
        unsafe extern "system" fn EdgeTraversalOptions<Impl: INetFwRule2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EdgeTraversalOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *loptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Impl: INetFwRule2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEdgeTraversalOptions(::core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base: INetFwRule_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EdgeTraversalOptions: EdgeTraversalOptions::<Impl, IMPL_OFFSET>,
            SetEdgeTraversalOptions: SetEdgeTraversalOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<INetFwRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRule3_Impl: Sized + super::super::System::Com::IDispatch_Impl + INetFwRule_Impl + INetFwRule2_Impl {
    fn LocalAppPackageId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalAppPackageId(&mut self, wszpackageid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalUserOwner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalUserOwner(&mut self, wszuserowner: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LocalUserAuthorizedList(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocalUserAuthorizedList(&mut self, wszuserauthlist: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoteUserAuthorizedList(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteUserAuthorizedList(&mut self, wszuserauthlist: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoteMachineAuthorizedList(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteMachineAuthorizedList(&mut self, wszuserauthlist: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SecureFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetSecureFlags(&mut self, loptions: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRule3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRule3_Vtbl {
        unsafe extern "system" fn LocalAppPackageId<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpackageid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAppPackageId() {
                ::core::result::Result::Ok(ok__) => {
                    *wszpackageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAppPackageId<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalAppPackageId(::core::mem::transmute_copy(&wszpackageid)).into()
        }
        unsafe extern "system" fn LocalUserOwner<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserowner: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *wszuserowner = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserOwner<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalUserOwner(::core::mem::transmute_copy(&wszuserowner)).into()
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserAuthorizedList() {
                ::core::result::Result::Ok(ok__) => {
                    *wszuserauthlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalUserAuthorizedList(::core::mem::transmute_copy(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteUserAuthorizedList() {
                ::core::result::Result::Ok(ok__) => {
                    *wszuserauthlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteUserAuthorizedList(::core::mem::transmute_copy(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteMachineAuthorizedList() {
                ::core::result::Result::Ok(ok__) => {
                    *wszuserauthlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteMachineAuthorizedList(::core::mem::transmute_copy(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn SecureFlags<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecureFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *loptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecureFlags<Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecureFlags(::core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base: INetFwRule2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LocalAppPackageId: LocalAppPackageId::<Impl, IMPL_OFFSET>,
            SetLocalAppPackageId: SetLocalAppPackageId::<Impl, IMPL_OFFSET>,
            LocalUserOwner: LocalUserOwner::<Impl, IMPL_OFFSET>,
            SetLocalUserOwner: SetLocalUserOwner::<Impl, IMPL_OFFSET>,
            LocalUserAuthorizedList: LocalUserAuthorizedList::<Impl, IMPL_OFFSET>,
            SetLocalUserAuthorizedList: SetLocalUserAuthorizedList::<Impl, IMPL_OFFSET>,
            RemoteUserAuthorizedList: RemoteUserAuthorizedList::<Impl, IMPL_OFFSET>,
            SetRemoteUserAuthorizedList: SetRemoteUserAuthorizedList::<Impl, IMPL_OFFSET>,
            RemoteMachineAuthorizedList: RemoteMachineAuthorizedList::<Impl, IMPL_OFFSET>,
            SetRemoteMachineAuthorizedList: SetRemoteMachineAuthorizedList::<Impl, IMPL_OFFSET>,
            SecureFlags: SecureFlags::<Impl, IMPL_OFFSET>,
            SetSecureFlags: SetSecureFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<INetFwRule as ::windows::core::Interface>::IID || iid == &<INetFwRule2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, rule: &::core::option::Option<INetFwRule>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Item(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<INetFwRule>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRules_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRules_Vtbl {
        unsafe extern "system" fn Count<Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&rule)).into()
        }
        unsafe extern "system" fn Remove<Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Item<Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRules as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwService_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Type(&mut self) -> ::windows::core::Result<NET_FW_SERVICE_TYPE>;
    fn Customized(&mut self) -> ::windows::core::Result<i16>;
    fn IpVersion(&mut self) -> ::windows::core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&mut self, ipversion: NET_FW_IP_VERSION) -> ::windows::core::Result<()>;
    fn Scope(&mut self) -> ::windows::core::Result<NET_FW_SCOPE>;
    fn SetScope(&mut self, scope: NET_FW_SCOPE) -> ::windows::core::Result<()>;
    fn RemoteAddresses(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRemoteAddresses(&mut self, remoteaddrs: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn GloballyOpenPorts(&mut self) -> ::windows::core::Result<INetFwOpenPorts>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwService_Vtbl {
        unsafe extern "system" fn Name<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Customized<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customized: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Customized() {
                ::core::result::Result::Ok(ok__) => {
                    *customized = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *ipversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpVersion(::core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *remoteaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn GloballyOpenPorts<Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GloballyOpenPorts() {
                ::core::result::Result::Ok(ok__) => {
                    *openports = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Customized: Customized::<Impl, IMPL_OFFSET>,
            IpVersion: IpVersion::<Impl, IMPL_OFFSET>,
            SetIpVersion: SetIpVersion::<Impl, IMPL_OFFSET>,
            Scope: Scope::<Impl, IMPL_OFFSET>,
            SetScope: SetScope::<Impl, IMPL_OFFSET>,
            RemoteAddresses: RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwService as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwServiceRestriction_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RestrictService(&mut self, servicename: &super::super::Foundation::BSTR, appname: &super::super::Foundation::BSTR, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::Result<()>;
    fn ServiceRestricted(&mut self, servicename: &super::super::Foundation::BSTR, appname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Rules(&mut self) -> ::windows::core::Result<INetFwRules>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwServiceRestriction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceRestriction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwServiceRestriction_Vtbl {
        unsafe extern "system" fn RestrictService<Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestrictService(::core::mem::transmute_copy(&servicename), ::core::mem::transmute_copy(&appname), ::core::mem::transmute_copy(&restrictservice), ::core::mem::transmute_copy(&servicesidrestricted)).into()
        }
        unsafe extern "system" fn ServiceRestricted<Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servicerestricted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceRestricted(::core::mem::transmute_copy(&servicename), ::core::mem::transmute_copy(&appname)) {
                ::core::result::Result::Ok(ok__) => {
                    *servicerestricted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rules() {
                ::core::result::Result::Ok(ok__) => {
                    *rules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RestrictService: RestrictService::<Impl, IMPL_OFFSET>,
            ServiceRestricted: ServiceRestricted::<Impl, IMPL_OFFSET>,
            Rules: Rules::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwServiceRestriction as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwServices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, svctype: NET_FW_SERVICE_TYPE) -> ::windows::core::Result<INetFwService>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwServices_Vtbl {
        unsafe extern "system" fn Count<Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&svctype)) {
                ::core::result::Result::Ok(ok__) => {
                    *service = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwServices as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingConfiguration_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SharingEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SharingConnectionType(&mut self) -> ::windows::core::Result<SHARINGCONNECTIONTYPE>;
    fn DisableSharing(&mut self) -> ::windows::core::Result<()>;
    fn EnableSharing(&mut self, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::Result<()>;
    fn InternetFirewallEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn DisableInternetFirewall(&mut self) -> ::windows::core::Result<()>;
    fn EnableInternetFirewall(&mut self) -> ::windows::core::Result<()>;
    fn EnumPortMappings(&mut self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPortMappingCollection>;
    fn AddPortMapping(&mut self, bstrname: &super::super::Foundation::BSTR, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: &super::super::Foundation::BSTR, etargettype: ICS_TARGETTYPE) -> ::windows::core::Result<INetSharingPortMapping>;
    fn RemovePortMapping(&mut self, pmapping: &::core::option::Option<INetSharingPortMapping>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingConfiguration_Vtbl {
        unsafe extern "system" fn SharingEnabled<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingConnectionType<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingConnectionType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableSharing<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableSharing().into()
        }
        unsafe extern "system" fn EnableSharing<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableSharing(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn InternetFirewallEnabled<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternetFirewallEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableInternetFirewall<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableInternetFirewall().into()
        }
        unsafe extern "system" fn EnableInternetFirewall<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableInternetFirewall().into()
        }
        unsafe extern "system" fn EnumPortMappings<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPortMappings(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcoll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortMapping<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPortMapping(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&ucipprotocol), ::core::mem::transmute_copy(&usexternalport), ::core::mem::transmute_copy(&usinternalport), ::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute_copy(&bstrtargetnameoripaddress), ::core::mem::transmute_copy(&etargettype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmapping = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortMapping<Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmapping: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePortMapping(::core::mem::transmute(&pmapping)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SharingEnabled: SharingEnabled::<Impl, IMPL_OFFSET>,
            SharingConnectionType: SharingConnectionType::<Impl, IMPL_OFFSET>,
            DisableSharing: DisableSharing::<Impl, IMPL_OFFSET>,
            EnableSharing: EnableSharing::<Impl, IMPL_OFFSET>,
            InternetFirewallEnabled: InternetFirewallEnabled::<Impl, IMPL_OFFSET>,
            DisableInternetFirewall: DisableInternetFirewall::<Impl, IMPL_OFFSET>,
            EnableInternetFirewall: EnableInternetFirewall::<Impl, IMPL_OFFSET>,
            EnumPortMappings: EnumPortMappings::<Impl, IMPL_OFFSET>,
            AddPortMapping: AddPortMapping::<Impl, IMPL_OFFSET>,
            RemovePortMapping: RemovePortMapping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingConfiguration as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingEveryConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingEveryConnectionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingEveryConnectionCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingEveryConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingEveryConnectionCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SharingInstalled(&mut self) -> ::windows::core::Result<i16>;
    fn EnumPublicConnections(&mut self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPublicConnectionCollection>;
    fn EnumPrivateConnections(&mut self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows::core::Result<INetSharingPrivateConnectionCollection>;
    fn INetSharingConfigurationForINetConnection(&mut self, pnetconnection: &::core::option::Option<INetConnection>) -> ::windows::core::Result<INetSharingConfiguration>;
    fn EnumEveryConnection(&mut self) -> ::windows::core::Result<INetSharingEveryConnectionCollection>;
    fn NetConnectionProps(&mut self, pnetconnection: &::core::option::Option<INetConnection>) -> ::windows::core::Result<INetConnectionProps>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingManager_Vtbl {
        unsafe extern "system" fn SharingInstalled<Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbinstalled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbinstalled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPublicConnections<Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPublicConnections(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcoll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPrivateConnections<Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPrivateConnections(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcoll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn INetSharingConfigurationForINetConnection<Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppnetsharingconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).INetSharingConfigurationForINetConnection(::core::mem::transmute(&pnetconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetsharingconfiguration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEveryConnection<Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEveryConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcoll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetConnectionProps<Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetConnectionProps(::core::mem::transmute(&pnetconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SharingInstalled: SharingInstalled::<Impl, IMPL_OFFSET>,
            EnumPublicConnections: EnumPublicConnections::<Impl, IMPL_OFFSET>,
            EnumPrivateConnections: EnumPrivateConnections::<Impl, IMPL_OFFSET>,
            INetSharingConfigurationForINetConnection: INetSharingConfigurationForINetConnection::<Impl, IMPL_OFFSET>,
            EnumEveryConnection: EnumEveryConnection::<Impl, IMPL_OFFSET>,
            NetConnectionProps: NetConnectionProps::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Disable(&mut self) -> ::windows::core::Result<()>;
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<INetSharingPortMappingProps>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMapping_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPortMapping_Vtbl {
        unsafe extern "system" fn Disable<Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable().into()
        }
        unsafe extern "system" fn Enable<Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Properties<Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnspmp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnspmp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Disable: Disable::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMapping as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMappingCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: INetSharingPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMappingCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMappingProps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IPProtocol(&mut self) -> ::windows::core::Result<u8>;
    fn ExternalPort(&mut self) -> ::windows::core::Result<i32>;
    fn InternalPort(&mut self) -> ::windows::core::Result<i32>;
    fn Options(&mut self) -> ::windows::core::Result<i32>;
    fn TargetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TargetIPAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMappingProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingProps_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPortMappingProps_Vtbl {
        unsafe extern "system" fn Name<Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPProtocol<Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucipprot: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pucipprot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pusport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pusport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetName<Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtargetname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetIPAddress<Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetIPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtargetipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            IPProtocol: IPProtocol::<Impl, IMPL_OFFSET>,
            ExternalPort: ExternalPort::<Impl, IMPL_OFFSET>,
            InternalPort: InternalPort::<Impl, IMPL_OFFSET>,
            Options: Options::<Impl, IMPL_OFFSET>,
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            TargetIPAddress: TargetIPAddress::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMappingProps as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPrivateConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPrivateConnectionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPrivateConnectionCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPrivateConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPrivateConnectionCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPublicConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPublicConnectionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPublicConnectionCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPublicConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPublicConnectionCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IStaticPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExternalPort(&mut self) -> ::windows::core::Result<i32>;
    fn InternalPort(&mut self) -> ::windows::core::Result<i32>;
    fn Protocol(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InternalClient(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EditInternalClient(&mut self, bstrinternalclient: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enable(&mut self, vb: i16) -> ::windows::core::Result<()>;
    fn EditDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EditInternalPort(&mut self, linternalport: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IStaticPortMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMapping_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStaticPortMapping_Vtbl {
        unsafe extern "system" fn ExternalIPAddress<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalIPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalClient() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EditInternalClient(::core::mem::transmute_copy(&bstrinternalclient)).into()
        }
        unsafe extern "system" fn Enable<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&vb)).into()
        }
        unsafe extern "system" fn EditDescription<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EditDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn EditInternalPort<Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EditInternalPort(::core::mem::transmute_copy(&linternalport)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Impl, IMPL_OFFSET>,
            ExternalPort: ExternalPort::<Impl, IMPL_OFFSET>,
            InternalPort: InternalPort::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            InternalClient: InternalClient::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            EditInternalClient: EditInternalClient::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            EditDescription: EditDescription::<Impl, IMPL_OFFSET>,
            EditInternalPort: EditInternalPort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStaticPortMapping as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IStaticPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR) -> ::windows::core::Result<IStaticPortMapping>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Remove(&mut self, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Add(&mut self, lexternalport: i32, bstrprotocol: &super::super::Foundation::BSTR, linternalport: i32, bstrinternalclient: &super::super::Foundation::BSTR, benabled: i16, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<IStaticPortMapping>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IStaticPortMappingCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStaticPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute_copy(&bstrprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppspm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute_copy(&bstrprotocol)).into()
        }
        unsafe extern "system" fn Add<Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&lexternalport), ::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&linternalport), ::core::mem::transmute_copy(&bstrinternalclient), ::core::mem::transmute_copy(&benabled), ::core::mem::transmute_copy(&bstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppspm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStaticPortMappingCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPNAT_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StaticPortMappingCollection(&mut self) -> ::windows::core::Result<IStaticPortMappingCollection>;
    fn DynamicPortMappingCollection(&mut self) -> ::windows::core::Result<IDynamicPortMappingCollection>;
    fn NATEventManager(&mut self) -> ::windows::core::Result<INATEventManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPNAT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPNAT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPNAT_Vtbl {
        unsafe extern "system" fn StaticPortMappingCollection<Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaticPortMappingCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppspms = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicPortMappingCollection<Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdpms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicPortMappingCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdpms = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NATEventManager<Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NATEventManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StaticPortMappingCollection: StaticPortMappingCollection::<Impl, IMPL_OFFSET>,
            DynamicPortMappingCollection: DynamicPortMappingCollection::<Impl, IMPL_OFFSET>,
            NATEventManager: NATEventManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPNAT as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
