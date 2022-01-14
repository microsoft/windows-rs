#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication_Impl: Sized + super::Com::IDispatch_Impl {
    fn MachineIdOfMachineName(&mut self, machinename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplication_Vtbl {
        unsafe extern "system" fn MachineIdOfMachineName<Impl: IMSMQApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineIdOfMachineName(::core::mem::transmute_copy(&machinename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MachineIdOfMachineName: MachineIdOfMachineName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication2_Impl: Sized + super::Com::IDispatch_Impl + IMSMQApplication_Impl {
    fn RegisterCertificate(&mut self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MachineNameOfMachineId(&mut self, bstrguid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MSMQVersionMajor(&mut self) -> ::windows::core::Result<i16>;
    fn MSMQVersionMinor(&mut self) -> ::windows::core::Result<i16>;
    fn MSMQVersionBuild(&mut self) -> ::windows::core::Result<i16>;
    fn IsDsEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplication2_Vtbl {
        unsafe extern "system" fn RegisterCertificate<Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCertificate(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&externalcertificate)).into()
        }
        unsafe extern "system" fn MachineNameOfMachineId<Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineNameOfMachineId(::core::mem::transmute_copy(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachinename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMajor<Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionmajor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMinor<Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionMinor() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionminor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionBuild<Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDsEnabled<Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisdsenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IMSMQApplication_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQApplication3_Impl: Sized + super::Com::IDispatch_Impl + IMSMQApplication_Impl + IMSMQApplication2_Impl {
    fn ActiveQueues(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn PrivateQueues(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DirectoryServiceServer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsConnected(&mut self) -> ::windows::core::Result<i16>;
    fn BytesInAllQueues(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetMachine(&mut self, bstrmachine: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Machine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Connect(&mut self) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn Tidy(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQApplication3_Vtbl {
        unsafe extern "system" fn ActiveQueues<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvactivequeues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateQueues<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvprivatequeues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryServiceServer<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryServiceServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdirectoryserviceserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInAllQueues<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesInAllQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbytesinallqueues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachine<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMachine(::core::mem::transmute_copy(&bstrmachine)).into()
        }
        unsafe extern "system" fn Machine<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Machine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Tidy<Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Tidy().into()
        }
        Self {
            base: IMSMQApplication2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarret = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQCoordinatedTransactionDispenser_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser_Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser2_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser2_Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser3_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser3_Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQDestination_Impl: Sized + super::Com::IDispatch_Impl {
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn IADs(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_IADs(&mut self, piads: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ADsPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetADsPath(&mut self, bstradspath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Destinations(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_Destinations(&mut self, pdestinations: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQDestination_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQDestination_Vtbl {
        unsafe extern "system" fn Open<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IADs<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiads: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IADs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiads = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_IADs<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piads: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_IADs(::core::mem::transmute(&piads)).into()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsPath<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetADsPath(::core::mem::transmute_copy(&bstradspath)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn Destinations<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestinations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destinations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestinations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Destinations<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinations: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Destinations(::core::mem::transmute(&pdestinations)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQEvent_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQEvent_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent2_Impl: Sized + super::Com::IDispatch_Impl + IMSMQEvent_Impl {
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQEvent2_Vtbl {
        unsafe extern "system" fn Properties<Impl: IMSMQEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IMSMQEvent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Properties: Properties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent3_Impl: Sized + super::Com::IDispatch_Impl + IMSMQEvent_Impl + IMSMQEvent2_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQEvent3_Vtbl {
        Self { base: IMSMQEvent2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQManagement_Impl: Sized + super::Com::IDispatch_Impl {
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
impl IMSMQManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQManagement_Vtbl {
        unsafe extern "system" fn Init<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&machine), ::core::mem::transmute_copy(&pathname), ::core::mem::transmute_copy(&formatname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Machine<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Machine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageCount<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plmessagecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForeignStatus<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForeignStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *plforeignstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueType<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueType() {
                ::core::result::Result::Ok(ok__) => {
                    *plqueuetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *pfislocal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionalStatus<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionalStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pltransactionalstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInQueue<Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQMessage_Impl: Sized + super::Com::IDispatch_Impl {
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
    fn putref_ResponseQueueInfo(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&mut self) -> ::windows::core::Result<i32>;
    fn Body(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&mut self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&mut self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&mut self) -> ::windows::core::Result<i32>;
    fn SetAck(&mut self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetSenderCertificate(&mut self, varsendercert: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&mut self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&mut self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&mut self, destinationqueue: &::core::option::Option<IMSMQQueue>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage_Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute_copy(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute_copy(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQMessage2_Impl: Sized + super::Com::IDispatch_Impl {
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
    fn putref_ResponseQueueInfo_v1(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&mut self) -> ::windows::core::Result<i32>;
    fn Body(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&mut self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&mut self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&mut self) -> ::windows::core::Result<i32>;
    fn SetAck(&mut self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetSenderCertificate(&mut self, varsendercert: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&mut self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&mut self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&mut self, destinationqueue: &::core::option::Option<IMSMQQueue2>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&mut self) -> ::windows::core::Result<()>;
    fn SenderVersion(&mut self) -> ::windows::core::Result<i32>;
    fn Extension(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&mut self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&mut self, bstrguidconnectortype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn DestinationSymmetricKey(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&mut self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&mut self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&mut self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&mut self, bstrauthprovname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&mut self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&mut self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&mut self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage2_Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute_copy(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute_copy(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtension(::core::mem::transmute_copy(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute_copy(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute_copy(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignature(::core::mem::transmute_copy(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute_copy(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute_copy(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQMessage3_Impl: Sized + super::Com::IDispatch_Impl {
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
    fn putref_ResponseQueueInfo_v1(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&mut self) -> ::windows::core::Result<i32>;
    fn Body(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&mut self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&mut self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&mut self) -> ::windows::core::Result<i32>;
    fn SetAck(&mut self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetSenderCertificate(&mut self, varsendercert: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&mut self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&mut self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&mut self, destinationqueue: &::core::option::Option<super::Com::IDispatch>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&mut self) -> ::windows::core::Result<()>;
    fn SenderVersion(&mut self) -> ::windows::core::Result<i32>;
    fn Extension(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&mut self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&mut self, bstrguidconnectortype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn DestinationSymmetricKey(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&mut self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&mut self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&mut self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&mut self, bstrauthprovname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&mut self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&mut self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&mut self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo_v2(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v2(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn putref_ResponseQueueInfo(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo3>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn putref_AdminQueueInfo(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo3>) -> ::windows::core::Result<()>;
    fn ResponseDestination(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&mut self, pdestresponse: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Destination(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsAuthenticated2(&mut self) -> ::windows::core::Result<i16>;
    fn IsFirstInTransaction2(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction2(&mut self) -> ::windows::core::Result<i16>;
    fn AttachCurrentSecurityContext2(&mut self) -> ::windows::core::Result<()>;
    fn SoapEnvelope(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CompoundMessage(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSoapHeader(&mut self, bstrsoapheader: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSoapBody(&mut self, bstrsoapbody: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage3_Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute_copy(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute_copy(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtension(::core::mem::transmute_copy(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute_copy(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute_copy(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignature(::core::mem::transmute_copy(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute_copy(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute_copy(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v2(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v2(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *psreceivedauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseDestination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseDestination(::core::mem::transmute(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarlookupid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext2().into()
        }
        unsafe extern "system" fn SoapEnvelope<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoapEnvelope() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsoapenvelope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompoundMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcompoundmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoapHeader(::core::mem::transmute_copy(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoapBody(::core::mem::transmute_copy(&bstrsoapbody)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQMessage4_Impl: Sized + super::Com::IDispatch_Impl {
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
    fn putref_ResponseQueueInfo_v1(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&mut self) -> ::windows::core::Result<i32>;
    fn Body(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&mut self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&mut self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&mut self) -> ::windows::core::Result<i32>;
    fn SetAck(&mut self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetSenderCertificate(&mut self, varsendercert: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&mut self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&mut self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&mut self, destinationqueue: &::core::option::Option<super::Com::IDispatch>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&mut self) -> ::windows::core::Result<()>;
    fn SenderVersion(&mut self) -> ::windows::core::Result<i32>;
    fn Extension(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&mut self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&mut self, bstrguidconnectortype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn DestinationSymmetricKey(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&mut self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&mut self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&mut self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&mut self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&mut self, bstrauthprovname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&mut self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&mut self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&mut self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo_v2(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v2(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&mut self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn putref_ResponseQueueInfo(&mut self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo4>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn putref_AdminQueueInfo(&mut self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo4>) -> ::windows::core::Result<()>;
    fn ResponseDestination(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&mut self, pdestresponse: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Destination(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupId(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsAuthenticated2(&mut self) -> ::windows::core::Result<i16>;
    fn IsFirstInTransaction2(&mut self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction2(&mut self) -> ::windows::core::Result<i16>;
    fn AttachCurrentSecurityContext2(&mut self) -> ::windows::core::Result<()>;
    fn SoapEnvelope(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CompoundMessage(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSoapHeader(&mut self, bstrsoapheader: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSoapBody(&mut self, bstrsoapbody: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQMessage4_Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute_copy(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute_copy(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtension(::core::mem::transmute_copy(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute_copy(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute_copy(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignature(::core::mem::transmute_copy(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute_copy(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute_copy(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo_v2(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo_v2(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *psreceivedauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseDestination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_ResponseDestination(::core::mem::transmute(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarlookupid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachCurrentSecurityContext2().into()
        }
        unsafe extern "system" fn SoapEnvelope<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoapEnvelope() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsoapenvelope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompoundMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcompoundmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoapHeader(::core::mem::transmute_copy(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoapBody(::core::mem::transmute_copy(&bstrsoapbody)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQOutgoingQueueManagement_Impl: Sized + super::Com::IDispatch_Impl + IMSMQManagement_Impl {
    fn State(&mut self) -> ::windows::core::Result<i32>;
    fn NextHops(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn EodGetSendInfo(&mut self) -> ::windows::core::Result<IMSMQCollection>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn EodResend(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQOutgoingQueueManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQOutgoingQueueManagement_Vtbl {
        unsafe extern "system" fn State<Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *plstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextHops<Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextHops() {
                ::core::result::Result::Ok(ok__) => {
                    *pvnexthops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetSendInfo<Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EodGetSendInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Pause<Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn EodResend<Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EodResend().into()
        }
        Self {
            base: IMSMQManagement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQPrivateDestination_Impl: Sized + super::Com::IDispatch_Impl {
    fn Handle(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetHandle(&mut self, varhandle: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateDestination_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateDestination_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQPrivateDestination_Vtbl {
        unsafe extern "system" fn Handle<Impl: IMSMQPrivateDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandle<Impl: IMSMQPrivateDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandle(::core::mem::transmute_copy(&varhandle)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Handle: Handle::<Impl, IMPL_OFFSET>,
            SetHandle: SetHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateDestination as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Hwnd(&mut self) -> ::windows::core::Result<i32>;
    fn FireArrivedEvent(&mut self, pq: &::core::option::Option<IMSMQQueue>, msgcursor: i32) -> ::windows::core::Result<()>;
    fn FireArrivedErrorEvent(&mut self, pq: &::core::option::Option<IMSMQQueue>, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQPrivateEvent_Vtbl {
        unsafe extern "system" fn Hwnd<Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireArrivedEvent<Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FireArrivedEvent(::core::mem::transmute(&pq), ::core::mem::transmute_copy(&msgcursor)).into()
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FireArrivedErrorEvent(::core::mem::transmute(&pq), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&msgcursor)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQuery_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery_Vtbl {
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), LookupQueue: LookupQueue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery2_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery2_Vtbl {
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LookupQueue: LookupQueue::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery3_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupQueue(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery3_Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue_v2(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQuery4_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupQueue(&mut self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQuery4_Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue_v2(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueue_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&mut self) -> ::windows::core::Result<i32>;
    fn ShareMode(&mut self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn Handle(&mut self) -> ::windows::core::Result<i32>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Receive(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&mut self, event: &::core::option::Option<IMSMQEvent>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue_Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueue2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&mut self) -> ::windows::core::Result<i32>;
    fn ShareMode(&mut self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn Handle(&mut self) -> ::windows::core::Result<i32>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Receive_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&mut self, event: &::core::option::Option<IMSMQEvent2>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
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
impl IMSMQQueue2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue2_Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueue3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&mut self) -> ::windows::core::Result<i32>;
    fn ShareMode(&mut self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn Handle(&mut self) -> ::windows::core::Result<i32>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Receive_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&mut self, event: &::core::option::Option<IMSMQEvent3>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
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
    fn ReceiveByLookupId(&mut self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveNextByLookupId(&mut self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceivePreviousByLookupId(&mut self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveFirstByLookupId(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveLastByLookupId(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekByLookupId(&mut self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekNextByLookupId(&mut self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekPreviousByLookupId(&mut self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekFirstByLookupId(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekLastByLookupId(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn Purge(&mut self) -> ::windows::core::Result<()>;
    fn IsOpen2(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue3_Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle2() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveNextByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivePreviousByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveFirstByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveLastByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNextByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekPreviousByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekFirstByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekLastByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Purge().into()
        }
        unsafe extern "system" fn IsOpen2<Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueue4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&mut self) -> ::windows::core::Result<i32>;
    fn ShareMode(&mut self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn Handle(&mut self) -> ::windows::core::Result<i32>;
    fn IsOpen(&mut self) -> ::windows::core::Result<i16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Receive_v1(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&mut self, event: &::core::option::Option<IMSMQEvent3>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
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
    fn ReceiveByLookupId(&mut self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveNextByLookupId(&mut self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceivePreviousByLookupId(&mut self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveFirstByLookupId(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveLastByLookupId(&mut self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekByLookupId(&mut self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekNextByLookupId(&mut self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekPreviousByLookupId(&mut self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekFirstByLookupId(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekLastByLookupId(&mut self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn Purge(&mut self) -> ::windows::core::Result<()>;
    fn IsOpen2(&mut self) -> ::windows::core::Result<i16>;
    fn ReceiveByLookupIdAllowPeek(&mut self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueue4_Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle2() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveNextByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivePreviousByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveFirstByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveLastByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNextByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekPreviousByLookupId(::core::mem::transmute_copy(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekFirstByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekLastByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Purge().into()
        }
        unsafe extern "system" fn IsOpen2<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueueInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&mut self, bstrguidservicetype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
impl IMSMQQueueInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo_Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute_copy(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueueInfo2_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&mut self, bstrguidservicetype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetSecurity(&mut self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo2_Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute_copy(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&varsecurity)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueueInfo3_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&mut self, bstrguidservicetype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetSecurity(&mut self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsTransactional2(&mut self) -> ::windows::core::Result<i16>;
    fn IsWorldReadable2(&mut self) -> ::windows::core::Result<i16>;
    fn MulticastAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMulticastAddress(&mut self, bstrmulticastaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ADsPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo3_Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute_copy(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional2() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmulticastaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMulticastAddress(::core::mem::transmute_copy(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueueInfo4_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&mut self, bstrguidservicetype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&mut self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&mut self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&mut self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetSecurity(&mut self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsTransactional2(&mut self) -> ::windows::core::Result<i16>;
    fn IsWorldReadable2(&mut self) -> ::windows::core::Result<i16>;
    fn MulticastAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMulticastAddress(&mut self, bstrmulticastaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ADsPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfo4_Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute_copy(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(::core::mem::transmute_copy(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPathName(::core::mem::transmute_copy(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute_copy(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional2() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmulticastaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMulticastAddress(::core::mem::transmute_copy(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueueInfos_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<IMSMQQueueInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos_Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos2_Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueueInfos3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos3_Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueueInfos4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueInfos4_Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQQueueManagement_Impl: Sized + super::Com::IDispatch_Impl + IMSMQManagement_Impl {
    fn JournalMessageCount(&mut self) -> ::windows::core::Result<i32>;
    fn BytesInJournal(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn EodGetReceiveInfo(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueManagement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQQueueManagement_Vtbl {
        unsafe extern "system" fn JournalMessageCount<Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalMessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalmessagecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInJournal<Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesInJournal() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbytesinjournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetReceiveInfo<Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: IMSMQManagement_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQTransaction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Transaction(&mut self) -> ::windows::core::Result<i32>;
    fn Commit(&mut self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Abort(&mut self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransaction_Vtbl {
        unsafe extern "system" fn Transaction<Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pltransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grftc), ::core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn Abort<Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&fasync)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMSMQTransaction2_Impl: Sized + super::Com::IDispatch_Impl + IMSMQTransaction_Impl {
    fn InitNew(&mut self, vartransaction: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransaction2_Vtbl {
        unsafe extern "system" fn InitNew<Impl: IMSMQTransaction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitNew(::core::mem::transmute_copy(&vartransaction)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransaction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IMSMQTransaction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitNew: InitNew::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction3_Impl: Sized + super::Com::IDispatch_Impl + IMSMQTransaction_Impl + IMSMQTransaction2_Impl {
    fn ITransaction(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransaction3_Vtbl {
        unsafe extern "system" fn ITransaction<Impl: IMSMQTransaction3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ITransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaritransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IMSMQTransaction2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ITransaction: ITransaction::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenser_Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser2_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction2>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenser2_Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser3_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&mut self) -> ::windows::core::Result<IMSMQTransaction3>;
    fn Properties(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMSMQTransactionDispenser3_Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginTransaction: BeginTransaction::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _DMSMQEventEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _DMSMQEventEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _DMSMQEventEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _DMSMQEventEvents_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_DMSMQEventEvents as ::windows::core::Interface>::IID
    }
}
