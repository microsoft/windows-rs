pub trait IEnumACDGroupImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumACDGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumACDGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumACDGroupVtbl {
        unsafe extern "system" fn Next<Impl: IEnumACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumACDGroup as ::windows::core::Interface>::IID
    }
}
pub trait IEnumAddressImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAddressVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAddress as ::windows::core::Interface>::IID
    }
}
pub trait IEnumAgentImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumAgentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgentVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgent as ::windows::core::Interface>::IID
    }
}
pub trait IEnumAgentHandlerImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumAgentHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgentHandlerVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgentHandler as ::windows::core::Interface>::IID
    }
}
pub trait IEnumAgentSessionImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumAgentSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgentSessionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgentSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumBstrImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumBstrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBstrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBstrVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBstrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBstrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBstrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBstrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBstr as ::windows::core::Interface>::IID
    }
}
pub trait IEnumCallImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumCallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCallVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCall as ::windows::core::Interface>::IID
    }
}
pub trait IEnumCallHubImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumCallHubVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallHubImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCallHubVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCallHub as ::windows::core::Interface>::IID
    }
}
pub trait IEnumCallingCardImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumCallingCardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallingCardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCallingCardVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCallingCard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumDialableAddrsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumDialableAddrsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDialableAddrsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDialableAddrsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDialableAddrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDialableAddrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDialableAddrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDialableAddrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDialableAddrs as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDirectoryImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumDirectoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDirectoryVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDirectory as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDirectoryObjectImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumDirectoryObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDirectoryObjectVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pval: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDirectoryObject as ::windows::core::Interface>::IID
    }
}
pub trait IEnumLocationImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumLocationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumLocationVtbl {
        unsafe extern "system" fn Next<Impl: IEnumLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumLocation as ::windows::core::Interface>::IID
    }
}
pub trait IEnumMcastScopeImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumMcastScopeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMcastScopeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumMcastScopeVtbl {
        unsafe extern "system" fn Next<Impl: IEnumMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppscopes: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMcastScope as ::windows::core::Interface>::IID
    }
}
pub trait IEnumPhoneImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumPhoneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPhoneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPhoneVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPhone as ::windows::core::Interface>::IID
    }
}
pub trait IEnumPluggableSuperclassInfoImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumPluggableSuperclassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableSuperclassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPluggableSuperclassInfoVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPluggableSuperclassInfo as ::windows::core::Interface>::IID
    }
}
pub trait IEnumPluggableTerminalClassInfoImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumPluggableTerminalClassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableTerminalClassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPluggableTerminalClassInfoVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPluggableTerminalClassInfo as ::windows::core::Interface>::IID
    }
}
pub trait IEnumQueueImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumQueueVtbl {
        unsafe extern "system" fn Next<Impl: IEnumQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumQueue as ::windows::core::Interface>::IID
    }
}
pub trait IEnumStreamImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumStreamVtbl {
        unsafe extern "system" fn Next<Impl: IEnumStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumStream as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSubStreamImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumSubStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSubStreamVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSubStream as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTerminalImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTerminalVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTerminal as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTerminalClassImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl IEnumTerminalClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTerminalClassVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTerminalClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTerminalClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTerminalClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumTerminalClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTerminalClass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastAddressAllocationImpl: Sized + IDispatchImpl {
    fn Scopes();
    fn EnumerateScopes();
    fn RequestAddress();
    fn RenewAddress();
    fn ReleaseAddress();
    fn CreateLeaseInfo();
    fn CreateLeaseInfoFromVariant();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastAddressAllocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastAddressAllocationVtbl {
        unsafe extern "system" fn Scopes<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateScopes<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenummcastscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscope: ::windows::core::RawPtr, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenewAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lreserved: i32, prenewrequest: ::windows::core::RawPtr, pprenewresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preleaserequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLeaseInfo<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const super::super::Foundation::PWSTR, prequestid: super::super::Foundation::PWSTR, pserveraddress: super::super::Foundation::PWSTR, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLeaseInfoFromVariant<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, prequestid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pserveraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Scopes::<Impl, IMPL_OFFSET>,
            EnumerateScopes::<Impl, IMPL_OFFSET>,
            RequestAddress::<Impl, IMPL_OFFSET>,
            RenewAddress::<Impl, IMPL_OFFSET>,
            ReleaseAddress::<Impl, IMPL_OFFSET>,
            CreateLeaseInfo::<Impl, IMPL_OFFSET>,
            CreateLeaseInfoFromVariant::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastAddressAllocation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastLeaseInfoImpl: Sized + IDispatchImpl {
    fn RequestID();
    fn LeaseStartTime();
    fn SetLeaseStartTime();
    fn LeaseStopTime();
    fn SetLeaseStopTime();
    fn AddressCount();
    fn ServerAddress();
    fn TTL();
    fn Addresses();
    fn EnumerateAddresses();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastLeaseInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastLeaseInfoVtbl {
        unsafe extern "system" fn RequestID<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprequestid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LeaseStartTime<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLeaseStartTime<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LeaseStopTime<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLeaseStopTime<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddressCount<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerAddress<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TTL<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Addresses<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddresses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            RequestID::<Impl, IMPL_OFFSET>,
            LeaseStartTime::<Impl, IMPL_OFFSET>,
            SetLeaseStartTime::<Impl, IMPL_OFFSET>,
            LeaseStopTime::<Impl, IMPL_OFFSET>,
            SetLeaseStopTime::<Impl, IMPL_OFFSET>,
            AddressCount::<Impl, IMPL_OFFSET>,
            ServerAddress::<Impl, IMPL_OFFSET>,
            TTL::<Impl, IMPL_OFFSET>,
            Addresses::<Impl, IMPL_OFFSET>,
            EnumerateAddresses::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastLeaseInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastScopeImpl: Sized + IDispatchImpl {
    fn ScopeID();
    fn ServerID();
    fn InterfaceID();
    fn ScopeDescription();
    fn TTL();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastScopeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScopeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastScopeVtbl {
        unsafe extern "system" fn ScopeID<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerID<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterfaceID<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScopeDescription<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TTL<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ScopeID::<Impl, IMPL_OFFSET>, ServerID::<Impl, IMPL_OFFSET>, InterfaceID::<Impl, IMPL_OFFSET>, ScopeDescription::<Impl, IMPL_OFFSET>, TTL::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastScope as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITACDGroupImpl: Sized + IDispatchImpl {
    fn Name();
    fn EnumerateQueues();
    fn Queues();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITACDGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITACDGroupVtbl {
        unsafe extern "system" fn Name<Impl: ITACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateQueues<Impl: ITACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Queues<Impl: ITACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, EnumerateQueues::<Impl, IMPL_OFFSET>, Queues::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITACDGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITACDGroupEventImpl: Sized + IDispatchImpl {
    fn Group();
    fn Event();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITACDGroupEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroupEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITACDGroupEventVtbl {
        unsafe extern "system" fn Group<Impl: ITACDGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITACDGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Group::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITACDGroupEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAMMediaFormatImpl: Sized {
    fn MediaFormat();
    fn SetMediaFormat();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ITAMMediaFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAMMediaFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAMMediaFormatVtbl {
        unsafe extern "system" fn MediaFormat<Impl: ITAMMediaFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMediaFormat<Impl: ITAMMediaFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MediaFormat::<Impl, IMPL_OFFSET>, SetMediaFormat::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAMMediaFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITASRTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITASRTerminalEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITASRTerminalEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITASRTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITASRTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Call<Impl: ITASRTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: ITASRTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Terminal::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Error::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITASRTerminalEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressImpl: Sized + IDispatchImpl {
    fn State();
    fn AddressName();
    fn ServiceProviderName();
    fn TAPIObject();
    fn CreateCall();
    fn Calls();
    fn EnumerateCalls();
    fn DialableAddress();
    fn CreateForwardInfoObject();
    fn Forward();
    fn CurrentForwardInfo();
    fn SetMessageWaiting();
    fn MessageWaiting();
    fn SetDoNotDisturb();
    fn DoNotDisturb();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressVtbl {
        unsafe extern "system" fn State<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddressName<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceProviderName<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TAPIObject<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCall<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Calls<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateCalls<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DialableAddress<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdialableaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateForwardInfoObject<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Forward<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pforwardinfo: ::windows::core::RawPtr, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentForwardInfo<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMessageWaiting<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmessagewaiting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MessageWaiting<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmessagewaiting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDoNotDisturb<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdonotdisturb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoNotDisturb<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdonotdisturb: *mut i16) -> ::windows::core::HRESULT {
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
            State::<Impl, IMPL_OFFSET>,
            AddressName::<Impl, IMPL_OFFSET>,
            ServiceProviderName::<Impl, IMPL_OFFSET>,
            TAPIObject::<Impl, IMPL_OFFSET>,
            CreateCall::<Impl, IMPL_OFFSET>,
            Calls::<Impl, IMPL_OFFSET>,
            EnumerateCalls::<Impl, IMPL_OFFSET>,
            DialableAddress::<Impl, IMPL_OFFSET>,
            CreateForwardInfoObject::<Impl, IMPL_OFFSET>,
            Forward::<Impl, IMPL_OFFSET>,
            CurrentForwardInfo::<Impl, IMPL_OFFSET>,
            SetMessageWaiting::<Impl, IMPL_OFFSET>,
            MessageWaiting::<Impl, IMPL_OFFSET>,
            SetDoNotDisturb::<Impl, IMPL_OFFSET>,
            DoNotDisturb::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddress2Impl: Sized + ITAddressImpl + IDispatchImpl {
    fn Phones();
    fn EnumeratePhones();
    fn GetPhoneFromTerminal();
    fn PreferredPhones();
    fn EnumeratePreferredPhones();
    fn EventFilter();
    fn SetEventFilter();
    fn DeviceSpecific();
    fn DeviceSpecificVariant();
    fn NegotiateExtVersion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddress2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddress2Vtbl {
        unsafe extern "system" fn Phones<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePhones<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPhoneFromTerminal<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredPhones<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePreferredPhones<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventFilter<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceSpecific<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NegotiateExtVersion<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
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
            State::<Impl, IMPL_OFFSET>,
            AddressName::<Impl, IMPL_OFFSET>,
            ServiceProviderName::<Impl, IMPL_OFFSET>,
            TAPIObject::<Impl, IMPL_OFFSET>,
            CreateCall::<Impl, IMPL_OFFSET>,
            Calls::<Impl, IMPL_OFFSET>,
            EnumerateCalls::<Impl, IMPL_OFFSET>,
            DialableAddress::<Impl, IMPL_OFFSET>,
            CreateForwardInfoObject::<Impl, IMPL_OFFSET>,
            Forward::<Impl, IMPL_OFFSET>,
            CurrentForwardInfo::<Impl, IMPL_OFFSET>,
            SetMessageWaiting::<Impl, IMPL_OFFSET>,
            MessageWaiting::<Impl, IMPL_OFFSET>,
            SetDoNotDisturb::<Impl, IMPL_OFFSET>,
            DoNotDisturb::<Impl, IMPL_OFFSET>,
            Phones::<Impl, IMPL_OFFSET>,
            EnumeratePhones::<Impl, IMPL_OFFSET>,
            GetPhoneFromTerminal::<Impl, IMPL_OFFSET>,
            PreferredPhones::<Impl, IMPL_OFFSET>,
            EnumeratePreferredPhones::<Impl, IMPL_OFFSET>,
            EventFilter::<Impl, IMPL_OFFSET>,
            SetEventFilter::<Impl, IMPL_OFFSET>,
            DeviceSpecific::<Impl, IMPL_OFFSET>,
            DeviceSpecificVariant::<Impl, IMPL_OFFSET>,
            NegotiateExtVersion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddress2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressCapabilitiesImpl: Sized + IDispatchImpl {
    fn AddressCapability();
    fn AddressCapabilityString();
    fn CallTreatments();
    fn EnumerateCallTreatments();
    fn CompletionMessages();
    fn EnumerateCompletionMessages();
    fn DeviceClasses();
    fn EnumerateDeviceClasses();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressCapabilitiesVtbl {
        unsafe extern "system" fn AddressCapability<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddressCapabilityString<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallTreatments<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateCallTreatments<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcalltreatment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompletionMessages<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateCompletionMessages<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcompletionmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceClasses<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateDeviceClasses<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdeviceclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            AddressCapability::<Impl, IMPL_OFFSET>,
            AddressCapabilityString::<Impl, IMPL_OFFSET>,
            CallTreatments::<Impl, IMPL_OFFSET>,
            EnumerateCallTreatments::<Impl, IMPL_OFFSET>,
            CompletionMessages::<Impl, IMPL_OFFSET>,
            EnumerateCompletionMessages::<Impl, IMPL_OFFSET>,
            DeviceClasses::<Impl, IMPL_OFFSET>,
            EnumerateDeviceClasses::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressDeviceSpecificEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Call();
    fn lParam1();
    fn lParam2();
    fn lParam3();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressDeviceSpecificEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressDeviceSpecificEventVtbl {
        unsafe extern "system" fn Address<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Call<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lParam1<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lParam2<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lParam3<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Address::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, lParam1::<Impl, IMPL_OFFSET>, lParam2::<Impl, IMPL_OFFSET>, lParam3::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressDeviceSpecificEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Event();
    fn Terminal();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressEventVtbl {
        unsafe extern "system" fn Address<Impl: ITAddressEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITAddressEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminal<Impl: ITAddressEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Address::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>, Terminal::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressTranslationImpl: Sized + IDispatchImpl {
    fn TranslateAddress();
    fn TranslateDialog();
    fn EnumerateLocations();
    fn Locations();
    fn EnumerateCallingCards();
    fn CallingCards();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressTranslationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressTranslationVtbl {
        unsafe extern "system" fn TranslateAddress<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresstotranslate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslateDialog<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: isize, paddressin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateLocations<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumlocation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Locations<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateCallingCards<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcallingcard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallingCards<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            TranslateAddress::<Impl, IMPL_OFFSET>,
            TranslateDialog::<Impl, IMPL_OFFSET>,
            EnumerateLocations::<Impl, IMPL_OFFSET>,
            Locations::<Impl, IMPL_OFFSET>,
            EnumerateCallingCards::<Impl, IMPL_OFFSET>,
            CallingCards::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressTranslation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressTranslationInfoImpl: Sized + IDispatchImpl {
    fn DialableString();
    fn DisplayableString();
    fn CurrentCountryCode();
    fn DestinationCountryCode();
    fn TranslationResults();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressTranslationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressTranslationInfoVtbl {
        unsafe extern "system" fn DialableString<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdialablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayableString<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplayablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCountryCode<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationCountryCode<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TranslationResults<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plresults: *mut i32) -> ::windows::core::HRESULT {
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
            DialableString::<Impl, IMPL_OFFSET>,
            DisplayableString::<Impl, IMPL_OFFSET>,
            CurrentCountryCode::<Impl, IMPL_OFFSET>,
            DestinationCountryCode::<Impl, IMPL_OFFSET>,
            TranslationResults::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressTranslationInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentImpl: Sized + IDispatchImpl {
    fn EnumerateAgentSessions();
    fn CreateSession();
    fn CreateSessionWithPIN();
    fn ID();
    fn User();
    fn SetState();
    fn State();
    fn SetMeasurementPeriod();
    fn MeasurementPeriod();
    fn OverallCallRate();
    fn NumberOfACDCalls();
    fn NumberOfIncomingCalls();
    fn NumberOfOutgoingCalls();
    fn TotalACDTalkTime();
    fn TotalACDCallTime();
    fn TotalWrapUpTime();
    fn AgentSessions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentVtbl {
        unsafe extern "system" fn EnumerateAgentSessions<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSession<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSessionWithPIN<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ID<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn User<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetState<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agentstate: AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMeasurementPeriod<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MeasurementPeriod<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OverallCallRate<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberOfACDCalls<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberOfIncomingCalls<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberOfOutgoingCalls<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalACDTalkTime<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalACDCallTime<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalWrapUpTime<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AgentSessions<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            EnumerateAgentSessions::<Impl, IMPL_OFFSET>,
            CreateSession::<Impl, IMPL_OFFSET>,
            CreateSessionWithPIN::<Impl, IMPL_OFFSET>,
            ID::<Impl, IMPL_OFFSET>,
            User::<Impl, IMPL_OFFSET>,
            SetState::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            SetMeasurementPeriod::<Impl, IMPL_OFFSET>,
            MeasurementPeriod::<Impl, IMPL_OFFSET>,
            OverallCallRate::<Impl, IMPL_OFFSET>,
            NumberOfACDCalls::<Impl, IMPL_OFFSET>,
            NumberOfIncomingCalls::<Impl, IMPL_OFFSET>,
            NumberOfOutgoingCalls::<Impl, IMPL_OFFSET>,
            TotalACDTalkTime::<Impl, IMPL_OFFSET>,
            TotalACDCallTime::<Impl, IMPL_OFFSET>,
            TotalWrapUpTime::<Impl, IMPL_OFFSET>,
            AgentSessions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentEventImpl: Sized + IDispatchImpl {
    fn Agent();
    fn Event();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentEventVtbl {
        unsafe extern "system" fn Agent<Impl: ITAgentEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITAgentEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Agent::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentHandlerImpl: Sized + IDispatchImpl {
    fn Name();
    fn CreateAgent();
    fn CreateAgentWithID();
    fn EnumerateACDGroups();
    fn EnumerateUsableAddresses();
    fn ACDGroups();
    fn UsableAddresses();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentHandlerVtbl {
        unsafe extern "system" fn Name<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAgent<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAgentWithID<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateACDGroups<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateUsableAddresses<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ACDGroups<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UsableAddresses<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            CreateAgent::<Impl, IMPL_OFFSET>,
            CreateAgentWithID::<Impl, IMPL_OFFSET>,
            EnumerateACDGroups::<Impl, IMPL_OFFSET>,
            EnumerateUsableAddresses::<Impl, IMPL_OFFSET>,
            ACDGroups::<Impl, IMPL_OFFSET>,
            UsableAddresses::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentHandlerEventImpl: Sized + IDispatchImpl {
    fn AgentHandler();
    fn Event();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentHandlerEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandlerEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentHandlerEventVtbl {
        unsafe extern "system" fn AgentHandler<Impl: ITAgentHandlerEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagenthandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITAgentHandlerEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, AgentHandler::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentHandlerEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentSessionImpl: Sized + IDispatchImpl {
    fn Agent();
    fn Address();
    fn ACDGroup();
    fn SetState();
    fn State();
    fn SessionStartTime();
    fn SessionDuration();
    fn NumberOfCalls();
    fn TotalTalkTime();
    fn AverageTalkTime();
    fn TotalCallTime();
    fn AverageCallTime();
    fn TotalWrapUpTime();
    fn AverageWrapUpTime();
    fn ACDCallRate();
    fn LongestTimeToAnswer();
    fn AverageTimeToAnswer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentSessionVtbl {
        unsafe extern "system" fn Agent<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Address<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ACDGroup<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetState<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionStartTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesessionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionDuration<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberOfCalls<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalTalkTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AverageTalkTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalCallTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AverageCallTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalWrapUpTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AverageWrapUpTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ACDCallRate<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LongestTimeToAnswer<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AverageTimeToAnswer<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
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
            Agent::<Impl, IMPL_OFFSET>,
            Address::<Impl, IMPL_OFFSET>,
            ACDGroup::<Impl, IMPL_OFFSET>,
            SetState::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            SessionStartTime::<Impl, IMPL_OFFSET>,
            SessionDuration::<Impl, IMPL_OFFSET>,
            NumberOfCalls::<Impl, IMPL_OFFSET>,
            TotalTalkTime::<Impl, IMPL_OFFSET>,
            AverageTalkTime::<Impl, IMPL_OFFSET>,
            TotalCallTime::<Impl, IMPL_OFFSET>,
            AverageCallTime::<Impl, IMPL_OFFSET>,
            TotalWrapUpTime::<Impl, IMPL_OFFSET>,
            AverageWrapUpTime::<Impl, IMPL_OFFSET>,
            ACDCallRate::<Impl, IMPL_OFFSET>,
            LongestTimeToAnswer::<Impl, IMPL_OFFSET>,
            AverageTimeToAnswer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentSessionEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Event();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentSessionEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSessionEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentSessionEventVtbl {
        unsafe extern "system" fn Session<Impl: ITAgentSessionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITAgentSessionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Session::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentSessionEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAllocatorPropertiesImpl: Sized {
    fn SetAllocatorProperties();
    fn GetAllocatorProperties();
    fn SetAllocateBuffers();
    fn GetAllocateBuffers();
    fn SetBufferSize();
    fn GetBufferSize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ITAllocatorPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAllocatorPropertiesVtbl {
        unsafe extern "system" fn SetAllocatorProperties<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocatorProperties<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllocateBuffers<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocateBuffers<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBufferSize<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferSize<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllocatorProperties::<Impl, IMPL_OFFSET>, GetAllocatorProperties::<Impl, IMPL_OFFSET>, SetAllocateBuffers::<Impl, IMPL_OFFSET>, GetAllocateBuffers::<Impl, IMPL_OFFSET>, SetBufferSize::<Impl, IMPL_OFFSET>, GetBufferSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAllocatorProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAutomatedPhoneControlImpl: Sized + IDispatchImpl {
    fn StartTone();
    fn StopTone();
    fn Tone();
    fn StartRinger();
    fn StopRinger();
    fn Ringer();
    fn SetPhoneHandlingEnabled();
    fn PhoneHandlingEnabled();
    fn SetAutoEndOfNumberTimeout();
    fn AutoEndOfNumberTimeout();
    fn SetAutoDialtone();
    fn AutoDialtone();
    fn SetAutoStopTonesOnOnHook();
    fn AutoStopTonesOnOnHook();
    fn SetAutoStopRingOnOffHook();
    fn AutoStopRingOnOffHook();
    fn SetAutoKeypadTones();
    fn AutoKeypadTones();
    fn SetAutoKeypadTonesMinimumDuration();
    fn AutoKeypadTonesMinimumDuration();
    fn SetAutoVolumeControl();
    fn AutoVolumeControl();
    fn SetAutoVolumeControlStep();
    fn AutoVolumeControlStep();
    fn SetAutoVolumeControlRepeatDelay();
    fn AutoVolumeControlRepeatDelay();
    fn SetAutoVolumeControlRepeatPeriod();
    fn AutoVolumeControlRepeatPeriod();
    fn SelectCall();
    fn UnselectCall();
    fn EnumerateSelectedCalls();
    fn SelectedCalls();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAutomatedPhoneControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAutomatedPhoneControlVtbl {
        unsafe extern "system" fn StartTone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopTone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Tone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptone: *mut PHONE_TONE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartRinger<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopRinger<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ringer<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfringing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPhoneHandlingEnabled<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PhoneHandlingEnabled<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoEndOfNumberTimeout<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoEndOfNumberTimeout<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltimeout: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoDialtone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoDialtone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoStopTonesOnOnHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoStopTonesOnOnHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoStopRingOnOffHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoStopRingOnOffHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoKeypadTones<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoKeypadTones<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoKeypadTonesMinimumDuration<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoKeypadTonesMinimumDuration<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoVolumeControl<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoVolumeControl<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoVolumeControlStep<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstepsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoVolumeControlStep<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstepsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatDelay<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatDelay<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatPeriod<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatPeriod<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectCall<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fselectdefaultterminals: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnselectCall<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateSelectedCalls<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectedCalls<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            StartTone::<Impl, IMPL_OFFSET>,
            StopTone::<Impl, IMPL_OFFSET>,
            Tone::<Impl, IMPL_OFFSET>,
            StartRinger::<Impl, IMPL_OFFSET>,
            StopRinger::<Impl, IMPL_OFFSET>,
            Ringer::<Impl, IMPL_OFFSET>,
            SetPhoneHandlingEnabled::<Impl, IMPL_OFFSET>,
            PhoneHandlingEnabled::<Impl, IMPL_OFFSET>,
            SetAutoEndOfNumberTimeout::<Impl, IMPL_OFFSET>,
            AutoEndOfNumberTimeout::<Impl, IMPL_OFFSET>,
            SetAutoDialtone::<Impl, IMPL_OFFSET>,
            AutoDialtone::<Impl, IMPL_OFFSET>,
            SetAutoStopTonesOnOnHook::<Impl, IMPL_OFFSET>,
            AutoStopTonesOnOnHook::<Impl, IMPL_OFFSET>,
            SetAutoStopRingOnOffHook::<Impl, IMPL_OFFSET>,
            AutoStopRingOnOffHook::<Impl, IMPL_OFFSET>,
            SetAutoKeypadTones::<Impl, IMPL_OFFSET>,
            AutoKeypadTones::<Impl, IMPL_OFFSET>,
            SetAutoKeypadTonesMinimumDuration::<Impl, IMPL_OFFSET>,
            AutoKeypadTonesMinimumDuration::<Impl, IMPL_OFFSET>,
            SetAutoVolumeControl::<Impl, IMPL_OFFSET>,
            AutoVolumeControl::<Impl, IMPL_OFFSET>,
            SetAutoVolumeControlStep::<Impl, IMPL_OFFSET>,
            AutoVolumeControlStep::<Impl, IMPL_OFFSET>,
            SetAutoVolumeControlRepeatDelay::<Impl, IMPL_OFFSET>,
            AutoVolumeControlRepeatDelay::<Impl, IMPL_OFFSET>,
            SetAutoVolumeControlRepeatPeriod::<Impl, IMPL_OFFSET>,
            AutoVolumeControlRepeatPeriod::<Impl, IMPL_OFFSET>,
            SelectCall::<Impl, IMPL_OFFSET>,
            UnselectCall::<Impl, IMPL_OFFSET>,
            EnumerateSelectedCalls::<Impl, IMPL_OFFSET>,
            SelectedCalls::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAutomatedPhoneControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicAudioTerminalImpl: Sized + IDispatchImpl {
    fn SetVolume();
    fn Volume();
    fn SetBalance();
    fn Balance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicAudioTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicAudioTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicAudioTerminalVtbl {
        unsafe extern "system" fn SetVolume<Impl: ITBasicAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Volume<Impl: ITBasicAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBalance<Impl: ITBasicAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Balance<Impl: ITBasicAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, SetVolume::<Impl, IMPL_OFFSET>, Volume::<Impl, IMPL_OFFSET>, SetBalance::<Impl, IMPL_OFFSET>, Balance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicAudioTerminal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicCallControlImpl: Sized + IDispatchImpl {
    fn Connect();
    fn Answer();
    fn Disconnect();
    fn Hold();
    fn HandoffDirect();
    fn HandoffIndirect();
    fn Conference();
    fn Transfer();
    fn BlindTransfer();
    fn SwapHold();
    fn ParkDirect();
    fn ParkIndirect();
    fn Unpark();
    fn SetQOS();
    fn Pickup();
    fn Dial();
    fn Finish();
    fn RemoveFromConference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicCallControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicCallControlVtbl {
        unsafe extern "system" fn Connect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Answer<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: DISCONNECT_CODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hold<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fhold: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandoffDirect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandoffIndirect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Conference<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Transfer<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BlindTransfer<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SwapHold<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParkDirect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparkaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParkIndirect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnondiraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unpark<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQOS<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pickup<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroupid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Dial<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finishmode: FINISH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFromConference<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Connect::<Impl, IMPL_OFFSET>,
            Answer::<Impl, IMPL_OFFSET>,
            Disconnect::<Impl, IMPL_OFFSET>,
            Hold::<Impl, IMPL_OFFSET>,
            HandoffDirect::<Impl, IMPL_OFFSET>,
            HandoffIndirect::<Impl, IMPL_OFFSET>,
            Conference::<Impl, IMPL_OFFSET>,
            Transfer::<Impl, IMPL_OFFSET>,
            BlindTransfer::<Impl, IMPL_OFFSET>,
            SwapHold::<Impl, IMPL_OFFSET>,
            ParkDirect::<Impl, IMPL_OFFSET>,
            ParkIndirect::<Impl, IMPL_OFFSET>,
            Unpark::<Impl, IMPL_OFFSET>,
            SetQOS::<Impl, IMPL_OFFSET>,
            Pickup::<Impl, IMPL_OFFSET>,
            Dial::<Impl, IMPL_OFFSET>,
            Finish::<Impl, IMPL_OFFSET>,
            RemoveFromConference::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicCallControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicCallControl2Impl: Sized + ITBasicCallControlImpl + IDispatchImpl {
    fn RequestTerminal();
    fn SelectTerminalOnCall();
    fn UnselectTerminalOnCall();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicCallControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicCallControl2Vtbl {
        unsafe extern "system" fn RequestTerminal<Impl: ITBasicCallControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrterminalclassguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectTerminalOnCall<Impl: ITBasicCallControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnselectTerminalOnCall<Impl: ITBasicCallControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Connect::<Impl, IMPL_OFFSET>,
            Answer::<Impl, IMPL_OFFSET>,
            Disconnect::<Impl, IMPL_OFFSET>,
            Hold::<Impl, IMPL_OFFSET>,
            HandoffDirect::<Impl, IMPL_OFFSET>,
            HandoffIndirect::<Impl, IMPL_OFFSET>,
            Conference::<Impl, IMPL_OFFSET>,
            Transfer::<Impl, IMPL_OFFSET>,
            BlindTransfer::<Impl, IMPL_OFFSET>,
            SwapHold::<Impl, IMPL_OFFSET>,
            ParkDirect::<Impl, IMPL_OFFSET>,
            ParkIndirect::<Impl, IMPL_OFFSET>,
            Unpark::<Impl, IMPL_OFFSET>,
            SetQOS::<Impl, IMPL_OFFSET>,
            Pickup::<Impl, IMPL_OFFSET>,
            Dial::<Impl, IMPL_OFFSET>,
            Finish::<Impl, IMPL_OFFSET>,
            RemoveFromConference::<Impl, IMPL_OFFSET>,
            RequestTerminal::<Impl, IMPL_OFFSET>,
            SelectTerminalOnCall::<Impl, IMPL_OFFSET>,
            UnselectTerminalOnCall::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicCallControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallHubImpl: Sized + IDispatchImpl {
    fn Clear();
    fn EnumerateCalls();
    fn Calls();
    fn NumCalls();
    fn State();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallHubVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallHubVtbl {
        unsafe extern "system" fn Clear<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateCalls<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Calls<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcalls: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumCalls<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>, EnumerateCalls::<Impl, IMPL_OFFSET>, Calls::<Impl, IMPL_OFFSET>, NumCalls::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallHub as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallHubEventImpl: Sized + IDispatchImpl {
    fn Event();
    fn CallHub();
    fn Call();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallHubEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallHubEventVtbl {
        unsafe extern "system" fn Event<Impl: ITCallHubEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallHub<Impl: ITCallHubEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Call<Impl: ITCallHubEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>, CallHub::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallHubEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfoImpl: Sized + IDispatchImpl {
    fn Address();
    fn CallState();
    fn Privilege();
    fn CallHub();
    fn CallInfoLong();
    fn SetCallInfoLong();
    fn CallInfoString();
    fn SetCallInfoString();
    fn CallInfoBuffer();
    fn SetCallInfoBuffer();
    fn GetCallInfoBuffer();
    fn SetCallInfoBuffer();
    fn ReleaseUserUserInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfoVtbl {
        unsafe extern "system" fn Address<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallState<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Privilege<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallHub<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallInfoLong<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCallInfoLong<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallInfoString<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCallInfoString<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseUserUserInfo<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Address::<Impl, IMPL_OFFSET>,
            CallState::<Impl, IMPL_OFFSET>,
            Privilege::<Impl, IMPL_OFFSET>,
            CallHub::<Impl, IMPL_OFFSET>,
            CallInfoLong::<Impl, IMPL_OFFSET>,
            SetCallInfoLong::<Impl, IMPL_OFFSET>,
            CallInfoString::<Impl, IMPL_OFFSET>,
            SetCallInfoString::<Impl, IMPL_OFFSET>,
            CallInfoBuffer::<Impl, IMPL_OFFSET>,
            SetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            GetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            SetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            ReleaseUserUserInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfo2Impl: Sized + ITCallInfoImpl + IDispatchImpl {
    fn EventFilter();
    fn SetEventFilter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfo2Vtbl {
        unsafe extern "system" fn EventFilter<Impl: ITCallInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITCallInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
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
            Address::<Impl, IMPL_OFFSET>,
            CallState::<Impl, IMPL_OFFSET>,
            Privilege::<Impl, IMPL_OFFSET>,
            CallHub::<Impl, IMPL_OFFSET>,
            CallInfoLong::<Impl, IMPL_OFFSET>,
            SetCallInfoLong::<Impl, IMPL_OFFSET>,
            CallInfoString::<Impl, IMPL_OFFSET>,
            SetCallInfoString::<Impl, IMPL_OFFSET>,
            CallInfoBuffer::<Impl, IMPL_OFFSET>,
            SetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            GetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            SetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            ReleaseUserUserInfo::<Impl, IMPL_OFFSET>,
            EventFilter::<Impl, IMPL_OFFSET>,
            SetEventFilter::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfoChangeEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Cause();
    fn CallbackInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfoChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoChangeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfoChangeEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallInfoChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cause<Impl: ITCallInfoChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallInfoChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Cause::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfoChangeEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallMediaEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn Error();
    fn Terminal();
    fn Stream();
    fn Cause();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallMediaEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallMediaEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminal<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stream<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cause<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>, Error::<Impl, IMPL_OFFSET>, Terminal::<Impl, IMPL_OFFSET>, Stream::<Impl, IMPL_OFFSET>, Cause::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallMediaEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallNotificationEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn CallbackInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallNotificationEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallNotificationEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallNotificationEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallNotificationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITCallNotificationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallNotificationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallNotificationEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallStateEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn State();
    fn Cause();
    fn CallbackInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallStateEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallStateEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallStateEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallStateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: ITCallStateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cause<Impl: ITCallStateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallStateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>, Cause::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallStateEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallingCardImpl: Sized + IDispatchImpl {
    fn PermanentCardID();
    fn NumberOfDigits();
    fn Options();
    fn CardName();
    fn SameAreaDialingRule();
    fn LongDistanceDialingRule();
    fn InternationalDialingRule();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallingCardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallingCardVtbl {
        unsafe extern "system" fn PermanentCardID<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberOfDigits<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldigits: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Options<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CardName<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcardname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SameAreaDialingRule<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LongDistanceDialingRule<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InternationalDialingRule<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            PermanentCardID::<Impl, IMPL_OFFSET>,
            NumberOfDigits::<Impl, IMPL_OFFSET>,
            Options::<Impl, IMPL_OFFSET>,
            CardName::<Impl, IMPL_OFFSET>,
            SameAreaDialingRule::<Impl, IMPL_OFFSET>,
            LongDistanceDialingRule::<Impl, IMPL_OFFSET>,
            InternationalDialingRule::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallingCard as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ITCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ITCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ITCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCollection2Impl: Sized + ITCollectionImpl + IDispatchImpl {
    fn Add();
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCollection2Vtbl {
        unsafe extern "system" fn Add<Impl: ITCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ITCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCollection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCustomToneImpl: Sized + IDispatchImpl {
    fn Frequency();
    fn SetFrequency();
    fn CadenceOn();
    fn SetCadenceOn();
    fn CadenceOff();
    fn SetCadenceOff();
    fn Volume();
    fn SetVolume();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCustomToneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomToneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCustomToneVtbl {
        unsafe extern "system" fn Frequency<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFrequency<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CadenceOn<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcadenceon: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCadenceOn<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cadenceon: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CadenceOff<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcadenceoff: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCadenceOff<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcadenceoff: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Volume<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolume<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
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
            Frequency::<Impl, IMPL_OFFSET>,
            SetFrequency::<Impl, IMPL_OFFSET>,
            CadenceOn::<Impl, IMPL_OFFSET>,
            SetCadenceOn::<Impl, IMPL_OFFSET>,
            CadenceOff::<Impl, IMPL_OFFSET>,
            SetCadenceOff::<Impl, IMPL_OFFSET>,
            Volume::<Impl, IMPL_OFFSET>,
            SetVolume::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCustomTone as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDetectToneImpl: Sized + IDispatchImpl {
    fn AppSpecific();
    fn SetAppSpecific();
    fn Duration();
    fn SetDuration();
    fn Frequency();
    fn SetFrequency();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDetectToneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectToneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDetectToneVtbl {
        unsafe extern "system" fn AppSpecific<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppSpecific<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Duration<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuration<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Frequency<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFrequency<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, lfrequency: i32) -> ::windows::core::HRESULT {
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
            AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific::<Impl, IMPL_OFFSET>,
            Duration::<Impl, IMPL_OFFSET>,
            SetDuration::<Impl, IMPL_OFFSET>,
            Frequency::<Impl, IMPL_OFFSET>,
            SetFrequency::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDetectTone as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitDetectionEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Digit();
    fn DigitMode();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitDetectionEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitDetectionEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Digit<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucdigit: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DigitMode<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigitmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Digit::<Impl, IMPL_OFFSET>, DigitMode::<Impl, IMPL_OFFSET>, TickCount::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitDetectionEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitGenerationEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn GenerationTermination();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitGenerationEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitGenerationEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitGenerationEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitGenerationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerationTermination<Impl: ITDigitGenerationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plgenerationtermination: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitGenerationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitGenerationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, GenerationTermination::<Impl, IMPL_OFFSET>, TickCount::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitGenerationEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitsGatheredEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Digits();
    fn GatherTermination();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitsGatheredEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitsGatheredEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Digits<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdigits: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GatherTermination<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Digits::<Impl, IMPL_OFFSET>, GatherTermination::<Impl, IMPL_OFFSET>, TickCount::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitsGatheredEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryImpl: Sized + IDispatchImpl {
    fn DirectoryType();
    fn DisplayName();
    fn IsDynamic();
    fn DefaultObjectTTL();
    fn SetDefaultObjectTTL();
    fn EnableAutoRefresh();
    fn Connect();
    fn Bind();
    fn AddDirectoryObject();
    fn ModifyDirectoryObject();
    fn RefreshDirectoryObject();
    fn DeleteDirectoryObject();
    fn DirectoryObjects();
    fn EnumerateDirectoryObjects();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryVtbl {
        unsafe extern "system" fn DirectoryType<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDynamic<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdynamic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultObjectTTL<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultObjectTTL<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableAutoRefresh<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connect<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsecure: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Bind<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DirectoryObjects<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateDirectoryObjects<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenumobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            DirectoryType::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            IsDynamic::<Impl, IMPL_OFFSET>,
            DefaultObjectTTL::<Impl, IMPL_OFFSET>,
            SetDefaultObjectTTL::<Impl, IMPL_OFFSET>,
            EnableAutoRefresh::<Impl, IMPL_OFFSET>,
            Connect::<Impl, IMPL_OFFSET>,
            Bind::<Impl, IMPL_OFFSET>,
            AddDirectoryObject::<Impl, IMPL_OFFSET>,
            ModifyDirectoryObject::<Impl, IMPL_OFFSET>,
            RefreshDirectoryObject::<Impl, IMPL_OFFSET>,
            DeleteDirectoryObject::<Impl, IMPL_OFFSET>,
            DirectoryObjects::<Impl, IMPL_OFFSET>,
            EnumerateDirectoryObjects::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectImpl: Sized + IDispatchImpl {
    fn ObjectType();
    fn Name();
    fn SetName();
    fn DialableAddrs();
    fn EnumerateDialableAddrs();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObjectVtbl {
        unsafe extern "system" fn ObjectType<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DialableAddrs<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddresstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateDialableAddrs<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecdes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecdes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ObjectType::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            DialableAddrs::<Impl, IMPL_OFFSET>,
            EnumerateDialableAddrs::<Impl, IMPL_OFFSET>,
            SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectConferenceImpl: Sized + IDispatchImpl {
    fn Protocol();
    fn Originator();
    fn SetOriginator();
    fn AdvertisingScope();
    fn SetAdvertisingScope();
    fn Url();
    fn SetUrl();
    fn Description();
    fn SetDescription();
    fn IsEncrypted();
    fn SetIsEncrypted();
    fn StartTime();
    fn SetStartTime();
    fn StopTime();
    fn SetStopTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectConferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObjectConferenceVtbl {
        unsafe extern "system" fn Protocol<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Originator<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginator: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOriginator<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poriginator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdvertisingScope<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAdvertisingScope<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Url<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUrl<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEncrypted<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfencrypted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsEncrypted<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fencrypted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStopTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
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
            Protocol::<Impl, IMPL_OFFSET>,
            Originator::<Impl, IMPL_OFFSET>,
            SetOriginator::<Impl, IMPL_OFFSET>,
            AdvertisingScope::<Impl, IMPL_OFFSET>,
            SetAdvertisingScope::<Impl, IMPL_OFFSET>,
            Url::<Impl, IMPL_OFFSET>,
            SetUrl::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            IsEncrypted::<Impl, IMPL_OFFSET>,
            SetIsEncrypted::<Impl, IMPL_OFFSET>,
            StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime::<Impl, IMPL_OFFSET>,
            StopTime::<Impl, IMPL_OFFSET>,
            SetStopTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObjectConference as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectUserImpl: Sized + IDispatchImpl {
    fn IPPhonePrimary();
    fn SetIPPhonePrimary();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObjectUserVtbl {
        unsafe extern "system" fn IPPhonePrimary<Impl: ITDirectoryObjectUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIPPhonePrimary<Impl: ITDirectoryObjectUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, IPPhonePrimary::<Impl, IMPL_OFFSET>, SetIPPhonePrimary::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObjectUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDispatchMapperImpl: Sized + IDispatchImpl {
    fn QueryDispatchInterface();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDispatchMapperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDispatchMapperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDispatchMapperVtbl {
        unsafe extern "system" fn QueryDispatchInterface<Impl: ITDispatchMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pinterfacetomap: ::windows::core::RawPtr, ppreturnedinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, QueryDispatchInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDispatchMapper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITFileTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Track();
    fn Call();
    fn State();
    fn Cause();
    fn Error();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITFileTerminalEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITFileTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Track<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrackterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Call<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cause<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Terminal::<Impl, IMPL_OFFSET>, Track::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>, Cause::<Impl, IMPL_OFFSET>, Error::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITFileTerminalEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITFileTrackImpl: Sized + IDispatchImpl {
    fn Format();
    fn SetFormat();
    fn ControllingTerminal();
    fn AudioFormatForScripting();
    fn SetAudioFormatForScripting();
    fn EmptyAudioFormatForScripting();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITFileTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITFileTrackVtbl {
        unsafe extern "system" fn Format<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormat<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ControllingTerminal<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrollingterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioformat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EmptyAudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Format::<Impl, IMPL_OFFSET>,
            SetFormat::<Impl, IMPL_OFFSET>,
            ControllingTerminal::<Impl, IMPL_OFFSET>,
            AudioFormatForScripting::<Impl, IMPL_OFFSET>,
            SetAudioFormatForScripting::<Impl, IMPL_OFFSET>,
            EmptyAudioFormatForScripting::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITFileTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITForwardInformationImpl: Sized + IDispatchImpl {
    fn SetNumRingsNoAnswer();
    fn NumRingsNoAnswer();
    fn SetForwardType();
    fn ForwardTypeDestination();
    fn ForwardTypeCaller();
    fn GetForwardType();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITForwardInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITForwardInformationVtbl {
        unsafe extern "system" fn SetNumRingsNoAnswer<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumrings: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumRingsNoAnswer<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumrings: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetForwardType<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForwardTypeDestination<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForwardTypeCaller<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForwardType<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            SetNumRingsNoAnswer::<Impl, IMPL_OFFSET>,
            NumRingsNoAnswer::<Impl, IMPL_OFFSET>,
            SetForwardType::<Impl, IMPL_OFFSET>,
            ForwardTypeDestination::<Impl, IMPL_OFFSET>,
            ForwardTypeCaller::<Impl, IMPL_OFFSET>,
            GetForwardType::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITForwardInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITForwardInformation2Impl: Sized + ITForwardInformationImpl + IDispatchImpl {
    fn SetForwardType2();
    fn GetForwardType2();
    fn ForwardTypeDestinationAddressType();
    fn ForwardTypeCallerAddressType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITForwardInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITForwardInformation2Vtbl {
        unsafe extern "system" fn SetForwardType2<Impl: ITForwardInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destaddresstype: i32, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, calleraddresstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForwardType2<Impl: ITForwardInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForwardTypeDestinationAddressType<Impl: ITForwardInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForwardTypeCallerAddressType<Impl: ITForwardInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
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
            SetNumRingsNoAnswer::<Impl, IMPL_OFFSET>,
            NumRingsNoAnswer::<Impl, IMPL_OFFSET>,
            SetForwardType::<Impl, IMPL_OFFSET>,
            ForwardTypeDestination::<Impl, IMPL_OFFSET>,
            ForwardTypeCaller::<Impl, IMPL_OFFSET>,
            GetForwardType::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            SetForwardType2::<Impl, IMPL_OFFSET>,
            GetForwardType2::<Impl, IMPL_OFFSET>,
            ForwardTypeDestinationAddressType::<Impl, IMPL_OFFSET>,
            ForwardTypeCallerAddressType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITForwardInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITILSConfigImpl: Sized + IDispatchImpl {
    fn Port();
    fn SetPort();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITILSConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITILSConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITILSConfigVtbl {
        unsafe extern "system" fn Port<Impl: ITILSConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPort<Impl: ITILSConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, port: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Port::<Impl, IMPL_OFFSET>, SetPort::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITILSConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITLegacyAddressMediaControlImpl: Sized {
    fn GetID();
    fn GetDevConfig();
    fn SetDevConfig();
}
#[cfg(feature = "Win32_Foundation")]
impl ITLegacyAddressMediaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyAddressMediaControlVtbl {
        unsafe extern "system" fn GetID<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDevConfig<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDevConfig<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetID::<Impl, IMPL_OFFSET>, GetDevConfig::<Impl, IMPL_OFFSET>, SetDevConfig::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITLegacyAddressMediaControl2Impl: Sized + ITLegacyAddressMediaControlImpl {
    fn ConfigDialog();
    fn ConfigDialogEdit();
}
#[cfg(feature = "Win32_Foundation")]
impl ITLegacyAddressMediaControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyAddressMediaControl2Vtbl {
        unsafe extern "system" fn ConfigDialog<Impl: ITLegacyAddressMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfigDialogEdit<Impl: ITLegacyAddressMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetID::<Impl, IMPL_OFFSET>, GetDevConfig::<Impl, IMPL_OFFSET>, SetDevConfig::<Impl, IMPL_OFFSET>, ConfigDialog::<Impl, IMPL_OFFSET>, ConfigDialogEdit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyCallMediaControlImpl: Sized + IDispatchImpl {
    fn DetectDigits();
    fn GenerateDigits();
    fn GetID();
    fn SetMediaType();
    fn MonitorMedia();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyCallMediaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyCallMediaControlVtbl {
        unsafe extern "system" fn DetectDigits<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateDigits<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetID<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMediaType<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonitorMedia<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, DetectDigits::<Impl, IMPL_OFFSET>, GenerateDigits::<Impl, IMPL_OFFSET>, GetID::<Impl, IMPL_OFFSET>, SetMediaType::<Impl, IMPL_OFFSET>, MonitorMedia::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyCallMediaControl2Impl: Sized + ITLegacyCallMediaControlImpl + IDispatchImpl {
    fn GenerateDigits2();
    fn GatherDigits();
    fn DetectTones();
    fn DetectTonesByCollection();
    fn GenerateTone();
    fn GenerateCustomTones();
    fn GenerateCustomTonesByCollection();
    fn CreateDetectToneObject();
    fn CreateCustomToneObject();
    fn GetIDAsVariant();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyCallMediaControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyCallMediaControl2Vtbl {
        unsafe extern "system" fn GenerateDigits2<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GatherDigits<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetectTones<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DetectTonesByCollection<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetecttonecollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateTone<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateCustomTones<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateCustomTonesByCollection<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustomtonecollection: ::windows::core::RawPtr, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDetectToneObject<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdetecttone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCustomToneObject<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcustomtone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIDAsVariant<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardeviceid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            DetectDigits::<Impl, IMPL_OFFSET>,
            GenerateDigits::<Impl, IMPL_OFFSET>,
            GetID::<Impl, IMPL_OFFSET>,
            SetMediaType::<Impl, IMPL_OFFSET>,
            MonitorMedia::<Impl, IMPL_OFFSET>,
            GenerateDigits2::<Impl, IMPL_OFFSET>,
            GatherDigits::<Impl, IMPL_OFFSET>,
            DetectTones::<Impl, IMPL_OFFSET>,
            DetectTonesByCollection::<Impl, IMPL_OFFSET>,
            GenerateTone::<Impl, IMPL_OFFSET>,
            GenerateCustomTones::<Impl, IMPL_OFFSET>,
            GenerateCustomTonesByCollection::<Impl, IMPL_OFFSET>,
            CreateDetectToneObject::<Impl, IMPL_OFFSET>,
            CreateCustomToneObject::<Impl, IMPL_OFFSET>,
            GetIDAsVariant::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyWaveSupportImpl: Sized + IDispatchImpl {
    fn IsFullDuplex();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyWaveSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyWaveSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyWaveSupportVtbl {
        unsafe extern "system" fn IsFullDuplex<Impl: ITLegacyWaveSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, IsFullDuplex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyWaveSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLocationInfoImpl: Sized + IDispatchImpl {
    fn PermanentLocationID();
    fn CountryCode();
    fn CountryID();
    fn Options();
    fn PreferredCardID();
    fn LocationName();
    fn CityCode();
    fn LocalAccessCode();
    fn LongDistanceAccessCode();
    fn TollPrefixList();
    fn CancelCallWaitingCode();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLocationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLocationInfoVtbl {
        unsafe extern "system" fn PermanentLocationID<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllocationid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CountryCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CountryID<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountryid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Options<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredCardID<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocationName<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplocationname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CityCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LocalAccessCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LongDistanceAccessCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TollPrefixList<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptolllist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelCallWaitingCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            PermanentLocationID::<Impl, IMPL_OFFSET>,
            CountryCode::<Impl, IMPL_OFFSET>,
            CountryID::<Impl, IMPL_OFFSET>,
            Options::<Impl, IMPL_OFFSET>,
            PreferredCardID::<Impl, IMPL_OFFSET>,
            LocationName::<Impl, IMPL_OFFSET>,
            CityCode::<Impl, IMPL_OFFSET>,
            LocalAccessCode::<Impl, IMPL_OFFSET>,
            LongDistanceAccessCode::<Impl, IMPL_OFFSET>,
            TollPrefixList::<Impl, IMPL_OFFSET>,
            CancelCallWaitingCode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLocationInfo as ::windows::core::Interface>::IID
    }
}
pub trait ITMSPAddressImpl: Sized {
    fn Initialize();
    fn Shutdown();
    fn CreateMSPCall();
    fn ShutdownMSPCall();
    fn ReceiveTSPData();
    fn GetEvent();
}
impl ITMSPAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMSPAddressVtbl {
        unsafe extern "system" fn Initialize<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMSPCall<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut ::core::ffi::c_void, ppstreamcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShutdownMSPCall<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveTSPData<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmspcall: *mut ::core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEvent<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Shutdown::<Impl, IMPL_OFFSET>, CreateMSPCall::<Impl, IMPL_OFFSET>, ShutdownMSPCall::<Impl, IMPL_OFFSET>, ReceiveTSPData::<Impl, IMPL_OFFSET>, GetEvent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMSPAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaControlImpl: Sized + IDispatchImpl {
    fn Start();
    fn Stop();
    fn Pause();
    fn MediaState();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaControlVtbl {
        unsafe extern "system" fn Start<Impl: ITMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: ITMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: ITMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaState<Impl: ITMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Start::<Impl, IMPL_OFFSET>, Stop::<Impl, IMPL_OFFSET>, Pause::<Impl, IMPL_OFFSET>, MediaState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaPlaybackImpl: Sized + IDispatchImpl {
    fn SetPlayList();
    fn PlayList();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaPlaybackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaPlaybackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaPlaybackVtbl {
        unsafe extern "system" fn SetPlayList<Impl: ITMediaPlaybackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlistvariant: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlayList<Impl: ITMediaPlaybackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylistvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, SetPlayList::<Impl, IMPL_OFFSET>, PlayList::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaPlayback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaRecordImpl: Sized + IDispatchImpl {
    fn SetFileName();
    fn FileName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaRecordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaRecordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaRecordVtbl {
        unsafe extern "system" fn SetFileName<Impl: ITMediaRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileName<Impl: ITMediaRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, SetFileName::<Impl, IMPL_OFFSET>, FileName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaRecord as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaSupportImpl: Sized + IDispatchImpl {
    fn MediaTypes();
    fn QueryMediaType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaSupportVtbl {
        unsafe extern "system" fn MediaTypes<Impl: ITMediaSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryMediaType<Impl: ITMediaSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, pfsupport: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, MediaTypes::<Impl, IMPL_OFFSET>, QueryMediaType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMultiTrackTerminalImpl: Sized + IDispatchImpl {
    fn TrackTerminals();
    fn EnumerateTrackTerminals();
    fn CreateTrackTerminal();
    fn MediaTypesInUse();
    fn DirectionsInUse();
    fn RemoveTrackTerminal();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMultiTrackTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMultiTrackTerminalVtbl {
        unsafe extern "system" fn TrackTerminals<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateTrackTerminals<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTrackTerminal<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaTypesInUse<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypesinuse: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DirectionsInUse<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveTrackTerminal<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrackterminaltoremove: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            TrackTerminals::<Impl, IMPL_OFFSET>,
            EnumerateTrackTerminals::<Impl, IMPL_OFFSET>,
            CreateTrackTerminal::<Impl, IMPL_OFFSET>,
            MediaTypesInUse::<Impl, IMPL_OFFSET>,
            DirectionsInUse::<Impl, IMPL_OFFSET>,
            RemoveTrackTerminal::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMultiTrackTerminal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneImpl: Sized + IDispatchImpl {
    fn Open();
    fn Close();
    fn Addresses();
    fn EnumerateAddresses();
    fn PhoneCapsLong();
    fn PhoneCapsString();
    fn Terminals();
    fn EnumerateTerminals();
    fn ButtonMode();
    fn SetButtonMode();
    fn ButtonFunction();
    fn SetButtonFunction();
    fn ButtonText();
    fn SetButtonText();
    fn ButtonState();
    fn HookSwitchState();
    fn SetHookSwitchState();
    fn SetRingMode();
    fn RingMode();
    fn SetRingVolume();
    fn RingVolume();
    fn Privilege();
    fn GetPhoneCapsBuffer();
    fn PhoneCapsBuffer();
    fn LampMode();
    fn SetLampMode();
    fn Display();
    fn SetDisplay();
    fn PreferredAddresses();
    fn EnumeratePreferredAddresses();
    fn DeviceSpecific();
    fn DeviceSpecificVariant();
    fn NegotiateExtVersion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhoneVtbl {
        unsafe extern "system" fn Open<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Addresses<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PhoneCapsLong<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PhoneCapsString<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminals<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ButtonMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetButtonMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ButtonFunction<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetButtonFunction<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ButtonText<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetButtonText<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, bstrbuttontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ButtonState<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HookSwitchState<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHookSwitchState<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRingMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RingMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRingVolume<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RingVolume<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Privilege<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPhoneCapsBuffer<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PhoneCapsBuffer<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LampMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLampMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Display<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplay<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferredAddresses<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePreferredAddresses<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceSpecific<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NegotiateExtVersion<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
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
            Open::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Addresses::<Impl, IMPL_OFFSET>,
            EnumerateAddresses::<Impl, IMPL_OFFSET>,
            PhoneCapsLong::<Impl, IMPL_OFFSET>,
            PhoneCapsString::<Impl, IMPL_OFFSET>,
            Terminals::<Impl, IMPL_OFFSET>,
            EnumerateTerminals::<Impl, IMPL_OFFSET>,
            ButtonMode::<Impl, IMPL_OFFSET>,
            SetButtonMode::<Impl, IMPL_OFFSET>,
            ButtonFunction::<Impl, IMPL_OFFSET>,
            SetButtonFunction::<Impl, IMPL_OFFSET>,
            ButtonText::<Impl, IMPL_OFFSET>,
            SetButtonText::<Impl, IMPL_OFFSET>,
            ButtonState::<Impl, IMPL_OFFSET>,
            HookSwitchState::<Impl, IMPL_OFFSET>,
            SetHookSwitchState::<Impl, IMPL_OFFSET>,
            SetRingMode::<Impl, IMPL_OFFSET>,
            RingMode::<Impl, IMPL_OFFSET>,
            SetRingVolume::<Impl, IMPL_OFFSET>,
            RingVolume::<Impl, IMPL_OFFSET>,
            Privilege::<Impl, IMPL_OFFSET>,
            GetPhoneCapsBuffer::<Impl, IMPL_OFFSET>,
            PhoneCapsBuffer::<Impl, IMPL_OFFSET>,
            LampMode::<Impl, IMPL_OFFSET>,
            SetLampMode::<Impl, IMPL_OFFSET>,
            Display::<Impl, IMPL_OFFSET>,
            SetDisplay::<Impl, IMPL_OFFSET>,
            PreferredAddresses::<Impl, IMPL_OFFSET>,
            EnumeratePreferredAddresses::<Impl, IMPL_OFFSET>,
            DeviceSpecific::<Impl, IMPL_OFFSET>,
            DeviceSpecificVariant::<Impl, IMPL_OFFSET>,
            NegotiateExtVersion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhone as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneDeviceSpecificEventImpl: Sized + IDispatchImpl {
    fn Phone();
    fn lParam1();
    fn lParam2();
    fn lParam3();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneDeviceSpecificEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneDeviceSpecificEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhoneDeviceSpecificEventVtbl {
        unsafe extern "system" fn Phone<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lParam1<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lParam2<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn lParam3<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Phone::<Impl, IMPL_OFFSET>, lParam1::<Impl, IMPL_OFFSET>, lParam2::<Impl, IMPL_OFFSET>, lParam3::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhoneDeviceSpecificEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneEventImpl: Sized + IDispatchImpl {
    fn Phone();
    fn Event();
    fn ButtonState();
    fn HookSwitchState();
    fn HookSwitchDevice();
    fn RingMode();
    fn ButtonLampId();
    fn NumberGathered();
    fn Call();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhoneEventVtbl {
        unsafe extern "system" fn Phone<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut PHONE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ButtonState<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HookSwitchState<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HookSwitchDevice<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RingMode<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ButtonLampId<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbuttonlampid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberGathered<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Call<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Phone::<Impl, IMPL_OFFSET>,
            Event::<Impl, IMPL_OFFSET>,
            ButtonState::<Impl, IMPL_OFFSET>,
            HookSwitchState::<Impl, IMPL_OFFSET>,
            HookSwitchDevice::<Impl, IMPL_OFFSET>,
            RingMode::<Impl, IMPL_OFFSET>,
            ButtonLampId::<Impl, IMPL_OFFSET>,
            NumberGathered::<Impl, IMPL_OFFSET>,
            Call::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhoneEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPluggableTerminalClassInfoImpl: Sized + IDispatchImpl {
    fn Name();
    fn Company();
    fn Version();
    fn TerminalClass();
    fn CLSID();
    fn Direction();
    fn MediaTypes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPluggableTerminalClassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalClassInfoVtbl {
        unsafe extern "system" fn Name<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Company<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompany: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Version<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TerminalClass<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CLSID<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Direction<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaTypes<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
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
            Company::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            TerminalClass::<Impl, IMPL_OFFSET>,
            CLSID::<Impl, IMPL_OFFSET>,
            Direction::<Impl, IMPL_OFFSET>,
            MediaTypes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalClassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalEventSinkImpl: Sized {
    fn FireEvent();
}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalEventSinkVtbl {
        unsafe extern "system" fn FireEvent<Impl: ITPluggableTerminalEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FireEvent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSink as ::windows::core::Interface>::IID
    }
}
pub trait ITPluggableTerminalEventSinkRegistrationImpl: Sized {
    fn RegisterSink();
    fn UnregisterSink();
}
impl ITPluggableTerminalEventSinkRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSinkRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalEventSinkRegistrationVtbl {
        unsafe extern "system" fn RegisterSink<Impl: ITPluggableTerminalEventSinkRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterSink<Impl: ITPluggableTerminalEventSinkRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterSink::<Impl, IMPL_OFFSET>, UnregisterSink::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSinkRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPluggableTerminalSuperclassInfoImpl: Sized + IDispatchImpl {
    fn Name();
    fn CLSID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPluggableTerminalSuperclassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalSuperclassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalSuperclassInfoVtbl {
        unsafe extern "system" fn Name<Impl: ITPluggableTerminalSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CLSID<Impl: ITPluggableTerminalSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, CLSID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalSuperclassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPrivateEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Call();
    fn CallHub();
    fn EventCode();
    fn EventInterface();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPrivateEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPrivateEventVtbl {
        unsafe extern "system" fn Address<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Call<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallHub<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventCode<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pleventcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventInterface<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Address::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, CallHub::<Impl, IMPL_OFFSET>, EventCode::<Impl, IMPL_OFFSET>, EventInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPrivateEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQOSEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn MediaType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQOSEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQOSEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQOSEventVtbl {
        unsafe extern "system" fn Call<Impl: ITQOSEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITQOSEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaType<Impl: ITQOSEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>, MediaType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQOSEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQueueImpl: Sized + IDispatchImpl {
    fn SetMeasurementPeriod();
    fn MeasurementPeriod();
    fn TotalCallsQueued();
    fn CurrentCallsQueued();
    fn TotalCallsAbandoned();
    fn TotalCallsFlowedIn();
    fn TotalCallsFlowedOut();
    fn LongestEverWaitTime();
    fn CurrentLongestWaitTime();
    fn AverageWaitTime();
    fn FinalDisposition();
    fn Name();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQueueVtbl {
        unsafe extern "system" fn SetMeasurementPeriod<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MeasurementPeriod<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalCallsQueued<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCallsQueued<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalCallsAbandoned<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalCallsFlowedIn<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalCallsFlowedOut<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LongestEverWaitTime<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentLongestWaitTime<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AverageWaitTime<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinalDisposition<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            SetMeasurementPeriod::<Impl, IMPL_OFFSET>,
            MeasurementPeriod::<Impl, IMPL_OFFSET>,
            TotalCallsQueued::<Impl, IMPL_OFFSET>,
            CurrentCallsQueued::<Impl, IMPL_OFFSET>,
            TotalCallsAbandoned::<Impl, IMPL_OFFSET>,
            TotalCallsFlowedIn::<Impl, IMPL_OFFSET>,
            TotalCallsFlowedOut::<Impl, IMPL_OFFSET>,
            LongestEverWaitTime::<Impl, IMPL_OFFSET>,
            CurrentLongestWaitTime::<Impl, IMPL_OFFSET>,
            AverageWaitTime::<Impl, IMPL_OFFSET>,
            FinalDisposition::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQueueEventImpl: Sized + IDispatchImpl {
    fn Queue();
    fn Event();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQueueEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQueueEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQueueEventVtbl {
        unsafe extern "system" fn Queue<Impl: ITQueueEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITQueueEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Queue::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQueueEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRendezvousImpl: Sized + IDispatchImpl {
    fn DefaultDirectories();
    fn EnumerateDefaultDirectories();
    fn CreateDirectory();
    fn CreateDirectoryObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRendezvousVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRendezvousImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRendezvousVtbl {
        unsafe extern "system" fn DefaultDirectories<Impl: ITRendezvousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateDefaultDirectories<Impl: ITRendezvousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdirectory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDirectory<Impl: ITRendezvousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDirectoryObject<Impl: ITRendezvousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdirectoryobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, DefaultDirectories::<Impl, IMPL_OFFSET>, EnumerateDefaultDirectories::<Impl, IMPL_OFFSET>, CreateDirectory::<Impl, IMPL_OFFSET>, CreateDirectoryObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRendezvous as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRequestImpl: Sized + IDispatchImpl {
    fn MakeCall();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRequestVtbl {
        unsafe extern "system" fn MakeCall<Impl: ITRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalledparty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, MakeCall::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRequestEventImpl: Sized + IDispatchImpl {
    fn RegistrationInstance();
    fn RequestMode();
    fn DestAddress();
    fn AppName();
    fn CalledParty();
    fn Comment();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRequestEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRequestEventVtbl {
        unsafe extern "system" fn RegistrationInstance<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plregistrationinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestMode<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrequestmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestAddress<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppName<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CalledParty<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcalledparty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Comment<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            RegistrationInstance::<Impl, IMPL_OFFSET>,
            RequestMode::<Impl, IMPL_OFFSET>,
            DestAddress::<Impl, IMPL_OFFSET>,
            AppName::<Impl, IMPL_OFFSET>,
            CalledParty::<Impl, IMPL_OFFSET>,
            Comment::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRequestEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITScriptableAudioFormatImpl: Sized + IDispatchImpl {
    fn Channels();
    fn SetChannels();
    fn SamplesPerSec();
    fn SetSamplesPerSec();
    fn AvgBytesPerSec();
    fn SetAvgBytesPerSec();
    fn BlockAlign();
    fn SetBlockAlign();
    fn BitsPerSample();
    fn SetBitsPerSample();
    fn FormatTag();
    fn SetFormatTag();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITScriptableAudioFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITScriptableAudioFormatVtbl {
        unsafe extern "system" fn Channels<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChannels<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SamplesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSamplesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AvgBytesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BlockAlign<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlockAlign<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BitsPerSample<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBitsPerSample<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatTag<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormatTag<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
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
            Channels::<Impl, IMPL_OFFSET>,
            SetChannels::<Impl, IMPL_OFFSET>,
            SamplesPerSec::<Impl, IMPL_OFFSET>,
            SetSamplesPerSec::<Impl, IMPL_OFFSET>,
            AvgBytesPerSec::<Impl, IMPL_OFFSET>,
            SetAvgBytesPerSec::<Impl, IMPL_OFFSET>,
            BlockAlign::<Impl, IMPL_OFFSET>,
            SetBlockAlign::<Impl, IMPL_OFFSET>,
            BitsPerSample::<Impl, IMPL_OFFSET>,
            SetBitsPerSample::<Impl, IMPL_OFFSET>,
            FormatTag::<Impl, IMPL_OFFSET>,
            SetFormatTag::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITScriptableAudioFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStaticAudioTerminalImpl: Sized + IDispatchImpl {
    fn WaveId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStaticAudioTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStaticAudioTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStaticAudioTerminalVtbl {
        unsafe extern "system" fn WaveId<Impl: ITStaticAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, WaveId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStaticAudioTerminal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStreamImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn Direction();
    fn Name();
    fn StartStream();
    fn PauseStream();
    fn StopStream();
    fn SelectTerminal();
    fn UnselectTerminal();
    fn EnumerateTerminals();
    fn Terminals();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStreamVtbl {
        unsafe extern "system" fn MediaType<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Direction<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartStream<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PauseStream<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopStream<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectTerminal<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnselectTerminal<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminals<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            MediaType::<Impl, IMPL_OFFSET>,
            Direction::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            StartStream::<Impl, IMPL_OFFSET>,
            PauseStream::<Impl, IMPL_OFFSET>,
            StopStream::<Impl, IMPL_OFFSET>,
            SelectTerminal::<Impl, IMPL_OFFSET>,
            UnselectTerminal::<Impl, IMPL_OFFSET>,
            EnumerateTerminals::<Impl, IMPL_OFFSET>,
            Terminals::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStreamControlImpl: Sized + IDispatchImpl {
    fn CreateStream();
    fn RemoveStream();
    fn EnumerateStreams();
    fn Streams();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStreamControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStreamControlVtbl {
        unsafe extern "system" fn CreateStream<Impl: ITStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStream<Impl: ITStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateStreams<Impl: ITStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Streams<Impl: ITStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreateStream::<Impl, IMPL_OFFSET>, RemoveStream::<Impl, IMPL_OFFSET>, EnumerateStreams::<Impl, IMPL_OFFSET>, Streams::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStreamControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITSubStreamImpl: Sized + IDispatchImpl {
    fn StartSubStream();
    fn PauseSubStream();
    fn StopSubStream();
    fn SelectTerminal();
    fn UnselectTerminal();
    fn EnumerateTerminals();
    fn Terminals();
    fn Stream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITSubStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSubStreamVtbl {
        unsafe extern "system" fn StartSubStream<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PauseSubStream<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopSubStream<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectTerminal<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnselectTerminal<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminals<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stream<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            StartSubStream::<Impl, IMPL_OFFSET>,
            PauseSubStream::<Impl, IMPL_OFFSET>,
            StopSubStream::<Impl, IMPL_OFFSET>,
            SelectTerminal::<Impl, IMPL_OFFSET>,
            UnselectTerminal::<Impl, IMPL_OFFSET>,
            EnumerateTerminals::<Impl, IMPL_OFFSET>,
            Terminals::<Impl, IMPL_OFFSET>,
            Stream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSubStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITSubStreamControlImpl: Sized + IDispatchImpl {
    fn CreateSubStream();
    fn RemoveSubStream();
    fn EnumerateSubStreams();
    fn SubStreams();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITSubStreamControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSubStreamControlVtbl {
        unsafe extern "system" fn CreateSubStream<Impl: ITSubStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSubStream<Impl: ITSubStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateSubStreams<Impl: ITSubStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubStreams<Impl: ITSubStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreateSubStream::<Impl, IMPL_OFFSET>, RemoveSubStream::<Impl, IMPL_OFFSET>, EnumerateSubStreams::<Impl, IMPL_OFFSET>, SubStreams::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSubStreamControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Shutdown();
    fn Addresses();
    fn EnumerateAddresses();
    fn RegisterCallNotifications();
    fn UnregisterNotifications();
    fn CallHubs();
    fn EnumerateCallHubs();
    fn SetCallHubTracking();
    fn EnumeratePrivateTAPIObjects();
    fn PrivateTAPIObjects();
    fn RegisterRequestRecipient();
    fn SetAssistedTelephonyPriority();
    fn SetApplicationPriority();
    fn SetEventFilter();
    fn EventFilter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIVtbl {
        unsafe extern "system" fn Initialize<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Addresses<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterCallNotifications<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterNotifications<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lregister: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallHubs<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateCallHubs<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCallHubTracking<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, btracking: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePrivateTAPIObjects<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivateTAPIObjects<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterRequestRecipient<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAssistedTelephonyPriority<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetApplicationPriority<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfiltermask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventFilter<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfiltermask: *mut i32) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Shutdown::<Impl, IMPL_OFFSET>,
            Addresses::<Impl, IMPL_OFFSET>,
            EnumerateAddresses::<Impl, IMPL_OFFSET>,
            RegisterCallNotifications::<Impl, IMPL_OFFSET>,
            UnregisterNotifications::<Impl, IMPL_OFFSET>,
            CallHubs::<Impl, IMPL_OFFSET>,
            EnumerateCallHubs::<Impl, IMPL_OFFSET>,
            SetCallHubTracking::<Impl, IMPL_OFFSET>,
            EnumeratePrivateTAPIObjects::<Impl, IMPL_OFFSET>,
            PrivateTAPIObjects::<Impl, IMPL_OFFSET>,
            RegisterRequestRecipient::<Impl, IMPL_OFFSET>,
            SetAssistedTelephonyPriority::<Impl, IMPL_OFFSET>,
            SetApplicationPriority::<Impl, IMPL_OFFSET>,
            SetEventFilter::<Impl, IMPL_OFFSET>,
            EventFilter::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPI2Impl: Sized + ITTAPIImpl + IDispatchImpl {
    fn Phones();
    fn EnumeratePhones();
    fn CreateEmptyCollectionObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPI2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPI2Vtbl {
        unsafe extern "system" fn Phones<Impl: ITTAPI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePhones<Impl: ITTAPI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEmptyCollectionObject<Impl: ITTAPI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Initialize::<Impl, IMPL_OFFSET>,
            Shutdown::<Impl, IMPL_OFFSET>,
            Addresses::<Impl, IMPL_OFFSET>,
            EnumerateAddresses::<Impl, IMPL_OFFSET>,
            RegisterCallNotifications::<Impl, IMPL_OFFSET>,
            UnregisterNotifications::<Impl, IMPL_OFFSET>,
            CallHubs::<Impl, IMPL_OFFSET>,
            EnumerateCallHubs::<Impl, IMPL_OFFSET>,
            SetCallHubTracking::<Impl, IMPL_OFFSET>,
            EnumeratePrivateTAPIObjects::<Impl, IMPL_OFFSET>,
            PrivateTAPIObjects::<Impl, IMPL_OFFSET>,
            RegisterRequestRecipient::<Impl, IMPL_OFFSET>,
            SetAssistedTelephonyPriority::<Impl, IMPL_OFFSET>,
            SetApplicationPriority::<Impl, IMPL_OFFSET>,
            SetEventFilter::<Impl, IMPL_OFFSET>,
            EventFilter::<Impl, IMPL_OFFSET>,
            Phones::<Impl, IMPL_OFFSET>,
            EnumeratePhones::<Impl, IMPL_OFFSET>,
            CreateEmptyCollectionObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPI2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPICallCenterImpl: Sized + IDispatchImpl {
    fn EnumerateAgentHandlers();
    fn AgentHandlers();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPICallCenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPICallCenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPICallCenterVtbl {
        unsafe extern "system" fn EnumerateAgentHandlers<Impl: ITTAPICallCenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AgentHandlers<Impl: ITTAPICallCenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, EnumerateAgentHandlers::<Impl, IMPL_OFFSET>, AgentHandlers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPICallCenter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIDispatchEventNotificationImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIDispatchEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIDispatchEventNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIDispatchEventNotificationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIDispatchEventNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIEventNotificationImpl: Sized {
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIEventNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIEventNotificationVtbl {
        unsafe extern "system" fn Event<Impl: ITTAPIEventNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Event::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIEventNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIObjectEventImpl: Sized + IDispatchImpl {
    fn TAPIObject();
    fn Event();
    fn Address();
    fn CallbackInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIObjectEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIObjectEventVtbl {
        unsafe extern "system" fn TAPIObject<Impl: ITTAPIObjectEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Event<Impl: ITTAPIObjectEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Address<Impl: ITTAPIObjectEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITTAPIObjectEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, TAPIObject::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>, Address::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIObjectEvent2Impl: Sized + ITTAPIObjectEventImpl + IDispatchImpl {
    fn Phone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIObjectEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIObjectEvent2Vtbl {
        unsafe extern "system" fn Phone<Impl: ITTAPIObjectEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, TAPIObject::<Impl, IMPL_OFFSET>, Event::<Impl, IMPL_OFFSET>, Address::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>, Phone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTTSTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTTSTerminalEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTTSTerminalEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTTSTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITTTSTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Call<Impl: ITTTSTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: ITTTSTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Terminal::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Error::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTTSTerminalEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalImpl: Sized + IDispatchImpl {
    fn Name();
    fn State();
    fn TerminalType();
    fn TerminalClass();
    fn MediaType();
    fn Direction();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminalVtbl {
        unsafe extern "system" fn Name<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TerminalType<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TerminalClass<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaType<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Direction<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
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
            State::<Impl, IMPL_OFFSET>,
            TerminalType::<Impl, IMPL_OFFSET>,
            TerminalClass::<Impl, IMPL_OFFSET>,
            MediaType::<Impl, IMPL_OFFSET>,
            Direction::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalSupportImpl: Sized + IDispatchImpl {
    fn StaticTerminals();
    fn EnumerateStaticTerminals();
    fn DynamicTerminalClasses();
    fn EnumerateDynamicTerminalClasses();
    fn CreateTerminal();
    fn GetDefaultStaticTerminal();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminalSupportVtbl {
        unsafe extern "system" fn StaticTerminals<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateStaticTerminals<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DynamicTerminalClasses<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateDynamicTerminalClasses<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTerminal<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultStaticTerminal<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            StaticTerminals::<Impl, IMPL_OFFSET>,
            EnumerateStaticTerminals::<Impl, IMPL_OFFSET>,
            DynamicTerminalClasses::<Impl, IMPL_OFFSET>,
            EnumerateDynamicTerminalClasses::<Impl, IMPL_OFFSET>,
            CreateTerminal::<Impl, IMPL_OFFSET>,
            GetDefaultStaticTerminal::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminalSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalSupport2Impl: Sized + ITTerminalSupportImpl + IDispatchImpl {
    fn PluggableSuperclasses();
    fn EnumeratePluggableSuperclasses();
    fn PluggableTerminalClasses();
    fn EnumeratePluggableTerminalClasses();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalSupport2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminalSupport2Vtbl {
        unsafe extern "system" fn PluggableSuperclasses<Impl: ITTerminalSupport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePluggableSuperclasses<Impl: ITTerminalSupport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuperclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PluggableTerminalClasses<Impl: ITTerminalSupport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrterminalsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumeratePluggableTerminalClasses<Impl: ITTerminalSupport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iidterminalsuperclass: ::windows::core::GUID, lmediatype: i32, ppclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            StaticTerminals::<Impl, IMPL_OFFSET>,
            EnumerateStaticTerminals::<Impl, IMPL_OFFSET>,
            DynamicTerminalClasses::<Impl, IMPL_OFFSET>,
            EnumerateDynamicTerminalClasses::<Impl, IMPL_OFFSET>,
            CreateTerminal::<Impl, IMPL_OFFSET>,
            GetDefaultStaticTerminal::<Impl, IMPL_OFFSET>,
            PluggableSuperclasses::<Impl, IMPL_OFFSET>,
            EnumeratePluggableSuperclasses::<Impl, IMPL_OFFSET>,
            PluggableTerminalClasses::<Impl, IMPL_OFFSET>,
            EnumeratePluggableTerminalClasses::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminalSupport2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITToneDetectionEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn AppSpecific();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITToneDetectionEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITToneDetectionEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITToneDetectionEventVtbl {
        unsafe extern "system" fn Call<Impl: ITToneDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppSpecific<Impl: ITToneDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TickCount<Impl: ITToneDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITToneDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, AppSpecific::<Impl, IMPL_OFFSET>, TickCount::<Impl, IMPL_OFFSET>, CallbackInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITToneDetectionEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITToneTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITToneTerminalEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITToneTerminalEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITToneTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITToneTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Call<Impl: ITToneTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: ITToneTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Terminal::<Impl, IMPL_OFFSET>, Call::<Impl, IMPL_OFFSET>, Error::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITToneTerminalEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub trait ITnefImpl: Sized {
    fn AddProps();
    fn ExtractProps();
    fn Finish();
    fn OpenTaggedBody();
    fn SetProps();
    fn EncodeRecips();
    fn FinishComponent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
impl ITnefVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITnefImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITnefVtbl {
        unsafe extern "system" fn AddProps<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtractProps<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenTaggedBody<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmessage: ::windows::core::RawPtr, ulflags: u32, lppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProps<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodeRecips<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprecipienttable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinishComponent<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddProps::<Impl, IMPL_OFFSET>, ExtractProps::<Impl, IMPL_OFFSET>, Finish::<Impl, IMPL_OFFSET>, OpenTaggedBody::<Impl, IMPL_OFFSET>, SetProps::<Impl, IMPL_OFFSET>, EncodeRecips::<Impl, IMPL_OFFSET>, FinishComponent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITnef as ::windows::core::Interface>::IID
    }
}
