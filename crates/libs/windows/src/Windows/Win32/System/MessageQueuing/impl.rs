#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplicationImpl: Sized + IDispatchImpl {
    fn MachineIdOfMachineName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplicationVtbl {
        unsafe extern "system" fn MachineIdOfMachineName<Impl: IMSMQApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, MachineIdOfMachineName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication2Impl: Sized + IMSMQApplicationImpl + IDispatchImpl {
    fn RegisterCertificate();
    fn MachineNameOfMachineId();
    fn MSMQVersionMajor();
    fn MSMQVersionMinor();
    fn MSMQVersionBuild();
    fn IsDsEnabled();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplication2Vtbl {
        unsafe extern "system" fn RegisterCertificate<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MachineNameOfMachineId<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MSMQVersionMajor<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MSMQVersionMinor<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MSMQVersionBuild<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDsEnabled<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            MachineIdOfMachineName::<Impl, IMPL_OFFSET>,
            RegisterCertificate::<Impl, IMPL_OFFSET>,
            MachineNameOfMachineId::<Impl, IMPL_OFFSET>,
            MSMQVersionMajor::<Impl, IMPL_OFFSET>,
            MSMQVersionMinor::<Impl, IMPL_OFFSET>,
            MSMQVersionBuild::<Impl, IMPL_OFFSET>,
            IsDsEnabled::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication3Impl: Sized + IMSMQApplication2Impl + IMSMQApplicationImpl + IDispatchImpl {
    fn ActiveQueues();
    fn PrivateQueues();
    fn DirectoryServiceServer();
    fn IsConnected();
    fn BytesInAllQueues();
    fn SetMachine();
    fn Machine();
    fn Connect();
    fn Disconnect();
    fn Tidy();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplication3Vtbl {
        unsafe extern "system" fn ActiveQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivateQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DirectoryServiceServer<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnected<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BytesInAllQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMachine<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Machine<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connect<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Tidy<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            MachineIdOfMachineName::<Impl, IMPL_OFFSET>,
            RegisterCertificate::<Impl, IMPL_OFFSET>,
            MachineNameOfMachineId::<Impl, IMPL_OFFSET>,
            MSMQVersionMajor::<Impl, IMPL_OFFSET>,
            MSMQVersionMinor::<Impl, IMPL_OFFSET>,
            MSMQVersionBuild::<Impl, IMPL_OFFSET>,
            IsDsEnabled::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            ActiveQueues::<Impl, IMPL_OFFSET>,
            PrivateQueues::<Impl, IMPL_OFFSET>,
            DirectoryServiceServer::<Impl, IMPL_OFFSET>,
            IsConnected::<Impl, IMPL_OFFSET>,
            BytesInAllQueues::<Impl, IMPL_OFFSET>,
            SetMachine::<Impl, IMPL_OFFSET>,
            Machine::<Impl, IMPL_OFFSET>,
            Connect::<Impl, IMPL_OFFSET>,
            Disconnect::<Impl, IMPL_OFFSET>,
            Tidy::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenserImpl: Sized + IDispatchImpl {
    fn BeginTransaction();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenserVtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BeginTransaction::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser2Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser2Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQCoordinatedTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BeginTransaction::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser3Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser3Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQCoordinatedTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BeginTransaction::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQDestinationImpl: Sized + IDispatchImpl {
    fn Open();
    fn Close();
    fn IsOpen();
    fn IADs();
    fn putref_IADs();
    fn ADsPath();
    fn SetADsPath();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn Destinations();
    fn putref_Destinations();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQDestinationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestinationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQDestinationVtbl {
        unsafe extern "system" fn Open<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IADs<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiads: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_IADs<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piads: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetADsPath<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destinations<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestinations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_Destinations<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinations: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            IsOpen::<Impl, IMPL_OFFSET>,
            IADs::<Impl, IMPL_OFFSET>,
            putref_IADs::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
            SetADsPath::<Impl, IMPL_OFFSET>,
            PathName::<Impl, IMPL_OFFSET>,
            SetPathName::<Impl, IMPL_OFFSET>,
            FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName::<Impl, IMPL_OFFSET>,
            Destinations::<Impl, IMPL_OFFSET>,
            putref_Destinations::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQDestination as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEventImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQEventVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent2Impl: Sized + IMSMQEventImpl + IDispatchImpl {
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQEvent2Vtbl {
        unsafe extern "system" fn Properties<Impl: IMSMQEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent3Impl: Sized + IMSMQEvent2Impl + IMSMQEventImpl + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQEvent3Vtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQManagementImpl: Sized + IDispatchImpl {
    fn Init();
    fn FormatName();
    fn Machine();
    fn MessageCount();
    fn ForeignStatus();
    fn QueueType();
    fn IsLocal();
    fn TransactionalStatus();
    fn BytesInQueue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQManagementVtbl {
        unsafe extern "system" fn Init<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Machine<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MessageCount<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForeignStatus<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueType<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLocal<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionalStatus<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BytesInQueue<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
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
            Init::<Impl, IMPL_OFFSET>,
            FormatName::<Impl, IMPL_OFFSET>,
            Machine::<Impl, IMPL_OFFSET>,
            MessageCount::<Impl, IMPL_OFFSET>,
            ForeignStatus::<Impl, IMPL_OFFSET>,
            QueueType::<Impl, IMPL_OFFSET>,
            IsLocal::<Impl, IMPL_OFFSET>,
            TransactionalStatus::<Impl, IMPL_OFFSET>,
            BytesInQueue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessageImpl: Sized + IDispatchImpl {
    fn Class();
    fn PrivLevel();
    fn SetPrivLevel();
    fn AuthLevel();
    fn SetAuthLevel();
    fn IsAuthenticated();
    fn Delivery();
    fn SetDelivery();
    fn Trace();
    fn SetTrace();
    fn Priority();
    fn SetPriority();
    fn Journal();
    fn SetJournal();
    fn ResponseQueueInfo();
    fn putref_ResponseQueueInfo();
    fn AppSpecific();
    fn SetAppSpecific();
    fn SourceMachineGuid();
    fn BodyLength();
    fn Body();
    fn SetBody();
    fn AdminQueueInfo();
    fn putref_AdminQueueInfo();
    fn Id();
    fn CorrelationId();
    fn SetCorrelationId();
    fn Ack();
    fn SetAck();
    fn Label();
    fn SetLabel();
    fn MaxTimeToReachQueue();
    fn SetMaxTimeToReachQueue();
    fn MaxTimeToReceive();
    fn SetMaxTimeToReceive();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn EncryptAlgorithm();
    fn SetEncryptAlgorithm();
    fn SentTime();
    fn ArrivedTime();
    fn DestinationQueueInfo();
    fn SenderCertificate();
    fn SetSenderCertificate();
    fn SenderId();
    fn SenderIdType();
    fn SetSenderIdType();
    fn Send();
    fn AttachCurrentSecurityContext();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessageVtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Class::<Impl, IMPL_OFFSET>,
            PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel::<Impl, IMPL_OFFSET>,
            AuthLevel::<Impl, IMPL_OFFSET>,
            SetAuthLevel::<Impl, IMPL_OFFSET>,
            IsAuthenticated::<Impl, IMPL_OFFSET>,
            Delivery::<Impl, IMPL_OFFSET>,
            SetDelivery::<Impl, IMPL_OFFSET>,
            Trace::<Impl, IMPL_OFFSET>,
            SetTrace::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            Journal::<Impl, IMPL_OFFSET>,
            SetJournal::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific::<Impl, IMPL_OFFSET>,
            SourceMachineGuid::<Impl, IMPL_OFFSET>,
            BodyLength::<Impl, IMPL_OFFSET>,
            Body::<Impl, IMPL_OFFSET>,
            SetBody::<Impl, IMPL_OFFSET>,
            AdminQueueInfo::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId::<Impl, IMPL_OFFSET>,
            Ack::<Impl, IMPL_OFFSET>,
            SetAck::<Impl, IMPL_OFFSET>,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            MaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            MaxTimeToReceive::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReceive::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SentTime::<Impl, IMPL_OFFSET>,
            ArrivedTime::<Impl, IMPL_OFFSET>,
            DestinationQueueInfo::<Impl, IMPL_OFFSET>,
            SenderCertificate::<Impl, IMPL_OFFSET>,
            SetSenderCertificate::<Impl, IMPL_OFFSET>,
            SenderId::<Impl, IMPL_OFFSET>,
            SenderIdType::<Impl, IMPL_OFFSET>,
            SetSenderIdType::<Impl, IMPL_OFFSET>,
            Send::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage2Impl: Sized + IDispatchImpl {
    fn Class();
    fn PrivLevel();
    fn SetPrivLevel();
    fn AuthLevel();
    fn SetAuthLevel();
    fn IsAuthenticated();
    fn Delivery();
    fn SetDelivery();
    fn Trace();
    fn SetTrace();
    fn Priority();
    fn SetPriority();
    fn Journal();
    fn SetJournal();
    fn ResponseQueueInfo_v1();
    fn putref_ResponseQueueInfo_v1();
    fn AppSpecific();
    fn SetAppSpecific();
    fn SourceMachineGuid();
    fn BodyLength();
    fn Body();
    fn SetBody();
    fn AdminQueueInfo_v1();
    fn putref_AdminQueueInfo_v1();
    fn Id();
    fn CorrelationId();
    fn SetCorrelationId();
    fn Ack();
    fn SetAck();
    fn Label();
    fn SetLabel();
    fn MaxTimeToReachQueue();
    fn SetMaxTimeToReachQueue();
    fn MaxTimeToReceive();
    fn SetMaxTimeToReceive();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn EncryptAlgorithm();
    fn SetEncryptAlgorithm();
    fn SentTime();
    fn ArrivedTime();
    fn DestinationQueueInfo();
    fn SenderCertificate();
    fn SetSenderCertificate();
    fn SenderId();
    fn SenderIdType();
    fn SetSenderIdType();
    fn Send();
    fn AttachCurrentSecurityContext();
    fn SenderVersion();
    fn Extension();
    fn SetExtension();
    fn ConnectorTypeGuid();
    fn SetConnectorTypeGuid();
    fn TransactionStatusQueueInfo();
    fn DestinationSymmetricKey();
    fn SetDestinationSymmetricKey();
    fn Signature();
    fn SetSignature();
    fn AuthenticationProviderType();
    fn SetAuthenticationProviderType();
    fn AuthenticationProviderName();
    fn SetAuthenticationProviderName();
    fn SetSenderId();
    fn MsgClass();
    fn SetMsgClass();
    fn Properties();
    fn TransactionId();
    fn IsFirstInTransaction();
    fn IsLastInTransaction();
    fn ResponseQueueInfo();
    fn putref_ResponseQueueInfo();
    fn AdminQueueInfo();
    fn putref_AdminQueueInfo();
    fn ReceivedAuthenticationLevel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage2Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
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
            Class::<Impl, IMPL_OFFSET>,
            PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel::<Impl, IMPL_OFFSET>,
            AuthLevel::<Impl, IMPL_OFFSET>,
            SetAuthLevel::<Impl, IMPL_OFFSET>,
            IsAuthenticated::<Impl, IMPL_OFFSET>,
            Delivery::<Impl, IMPL_OFFSET>,
            SetDelivery::<Impl, IMPL_OFFSET>,
            Trace::<Impl, IMPL_OFFSET>,
            SetTrace::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            Journal::<Impl, IMPL_OFFSET>,
            SetJournal::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific::<Impl, IMPL_OFFSET>,
            SourceMachineGuid::<Impl, IMPL_OFFSET>,
            BodyLength::<Impl, IMPL_OFFSET>,
            Body::<Impl, IMPL_OFFSET>,
            SetBody::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId::<Impl, IMPL_OFFSET>,
            Ack::<Impl, IMPL_OFFSET>,
            SetAck::<Impl, IMPL_OFFSET>,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            MaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            MaxTimeToReceive::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReceive::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SentTime::<Impl, IMPL_OFFSET>,
            ArrivedTime::<Impl, IMPL_OFFSET>,
            DestinationQueueInfo::<Impl, IMPL_OFFSET>,
            SenderCertificate::<Impl, IMPL_OFFSET>,
            SetSenderCertificate::<Impl, IMPL_OFFSET>,
            SenderId::<Impl, IMPL_OFFSET>,
            SenderIdType::<Impl, IMPL_OFFSET>,
            SetSenderIdType::<Impl, IMPL_OFFSET>,
            Send::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext::<Impl, IMPL_OFFSET>,
            SenderVersion::<Impl, IMPL_OFFSET>,
            Extension::<Impl, IMPL_OFFSET>,
            SetExtension::<Impl, IMPL_OFFSET>,
            ConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            SetConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            TransactionStatusQueueInfo::<Impl, IMPL_OFFSET>,
            DestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            SetDestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            SetSignature::<Impl, IMPL_OFFSET>,
            AuthenticationProviderType::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderType::<Impl, IMPL_OFFSET>,
            AuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetSenderId::<Impl, IMPL_OFFSET>,
            MsgClass::<Impl, IMPL_OFFSET>,
            SetMsgClass::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            TransactionId::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction::<Impl, IMPL_OFFSET>,
            IsLastInTransaction::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            AdminQueueInfo::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo::<Impl, IMPL_OFFSET>,
            ReceivedAuthenticationLevel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage3Impl: Sized + IDispatchImpl {
    fn Class();
    fn PrivLevel();
    fn SetPrivLevel();
    fn AuthLevel();
    fn SetAuthLevel();
    fn IsAuthenticated();
    fn Delivery();
    fn SetDelivery();
    fn Trace();
    fn SetTrace();
    fn Priority();
    fn SetPriority();
    fn Journal();
    fn SetJournal();
    fn ResponseQueueInfo_v1();
    fn putref_ResponseQueueInfo_v1();
    fn AppSpecific();
    fn SetAppSpecific();
    fn SourceMachineGuid();
    fn BodyLength();
    fn Body();
    fn SetBody();
    fn AdminQueueInfo_v1();
    fn putref_AdminQueueInfo_v1();
    fn Id();
    fn CorrelationId();
    fn SetCorrelationId();
    fn Ack();
    fn SetAck();
    fn Label();
    fn SetLabel();
    fn MaxTimeToReachQueue();
    fn SetMaxTimeToReachQueue();
    fn MaxTimeToReceive();
    fn SetMaxTimeToReceive();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn EncryptAlgorithm();
    fn SetEncryptAlgorithm();
    fn SentTime();
    fn ArrivedTime();
    fn DestinationQueueInfo();
    fn SenderCertificate();
    fn SetSenderCertificate();
    fn SenderId();
    fn SenderIdType();
    fn SetSenderIdType();
    fn Send();
    fn AttachCurrentSecurityContext();
    fn SenderVersion();
    fn Extension();
    fn SetExtension();
    fn ConnectorTypeGuid();
    fn SetConnectorTypeGuid();
    fn TransactionStatusQueueInfo();
    fn DestinationSymmetricKey();
    fn SetDestinationSymmetricKey();
    fn Signature();
    fn SetSignature();
    fn AuthenticationProviderType();
    fn SetAuthenticationProviderType();
    fn AuthenticationProviderName();
    fn SetAuthenticationProviderName();
    fn SetSenderId();
    fn MsgClass();
    fn SetMsgClass();
    fn Properties();
    fn TransactionId();
    fn IsFirstInTransaction();
    fn IsLastInTransaction();
    fn ResponseQueueInfo_v2();
    fn putref_ResponseQueueInfo_v2();
    fn AdminQueueInfo_v2();
    fn putref_AdminQueueInfo_v2();
    fn ReceivedAuthenticationLevel();
    fn ResponseQueueInfo();
    fn putref_ResponseQueueInfo();
    fn AdminQueueInfo();
    fn putref_AdminQueueInfo();
    fn ResponseDestination();
    fn putref_ResponseDestination();
    fn Destination();
    fn LookupId();
    fn IsAuthenticated2();
    fn IsFirstInTransaction2();
    fn IsLastInTransaction2();
    fn AttachCurrentSecurityContext2();
    fn SoapEnvelope();
    fn CompoundMessage();
    fn SetSoapHeader();
    fn SetSoapBody();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage3Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseDestination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseDestination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LookupId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAuthenticated2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFirstInTransaction2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLastInTransaction2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SoapEnvelope<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompoundMessage<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSoapHeader<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSoapBody<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Class::<Impl, IMPL_OFFSET>,
            PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel::<Impl, IMPL_OFFSET>,
            AuthLevel::<Impl, IMPL_OFFSET>,
            SetAuthLevel::<Impl, IMPL_OFFSET>,
            IsAuthenticated::<Impl, IMPL_OFFSET>,
            Delivery::<Impl, IMPL_OFFSET>,
            SetDelivery::<Impl, IMPL_OFFSET>,
            Trace::<Impl, IMPL_OFFSET>,
            SetTrace::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            Journal::<Impl, IMPL_OFFSET>,
            SetJournal::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific::<Impl, IMPL_OFFSET>,
            SourceMachineGuid::<Impl, IMPL_OFFSET>,
            BodyLength::<Impl, IMPL_OFFSET>,
            Body::<Impl, IMPL_OFFSET>,
            SetBody::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId::<Impl, IMPL_OFFSET>,
            Ack::<Impl, IMPL_OFFSET>,
            SetAck::<Impl, IMPL_OFFSET>,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            MaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            MaxTimeToReceive::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReceive::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SentTime::<Impl, IMPL_OFFSET>,
            ArrivedTime::<Impl, IMPL_OFFSET>,
            DestinationQueueInfo::<Impl, IMPL_OFFSET>,
            SenderCertificate::<Impl, IMPL_OFFSET>,
            SetSenderCertificate::<Impl, IMPL_OFFSET>,
            SenderId::<Impl, IMPL_OFFSET>,
            SenderIdType::<Impl, IMPL_OFFSET>,
            SetSenderIdType::<Impl, IMPL_OFFSET>,
            Send::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext::<Impl, IMPL_OFFSET>,
            SenderVersion::<Impl, IMPL_OFFSET>,
            Extension::<Impl, IMPL_OFFSET>,
            SetExtension::<Impl, IMPL_OFFSET>,
            ConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            SetConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            TransactionStatusQueueInfo::<Impl, IMPL_OFFSET>,
            DestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            SetDestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            SetSignature::<Impl, IMPL_OFFSET>,
            AuthenticationProviderType::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderType::<Impl, IMPL_OFFSET>,
            AuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetSenderId::<Impl, IMPL_OFFSET>,
            MsgClass::<Impl, IMPL_OFFSET>,
            SetMsgClass::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            TransactionId::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction::<Impl, IMPL_OFFSET>,
            IsLastInTransaction::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v2::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v2::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v2::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v2::<Impl, IMPL_OFFSET>,
            ReceivedAuthenticationLevel::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            AdminQueueInfo::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo::<Impl, IMPL_OFFSET>,
            ResponseDestination::<Impl, IMPL_OFFSET>,
            putref_ResponseDestination::<Impl, IMPL_OFFSET>,
            Destination::<Impl, IMPL_OFFSET>,
            LookupId::<Impl, IMPL_OFFSET>,
            IsAuthenticated2::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction2::<Impl, IMPL_OFFSET>,
            IsLastInTransaction2::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext2::<Impl, IMPL_OFFSET>,
            SoapEnvelope::<Impl, IMPL_OFFSET>,
            CompoundMessage::<Impl, IMPL_OFFSET>,
            SetSoapHeader::<Impl, IMPL_OFFSET>,
            SetSoapBody::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage4Impl: Sized + IDispatchImpl {
    fn Class();
    fn PrivLevel();
    fn SetPrivLevel();
    fn AuthLevel();
    fn SetAuthLevel();
    fn IsAuthenticated();
    fn Delivery();
    fn SetDelivery();
    fn Trace();
    fn SetTrace();
    fn Priority();
    fn SetPriority();
    fn Journal();
    fn SetJournal();
    fn ResponseQueueInfo_v1();
    fn putref_ResponseQueueInfo_v1();
    fn AppSpecific();
    fn SetAppSpecific();
    fn SourceMachineGuid();
    fn BodyLength();
    fn Body();
    fn SetBody();
    fn AdminQueueInfo_v1();
    fn putref_AdminQueueInfo_v1();
    fn Id();
    fn CorrelationId();
    fn SetCorrelationId();
    fn Ack();
    fn SetAck();
    fn Label();
    fn SetLabel();
    fn MaxTimeToReachQueue();
    fn SetMaxTimeToReachQueue();
    fn MaxTimeToReceive();
    fn SetMaxTimeToReceive();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn EncryptAlgorithm();
    fn SetEncryptAlgorithm();
    fn SentTime();
    fn ArrivedTime();
    fn DestinationQueueInfo();
    fn SenderCertificate();
    fn SetSenderCertificate();
    fn SenderId();
    fn SenderIdType();
    fn SetSenderIdType();
    fn Send();
    fn AttachCurrentSecurityContext();
    fn SenderVersion();
    fn Extension();
    fn SetExtension();
    fn ConnectorTypeGuid();
    fn SetConnectorTypeGuid();
    fn TransactionStatusQueueInfo();
    fn DestinationSymmetricKey();
    fn SetDestinationSymmetricKey();
    fn Signature();
    fn SetSignature();
    fn AuthenticationProviderType();
    fn SetAuthenticationProviderType();
    fn AuthenticationProviderName();
    fn SetAuthenticationProviderName();
    fn SetSenderId();
    fn MsgClass();
    fn SetMsgClass();
    fn Properties();
    fn TransactionId();
    fn IsFirstInTransaction();
    fn IsLastInTransaction();
    fn ResponseQueueInfo_v2();
    fn putref_ResponseQueueInfo_v2();
    fn AdminQueueInfo_v2();
    fn putref_AdminQueueInfo_v2();
    fn ReceivedAuthenticationLevel();
    fn ResponseQueueInfo();
    fn putref_ResponseQueueInfo();
    fn AdminQueueInfo();
    fn putref_AdminQueueInfo();
    fn ResponseDestination();
    fn putref_ResponseDestination();
    fn Destination();
    fn LookupId();
    fn IsAuthenticated2();
    fn IsFirstInTransaction2();
    fn IsLastInTransaction2();
    fn AttachCurrentSecurityContext2();
    fn SoapEnvelope();
    fn CompoundMessage();
    fn SetSoapHeader();
    fn SetSoapBody();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage4Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResponseDestination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn putref_ResponseDestination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Destination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LookupId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAuthenticated2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFirstInTransaction2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLastInTransaction2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SoapEnvelope<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompoundMessage<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSoapHeader<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSoapBody<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Class::<Impl, IMPL_OFFSET>,
            PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel::<Impl, IMPL_OFFSET>,
            AuthLevel::<Impl, IMPL_OFFSET>,
            SetAuthLevel::<Impl, IMPL_OFFSET>,
            IsAuthenticated::<Impl, IMPL_OFFSET>,
            Delivery::<Impl, IMPL_OFFSET>,
            SetDelivery::<Impl, IMPL_OFFSET>,
            Trace::<Impl, IMPL_OFFSET>,
            SetTrace::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            Journal::<Impl, IMPL_OFFSET>,
            SetJournal::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific::<Impl, IMPL_OFFSET>,
            SourceMachineGuid::<Impl, IMPL_OFFSET>,
            BodyLength::<Impl, IMPL_OFFSET>,
            Body::<Impl, IMPL_OFFSET>,
            SetBody::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId::<Impl, IMPL_OFFSET>,
            Ack::<Impl, IMPL_OFFSET>,
            SetAck::<Impl, IMPL_OFFSET>,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            MaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            MaxTimeToReceive::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReceive::<Impl, IMPL_OFFSET>,
            HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SentTime::<Impl, IMPL_OFFSET>,
            ArrivedTime::<Impl, IMPL_OFFSET>,
            DestinationQueueInfo::<Impl, IMPL_OFFSET>,
            SenderCertificate::<Impl, IMPL_OFFSET>,
            SetSenderCertificate::<Impl, IMPL_OFFSET>,
            SenderId::<Impl, IMPL_OFFSET>,
            SenderIdType::<Impl, IMPL_OFFSET>,
            SetSenderIdType::<Impl, IMPL_OFFSET>,
            Send::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext::<Impl, IMPL_OFFSET>,
            SenderVersion::<Impl, IMPL_OFFSET>,
            Extension::<Impl, IMPL_OFFSET>,
            SetExtension::<Impl, IMPL_OFFSET>,
            ConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            SetConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            TransactionStatusQueueInfo::<Impl, IMPL_OFFSET>,
            DestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            SetDestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            Signature::<Impl, IMPL_OFFSET>,
            SetSignature::<Impl, IMPL_OFFSET>,
            AuthenticationProviderType::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderType::<Impl, IMPL_OFFSET>,
            AuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetSenderId::<Impl, IMPL_OFFSET>,
            MsgClass::<Impl, IMPL_OFFSET>,
            SetMsgClass::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            TransactionId::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction::<Impl, IMPL_OFFSET>,
            IsLastInTransaction::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v2::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v2::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v2::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v2::<Impl, IMPL_OFFSET>,
            ReceivedAuthenticationLevel::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            AdminQueueInfo::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo::<Impl, IMPL_OFFSET>,
            ResponseDestination::<Impl, IMPL_OFFSET>,
            putref_ResponseDestination::<Impl, IMPL_OFFSET>,
            Destination::<Impl, IMPL_OFFSET>,
            LookupId::<Impl, IMPL_OFFSET>,
            IsAuthenticated2::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction2::<Impl, IMPL_OFFSET>,
            IsLastInTransaction2::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext2::<Impl, IMPL_OFFSET>,
            SoapEnvelope::<Impl, IMPL_OFFSET>,
            CompoundMessage::<Impl, IMPL_OFFSET>,
            SetSoapHeader::<Impl, IMPL_OFFSET>,
            SetSoapBody::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQOutgoingQueueManagementImpl: Sized + IMSMQManagementImpl + IDispatchImpl {
    fn State();
    fn NextHops();
    fn EodGetSendInfo();
    fn Resume();
    fn Pause();
    fn EodResend();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQOutgoingQueueManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQOutgoingQueueManagementVtbl {
        unsafe extern "system" fn State<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextHops<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EodGetSendInfo<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EodResend<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Init::<Impl, IMPL_OFFSET>,
            FormatName::<Impl, IMPL_OFFSET>,
            Machine::<Impl, IMPL_OFFSET>,
            MessageCount::<Impl, IMPL_OFFSET>,
            ForeignStatus::<Impl, IMPL_OFFSET>,
            QueueType::<Impl, IMPL_OFFSET>,
            IsLocal::<Impl, IMPL_OFFSET>,
            TransactionalStatus::<Impl, IMPL_OFFSET>,
            BytesInQueue::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            NextHops::<Impl, IMPL_OFFSET>,
            EodGetSendInfo::<Impl, IMPL_OFFSET>,
            Resume::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            EodResend::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQOutgoingQueueManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateDestinationImpl: Sized + IDispatchImpl {
    fn Handle();
    fn SetHandle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateDestinationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateDestinationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQPrivateDestinationVtbl {
        unsafe extern "system" fn Handle<Impl: IMSMQPrivateDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHandle<Impl: IMSMQPrivateDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Handle::<Impl, IMPL_OFFSET>, SetHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateDestination as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateEventImpl: Sized + IDispatchImpl {
    fn Hwnd();
    fn FireArrivedEvent();
    fn FireArrivedErrorEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQPrivateEventVtbl {
        unsafe extern "system" fn Hwnd<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FireArrivedEvent<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Hwnd::<Impl, IMPL_OFFSET>, FireArrivedEvent::<Impl, IMPL_OFFSET>, FireArrivedErrorEvent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueryImpl: Sized + IDispatchImpl {
    fn LookupQueue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueryVtbl {
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, LookupQueue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery2Impl: Sized + IDispatchImpl {
    fn LookupQueue();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery2Vtbl {
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, LookupQueue::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery3Impl: Sized + IDispatchImpl {
    fn LookupQueue_v2();
    fn Properties();
    fn LookupQueue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery3Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, LookupQueue_v2::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>, LookupQueue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery4Impl: Sized + IDispatchImpl {
    fn LookupQueue_v2();
    fn Properties();
    fn LookupQueue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery4Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, LookupQueue_v2::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>, LookupQueue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueImpl: Sized + IDispatchImpl {
    fn Access();
    fn ShareMode();
    fn QueueInfo();
    fn Handle();
    fn IsOpen();
    fn Close();
    fn Receive();
    fn Peek();
    fn EnableNotification();
    fn Reset();
    fn ReceiveCurrent();
    fn PeekNext();
    fn PeekCurrent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueVtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Access::<Impl, IMPL_OFFSET>,
            ShareMode::<Impl, IMPL_OFFSET>,
            QueueInfo::<Impl, IMPL_OFFSET>,
            Handle::<Impl, IMPL_OFFSET>,
            IsOpen::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Receive::<Impl, IMPL_OFFSET>,
            Peek::<Impl, IMPL_OFFSET>,
            EnableNotification::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            ReceiveCurrent::<Impl, IMPL_OFFSET>,
            PeekNext::<Impl, IMPL_OFFSET>,
            PeekCurrent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue2Impl: Sized + IDispatchImpl {
    fn Access();
    fn ShareMode();
    fn QueueInfo();
    fn Handle();
    fn IsOpen();
    fn Close();
    fn Receive_v1();
    fn Peek_v1();
    fn EnableNotification();
    fn Reset();
    fn ReceiveCurrent_v1();
    fn PeekNext_v1();
    fn PeekCurrent_v1();
    fn Receive();
    fn Peek();
    fn ReceiveCurrent();
    fn PeekNext();
    fn PeekCurrent();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue2Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Access::<Impl, IMPL_OFFSET>,
            ShareMode::<Impl, IMPL_OFFSET>,
            QueueInfo::<Impl, IMPL_OFFSET>,
            Handle::<Impl, IMPL_OFFSET>,
            IsOpen::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Receive_v1::<Impl, IMPL_OFFSET>,
            Peek_v1::<Impl, IMPL_OFFSET>,
            EnableNotification::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            ReceiveCurrent_v1::<Impl, IMPL_OFFSET>,
            PeekNext_v1::<Impl, IMPL_OFFSET>,
            PeekCurrent_v1::<Impl, IMPL_OFFSET>,
            Receive::<Impl, IMPL_OFFSET>,
            Peek::<Impl, IMPL_OFFSET>,
            ReceiveCurrent::<Impl, IMPL_OFFSET>,
            PeekNext::<Impl, IMPL_OFFSET>,
            PeekCurrent::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue3Impl: Sized + IDispatchImpl {
    fn Access();
    fn ShareMode();
    fn QueueInfo();
    fn Handle();
    fn IsOpen();
    fn Close();
    fn Receive_v1();
    fn Peek_v1();
    fn EnableNotification();
    fn Reset();
    fn ReceiveCurrent_v1();
    fn PeekNext_v1();
    fn PeekCurrent_v1();
    fn Receive();
    fn Peek();
    fn ReceiveCurrent();
    fn PeekNext();
    fn PeekCurrent();
    fn Properties();
    fn Handle2();
    fn ReceiveByLookupId();
    fn ReceiveNextByLookupId();
    fn ReceivePreviousByLookupId();
    fn ReceiveFirstByLookupId();
    fn ReceiveLastByLookupId();
    fn PeekByLookupId();
    fn PeekNextByLookupId();
    fn PeekPreviousByLookupId();
    fn PeekFirstByLookupId();
    fn PeekLastByLookupId();
    fn Purge();
    fn IsOpen2();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue3Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Handle2<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNextByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekFirstByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekLastByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Purge<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOpen2<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
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
            Access::<Impl, IMPL_OFFSET>,
            ShareMode::<Impl, IMPL_OFFSET>,
            QueueInfo::<Impl, IMPL_OFFSET>,
            Handle::<Impl, IMPL_OFFSET>,
            IsOpen::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Receive_v1::<Impl, IMPL_OFFSET>,
            Peek_v1::<Impl, IMPL_OFFSET>,
            EnableNotification::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            ReceiveCurrent_v1::<Impl, IMPL_OFFSET>,
            PeekNext_v1::<Impl, IMPL_OFFSET>,
            PeekCurrent_v1::<Impl, IMPL_OFFSET>,
            Receive::<Impl, IMPL_OFFSET>,
            Peek::<Impl, IMPL_OFFSET>,
            ReceiveCurrent::<Impl, IMPL_OFFSET>,
            PeekNext::<Impl, IMPL_OFFSET>,
            PeekCurrent::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            Handle2::<Impl, IMPL_OFFSET>,
            ReceiveByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveNextByLookupId::<Impl, IMPL_OFFSET>,
            ReceivePreviousByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveFirstByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveLastByLookupId::<Impl, IMPL_OFFSET>,
            PeekByLookupId::<Impl, IMPL_OFFSET>,
            PeekNextByLookupId::<Impl, IMPL_OFFSET>,
            PeekPreviousByLookupId::<Impl, IMPL_OFFSET>,
            PeekFirstByLookupId::<Impl, IMPL_OFFSET>,
            PeekLastByLookupId::<Impl, IMPL_OFFSET>,
            Purge::<Impl, IMPL_OFFSET>,
            IsOpen2::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue4Impl: Sized + IDispatchImpl {
    fn Access();
    fn ShareMode();
    fn QueueInfo();
    fn Handle();
    fn IsOpen();
    fn Close();
    fn Receive_v1();
    fn Peek_v1();
    fn EnableNotification();
    fn Reset();
    fn ReceiveCurrent_v1();
    fn PeekNext_v1();
    fn PeekCurrent_v1();
    fn Receive();
    fn Peek();
    fn ReceiveCurrent();
    fn PeekNext();
    fn PeekCurrent();
    fn Properties();
    fn Handle2();
    fn ReceiveByLookupId();
    fn ReceiveNextByLookupId();
    fn ReceivePreviousByLookupId();
    fn ReceiveFirstByLookupId();
    fn ReceiveLastByLookupId();
    fn PeekByLookupId();
    fn PeekNextByLookupId();
    fn PeekPreviousByLookupId();
    fn PeekFirstByLookupId();
    fn PeekLastByLookupId();
    fn Purge();
    fn IsOpen2();
    fn ReceiveByLookupIdAllowPeek();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue4Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Handle2<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekNextByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekFirstByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PeekLastByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Purge<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOpen2<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Access::<Impl, IMPL_OFFSET>,
            ShareMode::<Impl, IMPL_OFFSET>,
            QueueInfo::<Impl, IMPL_OFFSET>,
            Handle::<Impl, IMPL_OFFSET>,
            IsOpen::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            Receive_v1::<Impl, IMPL_OFFSET>,
            Peek_v1::<Impl, IMPL_OFFSET>,
            EnableNotification::<Impl, IMPL_OFFSET>,
            Reset::<Impl, IMPL_OFFSET>,
            ReceiveCurrent_v1::<Impl, IMPL_OFFSET>,
            PeekNext_v1::<Impl, IMPL_OFFSET>,
            PeekCurrent_v1::<Impl, IMPL_OFFSET>,
            Receive::<Impl, IMPL_OFFSET>,
            Peek::<Impl, IMPL_OFFSET>,
            ReceiveCurrent::<Impl, IMPL_OFFSET>,
            PeekNext::<Impl, IMPL_OFFSET>,
            PeekCurrent::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            Handle2::<Impl, IMPL_OFFSET>,
            ReceiveByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveNextByLookupId::<Impl, IMPL_OFFSET>,
            ReceivePreviousByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveFirstByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveLastByLookupId::<Impl, IMPL_OFFSET>,
            PeekByLookupId::<Impl, IMPL_OFFSET>,
            PeekNextByLookupId::<Impl, IMPL_OFFSET>,
            PeekPreviousByLookupId::<Impl, IMPL_OFFSET>,
            PeekFirstByLookupId::<Impl, IMPL_OFFSET>,
            PeekLastByLookupId::<Impl, IMPL_OFFSET>,
            Purge::<Impl, IMPL_OFFSET>,
            IsOpen2::<Impl, IMPL_OFFSET>,
            ReceiveByLookupIdAllowPeek::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfoImpl: Sized + IDispatchImpl {
    fn QueueGuid();
    fn ServiceTypeGuid();
    fn SetServiceTypeGuid();
    fn Label();
    fn SetLabel();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn IsTransactional();
    fn PrivLevel();
    fn SetPrivLevel();
    fn Journal();
    fn SetJournal();
    fn Quota();
    fn SetQuota();
    fn BasePriority();
    fn SetBasePriority();
    fn CreateTime();
    fn ModifyTime();
    fn Authenticate();
    fn SetAuthenticate();
    fn JournalQuota();
    fn SetJournalQuota();
    fn IsWorldReadable();
    fn Create();
    fn Delete();
    fn Open();
    fn Refresh();
    fn Update();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfoVtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            QueueGuid::<Impl, IMPL_OFFSET>,
            ServiceTypeGuid::<Impl, IMPL_OFFSET>,
            SetServiceTypeGuid::<Impl, IMPL_OFFSET>,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            PathName::<Impl, IMPL_OFFSET>,
            SetPathName::<Impl, IMPL_OFFSET>,
            FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName::<Impl, IMPL_OFFSET>,
            IsTransactional::<Impl, IMPL_OFFSET>,
            PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel::<Impl, IMPL_OFFSET>,
            Journal::<Impl, IMPL_OFFSET>,
            SetJournal::<Impl, IMPL_OFFSET>,
            Quota::<Impl, IMPL_OFFSET>,
            SetQuota::<Impl, IMPL_OFFSET>,
            BasePriority::<Impl, IMPL_OFFSET>,
            SetBasePriority::<Impl, IMPL_OFFSET>,
            CreateTime::<Impl, IMPL_OFFSET>,
            ModifyTime::<Impl, IMPL_OFFSET>,
            Authenticate::<Impl, IMPL_OFFSET>,
            SetAuthenticate::<Impl, IMPL_OFFSET>,
            JournalQuota::<Impl, IMPL_OFFSET>,
            SetJournalQuota::<Impl, IMPL_OFFSET>,
            IsWorldReadable::<Impl, IMPL_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Refresh::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo2Impl: Sized + IDispatchImpl {
    fn QueueGuid();
    fn ServiceTypeGuid();
    fn SetServiceTypeGuid();
    fn Label();
    fn SetLabel();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn IsTransactional();
    fn PrivLevel();
    fn SetPrivLevel();
    fn Journal();
    fn SetJournal();
    fn Quota();
    fn SetQuota();
    fn BasePriority();
    fn SetBasePriority();
    fn CreateTime();
    fn ModifyTime();
    fn Authenticate();
    fn SetAuthenticate();
    fn JournalQuota();
    fn SetJournalQuota();
    fn IsWorldReadable();
    fn Create();
    fn Delete();
    fn Open();
    fn Refresh();
    fn Update();
    fn PathNameDNS();
    fn Properties();
    fn Security();
    fn SetSecurity();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo2Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
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
            QueueGuid::<Impl, IMPL_OFFSET>,
            ServiceTypeGuid::<Impl, IMPL_OFFSET>,
            SetServiceTypeGuid::<Impl, IMPL_OFFSET>,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            PathName::<Impl, IMPL_OFFSET>,
            SetPathName::<Impl, IMPL_OFFSET>,
            FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName::<Impl, IMPL_OFFSET>,
            IsTransactional::<Impl, IMPL_OFFSET>,
            PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel::<Impl, IMPL_OFFSET>,
            Journal::<Impl, IMPL_OFFSET>,
            SetJournal::<Impl, IMPL_OFFSET>,
            Quota::<Impl, IMPL_OFFSET>,
            SetQuota::<Impl, IMPL_OFFSET>,
            BasePriority::<Impl, IMPL_OFFSET>,
            SetBasePriority::<Impl, IMPL_OFFSET>,
            CreateTime::<Impl, IMPL_OFFSET>,
            ModifyTime::<Impl, IMPL_OFFSET>,
            Authenticate::<Impl, IMPL_OFFSET>,
            SetAuthenticate::<Impl, IMPL_OFFSET>,
            JournalQuota::<Impl, IMPL_OFFSET>,
            SetJournalQuota::<Impl, IMPL_OFFSET>,
            IsWorldReadable::<Impl, IMPL_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Refresh::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
            PathNameDNS::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            Security::<Impl, IMPL_OFFSET>,
            SetSecurity::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo3Impl: Sized + IDispatchImpl {
    fn QueueGuid();
    fn ServiceTypeGuid();
    fn SetServiceTypeGuid();
    fn Label();
    fn SetLabel();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn IsTransactional();
    fn PrivLevel();
    fn SetPrivLevel();
    fn Journal();
    fn SetJournal();
    fn Quota();
    fn SetQuota();
    fn BasePriority();
    fn SetBasePriority();
    fn CreateTime();
    fn ModifyTime();
    fn Authenticate();
    fn SetAuthenticate();
    fn JournalQuota();
    fn SetJournalQuota();
    fn IsWorldReadable();
    fn Create();
    fn Delete();
    fn Open();
    fn Refresh();
    fn Update();
    fn PathNameDNS();
    fn Properties();
    fn Security();
    fn SetSecurity();
    fn IsTransactional2();
    fn IsWorldReadable2();
    fn MulticastAddress();
    fn SetMulticastAddress();
    fn ADsPath();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo3Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTransactional2<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWorldReadable2<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MulticastAddress<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMulticastAddress<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            QueueGuid::<Impl, IMPL_OFFSET>,
            ServiceTypeGuid::<Impl, IMPL_OFFSET>,
            SetServiceTypeGuid::<Impl, IMPL_OFFSET>,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            PathName::<Impl, IMPL_OFFSET>,
            SetPathName::<Impl, IMPL_OFFSET>,
            FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName::<Impl, IMPL_OFFSET>,
            IsTransactional::<Impl, IMPL_OFFSET>,
            PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel::<Impl, IMPL_OFFSET>,
            Journal::<Impl, IMPL_OFFSET>,
            SetJournal::<Impl, IMPL_OFFSET>,
            Quota::<Impl, IMPL_OFFSET>,
            SetQuota::<Impl, IMPL_OFFSET>,
            BasePriority::<Impl, IMPL_OFFSET>,
            SetBasePriority::<Impl, IMPL_OFFSET>,
            CreateTime::<Impl, IMPL_OFFSET>,
            ModifyTime::<Impl, IMPL_OFFSET>,
            Authenticate::<Impl, IMPL_OFFSET>,
            SetAuthenticate::<Impl, IMPL_OFFSET>,
            JournalQuota::<Impl, IMPL_OFFSET>,
            SetJournalQuota::<Impl, IMPL_OFFSET>,
            IsWorldReadable::<Impl, IMPL_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Refresh::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
            PathNameDNS::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            Security::<Impl, IMPL_OFFSET>,
            SetSecurity::<Impl, IMPL_OFFSET>,
            IsTransactional2::<Impl, IMPL_OFFSET>,
            IsWorldReadable2::<Impl, IMPL_OFFSET>,
            MulticastAddress::<Impl, IMPL_OFFSET>,
            SetMulticastAddress::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo4Impl: Sized + IDispatchImpl {
    fn QueueGuid();
    fn ServiceTypeGuid();
    fn SetServiceTypeGuid();
    fn Label();
    fn SetLabel();
    fn PathName();
    fn SetPathName();
    fn FormatName();
    fn SetFormatName();
    fn IsTransactional();
    fn PrivLevel();
    fn SetPrivLevel();
    fn Journal();
    fn SetJournal();
    fn Quota();
    fn SetQuota();
    fn BasePriority();
    fn SetBasePriority();
    fn CreateTime();
    fn ModifyTime();
    fn Authenticate();
    fn SetAuthenticate();
    fn JournalQuota();
    fn SetJournalQuota();
    fn IsWorldReadable();
    fn Create();
    fn Delete();
    fn Open();
    fn Refresh();
    fn Update();
    fn PathNameDNS();
    fn Properties();
    fn Security();
    fn SetSecurity();
    fn IsTransactional2();
    fn IsWorldReadable2();
    fn MulticastAddress();
    fn SetMulticastAddress();
    fn ADsPath();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo4Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTransactional2<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWorldReadable2<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MulticastAddress<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMulticastAddress<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            QueueGuid::<Impl, IMPL_OFFSET>,
            ServiceTypeGuid::<Impl, IMPL_OFFSET>,
            SetServiceTypeGuid::<Impl, IMPL_OFFSET>,
            Label::<Impl, IMPL_OFFSET>,
            SetLabel::<Impl, IMPL_OFFSET>,
            PathName::<Impl, IMPL_OFFSET>,
            SetPathName::<Impl, IMPL_OFFSET>,
            FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName::<Impl, IMPL_OFFSET>,
            IsTransactional::<Impl, IMPL_OFFSET>,
            PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel::<Impl, IMPL_OFFSET>,
            Journal::<Impl, IMPL_OFFSET>,
            SetJournal::<Impl, IMPL_OFFSET>,
            Quota::<Impl, IMPL_OFFSET>,
            SetQuota::<Impl, IMPL_OFFSET>,
            BasePriority::<Impl, IMPL_OFFSET>,
            SetBasePriority::<Impl, IMPL_OFFSET>,
            CreateTime::<Impl, IMPL_OFFSET>,
            ModifyTime::<Impl, IMPL_OFFSET>,
            Authenticate::<Impl, IMPL_OFFSET>,
            SetAuthenticate::<Impl, IMPL_OFFSET>,
            JournalQuota::<Impl, IMPL_OFFSET>,
            SetJournalQuota::<Impl, IMPL_OFFSET>,
            IsWorldReadable::<Impl, IMPL_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Refresh::<Impl, IMPL_OFFSET>,
            Update::<Impl, IMPL_OFFSET>,
            PathNameDNS::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            Security::<Impl, IMPL_OFFSET>,
            SetSecurity::<Impl, IMPL_OFFSET>,
            IsTransactional2::<Impl, IMPL_OFFSET>,
            IsWorldReadable2::<Impl, IMPL_OFFSET>,
            MulticastAddress::<Impl, IMPL_OFFSET>,
            SetMulticastAddress::<Impl, IMPL_OFFSET>,
            ADsPath::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfosImpl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfosVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfosImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfosVtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos2Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos2Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos3Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos3Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos4Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos4Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueManagementImpl: Sized + IMSMQManagementImpl + IDispatchImpl {
    fn JournalMessageCount();
    fn BytesInJournal();
    fn EodGetReceiveInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueManagementVtbl {
        unsafe extern "system" fn JournalMessageCount<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BytesInJournal<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EodGetReceiveInfo<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
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
            Init::<Impl, IMPL_OFFSET>,
            FormatName::<Impl, IMPL_OFFSET>,
            Machine::<Impl, IMPL_OFFSET>,
            MessageCount::<Impl, IMPL_OFFSET>,
            ForeignStatus::<Impl, IMPL_OFFSET>,
            QueueType::<Impl, IMPL_OFFSET>,
            IsLocal::<Impl, IMPL_OFFSET>,
            TransactionalStatus::<Impl, IMPL_OFFSET>,
            BytesInQueue::<Impl, IMPL_OFFSET>,
            JournalMessageCount::<Impl, IMPL_OFFSET>,
            BytesInJournal::<Impl, IMPL_OFFSET>,
            EodGetReceiveInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionImpl: Sized + IDispatchImpl {
    fn Transaction();
    fn Commit();
    fn Abort();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionVtbl {
        unsafe extern "system" fn Transaction<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abort<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Transaction::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, Abort::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction2Impl: Sized + IMSMQTransactionImpl + IDispatchImpl {
    fn InitNew();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransaction2Vtbl {
        unsafe extern "system" fn InitNew<Impl: IMSMQTransaction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransaction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Transaction::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, Abort::<Impl, IMPL_OFFSET>, InitNew::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction3Impl: Sized + IMSMQTransaction2Impl + IMSMQTransactionImpl + IDispatchImpl {
    fn ITransaction();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransaction3Vtbl {
        unsafe extern "system" fn ITransaction<Impl: IMSMQTransaction3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
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
            Transaction::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Abort::<Impl, IMPL_OFFSET>,
            InitNew::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            ITransaction::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenserImpl: Sized + IDispatchImpl {
    fn BeginTransaction();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenserVtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BeginTransaction::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser2Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenser2Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BeginTransaction::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser3Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenser3Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, BeginTransaction::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _DMSMQEventEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _DMSMQEventEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _DMSMQEventEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _DMSMQEventEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_DMSMQEventEvents as ::windows::core::Interface>::IID
    }
}
