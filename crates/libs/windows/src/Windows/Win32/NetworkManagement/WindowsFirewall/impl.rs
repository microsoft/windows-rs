#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDynamicPortMapping {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IDynamicPortMapping";
}
#[cfg(feature = "Win32_System_Com")]
impl IDynamicPortMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingImpl, const OFFSET: isize>() -> IDynamicPortMappingVtbl {
        unsafe extern "system" fn ExternalIPAddress<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalIPAddress(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteHost<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteHost(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalPort(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalPort(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalClient(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseDuration<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeaseDuration(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewLease<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewLease(lleasedurationdesired, ::core::mem::transmute_copy(&pleasedurationreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditInternalClient(&*(&bstrinternalclient as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable(vb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditDescription<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalPort<Impl: IDynamicPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditInternalPort(linternalport) {
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
            ::windows::core::GetRuntimeClassName::<IDynamicPortMapping>,
            ::windows::core::GetTrustLevel,
            ExternalIPAddress::<Impl, OFFSET>,
            RemoteHost::<Impl, OFFSET>,
            ExternalPort::<Impl, OFFSET>,
            Protocol::<Impl, OFFSET>,
            InternalPort::<Impl, OFFSET>,
            InternalClient::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            LeaseDuration::<Impl, OFFSET>,
            RenewLease::<Impl, OFFSET>,
            EditInternalClient::<Impl, OFFSET>,
            Enable::<Impl, OFFSET>,
            EditDescription::<Impl, OFFSET>,
            EditInternalPort::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDynamicPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
    fn Add();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDynamicPortMappingCollection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IDynamicPortMappingCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IDynamicPortMappingCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>() -> IDynamicPortMappingCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&bstrremotehost as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lexternalport, &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdpm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&bstrremotehost as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lexternalport, &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IDynamicPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrremotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lleaseduration: i32, ppdpm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(
                &*(&bstrremotehost as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lexternalport,
                &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                linternalport,
                &*(&bstrinternalclient as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                benabled,
                &*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lleaseduration,
                ::core::mem::transmute_copy(&ppdpm),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDynamicPortMappingCollection>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Add::<Impl, OFFSET>)
    }
}
pub trait IEnumNetConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNetConnection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IEnumNetConnection";
}
impl IEnumNetConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetConnectionImpl, const OFFSET: isize>() -> IEnumNetConnectionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNetConnection>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumNetSharingEveryConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNetSharingEveryConnection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IEnumNetSharingEveryConnection";
}
impl IEnumNetSharingEveryConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>() -> IEnumNetSharingEveryConnectionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingEveryConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNetSharingEveryConnection>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumNetSharingPortMappingImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNetSharingPortMapping {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IEnumNetSharingPortMapping";
}
impl IEnumNetSharingPortMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>() -> IEnumNetSharingPortMappingVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNetSharingPortMapping>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumNetSharingPrivateConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNetSharingPrivateConnection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IEnumNetSharingPrivateConnection";
}
impl IEnumNetSharingPrivateConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>() -> IEnumNetSharingPrivateConnectionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPrivateConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNetSharingPrivateConnection>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumNetSharingPublicConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNetSharingPublicConnection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IEnumNetSharingPublicConnection";
}
impl IEnumNetSharingPublicConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>() -> IEnumNetSharingPublicConnectionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetSharingPublicConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNetSharingPublicConnection>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INATEventManagerImpl: Sized + IDispatchImpl {
    fn SetExternalIPAddressCallback();
    fn SetNumberOfEntriesCallback();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INATEventManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INATEventManager";
}
#[cfg(feature = "Win32_System_Com")]
impl INATEventManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATEventManagerImpl, const OFFSET: isize>() -> INATEventManagerVtbl {
        unsafe extern "system" fn SetExternalIPAddressCallback<Impl: INATEventManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExternalIPAddressCallback(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfEntriesCallback<Impl: INATEventManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNumberOfEntriesCallback(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INATEventManager>, ::windows::core::GetTrustLevel, SetExternalIPAddressCallback::<Impl, OFFSET>, SetNumberOfEntriesCallback::<Impl, OFFSET>)
    }
}
pub trait INATExternalIPAddressCallbackImpl: Sized {
    fn NewExternalIPAddress();
}
impl ::windows::core::RuntimeName for INATExternalIPAddressCallback {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INATExternalIPAddressCallback";
}
impl INATExternalIPAddressCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATExternalIPAddressCallbackImpl, const OFFSET: isize>() -> INATExternalIPAddressCallbackVtbl {
        unsafe extern "system" fn NewExternalIPAddress<Impl: INATExternalIPAddressCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnewexternalipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewExternalIPAddress(&*(&bstrnewexternalipaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INATExternalIPAddressCallback>, ::windows::core::GetTrustLevel, NewExternalIPAddress::<Impl, OFFSET>)
    }
}
pub trait INATNumberOfEntriesCallbackImpl: Sized {
    fn NewNumberOfEntries();
}
impl ::windows::core::RuntimeName for INATNumberOfEntriesCallback {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INATNumberOfEntriesCallback";
}
impl INATNumberOfEntriesCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INATNumberOfEntriesCallbackImpl, const OFFSET: isize>() -> INATNumberOfEntriesCallbackVtbl {
        unsafe extern "system" fn NewNumberOfEntries<Impl: INATNumberOfEntriesCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnewnumberofentries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewNumberOfEntries(lnewnumberofentries) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INATNumberOfEntriesCallback>, ::windows::core::GetTrustLevel, NewNumberOfEntries::<Impl, OFFSET>)
    }
}
pub trait INetConnectionImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Delete();
    fn Duplicate();
    fn GetProperties();
    fn GetUiObjectClassId();
    fn Rename();
}
impl ::windows::core::RuntimeName for INetConnection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetConnection";
}
impl INetConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionImpl, const OFFSET: isize>() -> INetConnectionVtbl {
        unsafe extern "system" fn Connect<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duplicate<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwduplicatename: super::super::Foundation::PWSTR, ppcon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duplicate(&*(&pszwduplicatename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&ppprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiObjectClassId<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUiObjectClassId(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: INetConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwnewname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rename(&*(&pszwnewname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetConnection>, ::windows::core::GetTrustLevel, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>, Delete::<Impl, OFFSET>, Duplicate::<Impl, OFFSET>, GetProperties::<Impl, OFFSET>, GetUiObjectClassId::<Impl, OFFSET>, Rename::<Impl, OFFSET>)
    }
}
pub trait INetConnectionConnectUiImpl: Sized {
    fn SetConnection();
    fn Connect();
    fn Disconnect();
}
impl ::windows::core::RuntimeName for INetConnectionConnectUi {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetConnectionConnectUi";
}
impl INetConnectionConnectUiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionConnectUiImpl, const OFFSET: isize>() -> INetConnectionConnectUiVtbl {
        unsafe extern "system" fn SetConnection<Impl: INetConnectionConnectUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcon: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnection(&*(&pcon as *const <INetConnection as ::windows::core::Abi>::Abi as *const <INetConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: INetConnectionConnectUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: INetConnectionConnectUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetConnectionConnectUi>, ::windows::core::GetTrustLevel, SetConnection::<Impl, OFFSET>, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>)
    }
}
pub trait INetConnectionManagerImpl: Sized {
    fn EnumConnections();
}
impl ::windows::core::RuntimeName for INetConnectionManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetConnectionManager";
}
impl INetConnectionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionManagerImpl, const OFFSET: isize>() -> INetConnectionManagerVtbl {
        unsafe extern "system" fn EnumConnections<Impl: INetConnectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumConnections(flags, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetConnectionManager>, ::windows::core::GetTrustLevel, EnumConnections::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetConnectionPropsImpl: Sized + IDispatchImpl {
    fn Guid();
    fn Name();
    fn DeviceName();
    fn Status();
    fn MediaType();
    fn Characteristics();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetConnectionProps {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetConnectionProps";
}
#[cfg(feature = "Win32_System_Com")]
impl INetConnectionPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetConnectionPropsImpl, const OFFSET: isize>() -> INetConnectionPropsVtbl {
        unsafe extern "system" fn Guid<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid(::core::mem::transmute_copy(&pbstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName(::core::mem::transmute_copy(&pbstrdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType(::core::mem::transmute_copy(&pmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Impl: INetConnectionPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristics(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetConnectionProps>, ::windows::core::GetTrustLevel, Guid::<Impl, OFFSET>, Name::<Impl, OFFSET>, DeviceName::<Impl, OFFSET>, Status::<Impl, OFFSET>, MediaType::<Impl, OFFSET>, Characteristics::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwAuthorizedApplication {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwAuthorizedApplication";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwAuthorizedApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>() -> INetFwAuthorizedApplicationVtbl {
        unsafe extern "system" fn Name<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessImageFileName<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessImageFileName(::core::mem::transmute_copy(&imagefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessImageFileName<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProcessImageFileName(&*(&imagefilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpVersion(::core::mem::transmute_copy(&ipversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpVersion(ipversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope(::core::mem::transmute_copy(&scope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScope(scope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteAddresses(&*(&remoteaddrs as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwAuthorizedApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(enabled) {
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
            ::windows::core::GetRuntimeClassName::<INetFwAuthorizedApplication>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            ProcessImageFileName::<Impl, OFFSET>,
            SetProcessImageFileName::<Impl, OFFSET>,
            IpVersion::<Impl, OFFSET>,
            SetIpVersion::<Impl, OFFSET>,
            Scope::<Impl, OFFSET>,
            SetScope::<Impl, OFFSET>,
            RemoteAddresses::<Impl, OFFSET>,
            SetRemoteAddresses::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwAuthorizedApplicationsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwAuthorizedApplications {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwAuthorizedApplications";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwAuthorizedApplicationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>() -> INetFwAuthorizedApplicationsVtbl {
        unsafe extern "system" fn Count<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, app: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&app as *const <INetFwAuthorizedApplication as ::windows::core::Abi>::Abi as *const <INetFwAuthorizedApplication as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&imagefilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, app: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&imagefilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&app)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwAuthorizedApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwAuthorizedApplications>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwIcmpSettings {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwIcmpSettings";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwIcmpSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>() -> INetFwIcmpSettingsVtbl {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundDestinationUnreachable(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowOutboundDestinationUnreachable(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowRedirect<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowRedirect(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowRedirect<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowRedirect(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInboundEchoRequest(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowInboundEchoRequest(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundTimeExceeded(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowOutboundTimeExceeded(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundParameterProblem(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowOutboundParameterProblem(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundSourceQuench(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowOutboundSourceQuench(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInboundRouterRequest(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowInboundRouterRequest(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInboundTimestampRequest(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowInboundTimestampRequest(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInboundMaskRequest(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowInboundMaskRequest(allow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutboundPacketTooBig(::core::mem::transmute_copy(&allow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Impl: INetFwIcmpSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowOutboundPacketTooBig(allow) {
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
            ::windows::core::GetRuntimeClassName::<INetFwIcmpSettings>,
            ::windows::core::GetTrustLevel,
            AllowOutboundDestinationUnreachable::<Impl, OFFSET>,
            SetAllowOutboundDestinationUnreachable::<Impl, OFFSET>,
            AllowRedirect::<Impl, OFFSET>,
            SetAllowRedirect::<Impl, OFFSET>,
            AllowInboundEchoRequest::<Impl, OFFSET>,
            SetAllowInboundEchoRequest::<Impl, OFFSET>,
            AllowOutboundTimeExceeded::<Impl, OFFSET>,
            SetAllowOutboundTimeExceeded::<Impl, OFFSET>,
            AllowOutboundParameterProblem::<Impl, OFFSET>,
            SetAllowOutboundParameterProblem::<Impl, OFFSET>,
            AllowOutboundSourceQuench::<Impl, OFFSET>,
            SetAllowOutboundSourceQuench::<Impl, OFFSET>,
            AllowInboundRouterRequest::<Impl, OFFSET>,
            SetAllowInboundRouterRequest::<Impl, OFFSET>,
            AllowInboundTimestampRequest::<Impl, OFFSET>,
            SetAllowInboundTimestampRequest::<Impl, OFFSET>,
            AllowInboundMaskRequest::<Impl, OFFSET>,
            SetAllowInboundMaskRequest::<Impl, OFFSET>,
            AllowOutboundPacketTooBig::<Impl, OFFSET>,
            SetAllowOutboundPacketTooBig::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwMgrImpl: Sized + IDispatchImpl {
    fn LocalPolicy();
    fn CurrentProfileType();
    fn RestoreDefaults();
    fn IsPortAllowed();
    fn IsIcmpTypeAllowed();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwMgr {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwMgr";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwMgrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwMgrImpl, const OFFSET: isize>() -> INetFwMgrVtbl {
        unsafe extern "system" fn LocalPolicy<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPolicy(::core::mem::transmute_copy(&localpolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfileType<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProfileType(::core::mem::transmute_copy(&profiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDefaults<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestoreDefaults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPortAllowed<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPortAllowed(&*(&imagefilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ipversion, portnumber, &*(&localaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ipprotocol, ::core::mem::transmute_copy(&allowed), ::core::mem::transmute_copy(&restricted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Impl: INetFwMgrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: u8, allowed: *mut super::super::System::Com::VARIANT, restricted: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIcmpTypeAllowed(ipversion, &*(&localaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&allowed), ::core::mem::transmute_copy(&restricted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwMgr>, ::windows::core::GetTrustLevel, LocalPolicy::<Impl, OFFSET>, CurrentProfileType::<Impl, OFFSET>, RestoreDefaults::<Impl, OFFSET>, IsPortAllowed::<Impl, OFFSET>, IsIcmpTypeAllowed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwOpenPort {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwOpenPort";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwOpenPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPortImpl, const OFFSET: isize>() -> INetFwOpenPortVtbl {
        unsafe extern "system" fn Name<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpVersion(::core::mem::transmute_copy(&ipversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpVersion(ipversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol(::core::mem::transmute_copy(&ipprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProtocol(ipprotocol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Port<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Port(::core::mem::transmute_copy(&portnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPort(portnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope(::core::mem::transmute_copy(&scope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScope(scope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteAddresses(&*(&remoteaddrs as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuiltIn<Impl: INetFwOpenPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, builtin: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuiltIn(::core::mem::transmute_copy(&builtin)) {
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
            ::windows::core::GetRuntimeClassName::<INetFwOpenPort>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            IpVersion::<Impl, OFFSET>,
            SetIpVersion::<Impl, OFFSET>,
            Protocol::<Impl, OFFSET>,
            SetProtocol::<Impl, OFFSET>,
            Port::<Impl, OFFSET>,
            SetPort::<Impl, OFFSET>,
            Scope::<Impl, OFFSET>,
            SetScope::<Impl, OFFSET>,
            RemoteAddresses::<Impl, OFFSET>,
            SetRemoteAddresses::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            BuiltIn::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwOpenPortsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwOpenPorts {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwOpenPorts";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwOpenPortsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwOpenPortsImpl, const OFFSET: isize>() -> INetFwOpenPortsVtbl {
        unsafe extern "system" fn Count<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, port: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&port as *const <INetFwOpenPort as ::windows::core::Abi>::Abi as *const <INetFwOpenPort as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(portnumber, ipprotocol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(portnumber, ipprotocol, ::core::mem::transmute_copy(&openport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwOpenPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwOpenPorts>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwPolicyImpl: Sized + IDispatchImpl {
    fn CurrentProfile();
    fn GetProfileByType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwPolicy {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwPolicy";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicyImpl, const OFFSET: isize>() -> INetFwPolicyVtbl {
        unsafe extern "system" fn CurrentProfile<Impl: INetFwPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProfile(::core::mem::transmute_copy(&profile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileByType<Impl: INetFwPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfileByType(profiletype, ::core::mem::transmute_copy(&profile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwPolicy>, ::windows::core::GetTrustLevel, CurrentProfile::<Impl, OFFSET>, GetProfileByType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwPolicy2 {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwPolicy2";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwPolicy2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwPolicy2Impl, const OFFSET: isize>() -> INetFwPolicy2Vtbl {
        unsafe extern "system" fn CurrentProfileTypes<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProfileTypes(::core::mem::transmute_copy(&profiletypesbitmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallEnabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallEnabled(profiletype, ::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFirewallEnabled(profiletype, enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludedInterfaces<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExcludedInterfaces(profiletype, ::core::mem::transmute_copy(&interfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludedInterfaces<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExcludedInterfaces(profiletype, &*(&interfaces as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockAllInboundTraffic<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockAllInboundTraffic(profiletype, ::core::mem::transmute_copy(&block)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAllInboundTraffic<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlockAllInboundTraffic(profiletype, block) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotificationsDisabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationsDisabled(profiletype, ::core::mem::transmute_copy(&disabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotificationsDisabled(profiletype, disabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicastResponsesToMulticastBroadcastDisabled(profiletype, ::core::mem::transmute_copy(&disabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUnicastResponsesToMulticastBroadcastDisabled(profiletype, disabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rules(::core::mem::transmute_copy(&rules)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceRestriction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicerestriction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceRestriction(::core::mem::transmute_copy(&servicerestriction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableRuleGroup<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableRuleGroup(profiletypesbitmask, &*(&group as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), enable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRuleGroupEnabled(profiletypesbitmask, &*(&group as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestoreLocalFirewallDefaults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInboundAction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultInboundAction(profiletype, ::core::mem::transmute_copy(&action)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInboundAction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultInboundAction(profiletype, action) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultOutboundAction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultOutboundAction(profiletype, ::core::mem::transmute_copy(&action)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultOutboundAction<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultOutboundAction(profiletype, action) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRuleGroupCurrentlyEnabled<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRuleGroupCurrentlyEnabled(&*(&group as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPolicyModifyState<Impl: INetFwPolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPolicyModifyState(::core::mem::transmute_copy(&modifystate)) {
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
            ::windows::core::GetRuntimeClassName::<INetFwPolicy2>,
            ::windows::core::GetTrustLevel,
            CurrentProfileTypes::<Impl, OFFSET>,
            FirewallEnabled::<Impl, OFFSET>,
            SetFirewallEnabled::<Impl, OFFSET>,
            ExcludedInterfaces::<Impl, OFFSET>,
            SetExcludedInterfaces::<Impl, OFFSET>,
            BlockAllInboundTraffic::<Impl, OFFSET>,
            SetBlockAllInboundTraffic::<Impl, OFFSET>,
            NotificationsDisabled::<Impl, OFFSET>,
            SetNotificationsDisabled::<Impl, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled::<Impl, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled::<Impl, OFFSET>,
            Rules::<Impl, OFFSET>,
            ServiceRestriction::<Impl, OFFSET>,
            EnableRuleGroup::<Impl, OFFSET>,
            IsRuleGroupEnabled::<Impl, OFFSET>,
            RestoreLocalFirewallDefaults::<Impl, OFFSET>,
            DefaultInboundAction::<Impl, OFFSET>,
            SetDefaultInboundAction::<Impl, OFFSET>,
            DefaultOutboundAction::<Impl, OFFSET>,
            SetDefaultOutboundAction::<Impl, OFFSET>,
            IsRuleGroupCurrentlyEnabled::<Impl, OFFSET>,
            LocalPolicyModifyState::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwProductImpl: Sized + IDispatchImpl {
    fn RuleCategories();
    fn SetRuleCategories();
    fn DisplayName();
    fn SetDisplayName();
    fn PathToSignedProductExe();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwProduct {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwProduct";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwProductVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProductImpl, const OFFSET: isize>() -> INetFwProductVtbl {
        unsafe extern "system" fn RuleCategories<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulecategories: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuleCategories(::core::mem::transmute_copy(&rulecategories)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleCategories<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulecategories: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRuleCategories(&*(&rulecategories as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName(::core::mem::transmute_copy(&displayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisplayName(&*(&displayname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathToSignedProductExe<Impl: INetFwProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathToSignedProductExe(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwProduct>, ::windows::core::GetTrustLevel, RuleCategories::<Impl, OFFSET>, SetRuleCategories::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, SetDisplayName::<Impl, OFFSET>, PathToSignedProductExe::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwProductsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Register();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwProducts {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwProducts";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwProductsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProductsImpl, const OFFSET: isize>() -> INetFwProductsVtbl {
        unsafe extern "system" fn Count<Impl: INetFwProductsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: INetFwProductsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, product: ::windows::core::RawPtr, registration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register(&*(&product as *const <INetFwProduct as ::windows::core::Abi>::Abi as *const <INetFwProduct as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&registration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: INetFwProductsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, product: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&product)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwProductsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwProducts>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Register::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwProfile {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwProfile";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwProfileImpl, const OFFSET: isize>() -> INetFwProfileVtbl {
        unsafe extern "system" fn Type<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallEnabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallEnabled(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFirewallEnabled(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExceptionsNotAllowed(::core::mem::transmute_copy(&notallowed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notallowed: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExceptionsNotAllowed(notallowed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotificationsDisabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationsDisabled(::core::mem::transmute_copy(&disabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotificationsDisabled(disabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnicastResponsesToMulticastBroadcastDisabled(::core::mem::transmute_copy(&disabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUnicastResponsesToMulticastBroadcastDisabled(disabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAdminSettings<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteadminsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAdminSettings(::core::mem::transmute_copy(&remoteadminsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IcmpSettings<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmpsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IcmpSettings(::core::mem::transmute_copy(&icmpsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GloballyOpenPorts(::core::mem::transmute_copy(&openports)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, services: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Services(::core::mem::transmute_copy(&services)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthorizedApplications<Impl: INetFwProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorizedApplications(::core::mem::transmute_copy(&apps)) {
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
            ::windows::core::GetRuntimeClassName::<INetFwProfile>,
            ::windows::core::GetTrustLevel,
            Type::<Impl, OFFSET>,
            FirewallEnabled::<Impl, OFFSET>,
            SetFirewallEnabled::<Impl, OFFSET>,
            ExceptionsNotAllowed::<Impl, OFFSET>,
            SetExceptionsNotAllowed::<Impl, OFFSET>,
            NotificationsDisabled::<Impl, OFFSET>,
            SetNotificationsDisabled::<Impl, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled::<Impl, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled::<Impl, OFFSET>,
            RemoteAdminSettings::<Impl, OFFSET>,
            IcmpSettings::<Impl, OFFSET>,
            GloballyOpenPorts::<Impl, OFFSET>,
            Services::<Impl, OFFSET>,
            AuthorizedApplications::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwRemoteAdminSettings {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwRemoteAdminSettings";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRemoteAdminSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>() -> INetFwRemoteAdminSettingsVtbl {
        unsafe extern "system" fn IpVersion<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpVersion(::core::mem::transmute_copy(&ipversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpVersion(ipversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope(::core::mem::transmute_copy(&scope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScope(scope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteAddresses(&*(&remoteaddrs as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwRemoteAdminSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(enabled) {
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
            ::windows::core::GetRuntimeClassName::<INetFwRemoteAdminSettings>,
            ::windows::core::GetTrustLevel,
            IpVersion::<Impl, OFFSET>,
            SetIpVersion::<Impl, OFFSET>,
            Scope::<Impl, OFFSET>,
            SetScope::<Impl, OFFSET>,
            RemoteAddresses::<Impl, OFFSET>,
            SetRemoteAddresses::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwRule {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwRule";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRuleImpl, const OFFSET: isize>() -> INetFwRuleVtbl {
        unsafe extern "system" fn Name<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&desc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&desc as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationName(::core::mem::transmute_copy(&imagefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagefilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplicationName(&*(&imagefilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceName(::core::mem::transmute_copy(&servicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceName<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServiceName(&*(&servicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol(::core::mem::transmute_copy(&protocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProtocol(protocol) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPorts<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPorts(::core::mem::transmute_copy(&portnumbers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPorts<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalPorts(&*(&portnumbers as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePorts<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePorts(::core::mem::transmute_copy(&portnumbers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemotePorts<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portnumbers: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemotePorts(&*(&portnumbers as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAddresses<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddresses(::core::mem::transmute_copy(&localaddrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddresses<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalAddresses(&*(&localaddrs as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteAddresses(&*(&remoteaddrs as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IcmpTypesAndCodes(::core::mem::transmute_copy(&icmptypesandcodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIcmpTypesAndCodes(&*(&icmptypesandcodes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction(::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDirection(dir) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Interfaces<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interfaces(::core::mem::transmute_copy(&interfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaces<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInterfaces(&*(&interfaces as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceTypes<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacetypes: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceTypes(::core::mem::transmute_copy(&interfacetypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceTypes<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacetypes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInterfaceTypes(&*(&interfacetypes as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Grouping<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Grouping(::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGrouping<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGrouping(&*(&context as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profiles<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profiles(::core::mem::transmute_copy(&profiletypesbitmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfiles<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProfiles(profiletypesbitmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EdgeTraversal<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EdgeTraversal(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversal<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEdgeTraversal(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Action<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Action(::core::mem::transmute_copy(&action)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAction<Impl: INetFwRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAction(action) {
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
            ::windows::core::GetRuntimeClassName::<INetFwRule>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            ApplicationName::<Impl, OFFSET>,
            SetApplicationName::<Impl, OFFSET>,
            ServiceName::<Impl, OFFSET>,
            SetServiceName::<Impl, OFFSET>,
            Protocol::<Impl, OFFSET>,
            SetProtocol::<Impl, OFFSET>,
            LocalPorts::<Impl, OFFSET>,
            SetLocalPorts::<Impl, OFFSET>,
            RemotePorts::<Impl, OFFSET>,
            SetRemotePorts::<Impl, OFFSET>,
            LocalAddresses::<Impl, OFFSET>,
            SetLocalAddresses::<Impl, OFFSET>,
            RemoteAddresses::<Impl, OFFSET>,
            SetRemoteAddresses::<Impl, OFFSET>,
            IcmpTypesAndCodes::<Impl, OFFSET>,
            SetIcmpTypesAndCodes::<Impl, OFFSET>,
            Direction::<Impl, OFFSET>,
            SetDirection::<Impl, OFFSET>,
            Interfaces::<Impl, OFFSET>,
            SetInterfaces::<Impl, OFFSET>,
            InterfaceTypes::<Impl, OFFSET>,
            SetInterfaceTypes::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            Grouping::<Impl, OFFSET>,
            SetGrouping::<Impl, OFFSET>,
            Profiles::<Impl, OFFSET>,
            SetProfiles::<Impl, OFFSET>,
            EdgeTraversal::<Impl, OFFSET>,
            SetEdgeTraversal::<Impl, OFFSET>,
            Action::<Impl, OFFSET>,
            SetAction::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRule2Impl: Sized + INetFwRuleImpl + IDispatchImpl {
    fn EdgeTraversalOptions();
    fn SetEdgeTraversalOptions();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwRule2 {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwRule2";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule2Impl, const OFFSET: isize>() -> INetFwRule2Vtbl {
        unsafe extern "system" fn EdgeTraversalOptions<Impl: INetFwRule2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EdgeTraversalOptions(::core::mem::transmute_copy(&loptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Impl: INetFwRule2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEdgeTraversalOptions(loptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwRule2>, ::windows::core::GetTrustLevel, EdgeTraversalOptions::<Impl, OFFSET>, SetEdgeTraversalOptions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwRule3 {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwRule3";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRule3Impl, const OFFSET: isize>() -> INetFwRule3Vtbl {
        unsafe extern "system" fn LocalAppPackageId<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpackageid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAppPackageId(::core::mem::transmute_copy(&wszpackageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAppPackageId<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpackageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalAppPackageId(&*(&wszpackageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalUserOwner<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserowner: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserOwner(::core::mem::transmute_copy(&wszuserowner)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserOwner<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserowner: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalUserOwner(&*(&wszuserowner as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalUserAuthorizedList(::core::mem::transmute_copy(&wszuserauthlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalUserAuthorizedList(&*(&wszuserauthlist as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteUserAuthorizedList(::core::mem::transmute_copy(&wszuserauthlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteUserAuthorizedList(&*(&wszuserauthlist as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteMachineAuthorizedList(::core::mem::transmute_copy(&wszuserauthlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuserauthlist: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteMachineAuthorizedList(&*(&wszuserauthlist as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecureFlags<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecureFlags(::core::mem::transmute_copy(&loptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecureFlags<Impl: INetFwRule3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSecureFlags(loptions) {
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
            ::windows::core::GetRuntimeClassName::<INetFwRule3>,
            ::windows::core::GetTrustLevel,
            LocalAppPackageId::<Impl, OFFSET>,
            SetLocalAppPackageId::<Impl, OFFSET>,
            LocalUserOwner::<Impl, OFFSET>,
            SetLocalUserOwner::<Impl, OFFSET>,
            LocalUserAuthorizedList::<Impl, OFFSET>,
            SetLocalUserAuthorizedList::<Impl, OFFSET>,
            RemoteUserAuthorizedList::<Impl, OFFSET>,
            SetRemoteUserAuthorizedList::<Impl, OFFSET>,
            RemoteMachineAuthorizedList::<Impl, OFFSET>,
            SetRemoteMachineAuthorizedList::<Impl, OFFSET>,
            SecureFlags::<Impl, OFFSET>,
            SetSecureFlags::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRulesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwRules {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwRules";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRulesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwRulesImpl, const OFFSET: isize>() -> INetFwRulesVtbl {
        unsafe extern "system" fn Count<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&rule as *const <INetFwRule as ::windows::core::Abi>::Abi as *const <INetFwRule as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwRules>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwService {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwService";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceImpl, const OFFSET: isize>() -> INetFwServiceVtbl {
        unsafe extern "system" fn Name<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Customized<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customized: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Customized(::core::mem::transmute_copy(&customized)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpVersion<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpVersion(::core::mem::transmute_copy(&ipversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpVersion(ipversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope(::core::mem::transmute_copy(&scope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScope(scope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddresses<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddresses(::core::mem::transmute_copy(&remoteaddrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaddrs: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteAddresses(&*(&remoteaddrs as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Impl: INetFwServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GloballyOpenPorts(::core::mem::transmute_copy(&openports)) {
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
            ::windows::core::GetRuntimeClassName::<INetFwService>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            Customized::<Impl, OFFSET>,
            IpVersion::<Impl, OFFSET>,
            SetIpVersion::<Impl, OFFSET>,
            Scope::<Impl, OFFSET>,
            SetScope::<Impl, OFFSET>,
            RemoteAddresses::<Impl, OFFSET>,
            SetRemoteAddresses::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            SetEnabled::<Impl, OFFSET>,
            GloballyOpenPorts::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwServiceRestrictionImpl: Sized + IDispatchImpl {
    fn RestrictService();
    fn ServiceRestricted();
    fn Rules();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwServiceRestriction {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwServiceRestriction";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwServiceRestrictionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServiceRestrictionImpl, const OFFSET: isize>() -> INetFwServiceRestrictionVtbl {
        unsafe extern "system" fn RestrictService<Impl: INetFwServiceRestrictionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, restrictservice: i16, servicesidrestricted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestrictService(&*(&servicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&appname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), restrictservice, servicesidrestricted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceRestricted<Impl: INetFwServiceRestrictionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servicerestricted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceRestricted(&*(&servicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&appname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&servicerestricted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Impl: INetFwServiceRestrictionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rules(::core::mem::transmute_copy(&rules)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwServiceRestriction>, ::windows::core::GetTrustLevel, RestrictService::<Impl, OFFSET>, ServiceRestricted::<Impl, OFFSET>, Rules::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwServicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetFwServices {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetFwServices";
}
#[cfg(feature = "Win32_System_Com")]
impl INetFwServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetFwServicesImpl, const OFFSET: isize>() -> INetFwServicesVtbl {
        unsafe extern "system" fn Count<Impl: INetFwServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: INetFwServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(svctype, ::core::mem::transmute_copy(&service)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: INetFwServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetFwServices>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetSharingConfiguration {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetSharingConfiguration";
}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingConfigurationImpl, const OFFSET: isize>() -> INetSharingConfigurationVtbl {
        unsafe extern "system" fn SharingEnabled<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingEnabled(::core::mem::transmute_copy(&pbenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingConnectionType<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingConnectionType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableSharing<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableSharing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableSharing<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableSharing(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternetFirewallEnabled<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternetFirewallEnabled(::core::mem::transmute_copy(&pbenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableInternetFirewall<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableInternetFirewall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableInternetFirewall<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableInternetFirewall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPortMappings<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPortMappings(flags, ::core::mem::transmute_copy(&ppcoll)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortMapping<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPortMapping(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ucipprotocol, usexternalport, usinternalport, dwoptions, &*(&bstrtargetnameoripaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), etargettype, ::core::mem::transmute_copy(&ppmapping)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortMapping<Impl: INetSharingConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmapping: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePortMapping(&*(&pmapping as *const <INetSharingPortMapping as ::windows::core::Abi>::Abi as *const <INetSharingPortMapping as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<INetSharingConfiguration>,
            ::windows::core::GetTrustLevel,
            SharingEnabled::<Impl, OFFSET>,
            SharingConnectionType::<Impl, OFFSET>,
            DisableSharing::<Impl, OFFSET>,
            EnableSharing::<Impl, OFFSET>,
            InternetFirewallEnabled::<Impl, OFFSET>,
            DisableInternetFirewall::<Impl, OFFSET>,
            EnableInternetFirewall::<Impl, OFFSET>,
            EnumPortMappings::<Impl, OFFSET>,
            AddPortMapping::<Impl, OFFSET>,
            RemovePortMapping::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingEveryConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetSharingEveryConnectionCollection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetSharingEveryConnectionCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingEveryConnectionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingEveryConnectionCollectionImpl, const OFFSET: isize>() -> INetSharingEveryConnectionCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingEveryConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: INetSharingEveryConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetSharingEveryConnectionCollection>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingManagerImpl: Sized + IDispatchImpl {
    fn SharingInstalled();
    fn EnumPublicConnections();
    fn EnumPrivateConnections();
    fn INetSharingConfigurationForINetConnection();
    fn EnumEveryConnection();
    fn NetConnectionProps();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetSharingManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetSharingManager";
}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingManagerImpl, const OFFSET: isize>() -> INetSharingManagerVtbl {
        unsafe extern "system" fn SharingInstalled<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbinstalled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingInstalled(::core::mem::transmute_copy(&pbinstalled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPublicConnections<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPublicConnections(flags, ::core::mem::transmute_copy(&ppcoll)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPrivateConnections<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPrivateConnections(flags, ::core::mem::transmute_copy(&ppcoll)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn INetSharingConfigurationForINetConnection<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppnetsharingconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).INetSharingConfigurationForINetConnection(&*(&pnetconnection as *const <INetConnection as ::windows::core::Abi>::Abi as *const <INetConnection as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnetsharingconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEveryConnection<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcoll: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEveryConnection(::core::mem::transmute_copy(&ppcoll)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetConnectionProps<Impl: INetSharingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetconnection: ::windows::core::RawPtr, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetConnectionProps(&*(&pnetconnection as *const <INetConnection as ::windows::core::Abi>::Abi as *const <INetConnection as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprops)) {
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
            ::windows::core::GetRuntimeClassName::<INetSharingManager>,
            ::windows::core::GetTrustLevel,
            SharingInstalled::<Impl, OFFSET>,
            EnumPublicConnections::<Impl, OFFSET>,
            EnumPrivateConnections::<Impl, OFFSET>,
            INetSharingConfigurationForINetConnection::<Impl, OFFSET>,
            EnumEveryConnection::<Impl, OFFSET>,
            NetConnectionProps::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPortMappingImpl: Sized + IDispatchImpl {
    fn Disable();
    fn Enable();
    fn Properties();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetSharingPortMapping {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetSharingPortMapping";
}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingImpl, const OFFSET: isize>() -> INetSharingPortMappingVtbl {
        unsafe extern "system" fn Disable<Impl: INetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: INetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: INetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnspmp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppnspmp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: INetSharingPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetSharingPortMapping>, ::windows::core::GetTrustLevel, Disable::<Impl, OFFSET>, Enable::<Impl, OFFSET>, Properties::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetSharingPortMappingCollection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetSharingPortMappingCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingCollectionImpl, const OFFSET: isize>() -> INetSharingPortMappingCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: INetSharingPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetSharingPortMappingCollection>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetSharingPortMappingProps {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetSharingPortMappingProps";
}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>() -> INetSharingPortMappingPropsVtbl {
        unsafe extern "system" fn Name<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPProtocol<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucipprot: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPProtocol(::core::mem::transmute_copy(&pucipprot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalPort(::core::mem::transmute_copy(&pusport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalPort(::core::mem::transmute_copy(&pusport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options(::core::mem::transmute_copy(&pdwoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetName<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName(::core::mem::transmute_copy(&pbstrtargetname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetIPAddress<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetIPAddress(::core::mem::transmute_copy(&pbstrtargetipaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: INetSharingPortMappingPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pbool)) {
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
            ::windows::core::GetRuntimeClassName::<INetSharingPortMappingProps>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            IPProtocol::<Impl, OFFSET>,
            ExternalPort::<Impl, OFFSET>,
            InternalPort::<Impl, OFFSET>,
            Options::<Impl, OFFSET>,
            TargetName::<Impl, OFFSET>,
            TargetIPAddress::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPrivateConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetSharingPrivateConnectionCollection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetSharingPrivateConnectionCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPrivateConnectionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPrivateConnectionCollectionImpl, const OFFSET: isize>() -> INetSharingPrivateConnectionCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPrivateConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: INetSharingPrivateConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetSharingPrivateConnectionCollection>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPublicConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetSharingPublicConnectionCollection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.INetSharingPublicConnectionCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPublicConnectionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetSharingPublicConnectionCollectionImpl, const OFFSET: isize>() -> INetSharingPublicConnectionCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: INetSharingPublicConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: INetSharingPublicConnectionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetSharingPublicConnectionCollection>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IStaticPortMapping {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IStaticPortMapping";
}
#[cfg(feature = "Win32_System_Com")]
impl IStaticPortMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingImpl, const OFFSET: isize>() -> IStaticPortMappingVtbl {
        unsafe extern "system" fn ExternalIPAddress<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalIPAddress(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalPort(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalPort(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternalClient(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditInternalClient(&*(&bstrinternalclient as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable(vb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditDescription<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalPort<Impl: IStaticPortMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EditInternalPort(linternalport) {
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
            ::windows::core::GetRuntimeClassName::<IStaticPortMapping>,
            ::windows::core::GetTrustLevel,
            ExternalIPAddress::<Impl, OFFSET>,
            ExternalPort::<Impl, OFFSET>,
            InternalPort::<Impl, OFFSET>,
            Protocol::<Impl, OFFSET>,
            InternalClient::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            EditInternalClient::<Impl, OFFSET>,
            Enable::<Impl, OFFSET>,
            EditDescription::<Impl, OFFSET>,
            EditInternalPort::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStaticPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
    fn Add();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IStaticPortMappingCollection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IStaticPortMappingCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IStaticPortMappingCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>() -> IStaticPortMappingCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(lexternalport, &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppspm)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(lexternalport, &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IStaticPortMappingCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternalport: i32, bstrinternalclient: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, benabled: i16, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppspm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(
                lexternalport,
                &*(&bstrprotocol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                linternalport,
                &*(&bstrinternalclient as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                benabled,
                &*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppspm),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStaticPortMappingCollection>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Add::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPNATImpl: Sized + IDispatchImpl {
    fn StaticPortMappingCollection();
    fn DynamicPortMappingCollection();
    fn NATEventManager();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUPnPNAT {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WindowsFirewall.IUPnPNAT";
}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPNATVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUPnPNATImpl, const OFFSET: isize>() -> IUPnPNATVtbl {
        unsafe extern "system" fn StaticPortMappingCollection<Impl: IUPnPNATImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaticPortMappingCollection(::core::mem::transmute_copy(&ppspms)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicPortMappingCollection<Impl: IUPnPNATImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdpms: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicPortMappingCollection(::core::mem::transmute_copy(&ppdpms)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NATEventManager<Impl: IUPnPNATImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NATEventManager(::core::mem::transmute_copy(&ppnem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUPnPNAT>, ::windows::core::GetTrustLevel, StaticPortMappingCollection::<Impl, OFFSET>, DynamicPortMappingCollection::<Impl, OFFSET>, NATEventManager::<Impl, OFFSET>)
    }
}
