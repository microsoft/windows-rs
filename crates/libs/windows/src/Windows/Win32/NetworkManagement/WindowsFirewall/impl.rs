#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDynamicPortMappingImpl: Sized + IDispatchImpl {
    fn ExternalIPAddress();
    fn RemoteHost();
    fn ExternalPort();
    fn Protocol();
    fn InternalPort();
    fn InternalClient();
    fn Enabled();
    fn Description();
    fn LeaseDuration();
    fn RenewLease();
    fn EditInternalClient();
    fn Enable();
    fn EditDescription();
    fn EditInternalPort();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDynamicPortMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDynamicPortMappingVtbl {
        unsafe extern "system" fn ExternalIPAddress<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteHost<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExternalPort<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Protocol<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InternalPort<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InternalClient<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LeaseDuration<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenewLease<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditInternalClient<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enable<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditDescription<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditInternalPort<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ExternalIPAddress::<Impl, IMPL_OFFSET>,
            RemoteHost::<Impl, IMPL_OFFSET>,
            ExternalPort::<Impl, IMPL_OFFSET>,
            Protocol::<Impl, IMPL_OFFSET>,
            InternalPort::<Impl, IMPL_OFFSET>,
            InternalClient::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            LeaseDuration::<Impl, IMPL_OFFSET>,
            RenewLease::<Impl, IMPL_OFFSET>,
            EditInternalClient::<Impl, IMPL_OFFSET>,
            Enable::<Impl, IMPL_OFFSET>,
            EditDescription::<Impl, IMPL_OFFSET>,
            EditInternalPort::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDynamicPortMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDynamicPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
    fn Add();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDynamicPortMappingCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDynamicPortMappingCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lleaseduration: i32, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDynamicPortMappingCollection as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumNetConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetConnectionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingEveryConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingEveryConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingEveryConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetSharingEveryConnectionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingEveryConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPortMappingImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPortMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPortMappingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetSharingPortMappingVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPortMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPrivateConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPrivateConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPrivateConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetSharingPrivateConnectionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPrivateConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetSharingPublicConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetSharingPublicConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPublicConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetSharingPublicConnectionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetSharingPublicConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INATEventManagerImpl: Sized + IDispatchImpl {
    fn SetExternalIPAddressCallback();
    fn SetNumberOfEntriesCallback();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INATEventManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATEventManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INATEventManagerVtbl {
        unsafe extern "system" fn SetExternalIPAddressCallback<Impl: INATEventManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNumberOfEntriesCallback<Impl: INATEventManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, SetExternalIPAddressCallback::<Impl, IMPL_OFFSET>, SetNumberOfEntriesCallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATEventManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INATExternalIPAddressCallbackImpl: Sized {
    fn NewExternalIPAddress();
}
#[cfg(feature = "Win32_Foundation")]
impl INATExternalIPAddressCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATExternalIPAddressCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INATExternalIPAddressCallbackVtbl {
        unsafe extern "system" fn NewExternalIPAddress<Impl: INATExternalIPAddressCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewexternalipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NewExternalIPAddress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATExternalIPAddressCallback as ::windows::core::Interface>::IID
    }
}
pub trait INATNumberOfEntriesCallbackImpl: Sized {
    fn NewNumberOfEntries();
}
impl INATNumberOfEntriesCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATNumberOfEntriesCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INATNumberOfEntriesCallbackVtbl {
        unsafe extern "system" fn NewNumberOfEntries<Impl: INATNumberOfEntriesCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewnumberofentries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NewNumberOfEntries::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INATNumberOfEntriesCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetConnectionImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Delete();
    fn Duplicate();
    fn GetProperties();
    fn GetUiObjectClassId();
    fn Rename();
}
#[cfg(feature = "Win32_Foundation")]
impl INetConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetConnectionVtbl {
        unsafe extern "system" fn Connect<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Duplicate<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwduplicatename: super::super::Foundation::PWSTR, ppcon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperties<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUiObjectClassId<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rename<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwnewname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Connect::<Impl, IMPL_OFFSET>, Disconnect::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>, Duplicate::<Impl, IMPL_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>, GetUiObjectClassId::<Impl, IMPL_OFFSET>, Rename::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetConnectionConnectUiImpl: Sized {
    fn SetConnection();
    fn Connect();
    fn Disconnect();
}
#[cfg(feature = "Win32_Foundation")]
impl INetConnectionConnectUiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionConnectUiImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetConnectionConnectUiVtbl {
        unsafe extern "system" fn SetConnection<Impl: INetConnectionConnectUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connect<Impl: INetConnectionConnectUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: INetConnectionConnectUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetConnection::<Impl, IMPL_OFFSET>, Connect::<Impl, IMPL_OFFSET>, Disconnect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionConnectUi as ::windows::core::Interface>::IID
    }
}
pub trait INetConnectionManagerImpl: Sized {
    fn EnumConnections();
}
impl INetConnectionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetConnectionManagerVtbl {
        unsafe extern "system" fn EnumConnections<Impl: INetConnectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnumConnections::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetConnectionPropsImpl: Sized + IDispatchImpl {
    fn Guid();
    fn Name();
    fn DeviceName();
    fn Status();
    fn MediaType();
    fn Characteristics();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetConnectionPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetConnectionPropsVtbl {
        unsafe extern "system" fn Guid<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceName<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Status<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaType<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Characteristics<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Guid::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            DeviceName::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            MediaType::<Impl, IMPL_OFFSET>,
            Characteristics::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetConnectionProps as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwAuthorizedApplicationImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn ProcessImageFileName();
    fn SetProcessImageFileName();
    fn IpVersion();
    fn SetIpVersion();
    fn Scope();
    fn SetScope();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwAuthorizedApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwAuthorizedApplicationVtbl {
        unsafe extern "system" fn Name<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessImageFileName<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProcessImageFileName<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scope<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScope<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            ProcessImageFileName::<Impl, IMPL_OFFSET>,
            SetProcessImageFileName::<Impl, IMPL_OFFSET>,
            IpVersion::<Impl, IMPL_OFFSET>,
            SetIpVersion::<Impl, IMPL_OFFSET>,
            Scope::<Impl, IMPL_OFFSET>,
            SetScope::<Impl, IMPL_OFFSET>,
            RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwAuthorizedApplicationsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwAuthorizedApplicationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplicationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwAuthorizedApplicationsVtbl {
        unsafe extern "system" fn Count<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, app: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, app: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplications as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwIcmpSettingsImpl: Sized + IDispatchImpl {
    fn AllowOutboundDestinationUnreachable();
    fn SetAllowOutboundDestinationUnreachable();
    fn AllowRedirect();
    fn SetAllowRedirect();
    fn AllowInboundEchoRequest();
    fn SetAllowInboundEchoRequest();
    fn AllowOutboundTimeExceeded();
    fn SetAllowOutboundTimeExceeded();
    fn AllowOutboundParameterProblem();
    fn SetAllowOutboundParameterProblem();
    fn AllowOutboundSourceQuench();
    fn SetAllowOutboundSourceQuench();
    fn AllowInboundRouterRequest();
    fn SetAllowInboundRouterRequest();
    fn AllowInboundTimestampRequest();
    fn SetAllowInboundTimestampRequest();
    fn AllowInboundMaskRequest();
    fn SetAllowInboundMaskRequest();
    fn AllowOutboundPacketTooBig();
    fn SetAllowOutboundPacketTooBig();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwIcmpSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwIcmpSettingsVtbl {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowRedirect<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowRedirect<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            AllowOutboundDestinationUnreachable::<Impl, IMPL_OFFSET>,
            SetAllowOutboundDestinationUnreachable::<Impl, IMPL_OFFSET>,
            AllowRedirect::<Impl, IMPL_OFFSET>,
            SetAllowRedirect::<Impl, IMPL_OFFSET>,
            AllowInboundEchoRequest::<Impl, IMPL_OFFSET>,
            SetAllowInboundEchoRequest::<Impl, IMPL_OFFSET>,
            AllowOutboundTimeExceeded::<Impl, IMPL_OFFSET>,
            SetAllowOutboundTimeExceeded::<Impl, IMPL_OFFSET>,
            AllowOutboundParameterProblem::<Impl, IMPL_OFFSET>,
            SetAllowOutboundParameterProblem::<Impl, IMPL_OFFSET>,
            AllowOutboundSourceQuench::<Impl, IMPL_OFFSET>,
            SetAllowOutboundSourceQuench::<Impl, IMPL_OFFSET>,
            AllowInboundRouterRequest::<Impl, IMPL_OFFSET>,
            SetAllowInboundRouterRequest::<Impl, IMPL_OFFSET>,
            AllowInboundTimestampRequest::<Impl, IMPL_OFFSET>,
            SetAllowInboundTimestampRequest::<Impl, IMPL_OFFSET>,
            AllowInboundMaskRequest::<Impl, IMPL_OFFSET>,
            SetAllowInboundMaskRequest::<Impl, IMPL_OFFSET>,
            AllowOutboundPacketTooBig::<Impl, IMPL_OFFSET>,
            SetAllowOutboundPacketTooBig::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwIcmpSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwMgrImpl: Sized + IDispatchImpl {
    fn LocalPolicy();
    fn CurrentProfileType();
    fn RestoreDefaults();
    fn IsPortAllowed();
    fn IsIcmpTypeAllowed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwMgrVtbl {
        unsafe extern "system" fn LocalPolicy<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentProfileType<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreDefaults<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPortAllowed<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            LocalPolicy::<Impl, IMPL_OFFSET>,
            CurrentProfileType::<Impl, IMPL_OFFSET>,
            RestoreDefaults::<Impl, IMPL_OFFSET>,
            IsPortAllowed::<Impl, IMPL_OFFSET>,
            IsIcmpTypeAllowed::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwMgr as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwOpenPortImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn IpVersion();
    fn SetIpVersion();
    fn Protocol();
    fn SetProtocol();
    fn Port();
    fn SetPort();
    fn Scope();
    fn SetScope();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn Enabled();
    fn SetEnabled();
    fn BuiltIn();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwOpenPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPortImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwOpenPortVtbl {
        unsafe extern "system" fn Name<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Protocol<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProtocol<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Port<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPort<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scope<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScope<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BuiltIn<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, builtin: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            IpVersion::<Impl, IMPL_OFFSET>,
            SetIpVersion::<Impl, IMPL_OFFSET>,
            Protocol::<Impl, IMPL_OFFSET>,
            SetProtocol::<Impl, IMPL_OFFSET>,
            Port::<Impl, IMPL_OFFSET>,
            SetPort::<Impl, IMPL_OFFSET>,
            Scope::<Impl, IMPL_OFFSET>,
            SetScope::<Impl, IMPL_OFFSET>,
            RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            BuiltIn::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwOpenPort as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwOpenPortsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwOpenPortsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPortsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwOpenPortsVtbl {
        unsafe extern "system" fn Count<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, port: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwOpenPorts as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwPolicyImpl: Sized + IDispatchImpl {
    fn CurrentProfile();
    fn GetProfileByType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwPolicyVtbl {
        unsafe extern "system" fn CurrentProfile<Impl: INetFwPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProfileByType<Impl: INetFwPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CurrentProfile::<Impl, IMPL_OFFSET>, GetProfileByType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwPolicy2Impl: Sized + IDispatchImpl {
    fn CurrentProfileTypes();
    fn FirewallEnabled();
    fn SetFirewallEnabled();
    fn ExcludedInterfaces();
    fn SetExcludedInterfaces();
    fn BlockAllInboundTraffic();
    fn SetBlockAllInboundTraffic();
    fn NotificationsDisabled();
    fn SetNotificationsDisabled();
    fn UnicastResponsesToMulticastBroadcastDisabled();
    fn SetUnicastResponsesToMulticastBroadcastDisabled();
    fn Rules();
    fn ServiceRestriction();
    fn EnableRuleGroup();
    fn IsRuleGroupEnabled();
    fn RestoreLocalFirewallDefaults();
    fn DefaultInboundAction();
    fn SetDefaultInboundAction();
    fn DefaultOutboundAction();
    fn SetDefaultOutboundAction();
    fn IsRuleGroupCurrentlyEnabled();
    fn LocalPolicyModifyState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwPolicy2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwPolicy2Vtbl {
        unsafe extern "system" fn CurrentProfileTypes<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FirewallEnabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFirewallEnabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExcludedInterfaces<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExcludedInterfaces<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BlockAllInboundTraffic<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlockAllInboundTraffic<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotificationsDisabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotificationsDisabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rules<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceRestriction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicerestriction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableRuleGroup<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultInboundAction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultInboundAction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultOutboundAction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultOutboundAction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRuleGroupCurrentlyEnabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalPolicyModifyState<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            CurrentProfileTypes::<Impl, IMPL_OFFSET>,
            FirewallEnabled::<Impl, IMPL_OFFSET>,
            SetFirewallEnabled::<Impl, IMPL_OFFSET>,
            ExcludedInterfaces::<Impl, IMPL_OFFSET>,
            SetExcludedInterfaces::<Impl, IMPL_OFFSET>,
            BlockAllInboundTraffic::<Impl, IMPL_OFFSET>,
            SetBlockAllInboundTraffic::<Impl, IMPL_OFFSET>,
            NotificationsDisabled::<Impl, IMPL_OFFSET>,
            SetNotificationsDisabled::<Impl, IMPL_OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled::<Impl, IMPL_OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled::<Impl, IMPL_OFFSET>,
            Rules::<Impl, IMPL_OFFSET>,
            ServiceRestriction::<Impl, IMPL_OFFSET>,
            EnableRuleGroup::<Impl, IMPL_OFFSET>,
            IsRuleGroupEnabled::<Impl, IMPL_OFFSET>,
            RestoreLocalFirewallDefaults::<Impl, IMPL_OFFSET>,
            DefaultInboundAction::<Impl, IMPL_OFFSET>,
            SetDefaultInboundAction::<Impl, IMPL_OFFSET>,
            DefaultOutboundAction::<Impl, IMPL_OFFSET>,
            SetDefaultOutboundAction::<Impl, IMPL_OFFSET>,
            IsRuleGroupCurrentlyEnabled::<Impl, IMPL_OFFSET>,
            LocalPolicyModifyState::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwPolicy2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProductImpl: Sized + IDispatchImpl {
    fn RuleCategories();
    fn SetRuleCategories();
    fn DisplayName();
    fn SetDisplayName();
    fn PathToSignedProductExe();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProductVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProductImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwProductVtbl {
        unsafe extern "system" fn RuleCategories<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulecategories: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRuleCategories<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulecategories: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathToSignedProductExe<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            RuleCategories::<Impl, IMPL_OFFSET>,
            SetRuleCategories::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            PathToSignedProductExe::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProduct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProductsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Register();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProductsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProductsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwProductsVtbl {
        unsafe extern "system" fn Count<Impl: INetFwProductsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Register<Impl: INetFwProductsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, product: ::windows::core::RawPtr, registration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: INetFwProductsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, product: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwProductsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Register::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProducts as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwProfileImpl: Sized + IDispatchImpl {
    fn Type();
    fn FirewallEnabled();
    fn SetFirewallEnabled();
    fn ExceptionsNotAllowed();
    fn SetExceptionsNotAllowed();
    fn NotificationsDisabled();
    fn SetNotificationsDisabled();
    fn UnicastResponsesToMulticastBroadcastDisabled();
    fn SetUnicastResponsesToMulticastBroadcastDisabled();
    fn RemoteAdminSettings();
    fn IcmpSettings();
    fn GloballyOpenPorts();
    fn Services();
    fn AuthorizedApplications();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwProfileVtbl {
        unsafe extern "system" fn Type<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FirewallEnabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFirewallEnabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notallowed: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotificationsDisabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotificationsDisabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteAdminSettings<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteadminsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IcmpSettings<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmpsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GloballyOpenPorts<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Services<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, services: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthorizedApplications<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            FirewallEnabled::<Impl, IMPL_OFFSET>,
            SetFirewallEnabled::<Impl, IMPL_OFFSET>,
            ExceptionsNotAllowed::<Impl, IMPL_OFFSET>,
            SetExceptionsNotAllowed::<Impl, IMPL_OFFSET>,
            NotificationsDisabled::<Impl, IMPL_OFFSET>,
            SetNotificationsDisabled::<Impl, IMPL_OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled::<Impl, IMPL_OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled::<Impl, IMPL_OFFSET>,
            RemoteAdminSettings::<Impl, IMPL_OFFSET>,
            IcmpSettings::<Impl, IMPL_OFFSET>,
            GloballyOpenPorts::<Impl, IMPL_OFFSET>,
            Services::<Impl, IMPL_OFFSET>,
            AuthorizedApplications::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRemoteAdminSettingsImpl: Sized + IDispatchImpl {
    fn IpVersion();
    fn SetIpVersion();
    fn Scope();
    fn SetScope();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRemoteAdminSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRemoteAdminSettingsVtbl {
        unsafe extern "system" fn IpVersion<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scope<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScope<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            IpVersion::<Impl, IMPL_OFFSET>,
            SetIpVersion::<Impl, IMPL_OFFSET>,
            Scope::<Impl, IMPL_OFFSET>,
            SetScope::<Impl, IMPL_OFFSET>,
            RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRemoteAdminSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRuleImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationName();
    fn SetApplicationName();
    fn ServiceName();
    fn SetServiceName();
    fn Protocol();
    fn SetProtocol();
    fn LocalPorts();
    fn SetLocalPorts();
    fn RemotePorts();
    fn SetRemotePorts();
    fn LocalAddresses();
    fn SetLocalAddresses();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn IcmpTypesAndCodes();
    fn SetIcmpTypesAndCodes();
    fn Direction();
    fn SetDirection();
    fn Interfaces();
    fn SetInterfaces();
    fn InterfaceTypes();
    fn SetInterfaceTypes();
    fn Enabled();
    fn SetEnabled();
    fn Grouping();
    fn SetGrouping();
    fn Profiles();
    fn SetProfiles();
    fn EdgeTraversal();
    fn SetEdgeTraversal();
    fn Action();
    fn SetAction();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRuleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRuleVtbl {
        unsafe extern "system" fn Name<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplicationName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetApplicationName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Protocol<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProtocol<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalPorts<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalPorts<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemotePorts<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemotePorts<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalAddresses<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalAddresses<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Direction<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDirection<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Interfaces<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterfaces<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterfaceTypes<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacetypes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterfaceTypes<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Grouping<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGrouping<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Profiles<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProfiles<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EdgeTraversal<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEdgeTraversal<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Action<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAction<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationName::<Impl, IMPL_OFFSET>,
            SetApplicationName::<Impl, IMPL_OFFSET>,
            ServiceName::<Impl, IMPL_OFFSET>,
            SetServiceName::<Impl, IMPL_OFFSET>,
            Protocol::<Impl, IMPL_OFFSET>,
            SetProtocol::<Impl, IMPL_OFFSET>,
            LocalPorts::<Impl, IMPL_OFFSET>,
            SetLocalPorts::<Impl, IMPL_OFFSET>,
            RemotePorts::<Impl, IMPL_OFFSET>,
            SetRemotePorts::<Impl, IMPL_OFFSET>,
            LocalAddresses::<Impl, IMPL_OFFSET>,
            SetLocalAddresses::<Impl, IMPL_OFFSET>,
            RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            IcmpTypesAndCodes::<Impl, IMPL_OFFSET>,
            SetIcmpTypesAndCodes::<Impl, IMPL_OFFSET>,
            Direction::<Impl, IMPL_OFFSET>,
            SetDirection::<Impl, IMPL_OFFSET>,
            Interfaces::<Impl, IMPL_OFFSET>,
            SetInterfaces::<Impl, IMPL_OFFSET>,
            InterfaceTypes::<Impl, IMPL_OFFSET>,
            SetInterfaceTypes::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Grouping::<Impl, IMPL_OFFSET>,
            SetGrouping::<Impl, IMPL_OFFSET>,
            Profiles::<Impl, IMPL_OFFSET>,
            SetProfiles::<Impl, IMPL_OFFSET>,
            EdgeTraversal::<Impl, IMPL_OFFSET>,
            SetEdgeTraversal::<Impl, IMPL_OFFSET>,
            Action::<Impl, IMPL_OFFSET>,
            SetAction::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRule2Impl: Sized + INetFwRuleImpl + IDispatchImpl {
    fn EdgeTraversalOptions();
    fn SetEdgeTraversalOptions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRule2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRule2Vtbl {
        unsafe extern "system" fn EdgeTraversalOptions<Impl: INetFwRule2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Impl: INetFwRule2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationName::<Impl, IMPL_OFFSET>,
            SetApplicationName::<Impl, IMPL_OFFSET>,
            ServiceName::<Impl, IMPL_OFFSET>,
            SetServiceName::<Impl, IMPL_OFFSET>,
            Protocol::<Impl, IMPL_OFFSET>,
            SetProtocol::<Impl, IMPL_OFFSET>,
            LocalPorts::<Impl, IMPL_OFFSET>,
            SetLocalPorts::<Impl, IMPL_OFFSET>,
            RemotePorts::<Impl, IMPL_OFFSET>,
            SetRemotePorts::<Impl, IMPL_OFFSET>,
            LocalAddresses::<Impl, IMPL_OFFSET>,
            SetLocalAddresses::<Impl, IMPL_OFFSET>,
            RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            IcmpTypesAndCodes::<Impl, IMPL_OFFSET>,
            SetIcmpTypesAndCodes::<Impl, IMPL_OFFSET>,
            Direction::<Impl, IMPL_OFFSET>,
            SetDirection::<Impl, IMPL_OFFSET>,
            Interfaces::<Impl, IMPL_OFFSET>,
            SetInterfaces::<Impl, IMPL_OFFSET>,
            InterfaceTypes::<Impl, IMPL_OFFSET>,
            SetInterfaceTypes::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Grouping::<Impl, IMPL_OFFSET>,
            SetGrouping::<Impl, IMPL_OFFSET>,
            Profiles::<Impl, IMPL_OFFSET>,
            SetProfiles::<Impl, IMPL_OFFSET>,
            EdgeTraversal::<Impl, IMPL_OFFSET>,
            SetEdgeTraversal::<Impl, IMPL_OFFSET>,
            Action::<Impl, IMPL_OFFSET>,
            SetAction::<Impl, IMPL_OFFSET>,
            EdgeTraversalOptions::<Impl, IMPL_OFFSET>,
            SetEdgeTraversalOptions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRule3Impl: Sized + INetFwRule2Impl + INetFwRuleImpl + IDispatchImpl {
    fn LocalAppPackageId();
    fn SetLocalAppPackageId();
    fn LocalUserOwner();
    fn SetLocalUserOwner();
    fn LocalUserAuthorizedList();
    fn SetLocalUserAuthorizedList();
    fn RemoteUserAuthorizedList();
    fn SetRemoteUserAuthorizedList();
    fn RemoteMachineAuthorizedList();
    fn SetRemoteMachineAuthorizedList();
    fn SecureFlags();
    fn SetSecureFlags();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRule3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRule3Vtbl {
        unsafe extern "system" fn LocalAppPackageId<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpackageid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalAppPackageId<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalUserOwner<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserowner: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalUserOwner<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SecureFlags<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecureFlags<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationName::<Impl, IMPL_OFFSET>,
            SetApplicationName::<Impl, IMPL_OFFSET>,
            ServiceName::<Impl, IMPL_OFFSET>,
            SetServiceName::<Impl, IMPL_OFFSET>,
            Protocol::<Impl, IMPL_OFFSET>,
            SetProtocol::<Impl, IMPL_OFFSET>,
            LocalPorts::<Impl, IMPL_OFFSET>,
            SetLocalPorts::<Impl, IMPL_OFFSET>,
            RemotePorts::<Impl, IMPL_OFFSET>,
            SetRemotePorts::<Impl, IMPL_OFFSET>,
            LocalAddresses::<Impl, IMPL_OFFSET>,
            SetLocalAddresses::<Impl, IMPL_OFFSET>,
            RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            IcmpTypesAndCodes::<Impl, IMPL_OFFSET>,
            SetIcmpTypesAndCodes::<Impl, IMPL_OFFSET>,
            Direction::<Impl, IMPL_OFFSET>,
            SetDirection::<Impl, IMPL_OFFSET>,
            Interfaces::<Impl, IMPL_OFFSET>,
            SetInterfaces::<Impl, IMPL_OFFSET>,
            InterfaceTypes::<Impl, IMPL_OFFSET>,
            SetInterfaceTypes::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Grouping::<Impl, IMPL_OFFSET>,
            SetGrouping::<Impl, IMPL_OFFSET>,
            Profiles::<Impl, IMPL_OFFSET>,
            SetProfiles::<Impl, IMPL_OFFSET>,
            EdgeTraversal::<Impl, IMPL_OFFSET>,
            SetEdgeTraversal::<Impl, IMPL_OFFSET>,
            Action::<Impl, IMPL_OFFSET>,
            SetAction::<Impl, IMPL_OFFSET>,
            EdgeTraversalOptions::<Impl, IMPL_OFFSET>,
            SetEdgeTraversalOptions::<Impl, IMPL_OFFSET>,
            LocalAppPackageId::<Impl, IMPL_OFFSET>,
            SetLocalAppPackageId::<Impl, IMPL_OFFSET>,
            LocalUserOwner::<Impl, IMPL_OFFSET>,
            SetLocalUserOwner::<Impl, IMPL_OFFSET>,
            LocalUserAuthorizedList::<Impl, IMPL_OFFSET>,
            SetLocalUserAuthorizedList::<Impl, IMPL_OFFSET>,
            RemoteUserAuthorizedList::<Impl, IMPL_OFFSET>,
            SetRemoteUserAuthorizedList::<Impl, IMPL_OFFSET>,
            RemoteMachineAuthorizedList::<Impl, IMPL_OFFSET>,
            SetRemoteMachineAuthorizedList::<Impl, IMPL_OFFSET>,
            SecureFlags::<Impl, IMPL_OFFSET>,
            SetSecureFlags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRule3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwRulesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwRulesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRulesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwRulesVtbl {
        unsafe extern "system" fn Count<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwRules as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwServiceImpl: Sized + IDispatchImpl {
    fn Name();
    fn Type();
    fn Customized();
    fn IpVersion();
    fn SetIpVersion();
    fn Scope();
    fn SetScope();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn Enabled();
    fn SetEnabled();
    fn GloballyOpenPorts();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwServiceVtbl {
        unsafe extern "system" fn Name<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Customized<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customized: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scope<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScope<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GloballyOpenPorts<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Customized::<Impl, IMPL_OFFSET>,
            IpVersion::<Impl, IMPL_OFFSET>,
            SetIpVersion::<Impl, IMPL_OFFSET>,
            Scope::<Impl, IMPL_OFFSET>,
            SetScope::<Impl, IMPL_OFFSET>,
            RemoteAddresses::<Impl, IMPL_OFFSET>,
            SetRemoteAddresses::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            GloballyOpenPorts::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwServiceRestrictionImpl: Sized + IDispatchImpl {
    fn RestrictService();
    fn ServiceRestricted();
    fn Rules();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwServiceRestrictionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceRestrictionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwServiceRestrictionVtbl {
        unsafe extern "system" fn RestrictService<Impl: INetFwServiceRestrictionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceRestricted<Impl: INetFwServiceRestrictionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servicerestricted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rules<Impl: INetFwServiceRestrictionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, RestrictService::<Impl, IMPL_OFFSET>, ServiceRestricted::<Impl, IMPL_OFFSET>, Rules::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwServiceRestriction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetFwServicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetFwServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetFwServicesVtbl {
        unsafe extern "system" fn Count<Impl: INetFwServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: INetFwServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetFwServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingConfigurationImpl: Sized + IDispatchImpl {
    fn SharingEnabled();
    fn SharingConnectionType();
    fn DisableSharing();
    fn EnableSharing();
    fn InternetFirewallEnabled();
    fn DisableInternetFirewall();
    fn EnableInternetFirewall();
    fn EnumPortMappings();
    fn AddPortMapping();
    fn RemovePortMapping();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingConfigurationVtbl {
        unsafe extern "system" fn SharingEnabled<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SharingConnectionType<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableSharing<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableSharing<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InternetFirewallEnabled<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableInternetFirewall<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableInternetFirewall<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumPortMappings<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPortMapping<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemovePortMapping<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmapping: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            SharingEnabled::<Impl, IMPL_OFFSET>,
            SharingConnectionType::<Impl, IMPL_OFFSET>,
            DisableSharing::<Impl, IMPL_OFFSET>,
            EnableSharing::<Impl, IMPL_OFFSET>,
            InternetFirewallEnabled::<Impl, IMPL_OFFSET>,
            DisableInternetFirewall::<Impl, IMPL_OFFSET>,
            EnableInternetFirewall::<Impl, IMPL_OFFSET>,
            EnumPortMappings::<Impl, IMPL_OFFSET>,
            AddPortMapping::<Impl, IMPL_OFFSET>,
            RemovePortMapping::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingEveryConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingEveryConnectionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingEveryConnectionCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingEveryConnectionCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingEveryConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: INetSharingEveryConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingEveryConnectionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingManagerImpl: Sized + IDispatchImpl {
    fn SharingInstalled();
    fn EnumPublicConnections();
    fn EnumPrivateConnections();
    fn INetSharingConfigurationForINetConnection();
    fn EnumEveryConnection();
    fn NetConnectionProps();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingManagerVtbl {
        unsafe extern "system" fn SharingInstalled<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbinstalled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumPublicConnections<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumPrivateConnections<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn INetSharingConfigurationForINetConnection<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppnetsharingconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumEveryConnection<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetConnectionProps<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            SharingInstalled::<Impl, IMPL_OFFSET>,
            EnumPublicConnections::<Impl, IMPL_OFFSET>,
            EnumPrivateConnections::<Impl, IMPL_OFFSET>,
            INetSharingConfigurationForINetConnection::<Impl, IMPL_OFFSET>,
            EnumEveryConnection::<Impl, IMPL_OFFSET>,
            NetConnectionProps::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMappingImpl: Sized + IDispatchImpl {
    fn Disable();
    fn Enable();
    fn Properties();
    fn Delete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPortMappingVtbl {
        unsafe extern "system" fn Disable<Impl: INetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enable<Impl: INetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: INetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnspmp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: INetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Disable::<Impl, IMPL_OFFSET>, Enable::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMappingCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPortMappingCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: INetSharingPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMappingCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPortMappingPropsImpl: Sized + IDispatchImpl {
    fn Name();
    fn IPProtocol();
    fn ExternalPort();
    fn InternalPort();
    fn Options();
    fn TargetName();
    fn TargetIPAddress();
    fn Enabled();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPortMappingPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPortMappingPropsVtbl {
        unsafe extern "system" fn Name<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IPProtocol<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucipprot: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExternalPort<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InternalPort<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Options<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetName<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetIPAddress<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            IPProtocol::<Impl, IMPL_OFFSET>,
            ExternalPort::<Impl, IMPL_OFFSET>,
            InternalPort::<Impl, IMPL_OFFSET>,
            Options::<Impl, IMPL_OFFSET>,
            TargetName::<Impl, IMPL_OFFSET>,
            TargetIPAddress::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPortMappingProps as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPrivateConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPrivateConnectionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPrivateConnectionCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPrivateConnectionCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPrivateConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: INetSharingPrivateConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPrivateConnectionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetSharingPublicConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetSharingPublicConnectionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPublicConnectionCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetSharingPublicConnectionCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPublicConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: INetSharingPublicConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetSharingPublicConnectionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IStaticPortMappingImpl: Sized + IDispatchImpl {
    fn ExternalIPAddress();
    fn ExternalPort();
    fn InternalPort();
    fn Protocol();
    fn InternalClient();
    fn Enabled();
    fn Description();
    fn EditInternalClient();
    fn Enable();
    fn EditDescription();
    fn EditInternalPort();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IStaticPortMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStaticPortMappingVtbl {
        unsafe extern "system" fn ExternalIPAddress<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExternalPort<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InternalPort<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Protocol<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InternalClient<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditInternalClient<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enable<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditDescription<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditInternalPort<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ExternalIPAddress::<Impl, IMPL_OFFSET>,
            ExternalPort::<Impl, IMPL_OFFSET>,
            InternalPort::<Impl, IMPL_OFFSET>,
            Protocol::<Impl, IMPL_OFFSET>,
            InternalClient::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            EditInternalClient::<Impl, IMPL_OFFSET>,
            Enable::<Impl, IMPL_OFFSET>,
            EditDescription::<Impl, IMPL_OFFSET>,
            EditInternalPort::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStaticPortMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IStaticPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
    fn Add();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IStaticPortMappingCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStaticPortMappingCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStaticPortMappingCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUPnPNATImpl: Sized + IDispatchImpl {
    fn StaticPortMappingCollection();
    fn DynamicPortMappingCollection();
    fn NATEventManager();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUPnPNATVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPNATImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUPnPNATVtbl {
        unsafe extern "system" fn StaticPortMappingCollection<Impl: IUPnPNATImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DynamicPortMappingCollection<Impl: IUPnPNATImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdpms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NATEventManager<Impl: IUPnPNATImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, StaticPortMappingCollection::<Impl, IMPL_OFFSET>, DynamicPortMappingCollection::<Impl, IMPL_OFFSET>, NATEventManager::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUPnPNAT as ::windows::core::Interface>::IID
    }
}
