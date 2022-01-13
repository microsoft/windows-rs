#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplicationImpl: Sized + IDispatchImpl {
    fn MachineIdOfMachineName(&mut self, machinename: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplicationVtbl {
        unsafe extern "system" fn MachineIdOfMachineName<Impl: IMSMQApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineIdOfMachineName(::core::mem::transmute_copy(&machinename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), MachineIdOfMachineName: MachineIdOfMachineName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication2Impl: Sized + IDispatchImpl + IMSMQApplicationImpl {
    fn RegisterCertificate(&mut self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MachineNameOfMachineId(&mut self, bstrguid: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MSMQVersionMajor(&mut self) -> ::windows::core::Result<i16>;
    fn MSMQVersionMinor(&mut self) -> ::windows::core::Result<i16>;
    fn MSMQVersionBuild(&mut self) -> ::windows::core::Result<i16>;
    fn IsDsEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplication2Vtbl {
        unsafe extern "system" fn RegisterCertificate<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCertificate(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&externalcertificate)).into()
        }
        unsafe extern "system" fn MachineNameOfMachineId<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineNameOfMachineId(::core::mem::transmute_copy(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachinename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMajor<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionmajor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMinor<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionMinor() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionminor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionBuild<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDsEnabled<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisdsenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMSMQApplicationVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterCertificate: RegisterCertificate::<Impl, IMPL_OFFSET>,
            MachineNameOfMachineId: MachineNameOfMachineId::<Impl, IMPL_OFFSET>,
            MSMQVersionMajor: MSMQVersionMajor::<Impl, IMPL_OFFSET>,
            MSMQVersionMinor: MSMQVersionMinor::<Impl, IMPL_OFFSET>,
            MSMQVersionBuild: MSMQVersionBuild::<Impl, IMPL_OFFSET>,
            IsDsEnabled: IsDsEnabled::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication3Impl: Sized + IDispatchImpl + IMSMQApplicationImpl + IMSMQApplication2Impl {
    fn ActiveQueues(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn PrivateQueues(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DirectoryServiceServer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsConnected(&mut self) -> ::windows::core::Result<i16>;
    fn BytesInAllQueues(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetMachine(&mut self, bstrmachine: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Machine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Connect(&mut self) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn Tidy(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplication3Vtbl {
        unsafe extern "system" fn ActiveQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvactivequeues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvprivatequeues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryServiceServer<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryServiceServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdirectoryserviceserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInAllQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesInAllQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbytesinallqueues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachine<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMachine(::core::mem::transmute_copy(&bstrmachine)).into()
        }
        unsafe extern "system" fn Machine<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Machine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Tidy<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Tidy().into()
        }
        Self {
            base: IMSMQApplication2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActiveQueues: ActiveQueues::<Impl, IMPL_OFFSET>,
            PrivateQueues: PrivateQueues::<Impl, IMPL_OFFSET>,
            DirectoryServiceServer: DirectoryServiceServer::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            BytesInAllQueues: BytesInAllQueues::<Impl, IMPL_OFFSET>,
            SetMachine: SetMachine::<Impl, IMPL_OFFSET>,
            Machine: Machine::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Tidy: Tidy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCollectionImpl: Sized + IDispatchImpl {
    fn Item(&mut self, index: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarret = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenserImpl: Sized + IDispatchImpl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenserVtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser2Impl: Sized + IDispatchImpl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser2Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQCoordinatedTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser3Impl: Sized + IDispatchImpl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser3Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQCoordinatedTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQDestinationImpl: Sized + IDispatchImpl {
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn IADs(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_IADs(&mut self, piads: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ADsPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetADsPath(&mut self, bstradspath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Destinations(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_Destinations(&mut self, pdestinations: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQDestinationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestinationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQDestinationVtbl {
        unsafe extern "system" fn Open<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IADs<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiads: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IADs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiads = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_IADs<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piads: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_IADs(::core::mem::transmute(&piads)).into()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsPath<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetADsPath(::core::mem::transmute_copy(&bstradspath)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn Destinations<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestinations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destinations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestinations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Destinations<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinations: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Destinations(::core::mem::transmute(&pdestinations)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            IADs: IADs::<Impl, IMPL_OFFSET>,
            putref_IADs: putref_IADs::<Impl, IMPL_OFFSET>,
            ADsPath: ADsPath::<Impl, IMPL_OFFSET>,
            SetADsPath: SetADsPath::<Impl, IMPL_OFFSET>,
            PathName: PathName::<Impl, IMPL_OFFSET>,
            SetPathName: SetPathName::<Impl, IMPL_OFFSET>,
            FormatName: FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName: SetFormatName::<Impl, IMPL_OFFSET>,
            Destinations: Destinations::<Impl, IMPL_OFFSET>,
            putref_Destinations: putref_Destinations::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
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
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent2Impl: Sized + IDispatchImpl + IMSMQEventImpl {
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQEvent2Vtbl {
        unsafe extern "system" fn Properties<Impl: IMSMQEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IMSMQEventVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Properties: Properties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent3Impl: Sized + IDispatchImpl + IMSMQEventImpl + IMSMQEvent2Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQEvent3Vtbl {
        Self { base: IMSMQEvent2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQManagementImpl: Sized + IDispatchImpl {
    fn Init(&mut self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Machine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MessageCount(&mut self) -> ::windows::core::Result<i32>;
    fn ForeignStatus(&mut self) -> ::windows::core::Result<i32>;
    fn QueueType(&mut self) -> ::windows::core::Result<i32>;
    fn IsLocal(&mut self) -> ::windows::core::Result<i16>;
    fn TransactionalStatus(&mut self) -> ::windows::core::Result<i32>;
    fn BytesInQueue(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQManagementVtbl {
        unsafe extern "system" fn Init<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&machine), ::core::mem::transmute_copy(&pathname), ::core::mem::transmute_copy(&formatname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Machine<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Machine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageCount<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plmessagecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForeignStatus<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForeignStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *plforeignstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueType<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueType() {
                ::core::result::Result::Ok(ok__) => {
                    *plqueuetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *pfislocal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionalStatus<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionalStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pltransactionalstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInQueue<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesInQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbytesinqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            FormatName: FormatName::<Impl, IMPL_OFFSET>,
            Machine: Machine::<Impl, IMPL_OFFSET>,
            MessageCount: MessageCount::<Impl, IMPL_OFFSET>,
            ForeignStatus: ForeignStatus::<Impl, IMPL_OFFSET>,
            QueueType: QueueType::<Impl, IMPL_OFFSET>,
            IsLocal: IsLocal::<Impl, IMPL_OFFSET>,
            TransactionalStatus: TransactionalStatus::<Impl, IMPL_OFFSET>,
            BytesInQueue: BytesInQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessageImpl: Sized + IDispatchImpl {
    fn Class(&mut self) -> ::windows::core::Result<i32>;
    fn PrivLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&mut self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn AuthLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthLevel(&mut self, lauthlevel: i32) -> ::windows::core::Result<()>;
    fn IsAuthenticated(&mut self) -> ::windows::core::Result<i16>;
    fn Delivery(&mut self) -> ::windows::core::Result<i32>;
    fn SetDelivery(&mut self, ldelivery: i32) -> ::windows::core::Result<()>;
    fn Trace(&mut self) -> ::windows::core::Result<i32>;
    fn SetTrace(&mut self, ltrace: i32) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Journal(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournal(&mut self, ljournal: i32) -> ::windows::core::Result<()>;
    fn ResponseQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&mut self) -> ::windows::core::Result<i32>;
    fn Body(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&mut self, varbody: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&mut self, varmsgid: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&mut self) -> ::windows::core::Result<i32>;
    fn SetAck(&mut self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxTimeToReachQueue(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReachQueue(&mut self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()>;
    fn MaxTimeToReceive(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReceive(&mut self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<i32>;
    fn SetHashAlgorithm(&mut self, lhashalg: i32) -> ::windows::core::Result<()>;
    fn EncryptAlgorithm(&mut self) -> ::windows::core::Result<i32>;
    fn SetEncryptAlgorithm(&mut self, lencryptalg: i32) -> ::windows::core::Result<()>;
    fn SentTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ArrivedTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DestinationQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn SenderCertificate(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSenderCertificate(&mut self, varsendercert: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&mut self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&mut self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&mut self, destinationqueue: ::core::option::Option<IMSMQQueue>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessageVtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute_copy(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute_copy(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Class: Class::<Impl, IMPL_OFFSET>,
            PrivLevel: PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel: SetPrivLevel::<Impl, IMPL_OFFSET>,
            AuthLevel: AuthLevel::<Impl, IMPL_OFFSET>,
            SetAuthLevel: SetAuthLevel::<Impl, IMPL_OFFSET>,
            IsAuthenticated: IsAuthenticated::<Impl, IMPL_OFFSET>,
            Delivery: Delivery::<Impl, IMPL_OFFSET>,
            SetDelivery: SetDelivery::<Impl, IMPL_OFFSET>,
            Trace: Trace::<Impl, IMPL_OFFSET>,
            SetTrace: SetTrace::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            SetJournal: SetJournal::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            AppSpecific: AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific: SetAppSpecific::<Impl, IMPL_OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Impl, IMPL_OFFSET>,
            BodyLength: BodyLength::<Impl, IMPL_OFFSET>,
            Body: Body::<Impl, IMPL_OFFSET>,
            SetBody: SetBody::<Impl, IMPL_OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId: SetCorrelationId::<Impl, IMPL_OFFSET>,
            Ack: Ack::<Impl, IMPL_OFFSET>,
            SetAck: SetAck::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SentTime: SentTime::<Impl, IMPL_OFFSET>,
            ArrivedTime: ArrivedTime::<Impl, IMPL_OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Impl, IMPL_OFFSET>,
            SenderCertificate: SenderCertificate::<Impl, IMPL_OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Impl, IMPL_OFFSET>,
            SenderId: SenderId::<Impl, IMPL_OFFSET>,
            SenderIdType: SenderIdType::<Impl, IMPL_OFFSET>,
            SetSenderIdType: SetSenderIdType::<Impl, IMPL_OFFSET>,
            Send: Send::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage2Impl: Sized + IDispatchImpl {
    fn Class(&mut self) -> ::windows::core::Result<i32>;
    fn PrivLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&mut self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn AuthLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthLevel(&mut self, lauthlevel: i32) -> ::windows::core::Result<()>;
    fn IsAuthenticated(&mut self) -> ::windows::core::Result<i16>;
    fn Delivery(&mut self) -> ::windows::core::Result<i32>;
    fn SetDelivery(&mut self, ldelivery: i32) -> ::windows::core::Result<()>;
    fn Trace(&mut self) -> ::windows::core::Result<i32>;
    fn SetTrace(&mut self, ltrace: i32) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Journal(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournal(&mut self, ljournal: i32) -> ::windows::core::Result<()>;
    fn ResponseQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&mut self) -> ::windows::core::Result<i32>;
    fn Body(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&mut self, varbody: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&mut self, varmsgid: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&mut self) -> ::windows::core::Result<i32>;
    fn SetAck(&mut self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxTimeToReachQueue(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReachQueue(&mut self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()>;
    fn MaxTimeToReceive(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReceive(&mut self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<i32>;
    fn SetHashAlgorithm(&mut self, lhashalg: i32) -> ::windows::core::Result<()>;
    fn EncryptAlgorithm(&mut self) -> ::windows::core::Result<i32>;
    fn SetEncryptAlgorithm(&mut self, lencryptalg: i32) -> ::windows::core::Result<()>;
    fn SentTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ArrivedTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DestinationQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn SenderCertificate(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSenderCertificate(&mut self, varsendercert: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&mut self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&mut self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&mut self, destinationqueue: ::core::option::Option<IMSMQQueue2>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&mut self) -> ::windows::core::Result<()>;
    fn SenderVersion(&mut self) -> ::windows::core::Result<i32>;
    fn Extension(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&mut self, varextension: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&mut self, bstrguidconnectortype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn DestinationSymmetricKey(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&mut self, vardestsymmkey: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&mut self, varsignature: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&mut self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&mut self, bstrauthprovname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&mut self, varsenderid: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&mut self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&mut self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage2Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute_copy(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute_copy(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtension(::core::mem::transmute_copy(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute_copy(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute_copy(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignature(::core::mem::transmute_copy(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute_copy(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute_copy(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *psreceivedauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Class: Class::<Impl, IMPL_OFFSET>,
            PrivLevel: PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel: SetPrivLevel::<Impl, IMPL_OFFSET>,
            AuthLevel: AuthLevel::<Impl, IMPL_OFFSET>,
            SetAuthLevel: SetAuthLevel::<Impl, IMPL_OFFSET>,
            IsAuthenticated: IsAuthenticated::<Impl, IMPL_OFFSET>,
            Delivery: Delivery::<Impl, IMPL_OFFSET>,
            SetDelivery: SetDelivery::<Impl, IMPL_OFFSET>,
            Trace: Trace::<Impl, IMPL_OFFSET>,
            SetTrace: SetTrace::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            SetJournal: SetJournal::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            AppSpecific: AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific: SetAppSpecific::<Impl, IMPL_OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Impl, IMPL_OFFSET>,
            BodyLength: BodyLength::<Impl, IMPL_OFFSET>,
            Body: Body::<Impl, IMPL_OFFSET>,
            SetBody: SetBody::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId: SetCorrelationId::<Impl, IMPL_OFFSET>,
            Ack: Ack::<Impl, IMPL_OFFSET>,
            SetAck: SetAck::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SentTime: SentTime::<Impl, IMPL_OFFSET>,
            ArrivedTime: ArrivedTime::<Impl, IMPL_OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Impl, IMPL_OFFSET>,
            SenderCertificate: SenderCertificate::<Impl, IMPL_OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Impl, IMPL_OFFSET>,
            SenderId: SenderId::<Impl, IMPL_OFFSET>,
            SenderIdType: SenderIdType::<Impl, IMPL_OFFSET>,
            SetSenderIdType: SetSenderIdType::<Impl, IMPL_OFFSET>,
            Send: Send::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Impl, IMPL_OFFSET>,
            SenderVersion: SenderVersion::<Impl, IMPL_OFFSET>,
            Extension: Extension::<Impl, IMPL_OFFSET>,
            SetExtension: SetExtension::<Impl, IMPL_OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Impl, IMPL_OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            Signature: Signature::<Impl, IMPL_OFFSET>,
            SetSignature: SetSignature::<Impl, IMPL_OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Impl, IMPL_OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetSenderId: SetSenderId::<Impl, IMPL_OFFSET>,
            MsgClass: MsgClass::<Impl, IMPL_OFFSET>,
            SetMsgClass: SetMsgClass::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            TransactionId: TransactionId::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Impl, IMPL_OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Impl, IMPL_OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage3Impl: Sized + IDispatchImpl {
    fn Class(&mut self) -> ::windows::core::Result<i32>;
    fn PrivLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&mut self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn AuthLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthLevel(&mut self, lauthlevel: i32) -> ::windows::core::Result<()>;
    fn IsAuthenticated(&mut self) -> ::windows::core::Result<i16>;
    fn Delivery(&mut self) -> ::windows::core::Result<i32>;
    fn SetDelivery(&mut self, ldelivery: i32) -> ::windows::core::Result<()>;
    fn Trace(&mut self) -> ::windows::core::Result<i32>;
    fn SetTrace(&mut self, ltrace: i32) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Journal(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournal(&mut self, ljournal: i32) -> ::windows::core::Result<()>;
    fn ResponseQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&mut self) -> ::windows::core::Result<i32>;
    fn Body(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&mut self, varbody: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&mut self, varmsgid: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&mut self) -> ::windows::core::Result<i32>;
    fn SetAck(&mut self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxTimeToReachQueue(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReachQueue(&mut self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()>;
    fn MaxTimeToReceive(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReceive(&mut self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<i32>;
    fn SetHashAlgorithm(&mut self, lhashalg: i32) -> ::windows::core::Result<()>;
    fn EncryptAlgorithm(&mut self) -> ::windows::core::Result<i32>;
    fn SetEncryptAlgorithm(&mut self, lencryptalg: i32) -> ::windows::core::Result<()>;
    fn SentTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ArrivedTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DestinationQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn SenderCertificate(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSenderCertificate(&mut self, varsendercert: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&mut self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&mut self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&mut self, destinationqueue: ::core::option::Option<super::Com::IDispatch>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&mut self) -> ::windows::core::Result<()>;
    fn SenderVersion(&mut self) -> ::windows::core::Result<i32>;
    fn Extension(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&mut self, varextension: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&mut self, bstrguidconnectortype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn DestinationSymmetricKey(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&mut self, vardestsymmkey: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&mut self, varsignature: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&mut self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&mut self, bstrauthprovname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&mut self, varsenderid: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&mut self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&mut self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo_v2(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v2(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn putref_ResponseQueueInfo(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo3>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn putref_AdminQueueInfo(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo3>) -> ::windows::core::Result<()>;
    fn ResponseDestination(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&mut self, pdestresponse: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Destination(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsAuthenticated2(&mut self) -> ::windows::core::Result<i16>;
    fn IsFirstInTransaction2(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction2(&mut self) -> ::windows::core::Result<i16>;
    fn AttachCurrentSecurityContext2(&mut self) -> ::windows::core::Result<()>;
    fn SoapEnvelope(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CompoundMessage(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSoapHeader(&mut self, bstrsoapheader: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSoapBody(&mut self, bstrsoapbody: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage3Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute_copy(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute_copy(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtension(::core::mem::transmute_copy(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute_copy(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute_copy(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignature(::core::mem::transmute_copy(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute_copy(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute_copy(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v2(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v2(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *psreceivedauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseDestination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseDestination(::core::mem::transmute(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarlookupid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext2().into()
        }
        unsafe extern "system" fn SoapEnvelope<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoapEnvelope() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsoapenvelope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompoundMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcompoundmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoapHeader(::core::mem::transmute_copy(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoapBody(::core::mem::transmute_copy(&bstrsoapbody)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Class: Class::<Impl, IMPL_OFFSET>,
            PrivLevel: PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel: SetPrivLevel::<Impl, IMPL_OFFSET>,
            AuthLevel: AuthLevel::<Impl, IMPL_OFFSET>,
            SetAuthLevel: SetAuthLevel::<Impl, IMPL_OFFSET>,
            IsAuthenticated: IsAuthenticated::<Impl, IMPL_OFFSET>,
            Delivery: Delivery::<Impl, IMPL_OFFSET>,
            SetDelivery: SetDelivery::<Impl, IMPL_OFFSET>,
            Trace: Trace::<Impl, IMPL_OFFSET>,
            SetTrace: SetTrace::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            SetJournal: SetJournal::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            AppSpecific: AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific: SetAppSpecific::<Impl, IMPL_OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Impl, IMPL_OFFSET>,
            BodyLength: BodyLength::<Impl, IMPL_OFFSET>,
            Body: Body::<Impl, IMPL_OFFSET>,
            SetBody: SetBody::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId: SetCorrelationId::<Impl, IMPL_OFFSET>,
            Ack: Ack::<Impl, IMPL_OFFSET>,
            SetAck: SetAck::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SentTime: SentTime::<Impl, IMPL_OFFSET>,
            ArrivedTime: ArrivedTime::<Impl, IMPL_OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Impl, IMPL_OFFSET>,
            SenderCertificate: SenderCertificate::<Impl, IMPL_OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Impl, IMPL_OFFSET>,
            SenderId: SenderId::<Impl, IMPL_OFFSET>,
            SenderIdType: SenderIdType::<Impl, IMPL_OFFSET>,
            SetSenderIdType: SetSenderIdType::<Impl, IMPL_OFFSET>,
            Send: Send::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Impl, IMPL_OFFSET>,
            SenderVersion: SenderVersion::<Impl, IMPL_OFFSET>,
            Extension: Extension::<Impl, IMPL_OFFSET>,
            SetExtension: SetExtension::<Impl, IMPL_OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Impl, IMPL_OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            Signature: Signature::<Impl, IMPL_OFFSET>,
            SetSignature: SetSignature::<Impl, IMPL_OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Impl, IMPL_OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetSenderId: SetSenderId::<Impl, IMPL_OFFSET>,
            MsgClass: MsgClass::<Impl, IMPL_OFFSET>,
            SetMsgClass: SetMsgClass::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            TransactionId: TransactionId::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Impl, IMPL_OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Impl, IMPL_OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Impl, IMPL_OFFSET>,
            ResponseDestination: ResponseDestination::<Impl, IMPL_OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Impl, IMPL_OFFSET>,
            Destination: Destination::<Impl, IMPL_OFFSET>,
            LookupId: LookupId::<Impl, IMPL_OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Impl, IMPL_OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Impl, IMPL_OFFSET>,
            SoapEnvelope: SoapEnvelope::<Impl, IMPL_OFFSET>,
            CompoundMessage: CompoundMessage::<Impl, IMPL_OFFSET>,
            SetSoapHeader: SetSoapHeader::<Impl, IMPL_OFFSET>,
            SetSoapBody: SetSoapBody::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage4Impl: Sized + IDispatchImpl {
    fn Class(&mut self) -> ::windows::core::Result<i32>;
    fn PrivLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&mut self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn AuthLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthLevel(&mut self, lauthlevel: i32) -> ::windows::core::Result<()>;
    fn IsAuthenticated(&mut self) -> ::windows::core::Result<i16>;
    fn Delivery(&mut self) -> ::windows::core::Result<i32>;
    fn SetDelivery(&mut self, ldelivery: i32) -> ::windows::core::Result<()>;
    fn Trace(&mut self) -> ::windows::core::Result<i32>;
    fn SetTrace(&mut self, ltrace: i32) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Journal(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournal(&mut self, ljournal: i32) -> ::windows::core::Result<()>;
    fn ResponseQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&mut self) -> ::windows::core::Result<i32>;
    fn Body(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&mut self, varbody: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&mut self, varmsgid: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&mut self) -> ::windows::core::Result<i32>;
    fn SetAck(&mut self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxTimeToReachQueue(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReachQueue(&mut self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()>;
    fn MaxTimeToReceive(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReceive(&mut self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&mut self) -> ::windows::core::Result<i32>;
    fn SetHashAlgorithm(&mut self, lhashalg: i32) -> ::windows::core::Result<()>;
    fn EncryptAlgorithm(&mut self) -> ::windows::core::Result<i32>;
    fn SetEncryptAlgorithm(&mut self, lencryptalg: i32) -> ::windows::core::Result<()>;
    fn SentTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ArrivedTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DestinationQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn SenderCertificate(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSenderCertificate(&mut self, varsendercert: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&mut self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&mut self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&mut self, destinationqueue: ::core::option::Option<super::Com::IDispatch>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&mut self) -> ::windows::core::Result<()>;
    fn SenderVersion(&mut self) -> ::windows::core::Result<i32>;
    fn Extension(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&mut self, varextension: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&mut self, bstrguidconnectortype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn DestinationSymmetricKey(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&mut self, vardestsymmkey: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&mut self, varsignature: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&mut self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&mut self, bstrauthprovname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&mut self, varsenderid: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&mut self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&mut self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo_v2(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v2(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn putref_ResponseQueueInfo(&mut self, pqinforesponse: ::core::option::Option<IMSMQQueueInfo4>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn putref_AdminQueueInfo(&mut self, pqinfoadmin: ::core::option::Option<IMSMQQueueInfo4>) -> ::windows::core::Result<()>;
    fn ResponseDestination(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&mut self, pdestresponse: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Destination(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsAuthenticated2(&mut self) -> ::windows::core::Result<i16>;
    fn IsFirstInTransaction2(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction2(&mut self) -> ::windows::core::Result<i16>;
    fn AttachCurrentSecurityContext2(&mut self) -> ::windows::core::Result<()>;
    fn SoapEnvelope(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CompoundMessage(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSoapHeader(&mut self, bstrsoapheader: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSoapBody(&mut self, bstrsoapbody: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage4Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute_copy(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute_copy(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtension(::core::mem::transmute_copy(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute_copy(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute_copy(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignature(::core::mem::transmute_copy(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute_copy(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute_copy(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v2(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v2(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *psreceivedauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseDestination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseDestination(::core::mem::transmute(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarlookupid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext2().into()
        }
        unsafe extern "system" fn SoapEnvelope<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoapEnvelope() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsoapenvelope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompoundMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcompoundmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoapHeader(::core::mem::transmute_copy(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoapBody(::core::mem::transmute_copy(&bstrsoapbody)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Class: Class::<Impl, IMPL_OFFSET>,
            PrivLevel: PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel: SetPrivLevel::<Impl, IMPL_OFFSET>,
            AuthLevel: AuthLevel::<Impl, IMPL_OFFSET>,
            SetAuthLevel: SetAuthLevel::<Impl, IMPL_OFFSET>,
            IsAuthenticated: IsAuthenticated::<Impl, IMPL_OFFSET>,
            Delivery: Delivery::<Impl, IMPL_OFFSET>,
            SetDelivery: SetDelivery::<Impl, IMPL_OFFSET>,
            Trace: Trace::<Impl, IMPL_OFFSET>,
            SetTrace: SetTrace::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            SetJournal: SetJournal::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Impl, IMPL_OFFSET>,
            AppSpecific: AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific: SetAppSpecific::<Impl, IMPL_OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Impl, IMPL_OFFSET>,
            BodyLength: BodyLength::<Impl, IMPL_OFFSET>,
            Body: Body::<Impl, IMPL_OFFSET>,
            SetBody: SetBody::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
            SetCorrelationId: SetCorrelationId::<Impl, IMPL_OFFSET>,
            Ack: Ack::<Impl, IMPL_OFFSET>,
            SetAck: SetAck::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Impl, IMPL_OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Impl, IMPL_OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Impl, IMPL_OFFSET>,
            HashAlgorithm: HashAlgorithm::<Impl, IMPL_OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Impl, IMPL_OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Impl, IMPL_OFFSET>,
            SentTime: SentTime::<Impl, IMPL_OFFSET>,
            ArrivedTime: ArrivedTime::<Impl, IMPL_OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Impl, IMPL_OFFSET>,
            SenderCertificate: SenderCertificate::<Impl, IMPL_OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Impl, IMPL_OFFSET>,
            SenderId: SenderId::<Impl, IMPL_OFFSET>,
            SenderIdType: SenderIdType::<Impl, IMPL_OFFSET>,
            SetSenderIdType: SetSenderIdType::<Impl, IMPL_OFFSET>,
            Send: Send::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Impl, IMPL_OFFSET>,
            SenderVersion: SenderVersion::<Impl, IMPL_OFFSET>,
            Extension: Extension::<Impl, IMPL_OFFSET>,
            SetExtension: SetExtension::<Impl, IMPL_OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Impl, IMPL_OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Impl, IMPL_OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Impl, IMPL_OFFSET>,
            Signature: Signature::<Impl, IMPL_OFFSET>,
            SetSignature: SetSignature::<Impl, IMPL_OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Impl, IMPL_OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Impl, IMPL_OFFSET>,
            SetSenderId: SetSenderId::<Impl, IMPL_OFFSET>,
            MsgClass: MsgClass::<Impl, IMPL_OFFSET>,
            SetMsgClass: SetMsgClass::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            TransactionId: TransactionId::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Impl, IMPL_OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Impl, IMPL_OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Impl, IMPL_OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Impl, IMPL_OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Impl, IMPL_OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Impl, IMPL_OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Impl, IMPL_OFFSET>,
            ResponseDestination: ResponseDestination::<Impl, IMPL_OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Impl, IMPL_OFFSET>,
            Destination: Destination::<Impl, IMPL_OFFSET>,
            LookupId: LookupId::<Impl, IMPL_OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Impl, IMPL_OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Impl, IMPL_OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Impl, IMPL_OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Impl, IMPL_OFFSET>,
            SoapEnvelope: SoapEnvelope::<Impl, IMPL_OFFSET>,
            CompoundMessage: CompoundMessage::<Impl, IMPL_OFFSET>,
            SetSoapHeader: SetSoapHeader::<Impl, IMPL_OFFSET>,
            SetSoapBody: SetSoapBody::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQOutgoingQueueManagementImpl: Sized + IDispatchImpl + IMSMQManagementImpl {
    fn State(&mut self) -> ::windows::core::Result<i32>;
    fn NextHops(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn EodGetSendInfo(&mut self) -> ::windows::core::Result<IMSMQCollection>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn EodResend(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQOutgoingQueueManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQOutgoingQueueManagementVtbl {
        unsafe extern "system" fn State<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *plstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextHops<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextHops() {
                ::core::result::Result::Ok(ok__) => {
                    *pvnexthops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetSendInfo<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EodGetSendInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Pause<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn EodResend<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EodResend().into()
        }
        Self {
            base: IMSMQManagementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            NextHops: NextHops::<Impl, IMPL_OFFSET>,
            EodGetSendInfo: EodGetSendInfo::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            EodResend: EodResend::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQOutgoingQueueManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateDestinationImpl: Sized + IDispatchImpl {
    fn Handle(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetHandle(&mut self, varhandle: super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateDestinationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateDestinationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQPrivateDestinationVtbl {
        unsafe extern "system" fn Handle<Impl: IMSMQPrivateDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandle<Impl: IMSMQPrivateDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandle(::core::mem::transmute_copy(&varhandle)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Handle: Handle::<Impl, IMPL_OFFSET>,
            SetHandle: SetHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateDestination as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateEventImpl: Sized + IDispatchImpl {
    fn Hwnd(&mut self) -> ::windows::core::Result<i32>;
    fn FireArrivedEvent(&mut self, pq: ::core::option::Option<IMSMQQueue>, msgcursor: i32) -> ::windows::core::Result<()>;
    fn FireArrivedErrorEvent(&mut self, pq: ::core::option::Option<IMSMQQueue>, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQPrivateEventVtbl {
        unsafe extern "system" fn Hwnd<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireArrivedEvent<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FireArrivedEvent(::core::mem::transmute(&pq), ::core::mem::transmute_copy(&msgcursor)).into()
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FireArrivedErrorEvent(::core::mem::transmute(&pq), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&msgcursor)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Hwnd: Hwnd::<Impl, IMPL_OFFSET>,
            FireArrivedEvent: FireArrivedEvent::<Impl, IMPL_OFFSET>,
            FireArrivedErrorEvent: FireArrivedErrorEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueryImpl: Sized + IDispatchImpl {
    fn LookupQueue(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueryVtbl {
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), LookupQueue: LookupQueue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery2Impl: Sized + IDispatchImpl {
    fn LookupQueue(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery2Vtbl {
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LookupQueue: LookupQueue::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery3Impl: Sized + IDispatchImpl {
    fn LookupQueue_v2(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupQueue(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery3Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue_v2(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(
                ::core::mem::transmute_copy(&queueguid),
                ::core::mem::transmute_copy(&servicetypeguid),
                ::core::mem::transmute_copy(&label),
                ::core::mem::transmute_copy(&createtime),
                ::core::mem::transmute_copy(&modifytime),
                ::core::mem::transmute_copy(&relservicetype),
                ::core::mem::transmute_copy(&rellabel),
                ::core::mem::transmute_copy(&relcreatetime),
                ::core::mem::transmute_copy(&relmodifytime),
                ::core::mem::transmute_copy(&multicastaddress),
                ::core::mem::transmute_copy(&relmulticastaddress),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            LookupQueue: LookupQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery4Impl: Sized + IDispatchImpl {
    fn LookupQueue_v2(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupQueue(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery4Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue_v2(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(
                ::core::mem::transmute_copy(&queueguid),
                ::core::mem::transmute_copy(&servicetypeguid),
                ::core::mem::transmute_copy(&label),
                ::core::mem::transmute_copy(&createtime),
                ::core::mem::transmute_copy(&modifytime),
                ::core::mem::transmute_copy(&relservicetype),
                ::core::mem::transmute_copy(&rellabel),
                ::core::mem::transmute_copy(&relcreatetime),
                ::core::mem::transmute_copy(&relmodifytime),
                ::core::mem::transmute_copy(&multicastaddress),
                ::core::mem::transmute_copy(&relmulticastaddress),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            LookupQueue: LookupQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueImpl: Sized + IDispatchImpl {
    fn Access(&mut self) -> ::windows::core::Result<i32>;
    fn ShareMode(&mut self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn Handle(&mut self) -> ::windows::core::Result<i32>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Receive(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&mut self, event: ::core::option::Option<IMSMQEvent>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueVtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Access: Access::<Impl, IMPL_OFFSET>,
            ShareMode: ShareMode::<Impl, IMPL_OFFSET>,
            QueueInfo: QueueInfo::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Receive: Receive::<Impl, IMPL_OFFSET>,
            Peek: Peek::<Impl, IMPL_OFFSET>,
            EnableNotification: EnableNotification::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Impl, IMPL_OFFSET>,
            PeekNext: PeekNext::<Impl, IMPL_OFFSET>,
            PeekCurrent: PeekCurrent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue2Impl: Sized + IDispatchImpl {
    fn Access(&mut self) -> ::windows::core::Result<i32>;
    fn ShareMode(&mut self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn Handle(&mut self) -> ::windows::core::Result<i32>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Receive_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&mut self, event: ::core::option::Option<IMSMQEvent2>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Receive(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn Peek(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn ReceiveCurrent(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn PeekNext(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn PeekCurrent(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue2Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Access: Access::<Impl, IMPL_OFFSET>,
            ShareMode: ShareMode::<Impl, IMPL_OFFSET>,
            QueueInfo: QueueInfo::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Receive_v1: Receive_v1::<Impl, IMPL_OFFSET>,
            Peek_v1: Peek_v1::<Impl, IMPL_OFFSET>,
            EnableNotification: EnableNotification::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Impl, IMPL_OFFSET>,
            PeekNext_v1: PeekNext_v1::<Impl, IMPL_OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Impl, IMPL_OFFSET>,
            Receive: Receive::<Impl, IMPL_OFFSET>,
            Peek: Peek::<Impl, IMPL_OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Impl, IMPL_OFFSET>,
            PeekNext: PeekNext::<Impl, IMPL_OFFSET>,
            PeekCurrent: PeekCurrent::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue3Impl: Sized + IDispatchImpl {
    fn Access(&mut self) -> ::windows::core::Result<i32>;
    fn ShareMode(&mut self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn Handle(&mut self) -> ::windows::core::Result<i32>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Receive_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&mut self, event: ::core::option::Option<IMSMQEvent3>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Receive(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn Peek(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveCurrent(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekNext(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekCurrent(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Handle2(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ReceiveByLookupId(&mut self, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveNextByLookupId(&mut self, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceivePreviousByLookupId(&mut self, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveFirstByLookupId(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveLastByLookupId(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekByLookupId(&mut self, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekNextByLookupId(&mut self, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekPreviousByLookupId(&mut self, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekFirstByLookupId(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekLastByLookupId(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn Purge(&mut self) -> ::windows::core::Result<()>;
    fn IsOpen2(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue3Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle2() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveNextByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivePreviousByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveFirstByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveLastByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNextByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekPreviousByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekFirstByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekLastByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Purge().into()
        }
        unsafe extern "system" fn IsOpen2<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Access: Access::<Impl, IMPL_OFFSET>,
            ShareMode: ShareMode::<Impl, IMPL_OFFSET>,
            QueueInfo: QueueInfo::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Receive_v1: Receive_v1::<Impl, IMPL_OFFSET>,
            Peek_v1: Peek_v1::<Impl, IMPL_OFFSET>,
            EnableNotification: EnableNotification::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Impl, IMPL_OFFSET>,
            PeekNext_v1: PeekNext_v1::<Impl, IMPL_OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Impl, IMPL_OFFSET>,
            Receive: Receive::<Impl, IMPL_OFFSET>,
            Peek: Peek::<Impl, IMPL_OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Impl, IMPL_OFFSET>,
            PeekNext: PeekNext::<Impl, IMPL_OFFSET>,
            PeekCurrent: PeekCurrent::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Handle2: Handle2::<Impl, IMPL_OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Impl, IMPL_OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Impl, IMPL_OFFSET>,
            PeekByLookupId: PeekByLookupId::<Impl, IMPL_OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Impl, IMPL_OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Impl, IMPL_OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Impl, IMPL_OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Impl, IMPL_OFFSET>,
            Purge: Purge::<Impl, IMPL_OFFSET>,
            IsOpen2: IsOpen2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue4Impl: Sized + IDispatchImpl {
    fn Access(&mut self) -> ::windows::core::Result<i32>;
    fn ShareMode(&mut self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn Handle(&mut self) -> ::windows::core::Result<i32>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Receive_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&mut self, event: ::core::option::Option<IMSMQEvent3>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Receive(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn Peek(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveCurrent(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekNext(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekCurrent(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Handle2(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ReceiveByLookupId(&mut self, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveNextByLookupId(&mut self, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceivePreviousByLookupId(&mut self, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveFirstByLookupId(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveLastByLookupId(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekByLookupId(&mut self, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekNextByLookupId(&mut self, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekPreviousByLookupId(&mut self, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekFirstByLookupId(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekLastByLookupId(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn Purge(&mut self) -> ::windows::core::Result<()>;
    fn IsOpen2(&mut self) -> ::windows::core::Result<i16>;
    fn ReceiveByLookupIdAllowPeek(&mut self, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue4Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle2() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveNextByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivePreviousByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveFirstByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveLastByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNextByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekPreviousByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekFirstByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekLastByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Purge().into()
        }
        unsafe extern "system" fn IsOpen2<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveByLookupIdAllowPeek(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Access: Access::<Impl, IMPL_OFFSET>,
            ShareMode: ShareMode::<Impl, IMPL_OFFSET>,
            QueueInfo: QueueInfo::<Impl, IMPL_OFFSET>,
            Handle: Handle::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Receive_v1: Receive_v1::<Impl, IMPL_OFFSET>,
            Peek_v1: Peek_v1::<Impl, IMPL_OFFSET>,
            EnableNotification: EnableNotification::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Impl, IMPL_OFFSET>,
            PeekNext_v1: PeekNext_v1::<Impl, IMPL_OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Impl, IMPL_OFFSET>,
            Receive: Receive::<Impl, IMPL_OFFSET>,
            Peek: Peek::<Impl, IMPL_OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Impl, IMPL_OFFSET>,
            PeekNext: PeekNext::<Impl, IMPL_OFFSET>,
            PeekCurrent: PeekCurrent::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Handle2: Handle2::<Impl, IMPL_OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Impl, IMPL_OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Impl, IMPL_OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Impl, IMPL_OFFSET>,
            PeekByLookupId: PeekByLookupId::<Impl, IMPL_OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Impl, IMPL_OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Impl, IMPL_OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Impl, IMPL_OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Impl, IMPL_OFFSET>,
            Purge: Purge::<Impl, IMPL_OFFSET>,
            IsOpen2: IsOpen2::<Impl, IMPL_OFFSET>,
            ReceiveByLookupIdAllowPeek: ReceiveByLookupIdAllowPeek::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfoImpl: Sized + IDispatchImpl {
    fn QueueGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&mut self, bstrguidservicetype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsTransactional(&mut self) -> ::windows::core::Result<i16>;
    fn PrivLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&mut self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn Journal(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournal(&mut self, ljournal: i32) -> ::windows::core::Result<()>;
    fn Quota(&mut self) -> ::windows::core::Result<i32>;
    fn SetQuota(&mut self, lquota: i32) -> ::windows::core::Result<()>;
    fn BasePriority(&mut self) -> ::windows::core::Result<i32>;
    fn SetBasePriority(&mut self, lbasepriority: i32) -> ::windows::core::Result<()>;
    fn CreateTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ModifyTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Authenticate(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticate(&mut self, lauthenticate: i32) -> ::windows::core::Result<()>;
    fn JournalQuota(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournalQuota(&mut self, ljournalquota: i32) -> ::windows::core::Result<()>;
    fn IsWorldReadable(&mut self) -> ::windows::core::Result<i16>;
    fn Create(&mut self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Open(&mut self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfoVtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute_copy(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueueGuid: QueueGuid::<Impl, IMPL_OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Impl, IMPL_OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            PathName: PathName::<Impl, IMPL_OFFSET>,
            SetPathName: SetPathName::<Impl, IMPL_OFFSET>,
            FormatName: FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName: SetFormatName::<Impl, IMPL_OFFSET>,
            IsTransactional: IsTransactional::<Impl, IMPL_OFFSET>,
            PrivLevel: PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel: SetPrivLevel::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            SetJournal: SetJournal::<Impl, IMPL_OFFSET>,
            Quota: Quota::<Impl, IMPL_OFFSET>,
            SetQuota: SetQuota::<Impl, IMPL_OFFSET>,
            BasePriority: BasePriority::<Impl, IMPL_OFFSET>,
            SetBasePriority: SetBasePriority::<Impl, IMPL_OFFSET>,
            CreateTime: CreateTime::<Impl, IMPL_OFFSET>,
            ModifyTime: ModifyTime::<Impl, IMPL_OFFSET>,
            Authenticate: Authenticate::<Impl, IMPL_OFFSET>,
            SetAuthenticate: SetAuthenticate::<Impl, IMPL_OFFSET>,
            JournalQuota: JournalQuota::<Impl, IMPL_OFFSET>,
            SetJournalQuota: SetJournalQuota::<Impl, IMPL_OFFSET>,
            IsWorldReadable: IsWorldReadable::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo2Impl: Sized + IDispatchImpl {
    fn QueueGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&mut self, bstrguidservicetype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsTransactional(&mut self) -> ::windows::core::Result<i16>;
    fn PrivLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&mut self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn Journal(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournal(&mut self, ljournal: i32) -> ::windows::core::Result<()>;
    fn Quota(&mut self) -> ::windows::core::Result<i32>;
    fn SetQuota(&mut self, lquota: i32) -> ::windows::core::Result<()>;
    fn BasePriority(&mut self) -> ::windows::core::Result<i32>;
    fn SetBasePriority(&mut self, lbasepriority: i32) -> ::windows::core::Result<()>;
    fn CreateTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ModifyTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Authenticate(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticate(&mut self, lauthenticate: i32) -> ::windows::core::Result<()>;
    fn JournalQuota(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournalQuota(&mut self, ljournalquota: i32) -> ::windows::core::Result<()>;
    fn IsWorldReadable(&mut self) -> ::windows::core::Result<i16>;
    fn Create(&mut self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Open(&mut self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue2>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self) -> ::windows::core::Result<()>;
    fn PathNameDNS(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&mut self, varsecurity: super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo2Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute_copy(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&varsecurity)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueueGuid: QueueGuid::<Impl, IMPL_OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Impl, IMPL_OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            PathName: PathName::<Impl, IMPL_OFFSET>,
            SetPathName: SetPathName::<Impl, IMPL_OFFSET>,
            FormatName: FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName: SetFormatName::<Impl, IMPL_OFFSET>,
            IsTransactional: IsTransactional::<Impl, IMPL_OFFSET>,
            PrivLevel: PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel: SetPrivLevel::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            SetJournal: SetJournal::<Impl, IMPL_OFFSET>,
            Quota: Quota::<Impl, IMPL_OFFSET>,
            SetQuota: SetQuota::<Impl, IMPL_OFFSET>,
            BasePriority: BasePriority::<Impl, IMPL_OFFSET>,
            SetBasePriority: SetBasePriority::<Impl, IMPL_OFFSET>,
            CreateTime: CreateTime::<Impl, IMPL_OFFSET>,
            ModifyTime: ModifyTime::<Impl, IMPL_OFFSET>,
            Authenticate: Authenticate::<Impl, IMPL_OFFSET>,
            SetAuthenticate: SetAuthenticate::<Impl, IMPL_OFFSET>,
            JournalQuota: JournalQuota::<Impl, IMPL_OFFSET>,
            SetJournalQuota: SetJournalQuota::<Impl, IMPL_OFFSET>,
            IsWorldReadable: IsWorldReadable::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            PathNameDNS: PathNameDNS::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Security: Security::<Impl, IMPL_OFFSET>,
            SetSecurity: SetSecurity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo3Impl: Sized + IDispatchImpl {
    fn QueueGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&mut self, bstrguidservicetype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsTransactional(&mut self) -> ::windows::core::Result<i16>;
    fn PrivLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&mut self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn Journal(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournal(&mut self, ljournal: i32) -> ::windows::core::Result<()>;
    fn Quota(&mut self) -> ::windows::core::Result<i32>;
    fn SetQuota(&mut self, lquota: i32) -> ::windows::core::Result<()>;
    fn BasePriority(&mut self) -> ::windows::core::Result<i32>;
    fn SetBasePriority(&mut self, lbasepriority: i32) -> ::windows::core::Result<()>;
    fn CreateTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ModifyTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Authenticate(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticate(&mut self, lauthenticate: i32) -> ::windows::core::Result<()>;
    fn JournalQuota(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournalQuota(&mut self, ljournalquota: i32) -> ::windows::core::Result<()>;
    fn IsWorldReadable(&mut self) -> ::windows::core::Result<i16>;
    fn Create(&mut self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Open(&mut self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue3>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self) -> ::windows::core::Result<()>;
    fn PathNameDNS(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&mut self, varsecurity: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsTransactional2(&mut self) -> ::windows::core::Result<i16>;
    fn IsWorldReadable2(&mut self) -> ::windows::core::Result<i16>;
    fn MulticastAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMulticastAddress(&mut self, bstrmulticastaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ADsPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo3Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute_copy(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional2() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmulticastaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMulticastAddress(::core::mem::transmute_copy(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueueGuid: QueueGuid::<Impl, IMPL_OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Impl, IMPL_OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            PathName: PathName::<Impl, IMPL_OFFSET>,
            SetPathName: SetPathName::<Impl, IMPL_OFFSET>,
            FormatName: FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName: SetFormatName::<Impl, IMPL_OFFSET>,
            IsTransactional: IsTransactional::<Impl, IMPL_OFFSET>,
            PrivLevel: PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel: SetPrivLevel::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            SetJournal: SetJournal::<Impl, IMPL_OFFSET>,
            Quota: Quota::<Impl, IMPL_OFFSET>,
            SetQuota: SetQuota::<Impl, IMPL_OFFSET>,
            BasePriority: BasePriority::<Impl, IMPL_OFFSET>,
            SetBasePriority: SetBasePriority::<Impl, IMPL_OFFSET>,
            CreateTime: CreateTime::<Impl, IMPL_OFFSET>,
            ModifyTime: ModifyTime::<Impl, IMPL_OFFSET>,
            Authenticate: Authenticate::<Impl, IMPL_OFFSET>,
            SetAuthenticate: SetAuthenticate::<Impl, IMPL_OFFSET>,
            JournalQuota: JournalQuota::<Impl, IMPL_OFFSET>,
            SetJournalQuota: SetJournalQuota::<Impl, IMPL_OFFSET>,
            IsWorldReadable: IsWorldReadable::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            PathNameDNS: PathNameDNS::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Security: Security::<Impl, IMPL_OFFSET>,
            SetSecurity: SetSecurity::<Impl, IMPL_OFFSET>,
            IsTransactional2: IsTransactional2::<Impl, IMPL_OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Impl, IMPL_OFFSET>,
            MulticastAddress: MulticastAddress::<Impl, IMPL_OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Impl, IMPL_OFFSET>,
            ADsPath: ADsPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo4Impl: Sized + IDispatchImpl {
    fn QueueGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&mut self, bstrguidservicetype: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsTransactional(&mut self) -> ::windows::core::Result<i16>;
    fn PrivLevel(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&mut self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn Journal(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournal(&mut self, ljournal: i32) -> ::windows::core::Result<()>;
    fn Quota(&mut self) -> ::windows::core::Result<i32>;
    fn SetQuota(&mut self, lquota: i32) -> ::windows::core::Result<()>;
    fn BasePriority(&mut self) -> ::windows::core::Result<i32>;
    fn SetBasePriority(&mut self, lbasepriority: i32) -> ::windows::core::Result<()>;
    fn CreateTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ModifyTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Authenticate(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticate(&mut self, lauthenticate: i32) -> ::windows::core::Result<()>;
    fn JournalQuota(&mut self) -> ::windows::core::Result<i32>;
    fn SetJournalQuota(&mut self, ljournalquota: i32) -> ::windows::core::Result<()>;
    fn IsWorldReadable(&mut self) -> ::windows::core::Result<i16>;
    fn Create(&mut self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Open(&mut self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue4>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Update(&mut self) -> ::windows::core::Result<()>;
    fn PathNameDNS(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&mut self, varsecurity: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsTransactional2(&mut self) -> ::windows::core::Result<i16>;
    fn IsWorldReadable2(&mut self) -> ::windows::core::Result<i16>;
    fn MulticastAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMulticastAddress(&mut self, bstrmulticastaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ADsPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo4Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute_copy(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional2() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmulticastaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMulticastAddress(::core::mem::transmute_copy(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueueGuid: QueueGuid::<Impl, IMPL_OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Impl, IMPL_OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            PathName: PathName::<Impl, IMPL_OFFSET>,
            SetPathName: SetPathName::<Impl, IMPL_OFFSET>,
            FormatName: FormatName::<Impl, IMPL_OFFSET>,
            SetFormatName: SetFormatName::<Impl, IMPL_OFFSET>,
            IsTransactional: IsTransactional::<Impl, IMPL_OFFSET>,
            PrivLevel: PrivLevel::<Impl, IMPL_OFFSET>,
            SetPrivLevel: SetPrivLevel::<Impl, IMPL_OFFSET>,
            Journal: Journal::<Impl, IMPL_OFFSET>,
            SetJournal: SetJournal::<Impl, IMPL_OFFSET>,
            Quota: Quota::<Impl, IMPL_OFFSET>,
            SetQuota: SetQuota::<Impl, IMPL_OFFSET>,
            BasePriority: BasePriority::<Impl, IMPL_OFFSET>,
            SetBasePriority: SetBasePriority::<Impl, IMPL_OFFSET>,
            CreateTime: CreateTime::<Impl, IMPL_OFFSET>,
            ModifyTime: ModifyTime::<Impl, IMPL_OFFSET>,
            Authenticate: Authenticate::<Impl, IMPL_OFFSET>,
            SetAuthenticate: SetAuthenticate::<Impl, IMPL_OFFSET>,
            JournalQuota: JournalQuota::<Impl, IMPL_OFFSET>,
            SetJournalQuota: SetJournalQuota::<Impl, IMPL_OFFSET>,
            IsWorldReadable: IsWorldReadable::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            PathNameDNS: PathNameDNS::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Security: Security::<Impl, IMPL_OFFSET>,
            SetSecurity: SetSecurity::<Impl, IMPL_OFFSET>,
            IsTransactional2: IsTransactional2::<Impl, IMPL_OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Impl, IMPL_OFFSET>,
            MulticastAddress: MulticastAddress::<Impl, IMPL_OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Impl, IMPL_OFFSET>,
            ADsPath: ADsPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfosImpl: Sized + IDispatchImpl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfosVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfosImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfosVtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Reset: Reset::<Impl, IMPL_OFFSET>, Next: Next::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos2Impl: Sized + IDispatchImpl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos2Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos3Impl: Sized + IDispatchImpl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos3Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos4Impl: Sized + IDispatchImpl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos4Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueManagementImpl: Sized + IDispatchImpl + IMSMQManagementImpl {
    fn JournalMessageCount(&mut self) -> ::windows::core::Result<i32>;
    fn BytesInJournal(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn EodGetReceiveInfo(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueManagementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueManagementVtbl {
        unsafe extern "system" fn JournalMessageCount<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalMessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalmessagecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInJournal<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesInJournal() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbytesinjournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetReceiveInfo<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EodGetReceiveInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pvcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMSMQManagementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            JournalMessageCount: JournalMessageCount::<Impl, IMPL_OFFSET>,
            BytesInJournal: BytesInJournal::<Impl, IMPL_OFFSET>,
            EodGetReceiveInfo: EodGetReceiveInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionImpl: Sized + IDispatchImpl {
    fn Transaction(&mut self) -> ::windows::core::Result<i32>;
    fn Commit(&mut self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Abort(&mut self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionVtbl {
        unsafe extern "system" fn Transaction<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pltransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grftc), ::core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn Abort<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&fasync)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Transaction: Transaction::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction2Impl: Sized + IDispatchImpl + IMSMQTransactionImpl {
    fn InitNew(&mut self, vartransaction: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransaction2Vtbl {
        unsafe extern "system" fn InitNew<Impl: IMSMQTransaction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitNew(::core::mem::transmute_copy(&vartransaction)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransaction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMSMQTransactionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitNew: InitNew::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction3Impl: Sized + IDispatchImpl + IMSMQTransactionImpl + IMSMQTransaction2Impl {
    fn ITransaction(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransaction3Vtbl {
        unsafe extern "system" fn ITransaction<Impl: IMSMQTransaction3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ITransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaritransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IMSMQTransaction2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ITransaction: ITransaction::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenserImpl: Sized + IDispatchImpl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenserVtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser2Impl: Sized + IDispatchImpl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenser2Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser3Impl: Sized + IDispatchImpl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenser3Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
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
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_DMSMQEventEvents as ::windows::core::Interface>::IID
    }
}
